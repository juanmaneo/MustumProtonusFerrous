//! Oracle rows from Wine's `dlls/wsdapi/soap.c`, built verbatim under ASan and
//! UBSan: `str_to_uint64` (N01-N13 and Q01-Q02), `wide_text_to_ulonglong`
//! (W02-W07), `trim_trailing_slash` (T01-T04) and `build_uri_list` (L01-L11).
//! Each expected value is the output of that build. W01 is not a row, since its
//! build leaks one byte (LeakSanitizer). The rows here are 25 of them, and the
//! rest are in `edge_checks`.

use crate::number::{parse_uint64, parse_wide_uint64};
use crate::uri::{build_uri_list, trim_trailing_slash};
use crate::Error;

pub(super) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub(super) fn with_nul(s: &str) -> Vec<u16> {
    let mut v = wide(s);
    v.push(0);
    v
}

pub(super) fn join(items: &[&str], buffer_size: usize) -> Result<Vec<u16>, Error> {
    let owned: Vec<Vec<u16>> = items.iter().map(|s| wide(s)).collect();
    let refs: Vec<&[u16]> = owned.iter().map(Vec::as_slice).collect();
    build_uri_list(&refs, buffer_size)
}

#[test]
fn oracle_numbers() {
    let rows: &[(&str, &str, Result<u64, Error>)] = &[
        ("N01", "0", Ok(0)),
        ("N02", "18446744073709551615", Ok(u64::MAX)),
        ("N03", "18446744073709551616", Err(Error::NumericOverflow)),
        ("N04", "  42\t\r\n", Ok(42)),
        ("N05", "\t \r\n", Err(Error::InvalidFormat)),
        ("N06", "", Err(Error::InvalidFormat)),
        ("N07", "4x2", Err(Error::InvalidFormat)),
        ("N11", "99999999999999999999", Err(Error::NumericOverflow)),
        ("N13", "18446744073709551619", Err(Error::NumericOverflow)),
    ];
    for (id, text, want) in rows {
        assert_eq!(parse_uint64(text.as_bytes(), u64::MAX), *want, "{id}");
    }
}

#[test]
fn oracle_max_wrap() {
    assert_eq!(parse_uint64(b"9", 5), Ok(9), "Q01");
    assert_eq!(parse_uint64(b"12", 5), Err(Error::NumericOverflow), "Q02");
}

#[test]
fn oracle_wide_numbers() {
    let rows: Vec<(&str, Vec<u16>, Result<u64, Error>)> = vec![
        ("W02", wide("\u{e9}"), Err(Error::InvalidFormat)),
        ("W03", wide("12\0 34"), Ok(12)),
        ("W04", wide("  7 "), Ok(7)),
        (
            "W06",
            wide("18446744073709551616"),
            Err(Error::NumericOverflow),
        ),
        (
            "W07",
            wide("99999999999999999999\u{e9}"),
            Err(Error::NumericOverflow),
        ),
    ];
    for (id, text, want) in &rows {
        assert_eq!(parse_wide_uint64(text), *want, "{id}");
    }
}

#[test]
fn oracle_trim() {
    let rows = [
        ("T01", "http://a/", "http://a"),
        ("T02", "//", "/"),
        ("T03", "", ""),
        ("T04", "/", ""),
    ];
    for (id, uri, want) in rows {
        assert_eq!(trim_trailing_slash(&wide(uri)), &wide(want)[..], "{id}");
    }
}

#[test]
fn oracle_uri_list() {
    assert_eq!(join(&["a", "bc"], 16), Ok(with_nul("a bc")), "L01");
    assert_eq!(join(&["a", "bc"], 3), Err(Error::InvalidArg), "L02");
    assert_eq!(join(&["ab", "c"], 5), Err(Error::InvalidArg), "L04");
    assert_eq!(join(&["ab", "c"], 6), Ok(with_nul("ab c")), "L05");
    assert_eq!(join(&["abc", "d"], 7), Err(Error::InvalidArg), "L10");
}
