//! Replays of oracle rows E01 to E23, and tests for the branches that the
//! oracle rows do not reach. Every helper and closure here runs, so the
//! coverage gate counts them.

use crate::authority::{parse_password, parse_port, parse_userinfo, parse_username};
use crate::chars::{
    check_pct_encoded, is_alpha, is_auth_delim, is_forbidden_dos_path_char, is_hexdigit,
    is_implicit_file_path, is_num, is_path_delim, is_slash, is_subdelim, is_unreserved,
};
use crate::domain::find_domain_name;
use crate::flags::{
    apply_default_flags, has_invalid_flag_combination, ALLOW_NULL_TERM_PASSWORD,
    ALLOW_NULL_TERM_SCHEME, ALLOW_NULL_TERM_USER_NAME, URI_CREATE_ALLOW_IMPLICIT_FILE_SCHEME,
    URI_CREATE_CANONICALIZE, URI_CREATE_CRACK_UNKNOWN_SCHEMES, URI_CREATE_DECODE_EXTRA_INFO,
    URI_CREATE_FILE_USE_DOS_PATH, URI_CREATE_IE_SETTINGS, URI_CREATE_NO_CANONICALIZE,
    URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES, URI_CREATE_NO_DECODE_EXTRA_INFO,
    URI_CREATE_NO_IE_SETTINGS, URI_CREATE_NO_PRE_PROCESS_HTML_URI, URI_CREATE_PRE_PROCESS_HTML_URI,
};
use crate::host::{
    check_dec_octet, check_implicit_ipv4, check_ipv4address, parse_ipv4address, parse_ipvfuture,
};
use crate::oracle_checks::{text_is, wide};
use crate::path::{
    find_file_extension, parse_fragment, parse_path_hierarchical, parse_path_opaque, parse_query,
    remove_dot_segments,
};
use crate::scheme::{
    is_hierarchical_scheme, is_hierarchical_uri, parse_scheme, parse_scheme_name,
    parse_scheme_type, URL_SCHEME_FILE, URL_SCHEME_FTP, URL_SCHEME_GOPHER, URL_SCHEME_HTTP,
    URL_SCHEME_HTTPS, URL_SCHEME_MAILTO, URL_SCHEME_NNTP, URL_SCHEME_RES, URL_SCHEME_TELNET,
    URL_SCHEME_UNKNOWN, URL_SCHEME_WAIS, URL_SCHEME_WILDCARD,
};
use crate::{at, eq_ignore_ascii_case, ParseData, URI_HOST_IPV4};

#[test]
fn e01_port_above_65535_fails() {
    let s = wide("70000");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_port(&s, &mut pos, &mut d));
    assert_eq!(pos, 0);
    assert!(d.port.is_none());
}

#[test]
fn e02_port_stops_at_slash() {
    let s = wide("8080/");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_port(&s, &mut pos, &mut d));
    assert_eq!(pos, 4);
    assert!(text_is(d.port, "8080"));
    assert_eq!(d.port_value, 8080);
    assert!(d.has_port);
}

#[test]
fn e03_ipv4_keeps_colon_for_unknown_scheme() {
    let s = wide("10.0.0.1:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 9);
    assert!(text_is(d.host, "10.0.0.1:"));
    assert_eq!(d.host_type, URI_HOST_IPV4);
    assert!(!d.has_port);
    assert!(text_is(d.port, ""));
}

#[test]
fn e04_ipvfuture_needs_hex_digit() {
    let s = wide("v.1");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_ipvfuture(&s, &mut pos, &mut d));
    assert_eq!(pos, 0);
}

