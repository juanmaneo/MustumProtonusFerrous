//! Edge checks beyond the oracle rows: row padding boundaries, palette limits, and FOURCC NUL
//! handling. The E1 to E12 rows and the two `biClrUsed` conversions were run through the verbatim
//! C in `target/canon/msvfw32` with the sanitizers, and none of them was flagged. The Rust-only
//! checks cover C undefined behaviour that the sanitizers flag, so they have no oracle row.

use super::*;

#[test]
fn size_image_pads_each_row_to_four_bytes() {
    assert_eq!(get_size_image(1, 1, 8), 4);
    assert_eq!(get_size_image(4, 1, 8), 4);
    assert_eq!(get_size_image(5, 1, 8), 8);
    assert_eq!(get_size_image(1, 1, 32), 4);
    assert_eq!(get_size_image(1, 1, 1), 4);
    assert_eq!(get_size_image(8, 1, 1), 4);
    assert_eq!(get_size_image(9, 1, 1), 4);
    assert_eq!(get_size_image(17, 1, 1), 4);
    assert_eq!(get_size_image(33, 1, 1), 8);
}

#[test]
fn size_image_is_zero_for_an_empty_dib() {
    assert_eq!(get_size_image(0, 0, 0), 0);
    assert_eq!(get_size_image(0, 480, 24), 0);
    assert_eq!(get_size_image(640, 0, 24), 0);
    assert_eq!(get_size_image(640, 480, 0), 0);
}

#[test]
fn size_image_ignores_the_sign_of_height() {
    assert_eq!(get_size_image(10, -7, 24), get_size_image(10, 7, 24));
    assert_eq!(get_size_image(10, -1, 8), 12);
}

// Rust-only: `width * depth` is 2400000000 here, which overflows `int`. The C run is flagged by
// UBSan, so the value is the wrapped result and has no oracle row.
#[test]
fn size_image_wraps_when_width_times_depth_overflows_int() {
    assert_eq!(get_size_image(300_000_000, 9, 8), 2_700_000_000);
}

// Rust-only: `abs(INT_MIN)` is undefined in C. The sanitizers flag it, so this has no oracle row.
// The wrapped `abs` is 2^31, and 4 * 2^31 is 0 in 32 bits.
#[test]
fn size_image_wraps_when_abs_of_height_is_int_min() {
    assert_eq!(get_size_image(4, i32::MIN, 8), 0);
}

#[test]
fn num_colours_palette_depths_are_one_shifted_by_bit_count() {
    assert_eq!(num_colours(0, 1), 2);
    assert_eq!(num_colours(0, 0), 1);
    assert_eq!(num_colours(0, 7), 128);
    assert_eq!(num_colours(0, 8), 256);
    assert_eq!(num_colours(0, 9), 0);
    assert_eq!(num_colours(0, u16::MAX), 0);
}

#[test]
fn num_colours_uses_clr_used_when_it_is_set() {
    assert_eq!(num_colours(1, 24), 1);
    assert_eq!(num_colours(256, 8), 256);
    assert_eq!(num_colours(5, 0), 5);
}

// Run through the verbatim C, which converts the `DWORD` `biClrUsed` to `int`. The conversion is
// implementation-defined, and gcc wraps it. The sanitizers do not flag it.
#[test]
fn num_colours_oracle_clr_used_conversions() {
    assert_eq!(num_colours(2_147_483_648, 24), -2_147_483_648);
    assert_eq!(num_colours(4_294_967_295, 8), -1);
}

#[test]
fn fourcc_compares_ascii_letters_without_case() {
    assert!(fourcc_eq_ignore_case(0x5A, 0x7A));
    assert!(fourcc_eq_ignore_case(0x7A, 0x5A));
    assert!(fourcc_eq_ignore_case(0x4241_6162, 0x6261_4142));
    assert!(!fourcc_eq_ignore_case(0x5B, 0x7B));
}

// E1 to E12 were run through the verbatim C. The sanitized run of each row is listed here.
#[test]
fn fourcc_oracle_nul_stop_rows() {
    let rows: [(&str, u32, u32, bool); 12] = [
        ("E1", 0x0000_4100, 0x0000_4200, true),
        ("E2", 0x0000_6261, 0x0000_4241, true),
        ("E3", 0x0000_005B, 0x0000_007B, false),
        ("E4", 0x0000_005A, 0x0000_007A, true),
        ("E5", 0x6463_6261, 0x4443_4241, true),
        ("E6", 0x6463_6261, 0x4543_4241, false),
        ("E7", 0x0000_0061, 0x0000_0141, false),
        ("E8", 0x0059_5800, 0x0057_5A00, true),
        ("E9", 0x0000_0080, 0x0000_0080, true),
        ("E10", 0x4300_4241, 0x4400_6261, true),
        ("E11", 0x0043_4241, 0x7843_4241, false),
        ("E12", 0x0000_0000, 0x0000_0000, true),
    ];
    for (id, a, b, want) in rows {
        assert_eq!(fourcc_eq_ignore_case(a, b), want, "{id}");
    }
}

#[test]
fn fourcc_stops_at_the_first_shared_nul() {
    assert!(fourcc_eq_ignore_case(0x0000_0000, 0x0000_0000));
    assert!(fourcc_eq_ignore_case(0x0000_4100, 0x0000_4200));
    assert!(!fourcc_eq_ignore_case(0x0000_0041, 0x0000_0042));
}
