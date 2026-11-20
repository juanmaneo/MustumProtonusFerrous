//! Edge vectors. E01-E18 are from the same Wine build as the oracle rows, and
//! their expected values are its output. The rest follow from the code alone:
//! an empty list, text without a NUL, and the HRESULT values.

use super::oracle_checks::{join, wide, with_nul};
use crate::number::{parse_uint64, parse_wide_uint64};
use crate::uri::{build_uri_list, trim_trailing_slash};
use crate::Error;

const NUMBER_EDGES: &[(&str, &str, u64, Result<u64, Error>)] = &[
    ("E01", "1", 0, Ok(1)),
    ("E02", "0", 0, Ok(0)),
    ("E03", "9", 9, Ok(9)),
    ("E04", "10", 9, Err(Error::NumericOverflow)),
    ("E05", "\r", u64::MAX, Err(Error::InvalidFormat)),
    ("E06", " 9 ", 9, Ok(9)),
    (
        "E07",
        "184467440737095516150",
        u64::MAX,
        Err(Error::NumericOverflow),
    ),
    ("E08", "12 \t3", u64::MAX, Err(Error::InvalidFormat)),
];

const TRIM_EDGES: &[(&str, &str, &str)] = &[
    ("E14", "a//", "a/"),
    ("E15", "a/b", "a/b"),
    ("E16", "a", "a"),
];

#[test]
fn number_edges() {
    for (id, text, max, want) in NUMBER_EDGES {
        assert_eq!(parse_uint64(text.as_bytes(), *max), *want, "{id}");
    }
}

#[test]
fn wide_number_edges() {
    let rows: Vec<(&str, Vec<u16>, Result<u64, Error>)> = vec![
        ("E09", wide("0012"), Ok(12)),
        ("E10", wide("\u{3000}"), Err(Error::InvalidFormat)),
        ("E11", vec![0xD800], Err(Error::InvalidFormat)),
        ("E12", wide("\t\r\n 5"), Ok(5)),
        ("E13", wide(" "), Err(Error::InvalidFormat)),
    ];
    for (id, text, want) in &rows {
        assert_eq!(parse_wide_uint64(text), *want, "{id}");
    }
}

#[test]
fn trim_edges() {
    for (id, uri, want) in TRIM_EDGES {
        assert_eq!(trim_trailing_slash(&wide(uri)), &wide(want)[..], "{id}");
    }
    assert_eq!(
        trim_trailing_slash(&[0x61, 0x2f, 0, 0x62, 0x2f]),
        &[0x61][..]
    );
    assert_eq!(trim_trailing_slash(&[0x61, 0x2f]), &[0x61][..]);
}

#[test]
fn list_edges() {
    assert_eq!(join(&["a", "b"], 5), Ok(with_nul("a b")), "E17");
    assert_eq!(join(&["a", "b"], 4), Err(Error::InvalidArg), "E18");
    assert_eq!(join(&["a", "b", "c"], 7), Ok(with_nul("a b c")));
    assert_eq!(join(&[], 16), Err(Error::InvalidArg));
    assert_eq!(build_uri_list(&[&[0x61, 0, 0x62]], 8), Ok(vec![0x61, 0]));
    assert_eq!(build_uri_list(&[&[0, 0x61]], 8), Ok(vec![0]));
}

#[test]
fn text_without_nul_and_empty_text() {
    assert_eq!(parse_wide_uint64(&wide("12")), Ok(12));
    assert_eq!(parse_wide_uint64(&[]), Err(Error::Fail));
    assert_eq!(parse_wide_uint64(&[0, 0x31]), Err(Error::Fail));
}

#[test]
fn hresult_values() {
    assert_eq!(Error::Fail.hresult(), 0x8000_4005);
    assert_eq!(Error::InvalidArg.hresult(), 0x8007_0057);
    assert_eq!(Error::InvalidFormat.hresult(), 0x803D_0000);
    assert_eq!(Error::NumericOverflow.hresult(), 0x803D_0002);
}