#[test]
fn e05_drive_path_needs_implicit_file_flag() {
    let s = wide("C:\\x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme(&s, &mut pos, &mut d, 0, 0));
    assert_eq!(pos, 0);
}

#[test]
fn e06_scheme_before_colon() {
    let s = wide("foo:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme(&s, &mut pos, &mut d, 0, 0));
    assert_eq!(pos, 4);
    assert!(text_is(d.scheme, "foo"));
    assert_eq!(d.scheme_type, URL_SCHEME_UNKNOWN);
}

#[test]
fn e07_scheme_name_needs_colon() {
    let s = wide("foo");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme_name(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
}

#[test]
fn e08_scheme_name_may_end_at_nul() {
    let s = wide("foo");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme_name(
        &s,
        &mut pos,
        &mut d,
        ALLOW_NULL_TERM_SCHEME
    ));
    assert_eq!(pos, 4);
    assert!(text_is(d.scheme, "foo"));
}

#[test]
fn e09_query_stops_at_fragment() {
    let s = wide("?a=1#f");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_query(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 4);
    assert!(text_is(d.query, "?a=1"));
}

#[test]
fn e10_query_bad_escape_fails() {
    let s = wide("?a%zz");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_query(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
    assert!(d.query.is_none());
}

#[test]
fn e11_username_may_end_at_nul() {
    let s = wide("abc");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_username(
        &s,
        &mut pos,
        &mut d,
        ALLOW_NULL_TERM_USER_NAME
    ));
    assert_eq!(pos, 3);
    assert!(text_is(d.username, "abc"));
}

#[test]
fn e12_password_may_end_at_nul() {
    let s = wide("abc");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_password(
        &s,
        &mut pos,
        &mut d,
        ALLOW_NULL_TERM_PASSWORD
    ));
    assert_eq!(pos, 3);
    assert!(text_is(d.password, "abc"));
}

#[test]
fn e13_single_flag_is_valid() {
    assert!(!has_invalid_flag_combination(URI_CREATE_DECODE_EXTRA_INFO));
}

#[test]
fn e14_extension_ends_at_slash() {
    assert_eq!(find_file_extension(&wide("a.b/c")), -1);
}

#[test]
fn e15_edu_uk_has_no_domain() {
    assert_eq!(find_domain_name(&wide("edu.uk")), -1);
}

#[test]
fn e16_foo_uk_domain_is_whole_host() {
    assert_eq!(find_domain_name(&wide("foo.uk")), 0);
}

#[test]
fn e17_short_host_has_no_domain() {
    assert_eq!(find_domain_name(&wide("a.b")), -1);
}

#[test]
fn e18_dot_segment_is_removed() {
    let mut path = wide("/a/./b");
    let n = remove_dot_segments(&mut path);
    assert_eq!(n, 4);
    assert!(text_is(Some(&path[..n]), "/a/b"));
}

#[test]
fn e19_leading_dot_dot_is_removed() {
    let mut path = wide("/../a");
    let n = remove_dot_segments(&mut path);
    assert_eq!(n, 2);
    assert!(text_is(Some(&path[..n]), "/a"));
}

#[test]
fn e20_trailing_dot_dot_keeps_slash() {
    let mut path = wide("/a/b/..");
    let n = remove_dot_segments(&mut path);
    assert_eq!(n, 3);
    assert!(text_is(Some(&path[..n]), "/a/"));
}

#[test]
fn e21_double_slash_after_http() {
    let s = wide("//x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 2);
}

#[test]
fn e22_double_backslash_after_http() {
    let s = wide("\\\\x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 2);
}

#[test]
fn e23_single_slash_is_not_hierarchical() {
    let s = wide("/x");
    let mut pos = 0;
    let d = ParseData::default();
    assert!(!is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 0);
}

/// The number of ASCII units, from 0 to 0x7F, that `pred` accepts.
fn ascii_count(pred: impl Fn(u16) -> bool) -> usize {
    (0..0x80).filter(|&c| pred(c)).count()
}

#[test]
fn at_reads_past_end_as_nul() {
    let s = wide("ab");
    assert_eq!(at(&s, 1), u16::from(b'b'));
    assert_eq!(at(&s, 2), 0);
}

#[test]
fn eq_ignore_ascii_case_folds_ascii_only() {
    assert!(eq_ignore_ascii_case("http", &wide("HtTp")));
    assert!(!eq_ignore_ascii_case("http", &wide("htxp")));
    assert!(!eq_ignore_ascii_case("http", &wide("https")));
    assert!(!eq_ignore_ascii_case("http", &wide("\u{e9}ttp")));
}

#[test]
fn ascii_class_counts() {
    assert_eq!(ascii_count(is_alpha), 52);
    assert_eq!(ascii_count(is_num), 10);
    assert_eq!(ascii_count(is_hexdigit), 22);
    assert_eq!(ascii_count(is_unreserved), 66);
    assert_eq!(ascii_count(is_subdelim), 11);
    assert_eq!(ascii_count(|c| is_auth_delim(c, false)), 4);
    assert_eq!(ascii_count(|c| is_auth_delim(c, true)), 5);
    assert_eq!(ascii_count(is_slash), 2);
    assert_eq!(ascii_count(|c| is_path_delim(URL_SCHEME_HTTP, c)), 3);
    assert_eq!(ascii_count(|c| is_path_delim(URL_SCHEME_FILE, c)), 2);
    assert_eq!(ascii_count(is_forbidden_dos_path_char), 3);
}

#[test]
fn non_ascii_units_are_not_class_members() {
    assert!(!is_unreserved(0xE9));
    assert!(!is_subdelim(0xE9));
}

#[test]
fn implicit_file_path_shapes() {
    let s = wide("\\\\x");
    assert!(is_implicit_file_path(&s, 0));
    let s = wide("C:");
    assert!(is_implicit_file_path(&s, 0));
    let s = wide("Cx");
    assert!(!is_implicit_file_path(&s, 0));
    let s = wide("1:");
    assert!(!is_implicit_file_path(&s, 0));
}

#[test]
fn pct_encoded_needs_two_hex_digits() {
    let mut pos = 0;
    let s = wide("x");
    assert!(!check_pct_encoded(&s, &mut pos));
    assert_eq!(pos, 0);
    let s = wide("%zz");
    assert!(!check_pct_encoded(&s, &mut pos));
    let s = wide("%4z");
    assert!(!check_pct_encoded(&s, &mut pos));
    let s = wide("%41");
    assert!(check_pct_encoded(&s, &mut pos));
    assert_eq!(pos, 3);
}

#[test]
fn scheme_name_wildcard_only_at_start() {
    let s = wide("*");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme_name(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);

    let s = wide("*:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme_name(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 2);
    assert!(text_is(d.scheme, "*"));

    let s = wide("a*:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme_name(&s, &mut pos, &mut d, 0));
}

#[test]
fn scheme_name_allowed_characters() {
    let s = wide("a+-.9:");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_scheme_name(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 6);
    assert!(text_is(d.scheme, "a+-.9"));
}

#[test]
fn scheme_name_must_end_at_colon_or_allowed_nul() {
    let s = wide("");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme_name(&s, &mut pos, &mut d, 0));

    let s = wide("ab");
    assert!(!parse_scheme_name(&s, &mut pos, &mut d, 0));

    let s = wide("a b");
    assert!(!parse_scheme_name(
        &s,
        &mut pos,
        &mut d,
        ALLOW_NULL_TERM_SCHEME
    ));
    assert_eq!(pos, 0);
}

#[test]
fn scheme_type_lookup() {
    let s = wide("HTTP");
    let mut d = ParseData {
        scheme: Some(&s[..]),
        ..ParseData::default()
    };
    assert!(parse_scheme_type(&mut d));
    assert_eq!(d.scheme_type, URL_SCHEME_HTTP);

    let mut d = ParseData::default();
    assert!(!parse_scheme_type(&mut d));

    let mut d = ParseData {
        is_relative: true,
        ..ParseData::default()
    };
    assert!(parse_scheme_type(&mut d));
    assert_eq!(d.scheme_type, URL_SCHEME_UNKNOWN);
}

#[test]
fn scheme_implicit_file_and_missing_scheme() {
    let s = wide("\\\\x");
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
    assert_eq!(d.scheme_type, URL_SCHEME_FILE);

    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_scheme(&s, &mut pos, &mut d, 0, 0));

    let s = wide("a b");
    assert!(!parse_scheme(&s, &mut pos, &mut d, 0, 0));
}

#[test]
fn hierarchical_scheme_set() {
    let got: Vec<i32> = (0..=23).filter(|&s| is_hierarchical_scheme(s)).collect();
    assert_eq!(
        got,
        [
            URL_SCHEME_FTP,
            URL_SCHEME_HTTP,
            URL_SCHEME_GOPHER,
            URL_SCHEME_NNTP,
            URL_SCHEME_TELNET,
            URL_SCHEME_WAIS,
            URL_SCHEME_FILE,
            URL_SCHEME_HTTPS,
            URL_SCHEME_RES,
        ]
    );
}

#[test]
fn hierarchical_uri_markers() {
    let s = wide("");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_WILDCARD,
        ..ParseData::default()
    };
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 0);

    let s = wide("x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_FILE,
        has_implicit_scheme: true,
        ..ParseData::default()
    };
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 0);

    let s = wide("\\\\x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_FILE,
        ..ParseData::default()
    };
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 2);

    let s = wide("\\x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!is_hierarchical_uri(&s, &mut pos, &d));

    let s = wide("//x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_MAILTO,
        ..ParseData::default()
    };
    assert!(!is_hierarchical_uri(&s, &mut pos, &d));

    let s = wide("x");
    let mut pos = 0;
    let d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!is_hierarchical_uri(&s, &mut pos, &d));

    let s = wide("//x");
    let mut pos = 0;
    let d = ParseData::default();
    assert!(is_hierarchical_uri(&s, &mut pos, &d));
    assert_eq!(pos, 2);
}

