//! Replays of the oracle rows. The oracle compiles the verbatim `proxyodbc.c` bodies in
//! `target/canon/odbc32` with `-fsanitize=address,undefined`. Each test names the row it replays, and the
//! expected values are that row's line in `oracle.txt`. The vector `A={x}`, whose brace closes at the end of
//! the input, is excluded because ASan flags it as a heap-buffer-overflow. `edge_checks.rs` covers it instead.

use super::*;
use crate::types::*;

/// Encodes ASCII test text as the UTF-16 units of a Wine `WCHAR` string.
pub(crate) fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Builds the attribute list an oracle row expects.
pub(crate) fn pairs(list: &[(&str, &str)]) -> Vec<Attribute> {
    list.iter()
        .map(|(name, value)| Attribute {
            name: w(name),
            value: w(value),
        })
        .collect()
}

#[test]
fn oracle_p1_three_pairs() {
    // oracle: P1 ok DSN=abc|UID=u|PWD=p
    assert_eq!(
        parse_connect_string(&w("DSN=abc;UID=u;PWD=p")),
        Ok(pairs(&[("DSN", "abc"), ("UID", "u"), ("PWD", "p")]))
    );
}

#[test]
fn oracle_p2_braced_value_keeps_semicolon() {
    // oracle: P2 ok DSN=a;b|DRIVER=x
    assert_eq!(
        parse_connect_string(&w("DSN={a;b};DRIVER=x")),
        Ok(pairs(&[("DSN", "a;b"), ("DRIVER", "x")]))
    );
}

#[test]
fn oracle_p3_empty_input() {
    // oracle: P3 ok
    assert_eq!(parse_connect_string(&w("")), Ok(Vec::new()));
}

#[test]
fn oracle_p4_trailing_semicolon() {
    // oracle: P4 ok DRIVER=x
    assert_eq!(
        parse_connect_string(&w("DRIVER=x;")),
        Ok(pairs(&[("DRIVER", "x")]))
    );
}

#[test]
fn oracle_p5_name_spans_semicolon() {
    // oracle: P5 ok ABC;DSN=x
    assert_eq!(
        parse_connect_string(&w("ABC;DSN=x")),
        Ok(pairs(&[("ABC;DSN", "x")]))
    );
}

#[test]
fn oracle_p6_no_equals_gives_nothing() {
    // oracle: P6 ok
    assert_eq!(parse_connect_string(&w("NOEQUALS")), Ok(Vec::new()));
}

#[test]
fn oracle_p7_stops_at_segment_without_equals() {
    // oracle: P7 ok A=1
    assert_eq!(
        parse_connect_string(&w("A=1;NOEQ")),
        Ok(pairs(&[("A", "1")]))
    );
}

#[test]
fn oracle_p8_empty_name_stops_parse() {
    // oracle: P8 ok
    assert_eq!(parse_connect_string(&w("=val;B=2")), Ok(Vec::new()));
}

#[test]
fn oracle_p9_unterminated_brace_is_error() {
    // oracle: P9 err
    assert_eq!(
        parse_connect_string(&w("A={x")),
        Err(ParseError::UnterminatedBrace)
    );
}

#[test]
fn oracle_p10_unit_after_brace_is_skipped() {
    // oracle: P10 ok A=x
    assert_eq!(
        parse_connect_string(&w("A={x}B=2")),
        Ok(pairs(&[("A", "x")]))
    );
}

#[test]
fn oracle_p11_braced_value_then_pair() {
    // oracle: P11 ok A=x|B=2
    assert_eq!(
        parse_connect_string(&w("A={x};B=2")),
        Ok(pairs(&[("A", "x"), ("B", "2")]))
    );
}

#[test]
fn oracle_p12_empty_value() {
    // oracle: P12 ok A=|B=2
    assert_eq!(
        parse_connect_string(&w("A=;B=2")),
        Ok(pairs(&[("A", ""), ("B", "2")]))
    );
}

#[test]
fn oracle_p14_empty_segment_yields_semicolon_name() {
    // oracle: P14 ok A=1|;B=2
    assert_eq!(
        parse_connect_string(&w("A=1;;B=2")),
        Ok(pairs(&[("A", "1"), (";B", "2")]))
    );
}

#[test]
fn oracle_p15_trailing_name_without_equals() {
    // oracle: P15 ok A=1
    assert_eq!(parse_connect_string(&w("A=1;B")), Ok(pairs(&[("A", "1")])));
}

#[test]
fn oracle_b1_build_two_pairs() {
    // oracle: B1 DSN=abc;UID=u
    let attrs = pairs(&[("DSN", "abc"), ("UID", "u")]);
    assert_eq!(build_connect_string(&attrs), w("DSN=abc;UID=u"));
}

#[test]
fn oracle_b2_build_empty_list() {
    // oracle: B2
    assert_eq!(build_connect_string(&[]), w(""));
}

#[test]
fn oracle_g1_lookup_ignores_case() {
    // oracle: G1 abc
    let attrs = parse_connect_string(&w("DSN=abc;UID=u")).unwrap();
    assert_eq!(get_attribute(&attrs, &w("dsn")), Some(w("abc").as_slice()));
}

#[test]
fn oracle_g2_lookup_missing_name() {
    // oracle: G2 none
    let attrs = parse_connect_string(&w("DSN=abc;UID=u")).unwrap();
    assert_eq!(get_attribute(&attrs, &w("xyz")), None);
}

#[test]
fn oracle_h1_suffix_ignores_case() {
    // oracle: H1 1
    assert!(has_suffix(&w("C:\\dir\\FOO.DLL"), &w(".dll")));
}

#[test]
fn oracle_t1_odbc3_driver_maps_date_up() {
    // oracle: T1 91
    assert_eq!(map_type(true, 0x300, 3, SQL_DATE), SQL_TYPE_DATE);
}

#[test]
fn oracle_t2_odbc2_app_maps_timestamp_down() {
    // oracle: T2 11
    assert_eq!(map_type(false, 0x200, 2, SQL_TYPE_TIMESTAMP), SQL_TIMESTAMP);
}

#[test]
fn oracle_t3_odbc2_driver_maps_time_down() {
    // oracle: T3 10
    assert_eq!(map_type(true, 0x200, 3, SQL_TYPE_TIME), SQL_TIME);
}

#[test]
fn oracle_t4_odbc3_app_maps_time_up() {
    // oracle: T4 92
    assert_eq!(map_type(false, 0x200, 3, SQL_TIME), SQL_TYPE_TIME);
}

#[test]
fn oracle_l1_len_to_user_strided() {
    // oracle: L1 -3|42
    let mut bytes = [0u8; 32];
    bytes[0..8].copy_from_slice(&(-3i64).to_ne_bytes());
    bytes[16..24].copy_from_slice(&42i64.to_ne_bytes());
    let mut dst = [0i64; 2];
    assert_eq!(len_to_user(Some(&mut dst), &bytes, 2, 16), Ok(()));
    assert_eq!(dst, [-3, 42]);
}

#[test]
fn oracle_l2_len_from_user_strided() {
    // oracle: L2 ffffffffffffffff0700000000000000
    let mut bytes = [0u8; 16];
    assert_eq!(len_from_user(&mut bytes, Some(&[-1, 7]), 2, 8), Ok(()));
    let mut expected = [0xFFu8; 8].to_vec();
    expected.extend_from_slice(&7i64.to_ne_bytes());
    assert_eq!(bytes.to_vec(), expected);
}
