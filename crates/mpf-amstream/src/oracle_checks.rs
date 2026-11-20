//! Checks the port against rows recorded from the verbatim Wine C code (`dlls/amstream`: the
//! format check of `IAudioDataImpl_SetFormat` in `audiodata.c`, the media-type checks of
//! `audio_sink_ReceiveConnection` in `audiostream.c`, and `subtype_from_pf`,
//! `is_format_compatible` and `ddraw_sink_QueryAccept` in `ddrawstream.c`), built with ASan and
//! UBSan, with no reports. Each row is `(inputs, expected)`, with the expected value a
//! hex HRESULT, a subtype name (`none` when Wine leaves the subtype unset), or a bool.

use crate::audio::{check_receive_media_type, check_set_format, MediaType};
use crate::video::{
    is_format_compatible, query_accept, subtype_from_pixel_format, PixelFormat, StreamFormat,
    VideoMediaType,
};
use crate::{
    Guid, FORMAT_VIDEOINFO, FORMAT_WAVEFORMATEX, MEDIASUBTYPE_RGB24, MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB555, MEDIASUBTYPE_RGB565, MEDIASUBTYPE_RGB8, MEDIATYPE_AUDIO, MEDIATYPE_VIDEO,
};

const GUIDS: &[(&str, Guid)] = &[
    ("AUDIO", MEDIATYPE_AUDIO),
    ("VIDEO", MEDIATYPE_VIDEO),
    ("WAVE", FORMAT_WAVEFORMATEX),
    ("VIDEOINFO", FORMAT_VIDEOINFO),
    ("RGB8", MEDIASUBTYPE_RGB8),
    ("RGB555", MEDIASUBTYPE_RGB555),
    ("RGB565", MEDIASUBTYPE_RGB565),
    ("RGB24", MEDIASUBTYPE_RGB24),
    ("RGB32", MEDIASUBTYPE_RGB32),
    (
        "ARGB32",
        Guid {
            data1: 0x773c_9ac0,
            data2: 0x3274,
            data3: 0x11d0,
            data4: [0xb7, 0x24, 0x00, 0xaa, 0x00, 0x6c, 0x1a, 0x01],
        },
    ),
];

fn guid(name: &str) -> Guid {
    GUIDS.iter().find(|(n, _)| *n == name).unwrap().1
}

fn subtype_name(subtype: Option<Guid>) -> &'static str {
    match subtype {
        None => "none",
        Some(subtype) => GUIDS.iter().find(|(_, g)| *g == subtype).unwrap().0,
    }
}

#[rustfmt::skip]
const PIXEL_FORMAT_ROWS: &[(u32, u32, u32, &str)] = &[
    (0x40, 16, 0x7c00, "RGB555"),
    (0x40, 16, 0xf800, "RGB565"),
    (0x40, 16, 0x1f, "none"),
    (0x40, 24, 0xff0000, "RGB24"),
    (0x60, 8, 0x0, "RGB8"),
    (0x40, 8, 0x0, "none"),
];

type QueryAcceptRow = (
    &'static str,
    &'static str,
    &'static str,
    i32,
    u32,
    (u32, u32, u32, u32),
    bool,
    &'static str,
);

#[rustfmt::skip]
const QUERY_ACCEPT_ROWS: &[QueryAcceptRow] = &[
    ("VIDEO", "VIDEOINFO", "RGB8", 240, 0x0, (0x0, 0, 0x0, 0x0), false, "00000000"),
    ("VIDEO", "VIDEOINFO", "RGB32", 240, 0x0, (0x0, 0, 0x0, 0x0), true, "00000000"),
    ("VIDEO", "VIDEOINFO", "RGB32", -240, 0x0, (0x0, 0, 0x0, 0x0), true, "8004022a"),
    ("AUDIO", "VIDEOINFO", "RGB32", 240, 0x0, (0x0, 0, 0x0, 0x0), true, "8004022a"),
    ("VIDEO", "VIDEOINFO", "RGB555", 240, 0x1000, (0x40, 16, 0x7c00, 0x3e0), false, "00000000"),
    ("VIDEO", "VIDEOINFO", "RGB565", 240, 0x1000, (0x40, 16, 0x7c00, 0x3e0), true, "8004022a"),
    ("VIDEO", "VIDEOINFO", "ARGB32", 240, 0x0, (0x0, 0, 0x0, 0x0), true, "8004022a"),
];

type CompatibilityRow = (
    u32,
    (u32, u32),
    (u32, u32, u32),
    (u32, u32),
    (u32, u32, u32),
    bool,
);

