//! Rust-only tests for the branches that no oracle row reaches. Each expectation follows from the
//! C control flow in `dlls/quartz`, or from the deviation that `lib.rs` documents.

use crate::array::{reserve_plan, Reserve};
use crate::avi::*;
use crate::dsound::*;
use crate::pin_types::*;
use crate::plane::*;
use crate::surface::*;
use crate::{size_image, BitmapInfoHeader, Guid, Rect, GUID_NULL};

const GUID_A: Guid = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const GUID_B: Guid = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
const GUID_C: Guid = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];

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

fn allocation(flags: u32, width: u32, height: u32) -> AllocationInfo {
    AllocationInfo {
        flags,
        width,
        height,
    }
}

#[test]
fn reserve_keeps_capacity_when_count_fits() {
    assert_eq!(reserve_plan(8, 8, 4), Reserve::Kept);
    assert_eq!(reserve_plan(8, 3, 4), Reserve::Kept);
}

#[test]
fn reserve_doubles_until_count_fits() {
    assert_eq!(
        reserve_plan(4, 5, 8),
        Reserve::Grow {
            capacity: 8,
            bytes: 64
        }
    );
}

#[test]
fn reserve_zero_element_size_fails() {
    assert_eq!(reserve_plan(0, 1, 0), Reserve::Fail);
}

#[test]
fn reserve_byte_count_overflow_fails() {
    assert_eq!(reserve_plan(0, 1, 1 << 62), Reserve::Fail);
}

#[test]
fn copy_plane_rejects_short_source() {
    let src = [0u8; 11];
    let mut dst = [0u8; 18];
    assert!(copy_plane(&mut dst, 6, 3, &src, 4, 3).is_none());
}

#[test]
fn copy_plane_rejects_short_destination() {
    let src = [0u8; 12];
    let mut dst = [0u8; 17];
    assert!(copy_plane(&mut dst, 6, 3, &src, 4, 3).is_none());
}

#[test]
fn copy_plane_rejects_more_rows_than_destination_height() {
    let src = [0u8; 12];
    let mut dst = [0u8; 18];
    assert!(copy_plane(&mut dst, 6, 2, &src, 4, 3).is_none());
}

#[test]
fn copy_plane_rejects_overflowing_source_length() {
    let src = [0u8; 1];
    let mut dst = [0u8; 8];
    assert!(copy_plane(&mut dst, 4, 2, &src, usize::MAX, 2).is_none());
}

#[test]
fn copy_plane_rejects_overflowing_destination_length() {
    let src = [0u8; 4];
    let mut dst = [0u8; 8];
    assert!(copy_plane(&mut dst, usize::MAX, 2, &src, 4, 1).is_none());
}

#[test]
fn copy_plane_rejects_minimum_height() {
    let src = [0u8; 12];
    let mut dst = [0u8; 18];
    assert!(copy_plane(&mut dst, 6, 3, &src, 4, i32::MIN).is_none());
}

#[test]
fn copy_plane_zero_rows_only_advances_destination() {
    let mut dst = [0xee; 8];
    let adv = copy_plane(&mut dst, 4, 2, &[], 4, 0).unwrap();
    assert_eq!((adv.src, adv.dst), (0, 8));
    assert_eq!(dst, [0xee; 8]);
}

#[test]
fn surface_without_pow2_or_square_keeps_size() {
    let out = adjust_surface_size(0x0, allocation(0x4, 100, 3)).unwrap();
    assert_eq!((out.width, out.height), (100, 3));
}

#[test]
fn surface_pow2_zero_dimension_rounds_to_one() {
    let out = adjust_surface_size(0x2, allocation(0x4, 0, 3)).unwrap();
    assert_eq!((out.width, out.height), (1, 4));
}

#[test]
fn surface_square_equal_sides_stays_put() {
    let out = adjust_surface_size(0x20, allocation(0x4, 50, 50)).unwrap();
    assert_eq!((out.width, out.height), (50, 50));
}

#[test]
fn surface_power_above_32_bits_fails() {
    assert!(adjust_surface_size(0x2, allocation(0x4, 0x8000_0001, 4)).is_none());
    assert!(adjust_surface_size(0x2, allocation(0x4, 4, 0x8000_0001)).is_none());
}

#[test]
fn bitfields_size_ignores_colour_table_count() {
    let h = BitmapInfoHeader {
        size: 40,
        width: 0,
        height: 0,
        compression: 3,
        clr_used: 256,
    };
    assert_eq!(bitmap_info_size(&h), 52);
}

