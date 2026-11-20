//! Edge values and branches that no oracle row reaches. The expected values come from the same
//! verbatim C drivers in `target/canon/vbscript`, under the EDG, EDL, ERP, H, O, E, U, L, Rt, M,
//! C, RGB, A and RP labels of `oracle.txt`. The `W` rows come from the `-fwrapv` build, because
//! the sanitizer build flags the signed multiply in `get_next_rnd` as UB. The remaining checks
//! follow from the C control flow, and `lib.rs` documents the deviations.

use crate::array::element_count;
use crate::color::rgb;
use crate::escape::{escape, unescape};
use crate::literal::{amp_literal, LexError, Literal};
use crate::radix::{hex, oct};
use crate::replace::{expand, Match};
use crate::rnd::{randomize, rnd};
use crate::strings::{chr_w, left, mid, right};
use crate::VbsError;
use std::ops::Range;

fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(s: &[u16]) -> String {
    String::from_utf16(s).unwrap()
}

fn lit(value: i32, end: usize) -> Result<Option<Literal>, LexError> {
    Ok(Some(Literal { value, end }))
}

fn lex_err(error: VbsError, loc: usize) -> Result<Option<Literal>, LexError> {
    Err(LexError { error, loc })
}

fn one_group(whole: Range<usize>, group: Range<usize>) -> Match {
    Match {
        whole,
        groups: vec![Some(group)],
    }
}

#[test]
fn edg15_randomize_zero_seed_keeps_low_byte() {
    // EDG15 randomize state_in=0xff -> state=0xff
    assert_eq!(randomize(0.0, 0xff), 0xff);
}

#[test]
fn n3_rnd_negative_zero_is_zero() {
    // N3 rnd state_in=0x123456 arg=r8 -> state=0x123456 value_bits=0x3d91a2b0
    let (state, value) = rnd(0x123456, Some(-0.0));
    assert_eq!(state, 0x123456);
    assert_eq!(value.to_bits(), 0x3d91a2b0);
}

#[test]
fn n5_rnd_positive_argument_advances_from_zero() {
    // N5 rnd state_in=0x0 arg=r8 -> state=0xc39ec3 value_bits=0x3f439ec3
    let (state, value) = rnd(0x000000, Some(1.0));
    assert_eq!(state, 0xc39ec3);
    assert_eq!(value.to_bits(), 0x3f439ec3);
}

#[test]
fn w1_rnd_positive_argument_wraps_the_multiply() {
    // W1 (-fwrapv only) rnd state_in=0x123456 arg=r8 -> state=0x71d9c1 value_bits=0x3ee3b382
    let (state, value) = rnd(0x123456, Some(1.0));
    assert_eq!(state, 0x71d9c1);
    assert_eq!(value.to_bits(), 0x3ee3b382);
}

#[test]
fn w2_rnd_negative_argument_reseeds_from_bits_and_wraps() {
    // W2 (-fwrapv only) rnd state_in=0x0 arg=r8 -> state=0x395886 value_bits=0x3e656218
    // N4 is the same call. The sanitizer build flags it, so it is not an oracle row.
    let (state, value) = rnd(0x000000, Some(-1.0));
    assert_eq!(state, 0x395886);
    assert_eq!(value.to_bits(), 0x3e656218);
}

#[test]
fn h2_hex_of_byte_max() {
    // H2 ok "FF"
    assert_eq!(hex(255), "FF");
}

#[test]
fn h4_hex_of_i4_minus_one_is_full_width() {
    // H4 ok "FFFFFFFF"
    assert_eq!(hex((-1_i32) as u32), "FFFFFFFF");
}

#[test]
fn edg3_hex_and_edg4_oct_of_zero() {
    // EDG3 ok "0", EDG4 ok "0"
    assert_eq!(hex(0), "0");
    assert_eq!(oct(0), "0");
}

#[test]
fn edg5_hex_of_largest_positive_i4() {
    // EDG5 ok "7FFFFFFF"
    assert_eq!(hex(0x7fff_ffff), "7FFFFFFF");
}

#[test]
fn e2_escape_keeps_the_nonblank_ascii_set() {
    // E2 ok "Az09@*_+-./"
    assert_eq!(escape(&units("Az09@*_+-./")), units("Az09@*_+-./"));
}

#[test]
fn edg1_escape_stops_at_the_first_nul() {
    // EDG1 ok "a" from the three units a, NUL, b
    assert_eq!(escape(&[0x61, 0, 0x62]), units("a"));
}

