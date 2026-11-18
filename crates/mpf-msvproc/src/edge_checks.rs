//! Edge cases of the subtype mapping, the aperture geometry and the plane layout, beyond the
//! oracle vectors.

use crate::aperture::{content_rect, frame_dimensions, update_aperture};
use crate::format::{pixel_format_from_subtype, PixelFormat};
use crate::layout::{apply_pitch, arrays_aligned};
use crate::{Offset, Rect, VideoArea, VideoInfo};

fn fourcc(code: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*code)
}

fn info(width: u32, height: u32, ox: i16, oy: i16, cx: i32, cy: i32) -> VideoInfo {
    VideoInfo {
        width,
        height,
        minimum_display_aperture: VideoArea {
            offset_x: Offset {
                fract: 0,
                value: ox,
            },
            offset_y: Offset {
                fract: 0,
                value: oy,
            },
            cx,
            cy,
        },
    }
}

#[test]
fn unsupported_and_unknown_subtypes_have_no_format() {
    for data1 in [
        fourcc(b"420O"),
        fourcc(b"P208"),
        fourcc(b"v410"),
        fourcc(b"Y41P"),
        fourcc(b"Y41T"),
        fourcc(b"Y42T"),
        fourcc(b"nv12"),
        0,
        19,
        u32::MAX,
    ] {
        assert_eq!(
            pixel_format_from_subtype(data1),
            PixelFormat::None,
            "{data1:#x}"
        );
    }
}

#[test]
fn equivalent_fourccs_share_a_format() {
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"IYUV")),
        PixelFormat::Yuv420p
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"YV12")),
        PixelFormat::Yuv420p
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"NV12")),
        PixelFormat::Nv12
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"YUY2")),
        PixelFormat::Yuyv422
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"UYVY")),
        PixelFormat::Uyvy422
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"YVYU")),
        PixelFormat::Yvyu422
    );
    assert_eq!(
        pixel_format_from_subtype(fourcc(b"AYUV")),
        PixelFormat::Ayuv
    );
}

#[test]
fn d3dfmt_values_map_to_rgb_formats() {
    assert_eq!(pixel_format_from_subtype(20), PixelFormat::Bgr24);
    assert_eq!(pixel_format_from_subtype(21), PixelFormat::Bgra);
    assert_eq!(pixel_format_from_subtype(22), PixelFormat::Bgr0);
    assert_eq!(pixel_format_from_subtype(23), PixelFormat::Rgb565);
    assert_eq!(pixel_format_from_subtype(41), PixelFormat::Rgb8);
    assert_eq!(pixel_format_from_subtype(32), PixelFormat::Rgba);
    assert_eq!(pixel_format_from_subtype(24), PixelFormat::Rgb555);
}

#[test]
fn area_is_empty_only_when_all_fields_are_zero() {
    assert!(VideoArea::default().is_empty());
    let fract_only = VideoArea {
        offset_x: Offset { fract: 7, value: 0 },
        ..VideoArea::default()
    };
    assert!(fract_only.is_empty());

    let single_fields = [
        VideoArea {
            offset_x: Offset { fract: 0, value: 1 },
            ..VideoArea::default()
        },
        VideoArea {
            offset_y: Offset {
                fract: 0,
                value: -1,
            },
            ..VideoArea::default()
        },
        VideoArea {
            cx: 1,
            ..VideoArea::default()
        },
        VideoArea {
            cy: -1,
            ..VideoArea::default()
        },
    ];
    for area in single_fields {
        assert!(!area.is_empty(), "{area:?}");
    }
}

#[test]
fn content_rect_falls_back_to_the_frame_and_wraps_large_sizes() {
    assert_eq!(
        content_rect(&info(640, 480, 0, 0, 0, 0)),
        Rect {
            left: 0,
            top: 0,
            right: 640,
            bottom: 480,
        }
    );
    assert_eq!(
        content_rect(&info(u32::MAX, 0x8000_0000, 0, 0, 0, 0)),
        Rect {
            left: 0,
            top: 0,
            right: -1,
            bottom: i32::MIN,
        }
    );
    assert_eq!(
        content_rect(&info(640, 480, -4, 2, 10, 20)),
        Rect {
            left: -4,
            top: 2,
            right: 6,
            bottom: 22,
        }
    );
}

#[test]
fn frame_dimensions_use_the_aperture_only_when_set() {
    assert_eq!(frame_dimensions(&info(640, 480, 0, 0, 0, 0)), (640, 480));
    assert_eq!(frame_dimensions(&info(640, 480, 0, 0, 0, 5)), (0, 5));
    assert_eq!(frame_dimensions(&info(640, 480, 0, 3, 0, 0)), (0, 3));
    assert_eq!(frame_dimensions(&info(640, 480, 1, 0, 0, 0)), (1, 0));
    assert_eq!(
        frame_dimensions(&info(640, 480, 16, 8, 624, 464)),
        (640, 472)
    );
}