#[test]
fn each_opposite_flag_pair_is_invalid() {
    let pairs = [
        (
            URI_CREATE_DECODE_EXTRA_INFO,
            URI_CREATE_NO_DECODE_EXTRA_INFO,
        ),
        (URI_CREATE_CANONICALIZE, URI_CREATE_NO_CANONICALIZE),
        (
            URI_CREATE_CRACK_UNKNOWN_SCHEMES,
            URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES,
        ),
        (
            URI_CREATE_PRE_PROCESS_HTML_URI,
            URI_CREATE_NO_PRE_PROCESS_HTML_URI,
        ),
        (URI_CREATE_IE_SETTINGS, URI_CREATE_NO_IE_SETTINGS),
    ];
    for (a, b) in pairs {
        assert!(has_invalid_flag_combination(a | b));
        assert!(!has_invalid_flag_combination(a));
        assert!(!has_invalid_flag_combination(b));
    }
    assert!(!has_invalid_flag_combination(0));
}

#[test]
fn default_flags_yield_to_their_opposites() {
    assert_eq!(apply_default_flags(0), 0x4B40);
    assert_eq!(apply_default_flags(URI_CREATE_NO_CANONICALIZE), 0x4A50);
    let all_opposites = URI_CREATE_NO_CANONICALIZE
        | URI_CREATE_NO_DECODE_EXTRA_INFO
        | URI_CREATE_NO_CRACK_UNKNOWN_SCHEMES
        | URI_CREATE_NO_PRE_PROCESS_HTML_URI
        | URI_CREATE_IE_SETTINGS;
    assert_eq!(apply_default_flags(all_opposites), all_opposites);
}

