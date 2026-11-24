//! Edge cases that the oracle rows do not reach: empty and blank lines, tabs, NUL truncation, the size
//! boundary, sign and clamp handling, and the zero-capacity deviation. The `E` rows are Wine's output
//! from `target/canon/hnetcfg/main.c`. The last two tests are Rust-only checks of the port.

use crate::decimal_int;
use crate::is_valid_protocol;
use crate::oracle_checks::{show_loc, show_version, w};
use crate::ssdp;
use crate::upnp;

type SearchRow = (&'static str, &'static str, usize, &'static str);
type ServiceRow = (&'static str, &'static str, &'static str);

const SIZE: usize = 512;

#[rustfmt::skip]
const SEARCH_EDGE: &[SearchRow] = &[
    ("E01", "", SIZE, "none"),
    ("E02", " \r\nLOCATION: http://a/\r\n\r\n", SIZE, "none"),
    ("E03", "   \nHTTP/1.1 200 OK\r\nLOCATION: http://a/\r\n\r\n", SIZE, "none"),
    ("E04", "HTTP/1.1\r\nLOCATION: http://a/\r\n\r\n", SIZE, "none"),
    ("E05", "HTTP/1.1 200 OK\r\nLOCATION:\thttp://a/\r\n\r\n", SIZE, "none"),
    ("E06", "HTTP/1.1 200 OK\r\nLOCATION: http://é/\r\n\r\n", SIZE, "loc:http://\\u00e9/"),
    ("E07", "HTTP/1.1 200 OK\r\nLOCATION: http://a/\r\n\r\n", 18, "none"),
    ("E08", "HTTP/1.1 200 OK\r\nLOCATION: http://a/\r\n\r\n", 20, "loc:http://a/"),
    ("E09", "HTTP/1.1 200abc\r\nLOCATION: http://a/\r\n\r\n", SIZE, "loc:http://a/"),
    ("E10", "HTTP/1.1 -200 OK\r\nLOCATION: http://a/\r\n\r\n", SIZE, "none"),
    ("E11", "HTTP/1.1 200 OK\r\n\0LOCATION: http://a/\r\n\r\n", SIZE, "none"),
    ("E12", "HTTP/1.1 200 OK\r\n \nLOCATION: http://a/\r\n\r\n", SIZE, "loc:http://a/"),
    ("E13", "HTTP/1.1 200 OK\r\nlocation: http://a/\r\n\r\n", SIZE, "loc:http://a/"),
    ("E21", "HTTP/1.1 4294967496 OK\r\nLOCATION: http://a/\r\n\r\n", SIZE, "none"),
];

#[rustfmt::skip]
const SERVICE_EDGE: &[ServiceRow] = &[
    ("E14", "", "none"),
    ("E15", "urn:schemas-upnp-org:service:WANIPConnection:  7", "ver:7"),
    ("E16", "urn:schemas-upnp-org:service:WANIPConnection:+5", "ver:5"),
    ("E17", "urn:schemas-upnp-org:service:WANIPConnection:-99999999999", "ver:2147483648"),
    ("E18", "urn:schemas", "none"),
    ("E19", "urn:schemas-upnp-org:service:WANIPConnection:2\0junk", "ver:2"),
];

#[test]
fn search_location_edge_matches_wine() {
    for (label, response, size, want) in SEARCH_EDGE {
        let got = show_loc(ssdp::search_location(response.as_bytes(), *size));
        assert_eq!(got, *want, "row={label}");
    }
}

#[test]
fn wanip_service_version_edge_matches_wine() {
    for (label, value, want) in SERVICE_EDGE {
        let got = show_version(upnp::wanip_service_version(&w(value)));
        assert_eq!(got, *want, "row={label}");
    }
}

#[test]
fn protocol_edge_matches_wine() {
    assert!(is_valid_protocol(Some(&w("UDP\0junk"))), "E20");
}

#[test]
fn zero_capacity_is_none() {
    let response = b"HTTP/1.1 200 OK\r\nLOCATION: http://a/\r\n\r\n";
    assert_eq!(ssdp::search_location(response, 0), None);
    assert_eq!(ssdp::search_location(response, 1), None);
}

#[test]
fn decimal_int_clamps_like_atoi() {
    assert_eq!(decimal_int(b" +7x".iter().map(|&byte| u32::from(byte))), 7);
    assert_eq!(decimal_int(b"-".iter().map(|&byte| u32::from(byte))), 0);
    let big = "9".repeat(40);
    assert_eq!(decimal_int(big.bytes().map(u32::from)), i32::MAX);
    let small = format!("-{big}");
    assert_eq!(decimal_int(small.bytes().map(u32::from)), i32::MIN);
}