#[test]
fn update_aperture_describes_the_input_and_copies_it_to_the_output() {
    let mut input = info(640, 480, 16, 8, 624, 464);
    input.minimum_display_aperture.offset_x.fract = 7;
    let mut output = info(640, 480, 0, 0, 0, 0);
    output.minimum_display_aperture.offset_x.fract = 9;

    assert!(update_aperture(&mut input, &mut output));
    assert_eq!(
        input.minimum_display_aperture.offset_x,
        Offset {
            fract: 7,
            value: 16
        }
    );
    assert_eq!(
        output.minimum_display_aperture,
        input.minimum_display_aperture
    );
}

#[test]
fn update_aperture_of_matching_content_is_not_a_mismatch() {
    let mut input = info(640, 480, 0, 0, 0, 0);
    let mut output = info(640, 480, 0, 0, 0, 0);
    assert!(!update_aperture(&mut input, &mut output));
    assert_eq!(input.minimum_display_aperture.cx, 640);
    assert_eq!(input.minimum_display_aperture.cy, 480);
    assert_eq!(
        output.minimum_display_aperture,
        input.minimum_display_aperture
    );
}

#[test]
fn update_aperture_wraps_a_width_above_i32_max() {
    let mut input = info(u32::MAX, 1, 0, 0, 0, 0);
    let mut output = info(u32::MAX, 1, 0, 0, 0, 0);
    assert!(!update_aperture(&mut input, &mut output));
    assert_eq!(input.minimum_display_aperture.cx, -1);
}

#[test]
fn zero_pitch_keeps_strides_and_only_bottom_up_flips_the_first_plane() {
    let planes = [100, 200, 300, 0];
    let strides = [16, 8, 8, 0];
    assert_eq!(apply_pitch(planes, strides, 0, 5, false), (planes, strides));
    assert_eq!(
        apply_pitch(planes, strides, 0, 5, true),
        ([164, 200, 300, 0], [-16, 8, 8, 0])
    );
}

#[test]
fn pitch_equal_to_the_stride_is_not_rescaled() {
    let planes = [100, 200, 0, 0];
    let strides = [16, 8, 0, 0];
    assert_eq!(
        apply_pitch(planes, strides, 16, 5, false),
        (planes, strides)
    );
    assert_eq!(
        apply_pitch(planes, strides, -16, 5, true),
        ([164, 200, 0, 0], [-16, 8, 0, 0])
    );
}

#[test]
fn positive_pitch_disables_the_bottom_up_flip() {
    let planes = [100, 200, 0, 0];
    let strides = [16, 8, 0, 0];
    assert_eq!(
        apply_pitch(planes, strides, 32, 5, true),
        ([100, 292, 0, 0], [32, 16, 0, 0])
    );
}

#[test]
fn rescale_stops_at_the_first_null_plane() {
    assert_eq!(
        apply_pitch([100, 0, 0, 0], [4, 0, 0, 0], 8, 3, false),
        ([100, 0, 0, 0], [8, 0, 0, 0])
    );
    assert_eq!(
        apply_pitch([100, 200, 0, 0], [4, 2, 0, 0], 8, 3, false),
        ([100, 300, 0, 0], [8, 4, 0, 0])
    );
}

#[test]
fn rescale_walks_every_plane_with_rows_from_the_previous_plane() {
    assert_eq!(
        apply_pitch([0, 10, 20, 30], [4, 4, 4, 4], 8, 3, false),
        ([0, 16, 24, 32], [8, 8, 8, 8])
    );
}

#[test]
fn rescale_wraps_a_negative_row_count() {
    let (planes, _) = apply_pitch([0, 8, 4, 0], [4, 4, 4, 4], 8, 3, false);
    assert_eq!(planes[1], 16);
    assert_eq!(planes[2], 16usize.wrapping_sub(24));
}

#[test]
fn alignment_checks_the_size_every_plane_and_every_stride() {
    assert!(arrays_aligned([0, 16, 32, 48], [16, 32, -48, 0], 64, 16));
    assert!(!arrays_aligned([0, 16, 32, 48], [16, 32, 48, 0], 24, 16));
    assert!(!arrays_aligned([0, 16, 32, 8], [16, 32, 48, 0], 64, 16));
    assert!(!arrays_aligned([0, 16, 32, 48], [16, 12, 48, 0], 64, 16));
    assert!(arrays_aligned([0; 4], [0; 4], 0, 16));
}
