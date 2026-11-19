//! Edge checks for the cases the oracle rows do not reach: the tolerance edges of the frame-rate
//! table, the gcd and clamp paths, the zero and wrapping cases of the stride and size helpers,
//! every fixed-position failure of the GUID parser, and the derived trait impls. The expected values
//! that the C also produces were checked against the verbatim C under the sanitizers.

use crate::dxva::{convert_color_info_to_dxva, VideoColorInfo};
use crate::frame_rate::{average_time_to_frame_rate, frame_rate_to_average_time};
use crate::guid::{parse_guid, Guid};
use crate::oracle_checks::wide;
use crate::video_format::{format_stride, image_size, plane_size, stride_for_bitmap_info_header};
use crate::{media_subtype_guid, video_format_guid, E_INVALIDARG};

fn color_info(value: u32) -> VideoColorInfo {
    VideoColorInfo {
        interlace_mode: value,
        source_chroma_subsampling: value,
        nominal_range: value,
        transfer_matrix: value,
        source_lighting: value,
        color_primaries: value,
        transfer_function: value,
    }
}

#[test]
fn frame_rate_outside_the_table_is_computed_from_the_denominator() {
    assert_eq!(frame_rate_to_average_time(24, 1001), 417_083_333);
    assert_eq!(frame_rate_to_average_time(1, 1), 10_000_000);
    assert_eq!(
        frame_rate_to_average_time(0xffff_ffff, 0xffff_ffff),
        10_000_000
    );
    assert_eq!(
        frame_rate_to_average_time(1, 0xffff_ffff),
        42_949_672_950_000_000
    );
}

#[test]
fn zero_numerator_gives_zero_whatever_the_denominator() {
    assert_eq!(frame_rate_to_average_time(0, 0), 0);
}

#[test]
fn average_time_matches_known_rates_within_thirty_units() {
    assert_eq!(average_time_to_frame_rate(416_667 + 30), (24, 1));
    assert_eq!(average_time_to_frame_rate(416_667 - 30), (24, 1));
    assert_eq!(average_time_to_frame_rate(333_333 - 30), (30, 1));
    assert_eq!(average_time_to_frame_rate(333_333 + 30), (30, 1));
}

#[test]
fn average_time_just_outside_the_known_rate_tolerance_is_reduced_by_gcd() {
    assert_eq!(
        average_time_to_frame_rate(416_667 + 31),
        (5_000_000, 208_349)
    );
    assert_eq!(
        average_time_to_frame_rate(333_333 + 31),
        (2_500_000, 83_341)
    );
}

#[test]
fn average_time_is_reduced_by_its_gcd_with_ten_million() {
    assert_eq!(average_time_to_frame_rate(1), (10_000_000, 1));
    assert_eq!(average_time_to_frame_rate(5_000_000), (2, 1));
    assert_eq!(average_time_to_frame_rate(50_000_000), (1, 5));
}

#[test]
fn average_time_above_the_limit_is_clamped_to_one_hundred_million() {
    assert_eq!(average_time_to_frame_rate(100_000_000), (1, 10));
    assert_eq!(average_time_to_frame_rate(u64::MAX), (1, 10));
}

#[test]
fn zero_width_gives_zero_strides_and_sizes() {
    assert_eq!(format_stride(&video_format_guid(22), 0), Some((0, false)));
    assert_eq!(image_size(&video_format_guid(22), 0, 5), Ok(0));
    assert_eq!(plane_size(22, 0, 5), 0);
    assert_eq!(stride_for_bitmap_info_header(22, 0), Ok(0));
}

#[test]
fn sub_byte_formats_pack_bits_per_pixel() {
    let a1 = video_format_guid(118);
    assert_eq!(format_stride(&a1, 8), Some((1, false)));
    assert_eq!(format_stride(&a1, 3), Some((0, false)));
    let rgb4 = video_format_guid(u32::from_le_bytes(*b"4Pxx"));
    assert_eq!(format_stride(&rgb4, 5), Some((2, false)));
}

#[test]
fn p010_stride_is_two_bytes_per_pixel() {
    let p010 = video_format_guid(u32::from_le_bytes(*b"P010"));
    assert_eq!(format_stride(&p010, 5), Some((10, true)));
    assert_eq!(image_size(&p010, 5, 4), Ok(72));
    assert_eq!(plane_size(u32::from_le_bytes(*b"P010"), 5, 4), 60);
}

#[test]
fn yuv_format_reports_yuv_and_a_top_down_stride() {
    let ayuv = u32::from_le_bytes(*b"AYUV");
    assert_eq!(format_stride(&video_format_guid(ayuv), 3), Some((12, true)));
    assert_eq!(stride_for_bitmap_info_header(ayuv, 3), Ok(12));
}

#[test]
fn bottom_up_stride_of_int_min_wraps() {
    // The C negates INT_MIN here, which UBSan flags, so this vector is not an oracle row.
    assert_eq!(stride_for_bitmap_info_header(22, 0x2000_0000), Ok(i32::MIN));
}

