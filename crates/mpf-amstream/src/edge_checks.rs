//! Edge cases of the ported checks that the oracle rows do not cover: the remaining subtype
//! mappings, the FOURCC and bit-count branches of the compatibility check, and the null and
//! unknown-subtype paths of the video accept decision.

use crate::audio::{check_receive_media_type, check_set_format, MediaType, WAVEFORMATEX_SIZE};
use crate::video::{
    is_format_compatible, query_accept, subtype_from_pixel_format, PixelFormat, StreamFormat,
    VideoMediaType, DDPF_FOURCC, DDPF_PALETTEINDEXED8, DDPF_RGB, DDSD_HEIGHT, DDSD_PIXELFORMAT,
};
use crate::*;

fn pf(flags: u32, rgb_bit_count: u32, r_mask: u32, g_mask: u32) -> PixelFormat {
    PixelFormat {
        flags,
        rgb_bit_count,
        r_mask,
        g_mask,
    }
}

#[test]
fn subtype_mapping_covers_every_supported_format() {
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB, 16, 0x7c00, 0x03e0)),
        Some(MEDIASUBTYPE_RGB555)
    );
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB, 16, 0xf800, 0x07e0)),
        Some(MEDIASUBTYPE_RGB565)
    );
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB, 24, 0xff0000, 0x00ff00)),
        Some(MEDIASUBTYPE_RGB24)
    );
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB, 32, 0xff0000, 0x00ff00)),
        Some(MEDIASUBTYPE_RGB32)
    );
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB | DDPF_PALETTEINDEXED8, 8, 0, 0)),
        Some(MEDIASUBTYPE_RGB8)
    );
    // An 8-bit format without the palette flag, and unknown bit counts, are unmapped.
    assert_eq!(subtype_from_pixel_format(&pf(DDPF_RGB, 8, 0, 0)), None);
    assert_eq!(subtype_from_pixel_format(&pf(DDPF_RGB, 4, 0, 0)), None);
    // The bit count decides before the palette flag is consulted.
    assert_eq!(
        subtype_from_pixel_format(&pf(DDPF_RGB | DDPF_PALETTEINDEXED8, 16, 0x7c00, 0x03e0)),
        Some(MEDIASUBTYPE_RGB555)
    );
}

#[test]
fn compatibility_checks_dimensions_and_pixel_format() {
    let fmt = |flags: u32, pf: PixelFormat| StreamFormat {
        flags,
        width: 320,
        height: 240,
        pf,
    };
    let conn = pf(DDPF_RGB, 32, 0xff0000, 0x00ff00);

    // Without DDSD_HEIGHT the dimensions are not checked.
    assert!(is_format_compatible(&fmt(0, pf(0, 0, 0, 0)), 1, 1, &conn));
    // With DDSD_HEIGHT they must match.
    assert!(is_format_compatible(
        &fmt(DDSD_HEIGHT, pf(0, 0, 0, 0)),
        320,
        240,
        &conn
    ));
    assert!(!is_format_compatible(
        &fmt(DDSD_HEIGHT, pf(0, 0, 0, 0)),
        321,
        240,
        &conn
    ));
    assert!(!is_format_compatible(
        &fmt(DDSD_HEIGHT, pf(0, 0, 0, 0)),
        320,
        241,
        &conn
    ));

    // With DDSD_PIXELFORMAT a FOURCC connection format is refused outright.
    assert!(!is_format_compatible(
        &fmt(DDSD_PIXELFORMAT, pf(DDPF_FOURCC, 0, 0, 0)),
        320,
        240,
        &conn
    ));
    // A bit count mismatch is refused, and a 16-bit format also compares the green mask.
    assert!(!is_format_compatible(
        &fmt(DDSD_PIXELFORMAT, pf(DDPF_RGB, 16, 0, 0x03e0)),
        320,
        240,
        &pf(DDPF_RGB, 32, 0, 0x03e0)
    ));
    assert!(!is_format_compatible(
        &fmt(DDSD_PIXELFORMAT, pf(DDPF_RGB, 16, 0, 0x03e0)),
        320,
        240,
        &pf(DDPF_RGB, 16, 0, 0x07e0)
    ));
    assert!(is_format_compatible(
        &fmt(DDSD_PIXELFORMAT, pf(DDPF_RGB, 16, 0, 0x03e0)),
        320,
        240,
        &pf(DDPF_RGB, 16, 0, 0x03e0)
    ));
    // A 32-bit format does not compare the green mask.
    assert!(is_format_compatible(
        &fmt(DDSD_PIXELFORMAT, pf(DDPF_RGB, 32, 0, 0x00ff00)),
        320,
        240,
        &pf(DDPF_RGB, 32, 0, 0x000000)
    ));
}

