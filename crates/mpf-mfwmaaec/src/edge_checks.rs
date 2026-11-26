//! Edge-case tests for the ported mfwmaaec logic.
//!
//! These cover boundary values and the branch order of `check_dmo_media_type` and
//! `output_buffer_size`. Expectations marked "not oracle-derived" come from reading the
//! C control flow, from hand arithmetic, or from an independent u64 model. The harness
//! did not print them.

use super::*;

/// Packed little-endian `WAVEFORMATEX`, 18 bytes.
fn format18(tag: u16, channels: u16, rate: u32, avg: u32, block: u16, bits: u16) -> [u8; 18] {
    let mut b = [0u8; 18];
    b[0..2].copy_from_slice(&tag.to_le_bytes());
    b[2..4].copy_from_slice(&channels.to_le_bytes());
    b[4..8].copy_from_slice(&rate.to_le_bytes());
    b[8..12].copy_from_slice(&avg.to_le_bytes());
    b[12..14].copy_from_slice(&block.to_le_bytes());
    b[14..16].copy_from_slice(&bits.to_le_bytes());
    b
}

/// A PCM mono 8 kHz 16-bit format that every check accepts.
fn valid() -> [u8; 18] {
    format18(1, 1, 8000, 16000, 2, 16)
}

fn media_type(format: &[u8]) -> DmoMediaType<'_> {
    DmoMediaType {
        majortype: MEDIATYPE_AUDIO,
        subtype: MEDIASUBTYPE_PCM,
        formattype: FORMAT_WAVEFORMATEX,
        format,
    }
}

fn check(format: &[u8]) -> Result<(), u32> {
    check_dmo_media_type(&media_type(format))
}

#[test]
fn valid_format_is_accepted() {
    assert_eq!(check(&valid()), Ok(()));
}

#[test]
fn rates_outside_supported_table_are_rejected() {
    // Not oracle-derived: neighbours of each supported_rates entry, plus the extremes.
    for rate in [
        0,
        1,
        7999,
        8001,
        11024,
        11026,
        16001,
        22049,
        22051,
        u32::MAX,
    ] {
        let f = format18(1, 1, rate, 1, 1, 8);
        assert_eq!(check(&f), Err(DMO_E_TYPE_NOT_ACCEPTED), "rate {rate}");
    }
}

#[test]
fn format_shorter_than_waveformatex_is_rejected() {
    // Not oracle-derived: cbFormat below 18 fails before any field is read.
    let f = valid();
    for len in 0..18 {
        assert_eq!(check(&f[..len]), Err(DMO_E_TYPE_NOT_ACCEPTED), "len {len}");
    }
}

#[test]
fn format_of_18_or_more_bytes_is_read_from_the_first_18() {
    // Not oracle-derived: the exact-size boundary. Trailing bytes are ignored.
    let mut long = valid().to_vec();
    long.extend_from_slice(&[0xff; 64]);
    assert_eq!(check(&valid()), Ok(()));
    assert_eq!(check(&long), Ok(()));
}

#[test]
fn wrong_guid_is_rejected_before_format_fields_are_read() {
    // Not oracle-derived: the GUID tests run before the zero-field test, so an
    // all-zero format block still gives NOT_ACCEPTED.
    let zeros = format18(0, 0, 0, 0, 0, 0);
    let other = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    let cases = [
        DmoMediaType {
            majortype: other,
            ..media_type(&zeros)
        },
        DmoMediaType {
            formattype: other,
            ..media_type(&zeros)
        },
        DmoMediaType {
            subtype: other,
            ..media_type(&zeros)
        },
    ];
    for mt in &cases {
        assert_eq!(check_dmo_media_type(mt), Err(DMO_E_TYPE_NOT_ACCEPTED));
    }
}

#[test]
fn zero_fields_are_checked_before_channels_and_tag() {
    // Not oracle-derived: INVALIDTYPE wins over NOT_ACCEPTED for channels and tag.
    assert_eq!(
        check(&format18(3, 2, 8000, 16000, 2, 0)),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check(&format18(0, 0, 8000, 0, 2, 16)),
        Err(DMO_E_INVALIDTYPE)
    );
    assert_eq!(
        check(&format18(0, 0, 8000, 16000, 0, 16)),
        Err(DMO_E_INVALIDTYPE)
    );
}

#[test]
fn channels_and_format_tag_must_be_mono_pcm() {
    // Not oracle-derived: boundary values for nChannels and wFormatTag.
    let cases = [
        format18(1, 0, 8000, 16000, 2, 16),
        format18(1, 2, 8000, 16000, 2, 16),
        format18(0, 1, 8000, 16000, 2, 16),
        format18(2, 1, 8000, 16000, 2, 16),
        format18(0xFFFF, 1, 8000, 16000, 2, 16),
    ];
    for f in &cases {
        assert_eq!(check(f), Err(DMO_E_TYPE_NOT_ACCEPTED));
    }
}

#[test]
fn block_align_is_not_cross_checked_against_bits() {
    // Not oracle-derived: the C check never compares nBlockAlign or nAvgBytesPerSec
    // with the rate or wBitsPerSample. This is bug-compatible: such formats are accepted.
    assert_eq!(check(&format18(1, 1, 8000, 16000, 7, 16)), Ok(()));
    assert_eq!(check(&format18(1, 1, 8000, 1, 1, 1)), Ok(()));
    assert_eq!(check(&format18(1, 1, 8000, 1, 1, u16::MAX)), Ok(()));
}

#[test]
fn output_buffer_size_matches_u64_model_with_32_bit_wrap() {
    // Not oracle-derived: an independent model. The product is reduced mod 2^32
    // using u64 arithmetic, then divided by 8000.
    let rates = [
        0u32,
        1,
        8000,
        11025,
        22050,
        0x1000_0000,
        0x8000_0000,
        u32::MAX,
    ];
    let bits = [0u16, 1, 8, 16, 24, u16::MAX];
    for rate in rates {
        for b in bits {
            let product = 40u64 * u64::from(rate) * u64::from(b);
            let expect = (product % (1u64 << 32)) / 8000;
            assert_eq!(
                output_buffer_size(rate, b),
                expect as u32,
                "rate {rate} bits {b}"
            );
        }
    }
}

#[test]
fn output_buffer_size_small_and_exact_values() {
    // Not oracle-derived: hand-computed from 40 * rate * bits / 8000.
    assert_eq!(output_buffer_size(0, 16), 0);
    assert_eq!(output_buffer_size(1, 1), 0);
    assert_eq!(output_buffer_size(200, 1), 1);
    assert_eq!(output_buffer_size(8000, 1), 40);
    assert_eq!(output_buffer_size(8000, 8), 320);
}
