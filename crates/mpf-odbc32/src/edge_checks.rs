//! Edge cases the oracle rows do not reach: error paths, NUL handling, every `map_type` arm and the
//! buffer checks that the C code leaves to its callers.

use super::oracle_checks::{pairs, w};
use super::*;
use crate::types::*;
use crate::wide::eq_ignore_case;

#[test]
fn edge_parse_stops_at_first_nul() {
    let mut input = w("A=1");
    input.push(0);
    input.extend(w("B=2"));
    assert_eq!(parse_connect_string(&input), Ok(pairs(&[("A", "1")])));
}

#[test]
fn edge_parse_empty_slice() {
    assert_eq!(parse_connect_string(&[]), Ok(Vec::new()));
}

#[test]
fn edge_parse_error_discards_earlier_pairs() {
    assert_eq!(
        parse_connect_string(&w("A=1;B={x")),
        Err(ParseError::UnterminatedBrace)
    );
}

#[test]
fn edge_parse_brace_closing_at_end_stops_safely() {
    // The C code reads past the NUL here, which ASan reports as a heap-buffer-overflow. This port stops instead.
    assert_eq!(parse_connect_string(&w("A={x}")), Ok(pairs(&[("A", "x")])));
}

#[test]
fn edge_parse_empty_braced_value() {
    assert_eq!(
        parse_connect_string(&w("A={};B=2")),
        Ok(pairs(&[("A", ""), ("B", "2")]))
    );
}

#[test]
fn edge_parse_lone_equals_gives_nothing() {
    assert_eq!(parse_connect_string(&w("=")), Ok(Vec::new()));
}

#[test]
fn edge_parse_value_keeps_later_equals() {
    assert_eq!(
        parse_connect_string(&w("A=x=y")),
        Ok(pairs(&[("A", "x=y")]))
    );
}

#[test]
fn edge_lookup_first_duplicate_wins() {
    let attrs = parse_connect_string(&w("DSN=one;dsn=two")).unwrap();
    assert_eq!(get_attribute(&attrs, &w("DSN")), Some(w("one").as_slice()));
}

#[test]
fn edge_lookup_on_empty_list() {
    assert_eq!(get_attribute(&[], &w("DSN")), None);
}

#[test]
fn edge_lookup_rejects_other_lengths() {
    let attrs = pairs(&[("DSN", "abc")]);
    assert_eq!(get_attribute(&attrs, &w("DS")), None);
    assert_eq!(get_attribute(&attrs, &w("DSNX")), None);
}

#[test]
fn edge_lookup_does_not_fold_non_ascii() {
    let attrs = pairs(&[("\u{c9}X", "x")]);
    assert_eq!(get_attribute(&attrs, &w("\u{e9}x")), None);
}

#[test]
fn edge_build_single_pair_has_no_separator() {
    assert_eq!(build_connect_string(&pairs(&[("A", "1")])), w("A=1"));
}

#[test]
fn edge_build_empty_value() {
    assert_eq!(build_connect_string(&pairs(&[("A", "")])), w("A="));
}

#[test]
fn edge_build_round_trips_parse() {
    let text = "DSN=abc;UID=u;PWD=p";
    let attrs = parse_connect_string(&w(text)).unwrap();
    assert_eq!(build_connect_string(&attrs), w(text));
}

#[test]
fn edge_suffix_longer_than_string() {
    assert!(!has_suffix(&w("a.d"), &w(".dll")));
}

#[test]
fn edge_suffix_same_length_mismatch() {
    assert!(!has_suffix(&w("file.exe"), &w(".dll")));
}

#[test]
fn edge_suffix_exact_match_and_empty_suffix() {
    assert!(has_suffix(&w(".dll"), &w(".dll")));
    assert!(has_suffix(&w("abc"), &[]));
}

#[test]
fn edge_suffix_reads_only_before_nul() {
    assert!(has_suffix(&w("a.dll\0junk"), &w(".dll")));
    assert!(has_suffix(&w("x.dll"), &w(".dll\0zz")));
    assert!(!has_suffix(&w("abc"), &w(".dll\0zz")));
}

#[test]
fn edge_eq_ignore_case_compares_units() {
    assert!(eq_ignore_case(&w("Ab1"), &w("aB1")));
    assert!(!eq_ignore_case(&w("Ab1"), &w("aB2")));
    assert!(!eq_ignore_case(&w("ab"), &w("abc")));
    assert!(eq_ignore_case(&w("Z"), &w("z")));
    assert!(!eq_ignore_case(&w("["), &w("{")));
    assert!(!eq_ignore_case(&w("@"), &w("`")));
}