#[test]
fn username_scan_rules() {
    let s = wide("%41x@");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_username(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 4);
    assert!(text_is(d.username, "%41x"));

    let s = wide("%zz@");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_username(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 3);
    assert!(text_is(d.username, "%zz"));

    let s = wide("%zz@");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_username(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 0);
    assert!(d.username.is_none());

    let s = wide("a\\b@");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_username(&s, &mut pos, &mut d, 0));
    assert!(d.username.is_none());

    let s = wide("a\\b@");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_username(&s, &mut pos, &mut d, 0));
    assert_eq!(pos, 3);
    assert!(text_is(d.username, "a\\b"));
}

#[test]
fn password_stops_at_slash_in_any_scheme() {
    let s = wide("/");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(!parse_password(&s, &mut pos, &mut d, 0));
    assert!(d.password.is_none());

    let mut d = ParseData::default();
    assert!(!parse_password(&s, &mut pos, &mut d, 0));
    assert!(d.password.is_none());
}

#[test]
fn userinfo_without_password() {
    let s = wide("a@");
    let mut pos = 0;
    let mut d = ParseData::default();
    parse_userinfo(&s, &mut pos, &mut d);
    assert_eq!(pos, 2);
    assert!(text_is(d.username, "a"));
}

#[test]
fn userinfo_bad_password_rewinds() {
    let s = wide("a:/@");
    let mut pos = 0;
    let mut d = ParseData::default();
    parse_userinfo(&s, &mut pos, &mut d);
    assert_eq!(pos, 0);
    assert!(d.username.is_none());
}

