//! Oracle rows O01 to O24 for `mpf-urlmon`. Each row is a call to a function of
//! Wine's `uri.c`, `internet.c`, `sec_mgr.c` or `zone_id.c`, built verbatim with
//! `gcc -fsanitize=address,undefined`, and the value that the C harness printed
//! for it. The harness and its rows are in `target/canon/urlmon`, which is not
//! tracked. No row trips a sanitizer.

use crate::domain::find_domain_name;
use crate::pattern::matches_domain_pattern;
use crate::schema::{parse_schema, Schema};
use crate::scheme::{is_hierarchical_scheme, URL_SCHEME_FILE};
use crate::zone::{is_known_zone, URLZONE_INVALID, URLZONE_UNTRUSTED};

pub(crate) const S_OK: u32 = 0;
pub(crate) const E_POINTER: u32 = 0x8000_4003;

const FIND_ROWS: &[(&str, &str, i32)] = &[
    ("O01", "www.google.com", 4),
    ("O02", "www.google.com.uk", 4),
    ("O03", "www.google.foo.uk", 11),
    ("O04", "www.google.fo.uk", 4),
    ("O05", "www.google.test.us", 11),
    ("O06", "www.winehq.org.uk.test", 15),
    ("O07", "www.google.COM.uk", 4),
    ("O08", "a.b.edu.uk", 2),
    ("O09", ".com", 0),
    ("O10", "edu.uk", -1),
    ("O11", "ak.uk", -1),
    ("O12", "foo.uk", 0),
];

const SCHEMA_ROWS: &[(&str, &str, u32, u32, u32, &str)] = &[
    ("O13", "http://example.com", 16, S_OK, 4, "http"),
    ("O14", "http://example.com", 4, E_POINTER, 4, ""),
    ("O15", "mailto", 10, S_OK, 0, ""),
    ("O16", "a:b", 1, E_POINTER, 1, ""),
];

const PATTERN_ROWS: &[(&str, &str, &str, bool, Option<usize>)] = &[
    ("O17", "*.google.com", "www.google.com", false, Some(0)),
    (
        "O18",
        "*.google.com",
        "test.testing.google.com",
        false,
        None,
    ),
    (
        "O19",
        "*.google.com",
        "test.testing.google.com",
        true,
        Some(5),
    ),
    ("O20", "google.com", "wwwgoogle.com", true, None),
    ("O21", "google.com", "www.google.com", true, Some(4)),
];

pub(crate) fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// Checks `parse_schema` against an oracle row: `hr` and `rsize` always, and the
/// scheme text when the call succeeds.
pub(crate) fn check_schema(id: &str, url: &str, size: u32, hr: u32, rsize: u32, scheme: &str) {
    let got: Schema = parse_schema(&utf16(url), size);
    let got_hr = match &got.result {
        Ok(_) => S_OK,
        Err(e) => e.hresult(),
    };
    assert_eq!((got_hr, got.rsize), (hr, rsize), "{id}");
    if let Ok(buf) = got.result {
        assert_eq!(buf.last(), Some(&0), "{id} NUL");
        assert_eq!(
            String::from_utf16(&buf[..buf.len() - 1]).unwrap(),
            scheme,
            "{id}"
        );
    }
}

#[test]
fn oracle_find_domain_name_matches_wine() {
    for &(id, host, expected) in FIND_ROWS {
        let got = find_domain_name(&utf16(host)).map_or(-1, |start| start as i32);
        assert_eq!(got, expected, "{id}");
    }
}

#[test]
fn oracle_parse_schema_matches_wine() {
    for &(id, url, size, hr, rsize, scheme) in SCHEMA_ROWS {
        check_schema(id, url, size, hr, rsize, scheme);
    }
}

#[test]
fn oracle_matches_domain_pattern_matches_wine() {
    for &(id, pattern, host, implicit, expected) in PATTERN_ROWS {
        let got = matches_domain_pattern(&utf16(pattern), &utf16(host), implicit);
        assert_eq!(got, expected, "{id}");
    }
}

#[test]
fn oracle_zone_and_scheme_match_wine() {
    assert!(is_known_zone(URLZONE_UNTRUSTED), "O22");
    assert!(!is_known_zone(URLZONE_INVALID), "O23");
    assert!(is_hierarchical_scheme(URL_SCHEME_FILE), "O24");
}
