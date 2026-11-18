//! Rust-only edge cases: the refusals that replace Wine's out-of-bounds reads, the arms the oracle
//! scripts do not reach, and the stored state around them.

use super::oracle_checks::replay;
use crate::effect::{TypeState, DMO_SET_TYPEF_CLEAR};
use crate::format::{WaveFormat, WAVE_FORMAT_EX_SIZE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM};
use crate::media::{check_input_type, check_output_type, FormatKind, Major, MediaType, Subtype};
use crate::Error;

/// The 18 little-endian bytes of a `WAVEFORMATEX` with the given fields and a zero `cbSize`.
fn header(tag: u16, channels: u16, rate: u32, block_align: u16, bits: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&tag.to_le_bytes());
    bytes.extend_from_slice(&channels.to_le_bytes());
    bytes.extend_from_slice(&rate.to_le_bytes());
    bytes.extend_from_slice(&(rate * u32::from(block_align)).to_le_bytes());
    bytes.extend_from_slice(&block_align.to_le_bytes());
    bytes.extend_from_slice(&bits.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes
}

/// An audio `WAVEFORMATEX` media type of the given subtype over `format`.
fn media(subtype: Subtype, format: &[u8]) -> MediaType<'_> {
    MediaType {
        major: Major::Audio,
        subtype,
        format_kind: FormatKind::WaveFormatEx,
        format,
    }
}

#[test]
fn header_reads_the_packed_fields_and_ignores_the_tail() {
    let mut bytes = header(0xfffe, 2, 48000, 4, 16);
    bytes[16..18].copy_from_slice(&22u16.to_le_bytes());
    bytes.extend_from_slice(&[0xaa; 22]);

    let format = WaveFormat::parse(&bytes).expect("a full header");
    assert_eq!(format.tag, 0xfffe);
    assert_eq!(format.channels, 2);
    assert_eq!(format.samples_per_sec, 48000);
    assert_eq!(format.avg_bytes_per_sec, 192000);
    assert_eq!(format.block_align, 4);
    assert_eq!(format.bits_per_sample, 16);
    assert_eq!(format.cb_size, 22);
    assert_eq!(format.to_bytes().as_slice(), &bytes[..WAVE_FORMAT_EX_SIZE]);
}

#[test]
fn default_header_is_all_zero_bytes() {
    assert_eq!(WaveFormat::default().to_bytes(), [0; WAVE_FORMAT_EX_SIZE]);
    assert_eq!(TypeState::default().format(), WaveFormat::default());
}

#[test]
fn short_buffers_are_refused_by_both_setters() {
    let full = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    for len in [0, 1, WAVE_FORMAT_EX_SIZE - 1] {
        let short = &full[..len];
        assert_eq!(
            check_input_type(&media(Subtype::Pcm, short)),
            Err(Error::TypeNotAccepted),
            "input, {len} bytes"
        );
        assert_eq!(
            check_output_type(&media(Subtype::Pcm, short), WaveFormat::default()),
            Err(Error::TypeNotAccepted),
            "output, {len} bytes"
        );
    }
}

#[test]
fn clear_flag_does_not_read_the_buffer() {
    let mut state = TypeState::default();
    assert_eq!(
        state.set_input_type(DMO_SET_TYPEF_CLEAR, &media(Subtype::Pcm, &[])),
        Ok(())
    );
    assert_eq!(
        state.set_output_type(DMO_SET_TYPEF_CLEAR, &media(Subtype::Pcm, &[])),
        Ok(())
    );
}

#[test]
fn bit_depth_is_checked_against_the_tag() {
    for bits in 0..=64 {
        let pcm = header(WAVE_FORMAT_PCM, 2, 48000, 4, bits);
        assert_eq!(
            check_input_type(&media(Subtype::Pcm, &pcm)).is_ok(),
            matches!(bits, 8 | 16),
            "PCM at {bits} bits"
        );
        let float = header(WAVE_FORMAT_IEEE_FLOAT, 2, 48000, 8, bits);
        assert_eq!(
            check_input_type(&media(Subtype::Float, &float)).is_ok(),
            bits == 32,
            "float at {bits} bits"
        );
    }
}

#[test]
fn other_tags_are_refused_at_every_depth() {
    for tag in [0, 2, 6, 7, 0x0055, 0xfffe, 0xffff] {
        for bits in [8, 16, 32] {
            let format = header(tag, 2, 48000, 4, bits);
            assert_eq!(
                check_input_type(&media(Subtype::Pcm, &format)),
                Err(Error::TypeNotAccepted),
                "tag {tag:#x} at {bits} bits"
            );
        }
    }
}