#[test]
fn u2_unescape_leaves_incomplete_escapes() {
    // U2 ok "%4%u12"
    assert_eq!(unescape(&units("%4%u12")), units("%4%u12"));
}

#[test]
fn edg2_unescape_decodes_nul_and_stops_at_the_terminator() {
    // EDG2 ok "a\0b" from "a%00b", then NUL, then c
    assert_eq!(unescape(&units("a%00b\0c")), units("a\0b"));
}

#[test]
fn edg14_unescape_uses_either_hex_case() {
    // EDG14 ok "\u{e9}\u{e9}"
    assert_eq!(unescape(&units("%u00e9%E9")), units("\u{e9}\u{e9}"));
}

#[test]
fn unescape_percent_at_the_end_is_literal() {
    // Rust-only: a % with no digits after it is copied.
    assert_eq!(unescape(&units("a%")), units("a%"));
}

#[test]
fn edg6_chrw_largest_unsigned_short() {
    // EDG6 ok "\u{ffff}"
    assert_eq!(chr_w(65535), Ok(0xffff));
}

#[test]
fn edg7_chrw_smallest_short_keeps_its_bits() {
    // EDG7 ok "\u{8000}"
    assert_eq!(chr_w(-32768), Ok(0x8000));
}

#[test]
fn edg8_chrw_largest_short() {
    // EDG8 ok "\u{7fff}"
    assert_eq!(chr_w(32767), Ok(0x7fff));
}

#[test]
fn edg9_chrw_just_above_unsigned_short_is_illegal_call() {
    // EDG9 err 5
    assert_eq!(chr_w(65536), Err(VbsError::IllegalFuncCall));
}

#[test]
fn c1_chrw_cjk_unit() {
    // C1 ok "\u{4e2d}"
    assert_eq!(chr_w(0x4e2d), Ok(0x4e2d));
}

#[test]
fn c3_chrw_minus_one_is_ffff() {
    // C3 ok "\u{ffff}"
    assert_eq!(chr_w(-1), Ok(0xffff));
}

#[test]
fn c4_chrw_below_short_is_illegal_call() {
    // C4 err 5
    assert_eq!(chr_w(-32769), Err(VbsError::IllegalFuncCall));
}

#[test]
fn l1_left_two_of_abc() {
    // L1 ok "ab"
    assert_eq!(text(left(&units("abc"), 2).unwrap()), "ab");
}

#[test]
fn edg11_left_zero_is_empty() {
    // EDG11 ok ""
    assert_eq!(text(left(&units("abc"), 0).unwrap()), "");
}

#[test]
fn right_negative_count_is_illegal_call() {
    // Rust-only: the count check in Right, the counterpart of L2.
    assert_eq!(right(&units("abc"), -1), Err(VbsError::IllegalFuncCall));
}

#[test]
fn edg12_right_zero_is_empty() {
    // EDG12 ok ""
    assert_eq!(text(right(&units("abc"), 0).unwrap()), "");
}

#[test]
fn rt2_right_longer_than_string_is_whole_string() {
    // Rt2 ok "abc"
    assert_eq!(text(right(&units("abc"), 10).unwrap()), "abc");
}

#[test]
fn m2_mid_without_length_takes_the_rest() {
    // M2 ok "def"
    assert_eq!(text(mid(&units("abcdef"), 4, None).unwrap()), "def");
}

#[test]
fn m3_mid_start_zero_is_illegal_call() {
    // M3 err 5
    assert_eq!(
        mid(&units("abcdef"), 0, None),
        Err(VbsError::IllegalFuncCall)
    );
}

#[test]
fn m4_mid_negative_length_is_illegal_call() {
    // M4 err 5
    assert_eq!(
        mid(&units("abcdef"), 3, Some(-1)),
        Err(VbsError::IllegalFuncCall)
    );
}

#[test]
fn m5_mid_start_past_the_end_is_empty() {
    // M5 ok ""
    assert_eq!(text(mid(&units("abc"), 10, None).unwrap()), "");
}

#[test]
fn edg13_mid_length_clamps_to_the_rest() {
    // EDG13 ok "bc"
    assert_eq!(text(mid(&units("abc"), 2, Some(5)).unwrap()), "bc");
}

#[test]
fn rgb2_negative_green_is_illegal_call() {
    // RGB2 err 5
    assert_eq!(rgb(300, -1, 0), Err(VbsError::IllegalFuncCall));
}

#[test]
fn rgb_negative_red_is_illegal_call() {
    // Rust-only: the red channel's error path.
    assert_eq!(rgb(-1, 0, 0), Err(VbsError::IllegalFuncCall));
}