#[test]
fn rgb_size_without_colour_table_is_header_only() {
    assert_eq!(bitmap_info_size(&header(0, 0)), 40);
}

#[test]
fn drop_sample_at_late_mark_is_not_dropped() {
    let mut late = 500;
    assert!(!drop_sample(&mut late, 500));
    assert_eq!(late, -1);
}

#[test]
fn nontrivial_rect_left_or_top_offset() {
    let h = header(640, 480);
    assert!(is_nontrivial_rect(&h, &rect(1, 0, 0, 0)));
    assert!(is_nontrivial_rect(&h, &rect(0, 1, 0, 0)));
}

#[test]
fn nontrivial_rect_right_at_width_is_trivial() {
    assert!(!is_nontrivial_rect(&header(640, 480), &rect(0, 0, 640, 0)));
}

#[test]
fn nontrivial_rect_bottom_matches_absolute_height() {
    assert!(!is_nontrivial_rect(&header(640, -480), &rect(0, 0, 0, 480)));
    assert!(is_nontrivial_rect(&header(640, -480), &rect(0, 0, 0, 479)));
}

#[test]
fn nontrivial_rect_all_zero_is_trivial() {
    assert!(!is_nontrivial_rect(&header(640, 480), &rect(0, 0, 0, 0)));
}

#[test]
fn needs_decompressex_with_only_target_nontrivial() {
    let h = header(640, 480);
    assert!(needs_decompressex(
        &h,
        &rect(0, 0, 640, 480),
        &rect(0, 0, 320, 240)
    ));
}

#[test]
fn needs_decompressex_both_trivial_is_false() {
    let h = header(640, 480);
    assert!(!needs_decompressex(
        &h,
        &rect(0, 0, 640, 480),
        &rect(0, 0, 640, 480)
    ));
}

#[test]
fn size_image_negative_width_truncates_toward_zero() {
    assert_eq!(size_image(-10, 2, 8), 4_294_967_280);
}

#[test]
fn time_from_pos_zero_rate_fails() {
    assert_eq!(time_from_pos(1, 0), None);
}

#[test]
fn time_from_pos_zero_position_is_zero() {
    assert_eq!(time_from_pos(0, 88_200), Some(0));
}

#[test]
fn pos_from_time_zero_alignment_fails() {
    assert_eq!(pos_from_time(1, 1, 0), None);
}

#[test]
fn pos_from_time_product_overflow_fails() {
    assert_eq!(pos_from_time(i64::MAX, 2, 4), None);
}

#[test]
fn pos_from_time_exact_block_is_kept() {
    assert_eq!(pos_from_time(10_000_000, 4, 4), Some(4));
}

#[test]
fn pos_from_time_partial_block_rounds_down() {
    assert_eq!(pos_from_time(10_000_000, 7, 4), Some(4));
}

#[test]
fn match_types_empty_wanted_matches_any_pin() {
    assert!(match_types(true, &[(GUID_A, GUID_B)], &[]));
}

#[test]
fn match_types_empty_wanted_and_pins_fails() {
    assert!(!match_types(false, &[], &[]));
}

#[test]
fn match_types_no_pins_fails() {
    assert!(!match_types(false, &[], &[(GUID_A, GUID_B)]));
}

#[test]
fn match_types_inexact_null_minor_matches() {
    assert!(match_types(
        false,
        &[(GUID_A, GUID_NULL)],
        &[(GUID_A, GUID_B)]
    ));
}

#[test]
fn match_types_exact_null_minor_fails() {
    assert!(!match_types(
        true,
        &[(GUID_A, GUID_NULL)],
        &[(GUID_A, GUID_B)]
    ));
}

#[test]
fn match_types_wanted_null_matches_any() {
    assert!(match_types(
        true,
        &[(GUID_A, GUID_B)],
        &[(GUID_NULL, GUID_NULL)]
    ));
}

#[test]
fn match_types_major_mismatch_fails() {
    assert!(!match_types(
        false,
        &[(GUID_C, GUID_B)],
        &[(GUID_A, GUID_B)]
    ));
}

#[test]
fn match_types_minor_mismatch_fails() {
    assert!(!match_types(true, &[(GUID_A, GUID_A)], &[(GUID_A, GUID_B)]));
}

#[test]
fn match_types_inexact_null_major_still_needs_minor() {
    assert!(!match_types(
        false,
        &[(GUID_NULL, GUID_A)],
        &[(GUID_B, GUID_B)]
    ));
}