#[test]
fn port_digits_and_range() {
    let s = wide("12a");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_port(&s, &mut pos, &mut d));
    assert!(d.port.is_none());

    let s = wide("65535");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_port(&s, &mut pos, &mut d));
    assert_eq!(d.port_value, 65535);

    let s = wide("65536");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_port(&s, &mut pos, &mut d));
}

#[test]
fn port_ends_at_backslash_only_for_known_schemes() {
    let s = wide("80\\x");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_port(&s, &mut pos, &mut d));
    assert_eq!(pos, 2);

    let s = wide("80\\x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_port(&s, &mut pos, &mut d));
}

/// Checks one `dec-octet` at the start of `text`, and returns its length.
fn dec_octet(text: &str) -> Option<usize> {
    let s = wide(text);
    let mut pos = 0;
    check_dec_octet(&s, &mut pos).then_some(pos)
}

#[test]
fn dec_octet_shapes() {
    assert_eq!(dec_octet("a"), None);
    assert_eq!(dec_octet("1a"), Some(1));
    assert_eq!(dec_octet("12a"), Some(2));
    assert_eq!(dec_octet("123"), Some(3));
    assert_eq!(dec_octet("199"), Some(3));
    assert_eq!(dec_octet("200"), Some(3));
    assert_eq!(dec_octet("250"), Some(3));
    assert_eq!(dec_octet("255"), None);
    assert_eq!(dec_octet("300"), Some(3));
}

/// Checks an implicit IPv4 address at the start of `text`, and returns its length.
fn implicit_ipv4(text: &str) -> Option<usize> {
    let s = wide(text);
    let mut pos = 0;
    check_implicit_ipv4(&s, &mut pos).then_some(pos)
}

#[test]
fn implicit_ipv4_shapes() {
    assert_eq!(implicit_ipv4("a"), None);
    assert_eq!(implicit_ipv4("123a"), Some(3));
}

/// Checks a dotted IPv4 address at the start of `text`, and returns its length.
fn dotted_ipv4(text: &str, strict: bool) -> Option<usize> {
    let s = wide(text);
    let mut pos = 0;
    check_ipv4address(&s, &mut pos, strict).then_some(pos)
}

#[test]
fn dotted_ipv4_shapes() {
    assert_eq!(dotted_ipv4("1x", false), None);
    assert_eq!(dotted_ipv4("x", false), None);
    assert_eq!(dotted_ipv4("1.x", false), None);
    assert_eq!(dotted_ipv4("1.2x", false), Some(3));
    assert_eq!(dotted_ipv4("1.2x", true), None);
    assert_eq!(dotted_ipv4("1.2.3x", false), Some(5));
    assert_eq!(dotted_ipv4("1.2.3x", true), None);
    assert_eq!(dotted_ipv4("1.2.x", false), None);
    assert_eq!(dotted_ipv4("1.2.3.x", false), None);
    assert_eq!(dotted_ipv4("1.2.3.4", true), Some(7));
}

