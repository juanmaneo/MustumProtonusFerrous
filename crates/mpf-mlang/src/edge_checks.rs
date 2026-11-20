//! Replays of the edge rows E, J, S, PJ, PS and HZ in `target/canon/mlang/edge_oracle.txt`, and
//! Rust-only tests for the byte-class boundaries. The rows cover the branches that the oracle rows
//! O01 to O24 do not reach. Each expected value is the `=>` field of its row, or the C control
//! flow for the boundary tests. Row PS01 (`PS2J 21 40`) is left out: UBSan reports a negative
//! left shift in `sjis2jis` for it, so it is a sanitizer-flagged vector.

use crate::classes::*;
use crate::detect::japanese_code;
use crate::family::family_codepage;
use crate::jis::{han2zen, jis2sjis, jis_to_sjis, sjis2jis, sjis_to_jis};
use crate::{is_convert_inet_string_available, S_FALSE, S_OK};

#[test]
fn e01_detect_empty_is_zero() {
    assert_eq!(japanese_code(&[]), 0);
}

#[test]
fn e02_detect_esc_at_end_is_zero() {
    assert_eq!(japanese_code(&[0x1b]), 0);
}

#[test]
fn e03_detect_esc_dollar_at_end_is_zero() {
    assert_eq!(japanese_code(&[0x1b, b'$']), 0);
}

#[test]
fn e04_detect_esc_paren_b_is_zero() {
    assert_eq!(japanese_code(&[0x1b, b'(', b'B']), 0);
}

#[test]
fn e05_detect_esc_k_is_iso2022jp() {
    assert_eq!(japanese_code(&[0x1b, b'K']), 50220);
}

#[test]
fn e06_detect_esc_dollar_at_is_iso2022jp() {
    assert_eq!(japanese_code(&[0x1b, b'$', b'@']), 50220);
}

#[test]
fn e07_detect_esc_dollar_other_is_zero() {
    assert_eq!(japanese_code(&[0x1b, b'$', b'X']), 0);
}

#[test]
fn e08_detect_esc_dollar_k_is_iso2022jp() {
    assert_eq!(japanese_code(&[0x1b, b'$', b'K']), 50220);
}

#[test]
fn e09_detect_euc_pair_then_esc_is_iso2022jp() {
    assert_eq!(japanese_code(&[0xa4, 0xa2, 0x1b, b'$', b'B']), 50220);
}

#[test]
fn e10_detect_sjis_pair_stops_the_scan() {
    assert_eq!(japanese_code(&[0x82, 0xa0, 0x1b, b'$', b'B']), 932);
}

#[test]
fn e11_detect_8e_with_hankata_trail_is_eucjp() {
    assert_eq!(japanese_code(&[0x8e, 0xa1]), 51932);
}

#[test]
fn e12_detect_8e_with_low_trail_is_sjis() {
    assert_eq!(japanese_code(&[0x8e, 0x41]), 932);
}

#[test]
fn e13_detect_8e_with_high_trail_is_zero() {
    assert_eq!(japanese_code(&[0x8e, 0xe0]), 0);
}

#[test]
fn e14_detect_lone_8e_is_zero() {
    assert_eq!(japanese_code(&[0x8e]), 0);
}

#[test]
fn e15_detect_euc_pair_then_truncated_esc_keeps_code() {
    assert_eq!(japanese_code(&[0xa4, 0xa2, 0x1b]), 51932);
}

#[test]
fn e16_detect_euc_lead_with_ascii_trail_is_zero() {
    assert_eq!(japanese_code(&[0xa4, 0x20]), 0);
    assert_eq!(japanese_code(&[0xa1, 0x40]), 0);
}

#[test]
fn e17_detect_lead_81_with_high_trail_is_zero() {
    assert_eq!(japanese_code(&[0x81, 0xa1]), 0);
    assert_eq!(japanese_code(&[0x81, 0x20]), 0);
}

#[test]
fn e18_detect_euc_pairs_with_truncated_tail_is_eucjp() {
    assert_eq!(japanese_code(&[0xa4, 0xa2, 0xa4]), 51932);
}

#[test]
fn e19_jis_to_sjis_esc_paren_at_end_is_empty() {
    assert_eq!(jis_to_sjis(&[0x1b, b'(']), Some(vec![]));
}

#[test]
fn e20_jis_to_sjis_esc_dollar_at_end_is_empty() {
    assert_eq!(jis_to_sjis(&[0x1b, b'$']), Some(vec![]));
}