#[test]
fn edge_map_type_table() {
    // (input, driver_odbc_ver, attr_version, type, expected)
    let cases = [
        (true, 0x300, 2, SQL_DATE, SQL_TYPE_DATE),
        (true, 0x300, 2, SQL_TIME, SQL_TYPE_TIME),
        (true, 0x300, 2, SQL_TIMESTAMP, SQL_TYPE_TIMESTAMP),
        (true, 0x300, 2, SQL_TYPE_DATE, SQL_TYPE_DATE),
        (true, 0x300, 2, SQL_TYPE_TIME, SQL_TYPE_TIME),
        (true, 0x300, 2, SQL_TYPE_TIMESTAMP, SQL_TYPE_TIMESTAMP),
        (true, 0x2FF, 3, SQL_DATE, SQL_DATE),
        (true, 0x2FF, 3, SQL_TIME, SQL_TIME),
        (true, 0x2FF, 3, SQL_TIMESTAMP, SQL_TIMESTAMP),
        (true, 0x2FF, 3, SQL_TYPE_DATE, SQL_DATE),
        (true, 0x2FF, 3, SQL_TYPE_TIME, SQL_TIME),
        (true, 0x2FF, 3, SQL_TYPE_TIMESTAMP, SQL_TIMESTAMP),
        (false, 0x200, 3, SQL_DATE, SQL_TYPE_DATE),
        (false, 0x200, 3, SQL_TIME, SQL_TYPE_TIME),
        (false, 0x200, 3, SQL_TIMESTAMP, SQL_TYPE_TIMESTAMP),
        (false, 0x200, 2, SQL_TYPE_DATE, SQL_DATE),
        (false, 0x200, 2, SQL_TYPE_TIME, SQL_TIME),
        (false, 0x200, 2, SQL_TYPE_TIMESTAMP, SQL_TIMESTAMP),
        (false, 0x300, 2, SQL_DATE, SQL_DATE),
        (false, 0x300, 2, SQL_TYPE_DATE, SQL_DATE),
        (false, 0x300, 3, SQL_TYPE_DATE, SQL_TYPE_DATE),
        (false, 0x300, 1, SQL_DATE, SQL_DATE),
        (false, 0x300, 1, SQL_TYPE_DATE, SQL_TYPE_DATE),
        (false, 0x300, 0, SQL_TYPE_TIMESTAMP, SQL_TYPE_TIMESTAMP),
        (true, 0x300, 3, 1, 1),
        (false, 0x200, 2, 4, 4),
    ];
    for (input, driver, attr, sql_type, expected) in cases {
        assert_eq!(
            map_type(input, driver, attr, sql_type),
            expected,
            "input={input} driver={driver:#x} attr={attr} type={sql_type}"
        );
    }
}

#[test]
fn edge_len_to_user_none_ignores_buffers() {
    assert_eq!(len_to_user(None, &[], 5, 8), Ok(()));
}

#[test]
fn edge_len_to_user_zero_rows_reads_nothing() {
    let mut dst: [i64; 0] = [];
    assert_eq!(len_to_user(Some(&mut dst), &[], 0, 8), Ok(()));
}

#[test]
fn edge_len_to_user_rejects_short_destination() {
    let lens = [0u8; 16];
    let mut dst = [7i64; 1];
    assert_eq!(
        len_to_user(Some(&mut dst), &lens, 2, 8),
        Err(LengthError::BufferTooShort)
    );
    assert_eq!(dst, [7]);
}

#[test]
fn edge_len_to_user_rejects_short_source_without_writing() {
    let lens = [0u8; 15];
    let mut dst = [7i64; 2];
    assert_eq!(
        len_to_user(Some(&mut dst), &lens, 2, 8),
        Err(LengthError::BufferTooShort)
    );
    assert_eq!(dst, [7, 7]);
}

#[test]
fn edge_len_to_user_saturated_span_is_too_short() {
    let lens = [0u8; 64];
    let mut dst = [0i64; 2];
    assert_eq!(
        len_to_user(Some(&mut dst), &lens, 2, usize::MAX),
        Err(LengthError::BufferTooShort)
    );
}

#[test]
fn edge_len_to_user_overlapping_slots_read_native_values() {
    let lens: Vec<u8> = (1..=12).collect();
    let mut dst = [0i64; 2];
    assert_eq!(len_to_user(Some(&mut dst), &lens, 2, 4), Ok(()));
    assert_eq!(
        dst,
        [
            i64::from_ne_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            i64::from_ne_bytes([5, 6, 7, 8, 9, 10, 11, 12]),
        ]
    );
}

#[test]
fn edge_len_from_user_none_leaves_buffer() {
    let mut lens = [9u8; 4];
    assert_eq!(len_from_user(&mut lens, None, 3, 8), Ok(()));
    assert_eq!(lens, [9; 4]);
}

#[test]
fn edge_len_from_user_zero_rows_writes_nothing() {
    let mut lens = [9u8; 4];
    assert_eq!(len_from_user(&mut lens, Some(&[]), 0, 8), Ok(()));
    assert_eq!(lens, [9; 4]);
}

#[test]
fn edge_len_from_user_rejects_short_source() {
    let mut lens = [9u8; 16];
    assert_eq!(
        len_from_user(&mut lens, Some(&[1]), 2, 8),
        Err(LengthError::BufferTooShort)
    );
    assert_eq!(lens, [9; 16]);
}

#[test]
fn edge_len_from_user_rejects_short_buffer_without_writing() {
    let mut lens = [9u8; 15];
    assert_eq!(
        len_from_user(&mut lens, Some(&[1, 2]), 2, 8),
        Err(LengthError::BufferTooShort)
    );
    assert_eq!(lens, [9u8; 15]);
}

#[test]
fn edge_len_from_user_overlapping_slots_last_write_wins() {
    let first = i64::from_ne_bytes([1, 2, 3, 4, 5, 6, 7, 8]);
    let second = i64::from_ne_bytes([21, 22, 23, 24, 25, 26, 27, 28]);
    let mut lens = [0u8; 12];
    assert_eq!(
        len_from_user(&mut lens, Some(&[first, second]), 2, 4),
        Ok(())
    );
    let mut expected = vec![1, 2, 3, 4];
    expected.extend_from_slice(&second.to_ne_bytes());
    assert_eq!(lens.to_vec(), expected);
}