#[test]
fn ipv4_host_shapes() {
    let s = wide("10.0");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 4);
    assert!(text_is(d.host, "10.0"));

    let s = wide("10.0.0.1/x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 8);
    assert!(text_is(d.host, "10.0.0.1"));

    let s = wide("10.0.0.1x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_ipv4address(&s, &mut pos, &mut d));
    assert!(d.host.is_none());

    let s = wide("10.0.0.1:x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_ipv4address(&s, &mut pos, &mut d));
    assert!(d.host.is_none());

    let s = wide("10.0.0.1:80");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 11);
    assert!(text_is(d.host, "10.0.0.1"));
    assert!(d.has_port);

    let s = wide("10.0.0.1:");
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type: URL_SCHEME_HTTP,
        ..ParseData::default()
    };
    assert!(parse_ipv4address(&s, &mut pos, &mut d));
    assert_eq!(pos, 9);
    assert!(text_is(d.host, "10.0.0.1"));
    assert!(!d.has_port);

    let s = wide("x");
    let mut pos = 0;
    let mut d = ParseData::default();
    assert!(!parse_ipv4address(&s, &mut pos, &mut d));
    assert!(d.host.is_none());
}

/// Checks an IPvFuture host at the start of `text`, and returns its length.
fn ipvfuture(text: &str) -> Option<usize> {
    let s = wide(text);
    let mut pos = 0;
    let mut d = ParseData::default();
    parse_ipvfuture(&s, &mut pos, &mut d).then_some(pos)
}

#[test]
fn ipvfuture_shapes() {
    assert_eq!(ipvfuture("x"), None);
    assert_eq!(ipvfuture("v"), None);
    assert_eq!(ipvfuture("vA"), None);
    assert_eq!(ipvfuture("v1.@"), None);
    assert_eq!(ipvfuture("v1.:"), Some(4));
    assert_eq!(ipvfuture("v1.!"), Some(4));
    assert_eq!(ipvfuture("vAb.x"), Some(5));
    assert_eq!(ipvfuture("V1.a"), Some(4));
    assert_eq!(ipvfuture("v1.\u{e9}"), None);
}

/// Parses a hierarchical path, and returns the result, the end and the path.
fn hierarchical(
    text: &str,
    scheme_type: i32,
    must_have_path: bool,
    flags: u32,
) -> (bool, usize, Option<String>) {
    let s = wide(text);
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type,
        must_have_path,
        ..ParseData::default()
    };
    let ok = parse_path_hierarchical(&s, &mut pos, &mut d, flags);
    (ok, pos, d.path.map(String::from_utf16_lossy))
}

#[test]
fn hierarchical_path_shapes() {
    assert_eq!(
        hierarchical("", URL_SCHEME_WILDCARD, false, 0),
        (true, 0, None)
    );
    assert_eq!(
        hierarchical("", URL_SCHEME_WILDCARD, true, 0),
        (true, 0, Some("/".to_string()))
    );
    assert_eq!(
        hierarchical("", URL_SCHEME_HTTP, false, URI_CREATE_NO_CANONICALIZE),
        (true, 0, None)
    );
    assert_eq!(
        hierarchical("#x", URL_SCHEME_HTTP, false, 0),
        (true, 0, Some("/".to_string()))
    );
    assert_eq!(
        hierarchical("%41", URL_SCHEME_HTTP, false, 0),
        (true, 3, Some("%41".to_string()))
    );
    assert_eq!(
        hierarchical("%zz", URL_SCHEME_UNKNOWN, false, 0),
        (true, 3, Some("%zz".to_string()))
    );
    assert_eq!(
        hierarchical("%zz", URL_SCHEME_FILE, false, 0),
        (true, 3, Some("%zz".to_string()))
    );
    assert_eq!(
        hierarchical("%zz", URL_SCHEME_HTTP, false, 0),
        (false, 0, None)
    );
    assert_eq!(
        hierarchical("<", URL_SCHEME_FILE, false, URI_CREATE_FILE_USE_DOS_PATH),
        (false, 0, None)
    );
    assert_eq!(
        hierarchical("<", URL_SCHEME_FILE, false, 0),
        (true, 1, Some("<".to_string()))
    );
    assert_eq!(
        hierarchical("<", URL_SCHEME_HTTP, false, URI_CREATE_FILE_USE_DOS_PATH),
        (true, 1, Some("<".to_string()))
    );
    assert_eq!(
        hierarchical("a\\b", URL_SCHEME_HTTP, false, URI_CREATE_NO_CANONICALIZE),
        (false, 0, None)
    );
    assert_eq!(
        hierarchical("a\\b", URL_SCHEME_FILE, false, URI_CREATE_NO_CANONICALIZE),
        (true, 3, Some("a\\b".to_string()))
    );
    assert_eq!(
        hierarchical(
            "a\\b",
            URL_SCHEME_UNKNOWN,
            false,
            URI_CREATE_NO_CANONICALIZE
        ),
        (true, 3, Some("a\\b".to_string()))
    );
    assert_eq!(
        hierarchical("a\\b", URL_SCHEME_HTTP, false, 0),
        (true, 3, Some("a\\b".to_string()))
    );
    assert_eq!(
        hierarchical("a?b", URL_SCHEME_HTTP, false, 0),
        (true, 1, Some("a".to_string()))
    );
    assert_eq!(
        hierarchical("a#b", URL_SCHEME_FILE, false, 0),
        (true, 3, Some("a#b".to_string()))
    );
    assert_eq!(
        hierarchical("a#b", URL_SCHEME_HTTP, false, 0),
        (true, 1, Some("a".to_string()))
    );
}