#[test]
fn e21_jis_to_sjis_esc_at_end_is_none() {
    assert_eq!(jis_to_sjis(&[0x1b]), None);
}

#[test]
fn e22_jis_to_sjis_esc_paren_b_then_ascii() {
    assert_eq!(jis_to_sjis(&[0x1b, b'(', b'B', 0x41]), Some(vec![0x41]));
}

#[test]
fn e23_jis_to_sjis_unknown_escape_consumes_one_byte() {
    assert_eq!(
        jis_to_sjis(&[0x1b, b'X', 0x41, 0x41]),
        Some(vec![0x41, 0x41])
    );
}

#[test]
fn e24_jis_to_sjis_esc_k_at_end_is_empty() {
    assert_eq!(jis_to_sjis(&[0x1b, b'K']), Some(vec![]));
}

#[test]
fn e25_jis_to_sjis_shifted_pair_truncated_is_none() {
    assert_eq!(jis_to_sjis(&[0x1b, b'K', 0x21]), None);
}

#[test]
fn e26_jis_to_sjis_plain_ascii_is_copied() {
    assert_eq!(jis_to_sjis(&[0x41]), Some(vec![0x41]));
}

#[test]
fn e27_jis_to_sjis_shifted_pairs_with_high_trail() {
    let input = [0x1b, b'K', 0x21, 0x60];
    assert_eq!(jis_to_sjis(&input), Some(vec![0x81, 0x80]));
    let input = [0x1b, b'$', b'B', 0x60, 0x21];
    assert_eq!(jis_to_sjis(&input), Some(vec![0xe0, 0x9f]));
    let input = [0x1b, b'$', b'B', 0x61, 0x60];
    assert_eq!(jis_to_sjis(&input), Some(vec![0xe1, 0x80]));
}

#[test]
fn e28_jis_to_sjis_shifted_bytes_above_7f_wrap() {
    let input = [0x1b, b'$', b'B', 0xff, 0xff];
    assert_eq!(jis_to_sjis(&input), Some(vec![0x30, 0x1f]));
}

#[test]
fn e29_jis_to_sjis_shift_then_no_pair_is_empty() {
    assert_eq!(jis_to_sjis(&[0x1b, b'$', b'B']), Some(vec![]));
}

#[test]
fn e30_sjis_to_jis_empty_is_empty() {
    assert_eq!(sjis_to_jis(&[]), Some(vec![]));
}

#[test]
fn e31_sjis_to_jis_ascii_is_copied() {
    assert_eq!(sjis_to_jis(&[0x20, 0x41]), Some(vec![0x20, 0x41]));
}

#[test]
fn e32_sjis_to_jis_cr_lf_without_shift_is_copied() {
    assert_eq!(sjis_to_jis(&[0x0d, 0x0a]), Some(vec![0x0d, 0x0a]));
}

#[test]
fn e33_sjis_to_jis_newline_exits_shift() {
    let input = [0x82, 0xa0, 0x0a];
    let expected = vec![0x1b, b'$', b'B', 0x24, 0x22, 0x1b, b'(', b'B', 0x0a];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e34_sjis_to_jis_newline_before_shift_is_copied() {
    let input = [0x0a, 0x82, 0xa0];
    let expected = vec![0x0a, 0x1b, b'$', b'B', 0x24, 0x22, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e35_sjis_to_jis_non_trail_keeps_shift_open() {
    let input = [0x82, 0xa0, 0x81, 0x20];
    let expected = vec![0x1b, b'$', b'B', 0x24, 0x22, 0x81, 0x20, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e36_sjis_to_jis_consecutive_pairs_enter_once() {
    let input = [0x82, 0xa0, 0x82, 0xa2];
    let expected = vec![0x1b, b'$', b'B', 0x24, 0x22, 0x24, 0x24, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e37_sjis_to_jis_lead_with_high_trail_is_copied() {
    assert_eq!(sjis_to_jis(&[0x81, 0xfd]), Some(vec![0x81, 0xfd]));
}

#[test]
fn e38_sjis_to_jis_lead_at_end_is_none() {
    assert_eq!(sjis_to_jis(&[0x81]), None);
}

#[test]
fn e39_sjis_to_jis_hankata_without_mark_then_byte() {
    let input = [0xb1, 0x41];
    let expected = vec![0x1b, b'$', b'B', 0x25, 0x22, 0x1b, b'(', b'B', 0x41];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e40_sjis_to_jis_hankata_at_end_is_none() {
    assert_eq!(sjis_to_jis(&[0xb1]), None);
}

#[test]
fn e41_sjis_to_jis_hankata_then_truncated_hankata_is_none() {
    assert_eq!(sjis_to_jis(&[0xb1, 0xdf]), None);
    assert_eq!(sjis_to_jis(&[0x41, 0xde]), None);
}

#[test]
fn e42_sjis_to_jis_maru_and_nigori_marks() {
    let maru = vec![0x1b, b'$', b'B', 0x25, 0x51, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0xca, 0xdf]), Some(maru));
    let nigori = vec![0x1b, b'$', b'B', 0x25, 0x59, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0xcd, 0xde]), Some(nigori));
    let special = vec![0x1b, b'$', b'B', 0x25, 0x74, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0xb3, 0xde]), Some(special));
}

#[test]
fn e43_sjis_to_jis_hankata_while_shifted_does_not_reenter() {
    let input = [0x82, 0xa0, 0xb1, 0x41];
    let expected = vec![
        0x1b, b'$', b'B', 0x24, 0x22, 0x25, 0x22, 0x1b, b'(', b'B', 0x41,
    ];
    assert_eq!(sjis_to_jis(&input), Some(expected));
}

#[test]
fn e44_sjis_to_jis_hankata_truncated_while_shifted_is_none() {
    assert_eq!(sjis_to_jis(&[0x82, 0xa0, 0xb1, 0xdf]), None);
}

#[test]
fn e45_sjis_to_jis_lead_e0_uses_row_176() {
    let expected = vec![0x1b, b'$', b'B', 0x5f, 0x21, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0xe0, 0x40]), Some(expected));
}

