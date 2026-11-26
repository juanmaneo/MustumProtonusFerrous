//! Oracle-derived test vectors for the ported mfwmaaec logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `check_dmo_media_type` and the `cbSize` expression in `media_object_SetOutputType`
//! (`dlls/mfwmaaec/mfwmaaec.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/mfwmaaec/`. The harness printed 24 rows: 19 media-type checks and
//! 5 output-size rows. ASan and UBSan reported nothing, so no row is excluded.

use super::*;

/// The all-zero GUID the harness uses as a mismatching value.
const OTHER: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// Builds `cb_format` bytes of `WAVEFORMATEX` fields, the first 18 bytes packed
/// little-endian. Bytes past 18 are zero, and a shorter buffer is truncated, as in the harness `memcpy`.
fn wfx(
    tag: u16,
    channels: u16,
    rate: u32,
    avg: u32,
    block: u16,
    bits: u16,
    cb_format: usize,
) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&tag.to_le_bytes());
    b.extend_from_slice(&channels.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&avg.to_le_bytes());
    b.extend_from_slice(&block.to_le_bytes());
    b.extend_from_slice(&bits.to_le_bytes());
    b.extend_from_slice(&0u16.to_le_bytes());
    b.resize(cb_format, 0);
    b
}

/// `(name, major_ok, format_ok, subtype_ok, cb_format, tag, channels, rate, avg, block, bits, hresult)`.
type CheckRow = (
    &'static str,
    bool,
    bool,
    bool,
    usize,
    u16,
    u16,
    u32,
    u32,
    u16,
    u16,
    u32,
);

const CHECK_ROWS: &[CheckRow] = &[
    (
        "ok_8000_mono_16",
        true,
        true,
        true,
        18,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x0000_0000,
    ),
    (
        "ok_11025_mono_8",
        true,
        true,
        true,
        18,
        1,
        1,
        11025,
        11025,
        1,
        8,
        0x0000_0000,
    ),
    (
        "ok_16000_mono_16",
        true,
        true,
        true,
        18,
        1,
        1,
        16000,
        32000,
        2,
        16,
        0x0000_0000,
    ),
    (
        "ok_22050_mono_16",
        true,
        true,
        true,
        18,
        1,
        1,
        22050,
        44100,
        2,
        16,
        0x0000_0000,
    ),
    (
        "ok_cb40_extra_bytes",
        true,
        true,
        true,
        40,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x0000_0000,
    ),
    (
        "bad_major",
        false,
        true,
        true,
        18,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x8004_0205,
    ),
    (
        "bad_formattype",
        true,
        false,
        true,
        18,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x8004_0205,
    ),
    (
        "short_format_cb17",
        true,
        true,
        true,
        17,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x8004_0205,
    ),
    (
        "bad_subtype",
        true,
        true,
        false,
        18,
        1,
        1,
        8000,
        16000,
        2,
        16,
        0x8004_0205,
    ),
    (
        "bad_major_zero_bits",
        false,
        true,
        true,
        18,
        1,
        1,
        8000,
        16000,
        2,
        0,
        0x8004_0205,
    ),
    (
        "zero_bits",
        true,
        true,
        true,
        18,
        1,
        1,
        8000,
        16000,
        2,
        0,
        0x8004_0202,
    ),
    (
        "zero_avg",
        true,
        true,
        true,
        18,
        1,
        1,
        8000,
        0,
        2,
        16,
        0x8004_0202,
    ),
    (
        "zero_block",
        true,
        true,
        true,
        18,
        1,
        1,
        8000,
        16000,
        0,
        16,
        0x8004_0202,
    ),
    (
        "zero_bits_stereo",
        true,
        true,
        true,
        18,
        1,
        2,
        8000,
        16000,
        2,
        0,
        0x8004_0202,
    ),
    (
        "stereo_16bit",
        true,
        true,
        true,
        18,
        1,
        2,
        8000,
        32000,
        4,
        16,
        0x8004_0205,
    ),
    (
        "float_tag_3",
        true,
        true,
        true,
        18,
        3,
        1,
        8000,
        16000,
        2,
        16,
        0x8004_0205,
    ),
    (
        "float_tag_zero_bits",
        true,
        true,
        true,
        18,
        3,
        1,
        8000,
        16000,
        2,
        0,
        0x8004_0202,
    ),
    (
        "rate_44100",
        true,
        true,
        true,
        18,
        1,
        1,
        44100,
        88200,
        2,
        16,
        0x8004_0205,
    ),
    (
        "rate_8001",
        true,
        true,
        true,
        18,
        1,
        1,
        8001,
        16002,
        2,
        16,
        0x8004_0205,
    ),
];

/// `(name, samples_per_sec, bits_per_sample, cbSize)`.
const CB_ROWS: &[(&str, u32, u16, u32)] = &[
    ("cb_8000_16", 8000, 16, 640),
    ("cb_22050_16", 22050, 16, 1764),
    ("cb_11025_8", 11025, 8, 441),
    ("cb_wrap_ffffffff_16", 0xFFFF_FFFF, 16, 536_870),
    ("cb_wrap_2pow28_16", 0x1000_0000, 16, 0),
];

#[test]
fn oracle_check_dmo_media_type_rows() {
    for &(name, major, format, subtype, cb_format, tag, channels, rate, avg, block, bits, hr) in
        CHECK_ROWS
    {
        let bytes = wfx(tag, channels, rate, avg, block, bits, cb_format);
        let media_type = DmoMediaType {
            majortype: if major { MEDIATYPE_AUDIO } else { OTHER },
            subtype: if subtype { MEDIASUBTYPE_PCM } else { OTHER },
            formattype: if format { FORMAT_WAVEFORMATEX } else { OTHER },
            format: &bytes,
        };
        let got = check_dmo_media_type(&media_type).err().unwrap_or(0);
        assert_eq!(got, hr, "row {name}");
    }
}

#[test]
fn oracle_output_buffer_size_rows() {
    for &(name, samples_per_sec, bits, cb_size) in CB_ROWS {
        assert_eq!(
            output_buffer_size(samples_per_sec, bits),
            cb_size,
            "row {name}"
        );
    }
}