#[test]
fn query_accept_rejects_null_and_unknown_subtypes() {
    let format = StreamFormat::default();
    assert_eq!(query_accept(None, &format, false), E_POINTER);

    let mt = |subtype: Guid| VideoMediaType {
        majortype: MEDIATYPE_VIDEO,
        formattype: FORMAT_VIDEOINFO,
        subtype,
        bi_height: 240,
    };
    // An unknown subtype is refused even when connected.
    assert_eq!(
        query_accept(Some(&mt(MEDIATYPE_AUDIO)), &format, true),
        VFW_E_TYPE_NOT_ACCEPTED
    );
    // RGB8 is accepted without a connection, the others only with one.
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB8)), &format, false),
        S_OK
    );
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB24)), &format, false),
        VFW_E_TYPE_NOT_ACCEPTED
    );
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB24)), &format, true),
        S_OK
    );
    // A pinned pixel format must match the media type exactly.
    let pinned = StreamFormat {
        flags: DDSD_PIXELFORMAT,
        pf: pf(DDPF_RGB, 16, 0x7c00, 0x03e0),
        ..StreamFormat::default()
    };
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB555)), &pinned, false),
        S_OK
    );
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB565)), &pinned, false),
        VFW_E_TYPE_NOT_ACCEPTED
    );
    // An unmapped pinned format is refused even for a matching subtype name.
    let unmapped = StreamFormat {
        flags: DDSD_PIXELFORMAT,
        pf: pf(DDPF_RGB, 8, 0, 0),
        ..StreamFormat::default()
    };
    assert_eq!(
        query_accept(Some(&mt(MEDIASUBTYPE_RGB8)), &unmapped, false),
        VFW_E_TYPE_NOT_ACCEPTED
    );
}

#[test]
fn audio_checks_cover_the_short_format_and_null_cases() {
    let mt = |cb_format: u32, format_tag: u16| MediaType {
        majortype: MEDIATYPE_AUDIO,
        formattype: FORMAT_WAVEFORMATEX,
        cb_format,
        format_tag,
    };
    // A format shorter than WAVEFORMATEX is refused before the tag is read.
    assert_eq!(
        check_receive_media_type(&mt(WAVEFORMATEX_SIZE - 1, 1)),
        VFW_E_TYPE_NOT_ACCEPTED
    );
    assert_eq!(check_receive_media_type(&mt(WAVEFORMATEX_SIZE, 1)), S_OK);
    assert_eq!(
        check_receive_media_type(&mt(WAVEFORMATEX_SIZE, 2)),
        E_INVALIDARG
    );
    // A wrong majortype or formattype is refused regardless of the format.
    let wrong_major = MediaType {
        majortype: MEDIATYPE_VIDEO,
        ..mt(WAVEFORMATEX_SIZE, 1)
    };
    assert_eq!(
        check_receive_media_type(&wrong_major),
        VFW_E_TYPE_NOT_ACCEPTED
    );
    let wrong_format = MediaType {
        formattype: FORMAT_VIDEOINFO,
        ..mt(WAVEFORMATEX_SIZE, 1)
    };
    assert_eq!(
        check_receive_media_type(&wrong_format),
        VFW_E_TYPE_NOT_ACCEPTED
    );

    assert_eq!(check_set_format(None), E_POINTER);
    assert_eq!(check_set_format(Some(1)), S_OK);
    assert_eq!(check_set_format(Some(0)), E_INVALIDARG);
}