#[test]
fn e46_sjis_to_jis_trail_9f_uses_plain_cell() {
    let expected = vec![0x1b, b'$', b'B', 0x22, 0x21, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0x81, 0x9f]), Some(expected));
}

#[test]
fn e47_sjis_to_jis_trail_90_uses_wide_cell() {
    let expected = vec![0x1b, b'$', b'B', 0x21, 0x70, 0x1b, b'(', b'B'];
    assert_eq!(sjis_to_jis(&[0x81, 0x90]), Some(expected));
}

#[test]
fn e48_sjis_to_jis_control_byte_is_copied() {
    assert_eq!(sjis_to_jis(&[0x80]), Some(vec![0x80]));
}

#[test]
fn e49_jis2sjis_even_row_above_95() {
    let (mut p1, mut p2) = (0x60, 0x21);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0xe0, 0x9f));
}

#[test]
fn e50_jis2sjis_odd_row_above_95() {
    let (mut p1, mut p2) = (0x61, 0x21);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0xe1, 0x40));
    let (mut p1, mut p2) = (0x61, 0x60);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0xe1, 0x80));
}

#[test]
fn e51_jis2sjis_odd_row_below_95() {
    let (mut p1, mut p2) = (0x21, 0x21);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x81, 0x40));
}

#[test]
fn e52_jis2sjis_wraps_to_a_byte() {
    let (mut p1, mut p2) = (0xff, 0xff);
    jis2sjis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x30, 0x1f));
}

#[test]
fn e53_sjis2jis_trail_9f_no_shift() {
    let (mut p1, mut p2) = (0x81, 0x9f);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x22, 0x21));
}

#[test]
fn e54_sjis2jis_trail_90_wide_cell() {
    let (mut p1, mut p2) = (0x81, 0x90);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x21, 0x70));
}

#[test]
fn e55_sjis2jis_lead_above_160() {
    let (mut p1, mut p2) = (0xe0, 0x40);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x5f, 0x21));
}

#[test]
fn e56_sjis2jis_high_trails_without_shift() {
    let (mut p1, mut p2) = (0x81, 0xa0);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x22, 0x22));
    let (mut p1, mut p2) = (0x81, 0xfd);
    sjis2jis(&mut p1, &mut p2);
    assert_eq!((p1, p2), (0x22, 0x7f));
}

#[test]
fn e57_han2zen_plain_byte_is_unchanged() {
    let (mut p1, mut p2) = (0x41, 0xde);
    assert!(!han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x41, 0xde));
}

#[test]
fn e58_han2zen_179_takes_the_special_pair() {
    let (mut p1, mut p2) = (0xb3, 0xde);
    assert!(han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x94));
}

#[test]
fn e59_han2zen_maru_needs_df_and_a_maru_lead() {
    let (mut p1, mut p2) = (0xb1, 0xdf);
    assert!(!han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x41));
    let (mut p1, mut p2) = (0xca, 0xdf);
    assert!(han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x70));
}

