//! Replays of the oracle rows O01 to O25 in `target/canon/quartz/oracle.txt`. The oracle compiles
//! the verbatim Wine C with `-fsanitize=address,undefined`, and the sanitizers report nothing. Each
//! test names its row. The expected values are the fields of that row.

use crate::array::{reserve_plan, Reserve};
use crate::avi::*;
use crate::dsound::*;
use crate::pin_types::*;
use crate::plane::*;
use crate::surface::*;
use crate::{BitmapInfoHeader, Guid, Rect, GUID_NULL};

const GUID_A: Guid = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const GUID_B: Guid = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

fn header(width: i32, height: i32) -> BitmapInfoHeader {
    BitmapInfoHeader {
        size: 40,
        width,
        height,
        compression: 0,
        clr_used: 0,
    }
}

fn rect(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
    Rect {
        left,
        top,
        right,
        bottom,
    }
}

#[test]
fn o01_reserve_one_8_byte_element_grows_to_4() {
    assert_eq!(
        reserve_plan(0, 1, 8),
        Reserve::Grow {
            capacity: 4,
            bytes: 32
        }
    );
}

#[test]
fn o02_reserve_truncated_limit_rejects_count() {
    assert_eq!(reserve_plan(0, 1_610_612_736, 3), Reserve::Fail);
}

#[test]
fn o03_reserve_count_above_32_bits_caps_at_u32_max() {
    assert_eq!(
        reserve_plan(0, 2_147_483_649, 1),
        Reserve::Grow {
            capacity: u32::MAX,
            bytes: 4_294_967_295
        }
    );
}

#[test]
fn o04_copy_plane_copies_rows_with_the_smaller_pitch() {
    let src: [u8; 12] = core::array::from_fn(|i| i as u8 + 1);
    let mut dst = [0xee; 18];
    let adv = copy_plane(&mut dst, 6, 3, &src, 4, 3).unwrap();
    assert_eq!((adv.src, adv.dst), (12, 18));
    assert_eq!(
        dst,
        [1, 2, 3, 4, 0xee, 0xee, 5, 6, 7, 8, 0xee, 0xee, 9, 10, 11, 12, 0xee, 0xee]
    );
}

#[test]
fn o05_copy_plane_negative_height_flips_rows() {
    let src: [u8; 12] = core::array::from_fn(|i| i as u8 + 1);
    let mut dst = [0xee; 18];
    let adv = copy_plane(&mut dst, 6, 3, &src, 4, -3).unwrap();
    assert_eq!((adv.src, adv.dst), (12, 18));
    assert_eq!(
        dst,
        [9, 10, 11, 12, 0xee, 0xee, 5, 6, 7, 8, 0xee, 0xee, 1, 2, 3, 4, 0xee, 0xee]
    );
}

#[test]
fn o06_copy_plane_narrow_destination_pitch_truncates_rows() {
    let src: [u8; 12] = core::array::from_fn(|i| i as u8 + 1);
    let mut dst = [0xee; 6];
    let adv = copy_plane(&mut dst, 2, 3, &src, 4, 3).unwrap();
    assert_eq!((adv.src, adv.dst), (12, 6));
    assert_eq!(dst, [1, 2, 5, 6, 9, 10]);
}

fn allocation(flags: u32, width: u32, height: u32) -> AllocationInfo {
    AllocationInfo {
        flags,
        width,
        height,
    }
}

#[test]
fn o07_surface_without_texture_flag_is_unchanged() {
    let out = adjust_surface_size(0x0, allocation(0x0, 100, 3)).unwrap();
    assert_eq!((out.width, out.height), (100, 3));
}

#[test]
fn o08_surface_pow2_rounds_up_each_dimension() {
    let out = adjust_surface_size(0x2, allocation(0x4, 100, 3)).unwrap();
    assert_eq!((out.width, out.height), (128, 4));
}

#[test]
fn o09_surface_square_only_with_wide_frame_becomes_square() {
    let out = adjust_surface_size(0x22, allocation(0x4, 100, 3)).unwrap();
    assert_eq!((out.width, out.height), (100, 100));
}

#[test]
fn o10_surface_square_only_with_tall_frame_becomes_square() {
    let out = adjust_surface_size(0x20, allocation(0x4, 3, 100)).unwrap();
    assert_eq!((out.width, out.height), (100, 100));
}

#[test]
fn o11_bitfields_header_adds_three_masks() {
    let h = BitmapInfoHeader {
        size: 40,
        width: 0,
        height: 0,
        compression: 3,
        clr_used: 0,
    };
    assert_eq!(bitmap_info_size(&h), 52);
}

#[test]
fn o12_rgb_header_adds_colour_table() {
    let h = BitmapInfoHeader {
        size: 40,
        width: 0,
        height: 0,
        compression: 0,
        clr_used: 256,
    };
    assert_eq!(bitmap_info_size(&h), 1064);
}

#[test]
fn o13_drop_sample_with_no_late_mark_keeps_sample() {
    let mut late = -1;
    assert!(!drop_sample(&mut late, 500));
    assert_eq!(late, -1);
}

#[test]
fn o14_drop_sample_before_late_mark_drops_it() {
    let mut late = 500;
    assert!(drop_sample(&mut late, 100));
    assert_eq!(late, 500);
}

#[test]
fn o15_drop_sample_at_or_after_late_mark_clears_it() {
    let mut late = 500;
    assert!(!drop_sample(&mut late, 600));
    assert_eq!(late, -1);
}

#[test]
fn o16_half_size_target_rect_is_nontrivial() {
    assert!(is_nontrivial_rect(
        &header(640, -480),
        &rect(0, 0, 320, 240)
    ));
}

#[test]
fn o17_half_size_target_needs_decompressex() {
    let h = header(640, 480);
    assert!(needs_decompressex(
        &h,
        &rect(0, 0, 640, 480),
        &rect(0, 0, 320, 240)
    ));
}

#[test]
fn o18_size_image_pads_rows_to_four_bytes() {
    assert_eq!(size_image(320, 240, 24), 230_400);
}

#[test]
fn o19_size_image_negative_height_wraps_to_dword() {
    assert_eq!(size_image(640, -480, 16), 4_294_352_896);
}

#[test]
fn o20_time_from_pos_divides_by_average_rate() {
    assert_eq!(time_from_pos(4096, 88_200), Some(464_399));
}

#[test]
fn o21_pos_from_time_rounds_down_to_block() {
    assert_eq!(pos_from_time(1_000_001, 88_200, 16), Some(8816));
}

#[test]
fn o22_pos_from_time_negative_time_wraps_to_dword() {
    assert_eq!(pos_from_time(-10_000_000, 176_400, 4), Some(4_294_790_896));
}

#[test]
fn o23_match_types_exact_pair_matches() {
    assert!(match_types(true, &[(GUID_A, GUID_B)], &[(GUID_A, GUID_B)]));
}

#[test]
fn o24_match_types_inexact_null_pin_matches_any() {
    assert!(match_types(
        false,
        &[(GUID_NULL, GUID_NULL)],
        &[(GUID_A, GUID_B)]
    ));
}

#[test]
fn o25_match_types_exact_null_pin_does_not_match() {
    assert!(!match_types(
        true,
        &[(GUID_NULL, GUID_NULL)],
        &[(GUID_A, GUID_B)]
    ));
}