#[test]
fn channel_count_must_be_one_or_two() {
    for channels in [0, 1, 2, 3, 6, 0xffff] {
        let format = header(WAVE_FORMAT_PCM, channels, 48000, 4, 16);
        assert_eq!(
            check_input_type(&media(Subtype::Pcm, &format)).is_ok(),
            matches!(channels, 1 | 2),
            "{channels} channels"
        );
    }
}

#[test]
fn subtype_is_not_matched_against_the_tag() {
    let float_tag = header(WAVE_FORMAT_IEEE_FLOAT, 2, 48000, 8, 32);
    assert!(check_input_type(&media(Subtype::Pcm, &float_tag)).is_ok());
    let pcm_tag = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    assert!(check_input_type(&media(Subtype::Float, &pcm_tag)).is_ok());
}

#[test]
fn each_check_refuses_on_its_own() {
    let format = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    let stored = WaveFormat::parse(&format).expect("a full header");
    let cases = [
        (Major::Other, Subtype::Pcm, FormatKind::WaveFormatEx),
        (Major::Audio, Subtype::Other, FormatKind::WaveFormatEx),
        (Major::Audio, Subtype::Pcm, FormatKind::Other),
    ];
    for (major, subtype, format_kind) in cases {
        let media = MediaType {
            major,
            subtype,
            format_kind,
            format: &format,
        };
        assert_eq!(check_input_type(&media), Err(Error::TypeNotAccepted));
        assert_eq!(
            check_output_type(&media, stored),
            Err(Error::TypeNotAccepted)
        );
    }
}

#[test]
fn output_compares_every_header_byte_and_ignores_the_tail() {
    let stored = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    let mut state = TypeState::default();
    state
        .set_input_type(0, &media(Subtype::Pcm, &stored))
        .expect("the stored type is accepted");

    assert_eq!(
        state.set_output_type(0, &media(Subtype::Pcm, &stored)),
        Ok(())
    );
    let mut longer = stored.clone();
    longer.extend_from_slice(&[0xee; 22]);
    assert_eq!(
        state.set_output_type(0, &media(Subtype::Pcm, &longer)),
        Ok(())
    );

    for index in 0..WAVE_FORMAT_EX_SIZE {
        let mut other = stored.clone();
        other[index] ^= 1;
        assert_eq!(
            state.set_output_type(0, &media(Subtype::Pcm, &other)),
            Err(Error::TypeNotAccepted),
            "byte {index}"
        );
    }
}

#[test]
fn refused_input_keeps_the_stored_header() {
    let pcm = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    let mut state = TypeState::default();
    state
        .set_input_type(0, &media(Subtype::Pcm, &pcm))
        .expect("the PCM type is accepted");
    assert_eq!(
        state.set_input_type(1, &media(Subtype::Pcm, &[])),
        Err(Error::TypeNotAccepted)
    );
    assert_eq!(
        state.format(),
        WaveFormat::parse(&pcm).expect("a full header")
    );
}

#[test]
fn test_only_flag_does_not_clear() {
    let pcm = header(WAVE_FORMAT_PCM, 2, 48000, 4, 16);
    let mut state = TypeState::default();
    state
        .set_input_type(0, &media(Subtype::Pcm, &pcm))
        .expect("the PCM type is accepted");

    state
        .set_input_type(0x1, &media(Subtype::Pcm, &pcm))
        .expect("TEST_ONLY still checks and stores");
    assert_eq!(
        state.format(),
        WaveFormat::parse(&pcm).expect("a full header")
    );

    state
        .set_input_type(0x3, &media(Subtype::Pcm, &[]))
        .expect("CLEAR wins over TEST_ONLY");
    assert_eq!(state.format(), WaveFormat::default());
}

#[test]
fn output_clear_accepts_any_type_but_test_only_checks() {
    let state = TypeState::default();
    let other = MediaType {
        major: Major::Other,
        subtype: Subtype::Other,
        format_kind: FormatKind::Other,
        format: &[],
    };
    assert_eq!(state.set_output_type(DMO_SET_TYPEF_CLEAR, &other), Ok(()));
    assert_eq!(
        state.set_output_type(0x1, &other),
        Err(Error::TypeNotAccepted)
    );
}

#[test]
fn error_is_dmo_e_type_not_accepted() {
    assert_eq!(Error::TypeNotAccepted.hresult(), 0x8004_0205);
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn unknown_oracle_command_panics() {
    replay("bogus 0");
}