#[test]
fn rgb_negative_blue_is_illegal_call() {
    // Rust-only: the blue channel's error path.
    assert_eq!(rgb(0, 0, -1), Err(VbsError::IllegalFuncCall));
}

#[test]
fn edg10_rgb_clamps_a_large_channel_to_255() {
    // EDG10 ok i4 255
    assert_eq!(rgb(300, 0, 0), Ok(255));
}

#[test]
fn rgb3_rgb_packs_the_blue_channel_into_bits_16_to_23() {
    // RGB3 ok i4 13107713
    assert_eq!(rgb(1, 2, 200), Ok(13107713));
}

#[test]
fn a2_null_array_has_no_elements() {
    // A2 element_count=0
    assert_eq!(element_count(None), 0);
}

#[test]
fn a3_zero_length_dimension_gives_zero() {
    // A3 element_count=0
    assert_eq!(element_count(Some(&[0, 4])), 0);
}

#[test]
fn a4_array_with_no_dimensions_has_one_element() {
    // A4 element_count=1
    assert_eq!(element_count(Some(&[])), 1);
}

#[test]
fn a5_product_wraps_to_zero() {
    // A5 element_count=0
    assert_eq!(element_count(Some(&[0x10000, 0x10000])), 0);
}

#[test]
fn lx1_hex_ff_is_byte_255() {
    // LX1 literal value=255 end=4
    assert_eq!(amp_literal(&units("&HFF"), 0), lit(255, 4));
}

#[test]
fn lx7_overflow_points_at_the_ampersand_not_the_start() {
    // LX7 err code=1002 loc=1
    assert_eq!(
        amp_literal(&units("x&HFFFFFFFFF"), 1),
        lex_err(VbsError::SyntaxError, 1)
    );
}

#[test]
fn lx8_octal_overflow_at_the_ampersand() {
    // LX8 err code=1002 loc=0
    assert_eq!(
        amp_literal(&units("&O40000000000"), 0),
        lex_err(VbsError::SyntaxError, 0)
    );
}

#[test]
fn lx10_h_without_a_hex_digit_is_operator() {
    // LX10 operator end=1
    assert_eq!(amp_literal(&units("&hG"), 0), Ok(None));
}

#[test]
fn lx11_lowercase_octal_marker() {
    // LX11 literal value=511 end=5
    assert_eq!(amp_literal(&units("&o777"), 0), lit(511, 5));
}

#[test]
fn lx12_leading_zeros_then_eight_digits_with_suffix() {
    // LX12 literal value=-1 end=19
    assert_eq!(amp_literal(&units("&H00000000FFFFFFFF&"), 0), lit(-1, 19));
}

#[test]
fn lx13_hex_with_no_suffix_above_16_bits_is_long() {
    // LX13 literal value=305419896 end=10
    assert_eq!(amp_literal(&units("&H12345678"), 0), lit(305419896, 10));
}

#[test]
fn lx14_nine_significant_hex_digits_overflow() {
    // LX14 err code=1002 loc=0
    assert_eq!(
        amp_literal(&units("&H123456789"), 0),
        lex_err(VbsError::SyntaxError, 0)
    );
}

#[test]
fn lx15_leading_zeros_do_not_count_as_digits() {
    // LX15 literal value=-1 end=10
    assert_eq!(amp_literal(&units("&H0000FFFF"), 0), lit(-1, 10));
}

#[test]
fn lx16_hex_7fff_is_positive_integer() {
    // LX16 literal value=32767 end=6
    assert_eq!(amp_literal(&units("&H7FFF"), 0), lit(32767, 6));
}

#[test]
fn lx17_hex_8000_sign_extends_to_minus_32768() {
    // LX17 literal value=-32768 end=6
    assert_eq!(amp_literal(&units("&H8000"), 0), lit(-32768, 6));
}

#[test]
fn lx18_hex_with_suffix_keeps_the_unsigned_value() {
    // LX18 literal value=65535 end=7
    assert_eq!(amp_literal(&units("&HFFFF&"), 0), lit(65535, 7));
}

#[test]
fn lx19_bare_octal_digit_with_suffix() {
    // LX19 literal value=7 end=3
    assert_eq!(amp_literal(&units("&7&"), 0), lit(7, 3));
}

#[test]
fn lx20_octal_with_suffix_at_uint_max() {
    // LX20 literal value=-1 end=14
    assert_eq!(amp_literal(&units("&O37777777777&"), 0), lit(-1, 14));
}