#[rustfmt::skip]
const COMPAT_ROWS: &[CompatibilityRow] = &[
    (0x0, (0, 0), (0x0, 0, 0x0), (0, 0), (0x40, 16, 0x0), true),
    (0x2, (333, 444), (0x0, 0, 0x0), (332, 444), (0x40, 16, 0x0), false),
    (0x1000, (0, 0), (0x44, 32, 0x0), (0, 0), (0x40, 32, 0x0), false),
    (0x1000, (0, 0), (0x40, 16, 0x3e0), (0, 0), (0x40, 16, 0x7e0), false),
    (0x1000, (0, 0), (0x40, 16, 0x3e0), (0, 0), (0x40, 16, 0x3e0), true),
];

#[rustfmt::skip]
const AUDIO_ROWS: &[(&str, &str, u32, u16, &str)] = &[
    ("AUDIO", "WAVE", 18, 1, "00000000"),
    ("AUDIO", "WAVE", 18, 2, "80070057"),
    ("AUDIO", "WAVE", 17, 1, "8004022a"),
];

#[rustfmt::skip]
const SET_FORMAT_ROWS: &[(Option<u16>, &str)] = &[
    (Some(1), "00000000"),
    (Some(2), "80070057"),
    (None, "80004003"),
];

#[test]
fn oracle_pixel_formats_match_wine() {
    for &(flags, rgb_bit_count, r_mask, expected) in PIXEL_FORMAT_ROWS {
        let pf = PixelFormat {
            flags,
            rgb_bit_count,
            r_mask,
            g_mask: 0,
        };
        assert_eq!(
            subtype_name(subtype_from_pixel_format(&pf)),
            expected,
            "pf {flags:#x} {rgb_bit_count} {r_mask:#x}"
        );
    }
}

#[test]
fn oracle_query_accept_matches_wine() {
    for &(major, format_type, subtype, bi_height, fmt_flags, pf, connected, expected) in
        QUERY_ACCEPT_ROWS
    {
        let mt = VideoMediaType {
            majortype: guid(major),
            formattype: guid(format_type),
            subtype: guid(subtype),
            bi_height,
        };
        let (pf_flags, rgb_bit_count, r_mask, g_mask) = pf;
        let format = StreamFormat {
            flags: fmt_flags,
            width: 0,
            height: 0,
            pf: PixelFormat {
                flags: pf_flags,
                rgb_bit_count,
                r_mask,
                g_mask,
            },
        };
        assert_eq!(
            format!("{:08x}", query_accept(Some(&mt), &format, connected)),
            expected,
            "qa {major} {format_type} {subtype} {bi_height} {fmt_flags:#x} {connected}"
        );
    }
}

#[test]
fn oracle_format_compatibility_matches_wine() {
    for &(fmt_flags, (width_fmt, height_fmt), pf, (width, height), conn, expected) in COMPAT_ROWS {
        let (pf_flags, rgb_bit_count, g_mask) = pf;
        let (conn_flags, conn_bits, conn_g_mask) = conn;
        let format = StreamFormat {
            flags: fmt_flags,
            width: width_fmt,
            height: height_fmt,
            pf: PixelFormat {
                flags: pf_flags,
                rgb_bit_count,
                r_mask: 0,
                g_mask,
            },
        };
        let connection_pf = PixelFormat {
            flags: conn_flags,
            rgb_bit_count: conn_bits,
            r_mask: 0,
            g_mask: conn_g_mask,
        };
        assert_eq!(
            is_format_compatible(&format, width, height, &connection_pf),
            expected,
            "compat {fmt_flags:#x} {width_fmt}x{height_fmt} {width}x{height}"
        );
    }
}

#[test]
fn oracle_audio_receive_matches_wine() {
    for &(major, format_type, cb_format, format_tag, expected) in AUDIO_ROWS {
        let mt = MediaType {
            majortype: guid(major),
            formattype: guid(format_type),
            cb_format,
            format_tag,
        };
        assert_eq!(
            format!("{:08x}", check_receive_media_type(&mt)),
            expected,
            "audio {major} {format_type} {cb_format} {format_tag}"
        );
    }
}

#[test]
fn oracle_set_format_matches_wine() {
    for &(format_tag, expected) in SET_FORMAT_ROWS {
        assert_eq!(
            format!("{:08x}", check_set_format(format_tag)),
            expected,
            "set_format {format_tag:?}"
        );
    }
}
