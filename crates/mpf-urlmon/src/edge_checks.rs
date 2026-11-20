//! Replays of the edge rows E01 to E46, the truth tables and the enum values in
//! `target/canon/urlmon/rows.txt`, and Rust-only tests for the branches that no
//! oracle row reaches. E39 is not replayed: it matches `É` against `é` only
//! through Wine's Unicode case folding, which this port does not do.

use crate::domain::find_domain_name;
use crate::oracle_checks::{check_schema, utf16, E_POINTER, S_OK};
use crate::pattern::matches_domain_pattern;
use crate::scheme::{
    is_hierarchical_scheme, URL_SCHEME_FILE, URL_SCHEME_FTP, URL_SCHEME_GOPHER, URL_SCHEME_HTTP,
    URL_SCHEME_HTTPS, URL_SCHEME_NNTP, URL_SCHEME_RES, URL_SCHEME_TELNET, URL_SCHEME_WAIS,
};
use crate::zone::{
    is_known_zone, URLZONE_INTERNET, URLZONE_INTRANET, URLZONE_INVALID, URLZONE_LOCAL_MACHINE,
    URLZONE_TRUSTED, URLZONE_UNTRUSTED,
};

const EDGE_FIND: &[(&str, &str, i32)] = &[
    ("E01", "hostname", -1),
    ("E02", "a.b", -1),
    ("E03", "x.y.test", 2),
    ("E04", "a..uk", 0),
    ("E05", "a.b.c", 0),
    ("E06", "abc.", 0),
    ("E07", "com.", -1),
    ("E08", "www.a.b.com", 6),
    ("E09", "\u{e9}k.uk", 0),
    ("E10", "\u{e9}.uk", 0),
    ("E11", "abcde.fg", 0),
    ("E12", "www.google.edu.uk", 4),
    ("E13", "x.com.uk", 0),
    ("E14", "x.y.zz.uk", 2),
    ("E15", "a.bcd.ef", 2),
    ("E16", "", -1),
    ("E42", "a...uk", 2),
];

const EDGE_SCHEMA: &[(&str, &str, u32, u32, u32, &str)] = &[
    ("E17", "a:b", 2, S_OK, 1, "a"),
    ("E18", "mailto", 0, E_POINTER, 0, ""),
    ("E19", ":x", 1, S_OK, 0, ""),
    ("E20", "a:b:c", 3, S_OK, 1, "a"),
    ("E21", "", 1, S_OK, 0, ""),
    ("E22", "abcdef:", 7, S_OK, 6, "abcdef"),
    ("E23", "abcdef:", 6, E_POINTER, 6, ""),
    ("E40", "a\0b:c", 4, S_OK, 0, ""),
];

const EDGE_PATTERN: &[(&str, &str, &str, bool, Option<usize>)] = &[
    ("E24", "*.com", "a.com", false, None),
    ("E25", "*.com", "ab.com", false, Some(0)),
    ("E26", "*.com", "x.org", true, None),
    ("E27", "", "a.", true, Some(2)),
    ("E28", "*.google.com", "www.Google.COM", false, Some(0)),
    ("E29", "Google.COM", "google.com", false, Some(0)),
    ("E30", "google.com", "a.google.com", true, Some(2)),
    ("E31", "*.google.com", "google.com", true, None),
    ("E32", "google.com", "google.com", true, Some(0)),
    ("E33", "google.com", "a.google.com", false, None),
    ("E34", "*.google.com", "x.test.google.com", true, Some(2)),
    ("E35", "*.google.com", "x.test.google.com", false, None),
    ("E36", "google.com", "", true, None),
    ("E37", "*.google.com", "a.b.google.org", true, None),
    ("E38", "google.com", "a.googly.com", true, None),
    ("E41", "google.com\0x", "www.google.com", true, Some(4)),
    ("E43", "*", "abc", false, None),
    ("E44", "*", "*", false, Some(0)),
    ("E45", "", "", false, Some(0)),
    ("E46", "", "", true, Some(0)),
];

/// `is_known_zone` for each zone value in the truth table.
const ZONE_TABLE: &[(i32, bool)] = &[
    (-1, false),
    (0, true),
    (1, true),
    (2, true),
    (3, true),
    (4, true),
    (5, false),
    (999, false),
    (1000, false),
    (10000, false),
];

/// The scheme values in `1..=` that `is_hierarchical_scheme` accepts. Every other
/// value from -1 to 26 is rejected.
const HIERARCHICAL_SCHEMES: &[i32] = &[1, 2, 3, 6, 7, 8, 9, 11, 18];

#[test]
fn edge_find_domain_name_matches_wine() {
    for &(id, host, expected) in EDGE_FIND {
        let got = find_domain_name(&utf16(host)).map_or(-1, |start| start as i32);
        assert_eq!(got, expected, "{id}");
    }
}

#[test]
fn edge_parse_schema_matches_wine() {
    for &(id, url, size, hr, rsize, scheme) in EDGE_SCHEMA {
        check_schema(id, url, size, hr, rsize, scheme);
    }
}

#[test]
fn edge_matches_domain_pattern_matches_wine() {
    for &(id, pattern, host, implicit, expected) in EDGE_PATTERN {
        let got = matches_domain_pattern(&utf16(pattern), &utf16(host), implicit);
        assert_eq!(got, expected, "{id}");
    }
}

#[test]
fn edge_zone_table_matches_wine() {
    for &(zone, expected) in ZONE_TABLE {
        assert_eq!(is_known_zone(zone), expected, "zone {zone}");
    }
}

#[test]
fn edge_scheme_table_matches_wine() {
    for scheme in -1..=26 {
        assert_eq!(
            is_hierarchical_scheme(scheme),
            HIERARCHICAL_SCHEMES.contains(&scheme),
            "scheme {scheme}"
        );
    }
}

#[test]
fn enum_values_match_wine() {
    assert_eq!(URLZONE_INVALID, -1);
    assert_eq!(URLZONE_LOCAL_MACHINE, 0);
    assert_eq!(URLZONE_INTRANET, 1);
    assert_eq!(URLZONE_TRUSTED, 2);
    assert_eq!(URLZONE_INTERNET, 3);
    assert_eq!(URLZONE_UNTRUSTED, 4);
    assert_eq!(URL_SCHEME_FTP, 1);
    assert_eq!(URL_SCHEME_HTTP, 2);
    assert_eq!(URL_SCHEME_GOPHER, 3);
    assert_eq!(URL_SCHEME_NNTP, 6);
    assert_eq!(URL_SCHEME_TELNET, 7);
    assert_eq!(URL_SCHEME_WAIS, 8);
    assert_eq!(URL_SCHEME_FILE, 9);
    assert_eq!(URL_SCHEME_HTTPS, 11);
    assert_eq!(URL_SCHEME_RES, 18);
}

#[test]
fn pattern_longer_than_host_never_matches() {
    assert_eq!(matches_domain_pattern(&utf16("*"), &utf16(""), true), None);
    assert_eq!(
        matches_domain_pattern(&utf16("*.a"), &utf16("a"), false),
        None
    );
}
