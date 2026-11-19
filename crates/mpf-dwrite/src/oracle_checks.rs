//! Replays of the oracle rows in `target/canon/dwrite`. The oracle compiles the verbatim C blocks
//! of `dlls/dwrite/dwrite_private.h` and `dlls/dwrite/opentype.c` with
//! `-fsanitize=address,undefined`, and runs each row in its own process. Each test names the row
//! it reproduces, and its expected values are that row's output.
//!
//! Two vectors are excluded because the C code trips the sanitizer on them: a VDMX group at an
//! odd offset (a misaligned `WORD` load in `table_read_be_word`), and a VDMX header whose offset
//! array runs past the end of the table (a heap-buffer-overflow in `find_vdmx_group`).

use super::*;

/// Decodes a hex string, two digits per byte.
pub(crate) fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// r01: `container 774f4646`.
#[test]
fn r01_container_woff_signature() {
    assert_eq!(container_type(&hex("774f4646")), ContainerType::Woff);
}

/// r02: `container 774f4632`.
#[test]
fn r02_container_woff2_signature() {
    assert_eq!(container_type(&hex("774f4632")), ContainerType::Woff2);
}

/// r03: `container 46464f77`, the signature with its bytes reversed.
#[test]
fn r03_container_reversed_signature_is_unknown() {
    assert_eq!(container_type(&hex("46464f77")), ContainerType::Unknown);
}

/// r04: `container 774f46`, three bytes, too short for a signature.
#[test]
fn r04_container_short_data_is_unknown() {
    assert_eq!(container_type(&hex("774f46")), ContainerType::Unknown);
}

/// r05: `ztw 34f`.
#[test]
fn r05_zero_width_combining_grapheme_joiner() {
    assert!(is_zero_width(0x34f));
}

/// r06: `ztw 180f`.
#[test]
fn r06_zero_width_just_past_mongolian_selectors() {
    assert!(!is_zero_width(0x180f));
}

/// r07: `ztw 200f`.
#[test]
fn r07_zero_width_right_to_left_mark() {
    assert!(is_zero_width(0x200f));
}

/// r08: `ztw 2010`.
#[test]
fn r08_zero_width_hyphen_is_not_zero_width() {
    assert!(!is_zero_width(0x2010));
}

/// r09: `ign ad`.
#[test]
fn r09_default_ignorable_soft_hyphen() {
    assert!(is_default_ignorable(0xad));
}

/// r10: `ign 7f`, a control character below the ignorable ranges.
#[test]
fn r10_default_ignorable_below_0x80_is_not() {
    assert!(!is_default_ignorable(0x7f));
}

/// r11: `ign 2070`, just past the format block at 2060..206f.
#[test]
fn r11_default_ignorable_superscript_zero_is_not() {
    assert!(!is_default_ignorable(0x2070));
}

/// r12: `ign e0fff`, the last code point of the tag block range.
#[test]
fn r12_default_ignorable_last_tag_block_code_point() {
    assert!(is_default_ignorable(0xe0fff));
}

/// r13: `ws 9`, a horizontal tab.
#[test]
fn r13_whitespace_tab() {
    assert!(is_whitespace(0x9));
}

/// r14: `ws 200b`, a zero width space, which is not white space in this list.
#[test]
fn r14_whitespace_zero_width_space_is_not() {
    assert!(!is_whitespace(0x200b));
}

/// r15: `ws 3000`, the ideographic space.
#[test]
fn r15_whitespace_ideographic_space() {
    assert!(is_whitespace(0x3000));
}

/// r16: `ws e`, just past the control range 0x09..0x0d.
#[test]
fn r16_whitespace_shift_out_is_not() {
    assert!(!is_whitespace(0xe));
}

/// r17: `gasp 0000000200080001ffff0003 6`: the first range covers 6 ppem.
#[test]
fn r17_gasp_first_range_at_6_ppem() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0000000200080001ffff0003")), 6.0),
        1
    );
}

/// r18: `gasp 0000000200080001ffff0003 9`: 9 ppem is past the first range.
#[test]
fn r18_gasp_second_range_at_9_ppem() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0000000200080001ffff0003")), 9.0),
        3
    );
}

/// r19: `gasp 0000000200080001ffff0003 8.5`: a fraction above the first range's 8 ppem.
#[test]
fn r19_gasp_fractional_emsize_past_the_first_range() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0000000200080001ffff0003")), 8.5),
        3
    );
}

/// r20: `gasp 0002000200080001ffff0003 6`: version 2 is not supported, so the flags are 0.
#[test]
fn r20_gasp_unsupported_version_gives_no_flags() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0002000200080001ffff0003")), 6.0),
        0
    );
}

/// The VDMX table shared by r21 to r24: one ratio that matches 1:1, and one group with records
/// for 8 and 16 ppem (`yMax` 7 and 14, `yMin` -2 and -4), startsz 8 and endsz 20.
const VDMX: &str = "00000001000101010101000c0002081400080007fffe0010000efffc";

/// r21: `vdmx <table> 8`.
#[test]
fn r21_vdmx_exact_record_at_8_ppem() {
    assert_eq!(vdmx_size(Table::new(&hex(VDMX)), 8), Some((7, 2)));
}

/// r22: `vdmx <table> 16`: the scan passes the 8 ppem record first.
#[test]
fn r22_vdmx_exact_record_at_16_ppem() {
    assert_eq!(vdmx_size(Table::new(&hex(VDMX)), 16), Some((14, 4)));
}

/// r23: `vdmx <table> 12`: the 16 ppem record is above 12, and the C code does not interpolate.
#[test]
fn r23_vdmx_between_records_is_not_interpolated() {
    assert_eq!(vdmx_size(Table::new(&hex(VDMX)), 12), None);
}

/// r24: `vdmx <table> 20`: the group ends at endsz 20.
#[test]
fn r24_vdmx_emsize_at_endsz_is_outside_the_group() {
    assert_eq!(vdmx_size(Table::new(&hex(VDMX)), 20), None);
}

/// r25: `vdmx 00000001000101000000000c...  8`: the x and y ratios are zero, which match any device.
#[test]
fn r25_vdmx_zero_ratio_matches_any_device() {
    let table = "00000001000101000000000c0002081400080007fffe0010000efffc";
    assert_eq!(vdmx_size(Table::new(&hex(table)), 8), Some((7, 2)));
}