#[test]
fn image_size_rounds_widths_to_each_plane_layout() {
    assert_eq!(
        image_size(&video_format_guid(u32::from_le_bytes(*b"NV11")), 6, 2),
        Ok(24)
    );
    assert_eq!(
        image_size(&video_format_guid(u32::from_le_bytes(*b"I420")), 7, 4),
        Ok(48)
    );
}

#[test]
fn image_size_of_l8_is_one_byte_per_pixel() {
    // D3DFMT_L8.
    assert_eq!(image_size(&video_format_guid(50), 5, 3), Ok(15));
}

#[test]
fn image_size_rejects_a_subtype_outside_the_table() {
    assert_eq!(
        image_size(&video_format_guid(0x5858_5858), 4, 4),
        Err(E_INVALIDARG)
    );
}

#[test]
fn plane_size_of_a_format_outside_the_table_is_zero() {
    assert_eq!(plane_size(0x5858_5858, 4, 4), 0);
}

#[test]
fn plane_size_of_nv11_uses_the_three_halves_layout() {
    assert_eq!(plane_size(u32::from_le_bytes(*b"NV11"), 6, 2), 18);
}

#[test]
fn plane_size_of_a_packed_format_is_stride_times_height() {
    assert_eq!(plane_size(22, 3, 2), 24);
}

#[test]
fn video_format_guid_uses_the_mediatype_base() {
    assert_eq!(
        video_format_guid(22),
        Guid {
            data1: 22,
            data2: 0,
            data3: 0x10,
            data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
        }
    );
    assert_eq!(
        media_subtype_guid(0xe436_eb7d),
        Guid {
            data1: 0xe436_eb7d,
            data2: 0x524f,
            data3: 0x11ce,
            data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
        }
    );
}

#[test]
fn color_info_truncates_each_field_to_its_width() {
    assert_eq!(convert_color_info_to_dxva(&color_info(0)), 0);
    assert_eq!(convert_color_info_to_dxva(&color_info(u32::MAX)), u32::MAX);
    assert_eq!(convert_color_info_to_dxva(&color_info(3)), 0x18cd_b303);
}

#[test]
fn color_info_keeps_a_progressive_sample_format() {
    let progressive = VideoColorInfo {
        interlace_mode: 2,
        ..color_info(0)
    };
    assert_eq!(convert_color_info_to_dxva(&progressive), 2);
}

#[test]
fn parse_guid_accepts_the_zero_and_all_ones_guids() {
    assert_eq!(
        parse_guid(&wide("00000000-0000-0000-0000-000000000000")),
        Some(Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        })
    );
    assert_eq!(
        parse_guid(&wide("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF")),
        Some(Guid {
            data1: u32::MAX,
            data2: u16::MAX,
            data3: u16::MAX,
            data4: [0xff; 8],
        })
    );
}

#[test]
fn parse_guid_rejects_a_bad_digit_or_dash_at_every_position() {
    let valid = wide("12345678-ABCD-ef01-2345-6789abcdef01");
    for index in 0..36 {
        let mut text = valid.clone();
        text[index] = if matches!(index, 8 | 13 | 18 | 23) {
            u16::from(b'_')
        } else {
            u16::from(b'g')
        };
        assert_eq!(parse_guid(&text), None, "position {index}");
    }
}

#[test]
fn parse_guid_rejects_characters_on_both_sides_of_each_hex_range() {
    for bad in ['/', ':', '@', 'G', '`', 'g', '\u{100}'] {
        let mut text = wide("12345678-ABCD-ef01-2345-6789abcdef01");
        text[0] = bad as u16;
        assert_eq!(parse_guid(&text), None, "character {bad:?}");
    }
}

#[test]
fn parse_guid_needs_36_units_and_ignores_what_follows_a_nul() {
    let guid = "12345678-ABCD-ef01-2345-6789abcdef01";
    let expected = Some(Guid {
        data1: 0x1234_5678,
        data2: 0xabcd,
        data3: 0xef01,
        data4: [0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01],
    });
    assert_eq!(parse_guid(&[]), None);
    assert_eq!(parse_guid(&wide(&guid[..35])), None);
    assert_eq!(parse_guid(&wide(&format!("{guid}x"))), None);

    let mut with_tail = wide(guid);
    with_tail.extend(wide("\0junk"));
    assert_eq!(parse_guid(&with_tail), expected);
}

#[test]
fn guid_and_color_info_are_copy_clone_and_debug() {
    let guid = Guid {
        data1: 1,
        data2: 2,
        data3: 3,
        data4: [4; 8],
    };
    let copied = guid;
    assert_eq!(Clone::clone(&copied), guid);
    assert!(format!("{guid:?}").contains("data1: 1"));

    let info = color_info(3);
    assert_eq!(Clone::clone(&info), info);
    assert!(format!("{info:?}").contains("interlace_mode: 3"));
}
