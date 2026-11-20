//! Replays of the oracle rows in `target/canon/vbscript/oracle.txt`. The oracle compiles the
//! verbatim Wine C with `-fsanitize=address,undefined -fshort-wchar`, and the sanitizers report
//! nothing for these rows. Each test names its row, and the expected values are that row's fields.

use crate::array::element_count;
use crate::color::rgb;
use crate::escape::{escape, unescape};
use crate::literal::{amp_literal, LexError, Literal};
use crate::radix::{hex, oct};
use crate::replace::{expand, Match};
use crate::rnd::{randomize, rnd};
use crate::strings::{chr_w, left, mid, right};
use crate::VbsError;

fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn text(s: &[u16]) -> String {
    String::from_utf16(s).unwrap()
}

#[test]
fn r1_randomize_seed_1_5_from_zero_state() {
    // R1 randomize state_in=0x0 -> state=0x3ff800
    assert_eq!(randomize(1.5, 0x00), 0x3ff800);
}

#[test]
fn r2_randomize_negative_seed_keeps_low_byte() {
    // R2 randomize state_in=0x7f -> state=0xc0027f
    assert_eq!(randomize(-2.25, 0x7f), 0xc0027f);
}

#[test]
fn n1_rnd_without_argument_from_zero_state() {
    // N1 rnd state_in=0x0 arg=none -> state=0xc39ec3 value_bits=0x3f439ec3
    let (state, value) = rnd(0x000000, None);
    assert_eq!(state, 0xc39ec3);
    assert_eq!(value.to_bits(), 0x3f439ec3);
}

#[test]
fn n2_rnd_zero_argument_keeps_state() {
    // N2 rnd state_in=0x123456 arg=r8 -> state=0x123456 value_bits=0x3d91a2b0
    let (state, value) = rnd(0x123456, Some(0.0));
    assert_eq!(state, 0x123456);
    assert_eq!(value.to_bits(), 0x3d91a2b0);
}

#[test]
fn h3_hex_of_i2_minus_one_is_zero_extended() {
    // H3 ok "FFFF"
    assert_eq!(hex(u32::from((-1_i16) as u16)), "FFFF");
}

#[test]
fn o1_oct_of_eight() {
    // O1 ok "10"
    assert_eq!(oct(8), "10");
}

#[test]
fn e1_escape_of_space_punctuation_and_wide_unit() {
    // E1 ok "Hi%20there%21%7E%u4E2D"
    assert_eq!(
        escape(&units("Hi there!~\u{4e2d}")),
        units("Hi%20there%21%7E%u4E2D")
    );
}

#[test]
fn u1_unescape_of_byte_wide_and_invalid_escapes() {
    // U1 ok "A\u{4e2d}%zz%u12%"
    assert_eq!(
        unescape(&units("%41%u4e2D%zz%u12%")),
        units("A\u{4e2d}%zz%u12%")
    );
}

#[test]
fn l2_left_negative_count_is_illegal_call() {
    // L2 err 5
    assert_eq!(left(&units("abc"), -1), Err(VbsError::IllegalFuncCall));
    assert_eq!(VbsError::IllegalFuncCall.code(), 5);
}

#[test]
fn rt1_right_two_of_abcdef() {
    // Rt1 ok "ef"
    assert_eq!(text(right(&units("abcdef"), 2).unwrap()), "ef");
}

#[test]
fn m1_mid_from_two_for_three_of_abcdef() {
    // M1 ok "bcd"
    assert_eq!(text(mid(&units("abcdef"), 2, Some(3)).unwrap()), "bcd");
}

#[test]
fn c2_chrw_above_sixteen_bits_is_illegal_call() {
    // C2 err 5
    assert_eq!(chr_w(70000), Err(VbsError::IllegalFuncCall));
}

#[test]
fn rgb1_rgb_255_128_0() {
    // RGB1 ok i4 33023
    assert_eq!(rgb(255, 128, 0), Ok(33023));
}

#[test]
fn a1_element_count_of_three_by_four() {
    // A1 element_count=12
    assert_eq!(element_count(Some(&[3, 4])), 12);
}

#[test]
fn lx2_hex_ffff_is_integer_minus_one() {
    // LX2 literal value=-1 end=6
    assert_eq!(
        amp_literal(&units("&HFFFF"), 0),
        Ok(Some(Literal { value: -1, end: 6 }))
    );
}

#[test]
fn lx3_hex_with_ampersand_suffix_is_long_min() {
    // LX3 literal value=-2147483648 end=11
    assert_eq!(
        amp_literal(&units("&H80000000&"), 0),
        Ok(Some(Literal {
            value: i32::MIN,
            end: 11
        }))
    );
}

#[test]
fn lx4_bare_ampersand_octal_digits() {
    // LX4 literal value=15 end=3
    assert_eq!(
        amp_literal(&units("&17"), 0),
        Ok(Some(Literal { value: 15, end: 3 }))
    );
}

#[test]
fn lx5_octal_digit_eight_is_syntax_error_at_the_digit() {
    // LX5 err code=1002 loc=3
    assert_eq!(
        amp_literal(&units("&O18"), 0),
        Err(LexError {
            error: VbsError::SyntaxError,
            loc: 3
        })
    );
    assert_eq!(VbsError::SyntaxError.code(), 1002);
}

#[test]
fn lx6_nine_hex_digits_overflow_at_the_ampersand() {
    // LX6 err code=1002 loc=0
    assert_eq!(
        amp_literal(&units("&HFFFFFFFFF"), 0),
        Err(LexError {
            error: VbsError::SyntaxError,
            loc: 0
        })
    );
}

#[test]
fn lx9_lone_ampersand_is_operator() {
    // LX9 operator end=1
    assert_eq!(amp_literal(&units("&"), 0), Ok(None));
}

#[test]
fn rp1_ampersand_backtick_and_quote_patterns() {
    // RP1 "[cd|ab|ef]"
    let m = Match {
        whole: 2..4,
        groups: vec![],
    };
    assert_eq!(
        expand(&units("[$&|$`|$']"), &units("abcdef"), &m),
        units("[cd|ab|ef]")
    );
}

#[test]
fn rp2_group_one_then_dollar_pairs_with_missing_group_two() {
    // RP2 "b$"
    let m = Match {
        whole: 1..4,
        groups: vec![Some(1..2), None],
    };
    assert_eq!(expand(&units("$1$2$$"), &units("abcdef"), &m), units("b$"));
}

#[test]
fn rp3_two_digits_that_name_no_group_fall_back_to_one_digit() {
    // RP3 "b2x$0$9$"
    let m = Match {
        whole: 1..4,
        groups: vec![Some(1..2)],
    };
    assert_eq!(
        expand(&units("$12x$0$9$"), &units("abcdef"), &m),
        units("b2x$0$9$")
    );
}

#[test]
fn rp4_leading_zero_names_group_one() {
    // RP4 "b"
    let m = Match {
        whole: 1..4,
        groups: vec![Some(1..2)],
    };
    assert_eq!(expand(&units("$01"), &units("abcdef"), &m), units("b"));
}