/// Parses an opaque path, and returns the result, the end and the path.
fn opaque(text: &str, scheme_type: i32, flags: u32) -> (bool, usize, Option<String>) {
    let s = wide(text);
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type,
        ..ParseData::default()
    };
    let ok = parse_path_opaque(&s, &mut pos, &mut d, flags);
    (ok, pos, d.path.map(String::from_utf16_lossy))
}

#[test]
fn opaque_path_shapes() {
    assert_eq!(opaque("//", URL_SCHEME_MAILTO, 0), (true, 2, None));
    assert_eq!(
        opaque("/a", URL_SCHEME_MAILTO, 0),
        (true, 2, Some("/a".to_string()))
    );
    assert_eq!(
        opaque("a", URL_SCHEME_MAILTO, 0),
        (true, 1, Some("a".to_string()))
    );
    assert_eq!(
        opaque("a", URL_SCHEME_HTTP, 0),
        (true, 1, Some("a".to_string()))
    );
    assert_eq!(opaque("%zz", URL_SCHEME_HTTP, 0), (false, 0, None));
    assert_eq!(
        opaque("%zz", URL_SCHEME_UNKNOWN, 0),
        (true, 3, Some("%zz".to_string()))
    );
    assert_eq!(
        opaque("%41", URL_SCHEME_HTTP, 0),
        (true, 3, Some("%41".to_string()))
    );
    assert_eq!(
        opaque("<", URL_SCHEME_FILE, URI_CREATE_FILE_USE_DOS_PATH),
        (false, 0, None)
    );
    assert_eq!(
        opaque("<", URL_SCHEME_FILE, 0),
        (true, 1, Some("<".to_string()))
    );
    assert_eq!(
        opaque("<", URL_SCHEME_HTTP, URI_CREATE_FILE_USE_DOS_PATH),
        (true, 1, Some("<".to_string()))
    );
    assert_eq!(
        opaque("a?b", URL_SCHEME_HTTP, 0),
        (true, 1, Some("a".to_string()))
    );
}

/// Parses a query, and returns the result, the end and the query.
fn query(text: &str, scheme_type: i32, flags: u32) -> (bool, usize, Option<String>) {
    let s = wide(text);
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type,
        ..ParseData::default()
    };
    let ok = parse_query(&s, &mut pos, &mut d, flags);
    (ok, pos, d.query.map(String::from_utf16_lossy))
}

#[test]
fn query_shapes() {
    assert_eq!(query("a", URL_SCHEME_UNKNOWN, 0), (true, 0, None));
    assert_eq!(
        query("?", URL_SCHEME_UNKNOWN, 0),
        (true, 1, Some("?".to_string()))
    );
    assert_eq!(
        query("?%41", URL_SCHEME_HTTP, 0),
        (true, 4, Some("?%41".to_string()))
    );
    assert_eq!(
        query("?%zz", URL_SCHEME_UNKNOWN, 0),
        (true, 4, Some("?%zz".to_string()))
    );
    assert_eq!(
        query("?%zz", URL_SCHEME_HTTP, URI_CREATE_NO_DECODE_EXTRA_INFO),
        (true, 4, Some("?%zz".to_string()))
    );
    assert_eq!(query("?%zz", URL_SCHEME_HTTP, 0), (false, 0, None));
}

