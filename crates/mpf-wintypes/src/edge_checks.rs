//! Edge-case tests for the ported wintypes functions.
//!
//! These cover boundaries and inputs outside the oracle rows. Expected values follow the verbatim
//! C control flow, and the type-name cases were also checked against the oracle over a random set.

use super::*;

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn parts(input: &str) -> Vec<String> {
    parse_type_name(input).expect("valid type name")
}

fn err(input: &str) -> Error {
    parse_type_name(input).expect_err("invalid type name")
}

#[test]
fn type_name_rejects_empty_and_leading_space() {
    assert_eq!(err(""), Error::InvalidArg);
    assert_eq!(err(" "), Error::InvalidTypeFormat);
    assert_eq!(err(" A"), Error::InvalidTypeFormat);
    assert_eq!(err("A B"), Error::InvalidTypeFormat);
    assert_eq!(err("A `1<B>"), Error::InvalidTypeFormat);
}

#[test]
fn type_name_trims_only_spaces() {
    assert_eq!(parts("A`1< B >"), vec!["A`1", "B"]);
    assert_eq!(parts("A`1<B> "), vec!["A`1", "B"]);
    assert_eq!(parts("A`1<\tB>"), vec!["A`1", "\tB"]);
    assert_eq!(err("A`1<B>x"), Error::InvalidTypeFormat);
    assert_eq!(err("A`1<B>,"), Error::InvalidTypeFormat);
}

#[test]
fn type_name_truncates_at_nul_like_wcslen() {
    assert_eq!(parts("A`1<B>\0junk"), vec!["A`1", "B"]);
    assert_eq!(err("\0"), Error::InvalidTypeFormat);
    assert_eq!(err("\0A"), Error::InvalidTypeFormat);
}

#[test]
fn type_name_rejects_malformed_arity() {
    assert_eq!(err("`1<B>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`"), Error::InvalidTypeFormat);
    assert_eq!(err("A`1"), Error::InvalidTypeFormat);
    assert_eq!(err("A`10<B>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`x<B>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`9<"), Error::InvalidTypeFormat);
    assert_eq!(err("A<B>"), Error::InvalidTypeFormat);
    assert_eq!(err("A>"), Error::InvalidTypeFormat);
    assert_eq!(err("A,B"), Error::InvalidTypeFormat);
}

#[test]
fn type_name_rejects_parameter_count_and_nesting_errors() {
    assert_eq!(err("A`1<B,C>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`1<B<C>>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`1<B`1<C>>x"), Error::InvalidTypeFormat);
    assert_eq!(err("A`2<,B>"), Error::InvalidTypeFormat);
    assert_eq!(err("A`2<B,>"), Error::InvalidTypeFormat);
    // The extra '>' drives the nesting counter below zero; that must reject, not panic.
    assert_eq!(err("A`2<B>,C>"), Error::InvalidTypeFormat);
}

#[test]
fn type_name_splits_nested_parameters() {
    assert_eq!(parts("A`2<B,C>"), vec!["A`2", "B", "C"]);
    assert_eq!(parts("A`1<B`1<C>>"), vec!["A`1", "B`1", "C"]);
    assert_eq!(
        parts("A`1<B`1<C`1<D`1<E>>>>"),
        vec!["A`1", "B`1", "C`1", "D`1", "E"]
    );
}

#[test]
fn type_name_keeps_non_ascii_text() {
    assert_eq!(parts("Ärger`1<é>"), vec!["Ärger`1", "é"]);
}

#[test]
fn api_contract_version_boundary() {
    let name = "Windows.Foundation.UniversalApiContract";
    assert!(is_api_contract_present(name, 0));
    assert!(is_api_contract_present(name, 10));
    assert!(!is_api_contract_present(name, 11));
    assert!(!is_api_contract_present(name, u32::MAX));
}

#[test]
fn api_contract_name_matches_ascii_case_insensitively_and_exactly() {
    assert!(is_api_contract_present(
        "windows.FOUNDATION.universalAPIcontract",
        1
    ));
    assert!(!is_api_contract_present(
        "Windows.Foundation.UniversalApiContractX",
        1
    ));
    assert!(!is_api_contract_present("Windows.Foundation.Universal", 1));
    assert!(!is_api_contract_present("Windows.Foundation.Other", 1));
    assert!(!is_api_contract_present("", 1));
}

#[test]
fn api_contract_name_is_truncated_at_nul() {
    assert!(is_api_contract_present(
        "Windows.Foundation.UniversalApiContract\0junk",
        10
    ));
    assert!(!is_api_contract_present(
        "\0Windows.Foundation.UniversalApiContract",
        1
    ));
}

#[test]
fn buffer_length_bounds() {
    let mut buffer = Buffer::new(10);
    assert_eq!((buffer.capacity(), buffer.length()), (10, 0));
    assert_eq!(buffer.set_length(7), Ok(()));
    assert_eq!(buffer.set_length(11), Err(Error::InvalidArg));
    assert_eq!(buffer.length(), 7);
    assert_eq!(buffer.set_length(10), Ok(()));
    assert_eq!(buffer.length(), 10);
}

#[test]
fn buffer_zero_and_max_capacity() {
    let mut empty = Buffer::new(0);
    assert_eq!(empty.set_length(0), Ok(()));
    assert_eq!(empty.set_length(1), Err(Error::InvalidArg));

    let mut max = Buffer::new(u32::MAX);
    assert_eq!(max.set_length(u32::MAX), Ok(()));
    assert_eq!(max.length(), u32::MAX);
}

#[test]
fn error_hresults_match_winerror() {
    assert_eq!(Error::InvalidArg.hresult(), 0x8007_0057);
    assert_eq!(Error::InvalidTypeFormat.hresult(), 0x8000_0011);
}

#[test]
fn map_lower_bound_empty_and_single_key() {
    let keys: Vec<Vec<u16>> = Vec::new();
    assert_eq!(map_lower_bound(&keys, &utf16("a")), (0, Ordering::Less));

    let keys = vec![utf16("")];
    assert_eq!(map_lower_bound(&keys, &utf16("")), (0, Ordering::Equal));
    assert_eq!(map_lower_bound(&keys, &utf16("a")), (1, Ordering::Less));
}

#[test]
fn map_lower_bound_prefixes_and_before_all() {
    let keys = vec![utf16("a"), utf16("ab")];
    assert_eq!(map_lower_bound(&keys, &utf16("a")), (0, Ordering::Equal));
    assert_eq!(map_lower_bound(&keys, &utf16("aa")), (1, Ordering::Less));
    assert_eq!(map_lower_bound(&keys, &utf16("0")), (0, Ordering::Greater));
}

#[test]
fn map_lower_bound_orders_by_code_unit() {
    // U+1F600 is stored as the surrogates D83D DE00, which sort below U+FFFD.
    let keys = vec![utf16("\u{FFFD}")];
    assert_eq!(
        map_lower_bound(&keys, &utf16("\u{1F600}")),
        (0, Ordering::Greater)
    );
}

#[test]
fn map_lower_bound_finds_every_key_and_gaps() {
    let keys: Vec<Vec<u16>> = ["a", "c", "e", "g"].iter().map(|k| utf16(k)).collect();
    for (index, key) in keys.iter().enumerate() {
        assert_eq!(map_lower_bound(&keys, key), (index, Ordering::Equal));
    }
    assert_eq!(map_lower_bound(&keys, &utf16("d")), (2, Ordering::Less));
    assert_eq!(map_lower_bound(&keys, &utf16("z")), (4, Ordering::Less));
}