#[test]
fn e60_han2zen_nigori_in_range_110_to_122() {
    let (mut p1, mut p2) = (0xcd, 0xde);
    assert!(han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x78));
}

#[test]
fn e61_han2zen_non_katakana_lead_is_unchanged() {
    let (mut p1, mut p2) = (0x8e, 0x41);
    assert!(!han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x8e, 0x41));
}

#[test]
fn e62_han2zen_plain_katakana_maps_without_mark() {
    let (mut p1, mut p2) = (0xb1, 0x40);
    assert!(!han2zen(&mut p1, &mut p2));
    assert_eq!((p1, p2), (0x83, 0x41));
}

#[test]
fn e63_family_codepage_unknown_is_none() {
    assert_eq!(family_codepage(0), None);
    assert_eq!(family_codepage(1), None);
}

#[test]
fn e64_family_codepage_finds_each_group() {
    assert_eq!(family_codepage(1200), Some(1200));
    assert_eq!(family_codepage(65000), Some(1200));
    assert_eq!(family_codepage(50220), Some(932));
    assert_eq!(family_codepage(52936), Some(936));
    assert_eq!(family_codepage(874), Some(874));
    assert_eq!(family_codepage(1258), Some(1258));
    assert_eq!(family_codepage(37), Some(1252));
}

#[test]
fn e65_available_unknown_source_is_false() {
    assert_eq!(is_convert_inet_string_available(99999, 1252), S_FALSE);
}

#[test]
fn e66_available_unknown_destination_is_false() {
    assert_eq!(is_convert_inet_string_available(1252, 99999), S_FALSE);
}

#[test]
fn e67_available_same_family_is_true() {
    assert_eq!(is_convert_inet_string_available(37, 1252), S_OK);
}

#[test]
fn e68_available_unicode_either_side_is_true() {
    assert_eq!(is_convert_inet_string_available(932, 1200), S_OK);
    assert_eq!(is_convert_inet_string_available(1200, 932), S_OK);
}

#[test]
fn e69_available_different_families_is_false() {
    assert_eq!(is_convert_inet_string_available(932, 1252), S_FALSE);
}

#[test]
fn e70_classes_hankata_boundaries() {
    assert!(!is_hankata(160));
    assert!(is_hankata(161));
    assert!(is_hankata(223));
    assert!(!is_hankata(224));
}

#[test]
fn e71_classes_euc_boundaries() {
    assert!(!is_euc(160));
    assert!(is_euc(161));
    assert!(is_euc(254));
    assert!(!is_euc(255));
}

#[test]
fn e72_classes_not_euc_pair_boundaries() {
    assert!(!is_not_euc_pair(128, 64));
    assert!(!is_not_euc_pair(129, 63));
    assert!(is_not_euc_pair(129, 64));
    assert!(is_not_euc_pair(159, 160));
    assert!(!is_not_euc_pair(159, 161));
    assert!(!is_not_euc_pair(160, 64));
}

#[test]
fn e73_classes_sjis_lead_boundaries() {
    assert!(!is_sjis_lead(128));
    assert!(is_sjis_lead(129));
    assert!(is_sjis_lead(159));
    assert!(!is_sjis_lead(160));
    assert!(!is_sjis_lead(223));
    assert!(is_sjis_lead(224));
    assert!(is_sjis_lead(239));
    assert!(!is_sjis_lead(240));
}

#[test]
fn e74_classes_sjis_trail_boundaries() {
    assert!(!is_sjis_trail(63));
    assert!(is_sjis_trail(64));
    assert!(is_sjis_trail(252));
    assert!(!is_sjis_trail(253));
}

#[test]
fn e75_classes_maru_boundaries() {
    assert!(!is_maru(201));
    assert!(is_maru(202));
    assert!(is_maru(206));
    assert!(!is_maru(207));
}

#[test]
fn e76_classes_nigori_boundaries() {
    assert!(!is_nigori(181));
    assert!(is_nigori(182));
    assert!(is_nigori(196));
    assert!(!is_nigori(197));
    assert!(!is_nigori(201));
    assert!(is_nigori(202));
    assert!(is_nigori(206));
    assert!(!is_nigori(207));
}

#[test]
fn e77_family_excludes_if0_koi8r_878() {
    assert_eq!(family_codepage(878), None);
    assert_eq!(is_convert_inet_string_available(855, 878), S_FALSE);
}