/// Parses a fragment, and returns the result, the end and the fragment.
fn fragment(text: &str, scheme_type: i32, flags: u32) -> (bool, usize, Option<String>) {
    let s = wide(text);
    let mut pos = 0;
    let mut d = ParseData {
        scheme_type,
        ..ParseData::default()
    };
    let ok = parse_fragment(&s, &mut pos, &mut d, flags);
    (ok, pos, d.fragment.map(String::from_utf16_lossy))
}

#[test]
fn fragment_shapes() {
    assert_eq!(fragment("a", URL_SCHEME_UNKNOWN, 0), (true, 0, None));
    assert_eq!(
        fragment("#x", URL_SCHEME_UNKNOWN, 0),
        (true, 2, Some("#x".to_string()))
    );
    assert_eq!(
        fragment("#%41", URL_SCHEME_HTTP, 0),
        (true, 4, Some("#%41".to_string()))
    );
    assert_eq!(
        fragment("#%zz", URL_SCHEME_UNKNOWN, 0),
        (true, 4, Some("#%zz".to_string()))
    );
    assert_eq!(
        fragment("#%zz", URL_SCHEME_HTTP, URI_CREATE_NO_DECODE_EXTRA_INFO),
        (true, 4, Some("#%zz".to_string()))
    );
    assert_eq!(fragment("#%zz", URL_SCHEME_HTTP, 0), (false, 0, None));
}

/// Removes the dot segments of `text`, and returns the length and the result.
fn dots(text: &str) -> (usize, String) {
    let mut path = wide(text);
    let n = remove_dot_segments(&mut path);
    (n, String::from_utf16_lossy(&path[..n]))
}

#[test]
fn dot_segment_shapes() {
    assert_eq!(dots("/a/./../b"), (2, "/b".to_string()));
    assert_eq!(dots("a/../b"), (1, "b".to_string()));
    assert_eq!(dots("abc"), (3, "abc".to_string()));
    assert_eq!(dots("/a/"), (3, "/a/".to_string()));
    assert_eq!(dots("/a/."), (3, "/a/".to_string()));
    assert_eq!(dots("/.a"), (3, "/.a".to_string()));
    assert_eq!(dots("/..a"), (4, "/..a".to_string()));
    assert_eq!(dots(""), (0, String::new()));
    assert_eq!(dots("/a/.."), (1, "/".to_string()));
    assert_eq!(dots("/abc/../d"), (2, "/d".to_string()));
}

#[test]
fn file_extension_is_last_dot_in_last_segment() {
    assert_eq!(find_file_extension(&wide("")), -1);
    assert_eq!(find_file_extension(&wide("ab")), -1);
    assert_eq!(find_file_extension(&wide("a.b")), 1);
}

/// Finds the domain name in `text`.
fn domain_of(text: &str) -> i32 {
    find_domain_name(&wide(text))
}

#[test]
fn domain_name_shapes() {
    assert_eq!(domain_of("www.google.com.uk"), 4);
    assert_eq!(domain_of("www.google.foo.uk"), 11);
    assert_eq!(domain_of("www.google.fo.uk"), 4);
    assert_eq!(domain_of("www.google.test.us"), 11);
    assert_eq!(domain_of("www.google.org.uk"), 4);
    assert_eq!(domain_of("www.google.COM.uk"), 4);
    assert_eq!(domain_of("a..uk"), 0);
    assert_eq!(domain_of("a.b.test"), 2);
    assert_eq!(domain_of("hostname"), -1);
    assert_eq!(domain_of("ak.uk"), -1);
    assert_eq!(domain_of("\u{e9}k.uk"), 0);
    assert_eq!(domain_of(".com"), 0);
    assert_eq!(domain_of("abcde.fg"), 0);
    assert_eq!(domain_of("a.uk"), -1);
}
