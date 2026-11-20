//! Replays of oracle rows O01 to O25. The rows come from compiling Wine's
//! `uri.c` verbatim with ASan and UBSan. The harness lives in
//! `target/canon/iertutil`, which is not tracked. Its sanitizer log was empty,
//! so no row was excluded.

use crate::authority::parse_userinfo;
use crate::domain::find_domain_name;
use crate::flags::{
    apply_default_flags, has_invalid_flag_combination, URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME,
    URI_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME, URI_CREATE_ALLOW_RELATIVE,
    URI_CREATE_DECODE_EXTRA_INFO, URI_CREATE_NO_DECODE_EXTRA_INFO,
};
use crate::host::{
    check_dec_octet, check_implicit_ipv4, check_ipv4address, parse_ipv4address, parse_ipvfuture,
};
use crate::path::{
    find_file_extension, parse_fragment, parse_path_hierarchical, parse_path_opaque,
    remove_dot_segments,
};
use crate::scheme::{
    parse_scheme, URL_SCHEME_FILE, URL_SCHEME_HTTP, URL_SCHEME_MAILTO, URL_SCHEME_WILDCARD,
};
use crate::{ParseData, URI_HOST_IPV4, URI_HOST_UNKNOWN};

/// The UTF-16 units of `text`.
pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// Whether `got` is present and holds the units of `want`.
pub(crate) fn text_is(got: Option<&[u16]>, want: &str) -> bool {
    got.is_some_and(|g| g == wide(want).as_slice())
}

#[test]
fn host_rows() {
    // O01
    let s = wide("255");
    let mut pos = 0;
    assert!(!check_dec_octet(&s, &mut pos));
    assert_eq!(pos, 0);

    // O02
    let s = wide("300");
    let mut pos = 0;
    assert!(check_dec_octet(&s, &mut pos));
    assert_eq!(pos, 3);

    // O03
    let s = wide("10.0");
    let mut pos = 0;
    assert!(check_ipv4address(&s, &mut pos, false));
    assert_eq!(pos, 4);

    // O04
    let mut pos = 0;
    assert!(!check_ipv4address(&s, &mut pos, true));
    assert_eq!(pos, 0);

    // O05
    let s = wide("4294967295");
    let mut pos = 0;
    assert!(check_implicit_ipv4(&s, &mut pos));
    assert_eq!(pos, 10);

    // O06
    let s = wide("4294967296");
    let mut pos = 0;
    assert!(!check_implicit_ipv4(&s, &mut pos));
    assert_eq!(pos, 0);

    // O07
    let s = wide("256");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 3);
    assert!(text_is(d.host, "256"));
    assert_eq!(d.host_type, URI_HOST_IPV4);

    // O08
    let s = wide("10.0.0.1:80");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 11);
    assert!(text_is(d.host, "10.0.0.1"));
    assert!(text_is(d.port, "80"));
    assert_eq!(d.port_value, 80);
    assert!(d.has_port);

    // O09
    let s = wide("v1.a:b");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_ipvfuture(&s, &mut pos, &mut d));
    assert_eq!(pos, 6);
    assert_eq!(d.host_type, URI_HOST_UNKNOWN);
    assert!(d.host.is_none());
}

#[test]
fn scheme_rows() {
    // O10
    let s = wide("HTTP:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme(&s, &mut pos, &mut d, 0, 0));
    assert_eq!(pos, 5);
    assert!(text_is(d.scheme, "HTTP"));
    assert_eq!(d.scheme_type, URL_SCHEME_HTTP);

    // O11
    let s = wide("C:\\x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme(
        &s,
        &mut pos,
        &mut d,
        URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME,
        0
    ));
    assert_eq!(pos, 0);
    assert!(text_is(d.scheme, "file"));
    assert_eq!(d.scheme_type, URL_SCHEME_FILE);
    assert!(d.has_implicit_scheme);

    // O12
    let s = wide("*");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme(
        &s,
        &mut pos,
        &mut d,
        URI_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME,
        0
    ));
    assert_eq!(pos, 0);
    assert!(text_is(d.scheme, "*"));
    assert_eq!(d.scheme_type, URL_SCHEME_WILDCARD);
    assert!(d.has_implicit_scheme);

    // O13
    let s = wide("a b");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme(
        &s,
        &mut pos,
        &mut d,
        URI_CREATE_ALLOW_RELATIVE,
        0
    ));
    assert_eq!(pos, 0);
    assert!(d.scheme.is_none());
    assert!(d.is_relative);
    assert_eq!(d.scheme_type, 0);
}

#[test]
fn userinfo_rows() {
    // O14
    let s = wide("user:pa:ss@");
    let mut pos = 0;
    let mut d = ParseData::default();
    parse_userinfo(&s, &mut pos, &mut d);
    assert_eq!(pos, 11);
    assert!(text_is(d.username, "user"));
    assert!(text_is(d.password, "pa:ss"));

    // O15
    let s = wide("a/b@");
    let mut pos = 0;
    let mut d = ParseData::default();
    parse_userinfo(&s, &mut pos, &mut d);
    assert_eq!(pos, 0);
    assert!(d.username.is_none());
}

#[test]
fn path_and_fragment_rows() {
    // O16
    let s = wide("");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_path_hierarchical(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
    assert!(text_is(d.path, "/"));

    // O17
    let s = wide("%zz");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_path_hierarchical(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
    assert!(d.path.is_none());

    // O18
    let s = wide("//a@b");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_MAILTO,
        ..ParseData::default()
    };
    assert!(parse_path_opaque(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 5);
    assert!(text_is(d.path, "a@b"));

    // O19
    let s = wide("//%zz");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_MAILTO,
        ..ParseData::default()
    };
    assert!(!parse_path_opaque(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 2);
    assert!(d.path.is_none());

    // O20
    let s = wide("#a%zz");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_fragment(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
    assert!(d.fragment.is_none());
}

#[test]
fn dot_domain_and_file_rows() {
    // O21
    let mut path = wide("/a/b/../c/./d");
    let n = remove_dot_segments(&mut path);
    assert_eq!(n, 6);
    assert!(text_is(Some(&path[..n]), "/a/c/d"));

    // O22
    assert_eq!(find_domain_name(&wide("www.google.com")), 4);

    // O23
    assert_eq!(find_file_extension(&wide("a/b.txt")), 3);
}

#[test]
fn flag_rows() {
    // O24
    assert!(has_invalid_flag_combination(
        URI_CREATE_DECODE_EXTRA_INFO | URI_CREATE_NO_DECODE_EXTRA_INFO
    ));

    // O25
    assert_eq!(apply_default_flags(0), 0x4B40);
}
