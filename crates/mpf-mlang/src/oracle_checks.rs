//! Replays of the oracle rows O01 to O24 in `target/canon/mlang/oracle.txt`. The oracle compiles
//! the verbatim `dlls/mlang/mlang.c` codec and family code with `-fsanitize=address,undefined`.
//! Each test names the row it replays, and its expected values are the `=>` fields of that row.

use crate::detect::japanese_code;
use crate::family::family_codepage;
use crate::jis::{han2zen, jis2sjis, jis_to_sjis, sjis2jis, sjis_to_jis};
use crate::{is_convert_inet_string_available, S_FALSE, S_OK};

#[test]
fn o01_detect_ascii_is_zero() {
    assert_eq!(japanese_code(b"ABC"), 0);
}

#[test]
fn o02_detect_esc_dollar_b_is_iso2022jp() {
    assert_eq!(japanese_code(&[0x1b, b'$', b'B']), 50220);
}

#[test]
fn o03_detect_euc_pair_is_eucjp() {
    assert_eq!(japanese_code(&[0xa4, 0xa2]), 51932);
}

#[test]
fn o04_detect_sjis_pair_is_932() {
    assert_eq!(japanese_code(&[0x82, 0xa0]), 932);
}

#[test]
fn o05_detect_truncated_pair_is_zero() {
    assert_eq!(japanese_code(&[0xa4]), 0);
}

#[test]
fn o06_jis_to_sjis_two_pairs_then_exit() {
    let input = [0x1b, b'$', b'B', 0x24, 0x22, 0x1b, b'(', b'B', 0x41];
    assert_eq!(jis_to_sjis(&input), Some(vec![0x82, 0xa0, 0x41]));
}

#[test]
fn o07_jis_to_sjis_hiragana_pair() {
    let input = [0x1b, b'$', b'B', 0x30, 0x21];
    assert_eq!(jis_to_sjis(&input), Some(vec![0x88, 0x9f]));
}

#[test]
fn o08_jis_to_sjis_esc_k_shifts() {
    let input = [0x1b, b'K', 0x21, 0x21];
    assert_eq!(jis_to_sjis(&input), Some(vec![0x81, 0x40]));
}

#[test]
fn o09_jis_to_sjis_truncated_pair_is_none() {
    let input = [0x1b, b'$', b'B', 0x30];
    assert_eq!(jis_to_sjis(&input), None);
}

#[test]
fn o10_sjis_to_jis_pair_enters_and_exits() {
    let expected = vec![0x1b, b'$', b'B', 0x24, 0x22, 0x1b, b'(', b'B', 0x41];
    assert_eq!(sjis_to_jis(&[0x82, 0xa0, 0x41]), Some(expected));
}

#[test]
fn o11_sjis_to_jis_non_trail_pair_is_copied() {
    assert_eq!(sjis_to_jis(&[0x81, 0x20]), Some(vec![0x81, 0x20]));
}

#[test]
fn o12_sjis_to_jis_voiced_katakana_at_end() {
    let expected = vec![0x1b, b'$', b'B', 0x25, 0x2c, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0xb6, 0xde]), Some(expected));
}

#[test]
fn o13_sjis_to_jis_lead_at_end_is_none() {
    assert_eq!(sjis_to_jis(&[0x82]), None);
}

#[test]
fn o14_sjis_to_jis_newline_exits_before_cr_lf() {
    let input = [0x88, 0x40, 0x0d, 0x0a, 0x41];
    let expected = vec![
        0x1b, b'$', b'B', 0x2f, 0x21, 0x1b, b'(', b'B', 0x0d, 0x0a, 0x41,
    ];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn o15_jis2sjis_pair_0x24_0x22() {
    let (mut p1, mut p2) = (0x24, 0x22);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x82, 0xa0));
}

#[test]
fn o16_sjis2jis_pair_0x81_0x40() {
    let (mut p1, mut p2) = (0x81, 0x40);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x21, 0x21));
}

#[test]
fn o17_han2zen_voiced_pair_is_mapped() {
    let (mut p1, mut p2) = (0xb6, 0xde);
    assert!(han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x4b));
}

#[test]
fn o18_han2zen_plain_pair_is_mapped_without_mark() {
    let (mut p1, mut p2) = (0xb1, 0x41);
    assert!(!han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x41));
}

#[test]
fn o19_family_of_932_is_932() {
    assert_eq!(family_codepage(932), Some(932));
}

#[test]
fn o20_family_of_utf8_is_unicode() {
    assert_eq!(family_codepage(65001), Some(1200));
}

#[test]
fn o21_family_of_unknown_codepage_is_none() {
    assert_eq!(family_codepage(12345), None);
}

#[test]
fn o22_cross_family_pair_is_not_available() {
    assert_eq!(is_convert_inet_string_available(1252, 1251), S_FALSE);
}

#[test]
fn o23_unicode_pair_is_available() {
    assert_eq!(is_convert_inet_string_available(65001, 1251), S_OK);
}

#[test]
fn o24_unknown_destination_is_not_available() {
    assert_eq!(is_convert_inet_string_available(932, 99999), S_FALSE);
}