#[test]
fn lx21_octal_above_uint_max_overflows() {
    // LX21 err code=1002 loc=0
    assert_eq!(
        amp_literal(&units("&O377777777777&"), 0),
        lex_err(VbsError::SyntaxError, 0)
    );
}

#[test]
fn edl1_lone_h_is_operator() {
    // EDL1 operator end=1
    assert_eq!(amp_literal(&units("&H"), 0), Ok(None));
}

#[test]
fn edl2_lowercase_h_zero_is_zero() {
    // EDL2 literal value=0 end=3
    assert_eq!(amp_literal(&units("&h0"), 0), lit(0, 3));
}

#[test]
fn edl3_octal_marker_with_eight_is_operator() {
    // EDL3 operator end=1
    assert_eq!(amp_literal(&units("&O8"), 0), Ok(None));
}

#[test]
fn edl4_bare_octal_then_nine_is_syntax_error() {
    // EDL4 err code=1002 loc=2
    assert_eq!(
        amp_literal(&units("&19"), 0),
        lex_err(VbsError::SyntaxError, 2)
    );
}

#[test]
fn edl5_leading_zeros_before_a_single_digit() {
    // EDL5 literal value=1 end=19
    assert_eq!(amp_literal(&units("&H00000000000000001"), 0), lit(1, 19));
}

#[test]
fn edl6_overflow_scan_stops_on_the_suffix_ampersand() {
    // EDL6 err code=1002 loc=11
    assert_eq!(
        amp_literal(&units("&HFFFFFFFFF&"), 0),
        lex_err(VbsError::SyntaxError, 11)
    );
}

#[test]
fn edl7_octal_overflow_with_suffix() {
    // EDL7 err code=1002 loc=0
    assert_eq!(
        amp_literal(&units("&O40000000000&"), 0),
        lex_err(VbsError::SyntaxError, 0)
    );
}

#[test]
fn erp1_lone_dollar_at_the_end_is_text() {
    // ERP1 "$"
    assert_eq!(
        expand(&units("$"), &units("abcdef"), &one_group(1..4, 1..2)),
        units("$")
    );
}

#[test]
fn erp2_group_beyond_the_count_is_text() {
    // ERP2 "x$1" with no groups
    let m = Match {
        whole: 1..4,
        groups: vec![],
    };
    assert_eq!(expand(&units("x$1"), &units("abcdef"), &m), units("x$1"));
}

#[test]
fn erp3_dollar_pair_is_one_dollar() {
    // ERP3 "$"
    assert_eq!(
        expand(&units("$$"), &units("abcdef"), &one_group(1..4, 1..2)),
        units("$")
    );
}

#[test]
fn erp4_two_digit_group_twelve() {
    // ERP4 "bc": $12 names group 12, which is [1..3)
    let mut groups = vec![None; 12];
    groups[0] = Some(1..2);
    groups[11] = Some(1..3);
    let m = Match {
        whole: 1..4,
        groups,
    };
    assert_eq!(expand(&units("$12"), &units("abcdef"), &m), units("bc"));
}

#[test]
fn erp5_two_digit_past_the_count_falls_back_to_one_digit() {
    // ERP5 "b3": $13 with twelve groups falls back to group 1, and the 3 is text
    let mut groups = vec![None; 12];
    groups[0] = Some(1..2);
    groups[11] = Some(1..3);
    let m = Match {
        whole: 1..4,
        groups,
    };
    assert_eq!(expand(&units("$13"), &units("abcdef"), &m), units("b3"));
}

#[test]
fn erp6_dollar_then_letter_is_text() {
    // ERP6 "$x"
    let m = Match {
        whole: 1..4,
        groups: vec![],
    };
    assert_eq!(expand(&units("$x"), &units("abcdef"), &m), units("$x"));
}

#[test]
fn rp5_dollar_zero_zero_is_copied_as_text() {
    // RP5 is excluded from the oracle. C reads parens[-1] for "$00" and crashes. The port copies
    // it as text instead.
    assert_eq!(
        expand(&units("$00"), &units("abcdef"), &one_group(1..4, 1..2)),
        units("$00")
    );
}

#[test]
fn rp6_trailing_dollar_is_text() {
    // RP6 "a$"
    let m = Match {
        whole: 1..4,
        groups: vec![],
    };
    assert_eq!(expand(&units("a$"), &units("abcdef"), &m), units("a$"));
}

#[test]
fn expand_copies_text_without_dollars() {
    // Rust-only: a template with no pattern is copied unchanged.
    let m = Match {
        whole: 0..0,
        groups: vec![],
    };
    assert_eq!(expand(&units("plain"), &units("abc"), &m), units("plain"));
}
