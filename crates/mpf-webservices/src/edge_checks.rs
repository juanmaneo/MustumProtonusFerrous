//! Edge checks: inputs outside the oracle rows, the quirks that Wine shows at the edges of its
//! ranges, and the trait and error plumbing. Expectations come from the oracle run in
//! `target/canon/webservices` unless a comment says that they assert this port's own behavior.

use crate::datetime::{
    datetime_to_filetime, filetime_to_datetime, str_to_datetime, DateTime, DateTimeFormat,
};
use crate::text::{
    codepoint_to_utf8, decode_text, str_to_bool, str_to_bytes, str_to_int64, str_to_uint64, trim,
};
use crate::url::{decode_url, encode_url, HttpUrl, Scheme};
use crate::Error;

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// `before`, then the raw units that `str` cannot hold (lone surrogates), then `after`.
fn around(before: &str, raw: &[u16], after: &str) -> Vec<u16> {
    let mut out = wide(before);
    out.extend_from_slice(raw);
    out.extend(after.encode_utf16());
    out
}

fn decode(text: &str) -> Result<HttpUrl, Error> {
    decode_url(&wide(text), 0)
}

fn decode_units(units: &[u16]) -> Result<HttpUrl, Error> {
    decode_url(units, 0)
}

fn encode(url: &HttpUrl) -> Result<String, Error> {
    encode_url(url, 0).map(|units| String::from_utf16(&units).expect("encoded text is ASCII"))
}

fn url_of(
    scheme: Scheme,
    host: &str,
    port: u16,
    port_str: &str,
    path: &str,
    query: &str,
    fragment: &str,
) -> HttpUrl {
    HttpUrl {
        scheme,
        host: wide(host),
        port,
        port_as_string: wide(port_str),
        path: wide(path),
        query: wide(query),
        fragment: wide(fragment),
    }
}

fn utc(ticks: u64) -> DateTime {
    DateTime {
        ticks,
        format: DateTimeFormat::Utc,
    }
}

fn local(ticks: u64) -> DateTime {
    DateTime {
        ticks,
        format: DateTimeFormat::Local,
    }
}

#[test]
fn error_hresults_match_wine() {
    assert_eq!(Error::InvalidArgument.hresult() as u32, 0x8007_0057);
    assert_eq!(Error::NotImplemented.hresult() as u32, 0x8000_4001);
    assert_eq!(Error::InvalidFormat.hresult() as u32, 0x803d_0000);
    assert_eq!(Error::NumericOverflow.hresult() as u32, 0x803d_0002);
    assert_eq!(Error::QuotaExceeded.hresult() as u32, 0x803d_0008);
}

#[test]
fn derived_traits_work() {
    let error = Error::QuotaExceeded;
    assert_eq!(Clone::clone(&error), error);
    assert_ne!(error, Error::InvalidFormat);
    assert_eq!(format!("{error:?}"), "QuotaExceeded");

    let scheme = Scheme::NetPipe;
    assert_eq!(Clone::clone(&scheme), scheme);
    assert_ne!(scheme, Scheme::Http);
    assert_eq!(format!("{scheme:?}"), "NetPipe");

    let parsed = url_of(Scheme::Https, "h", 443, "", "/a", "", "");
    assert_eq!(Clone::clone(&parsed), parsed);
    assert_ne!(parsed, decode("http://h/a").unwrap());
    assert!(format!("{parsed:?}").starts_with("HttpUrl"));

    let kind = DateTimeFormat::Local;
    assert_eq!(Clone::clone(&kind), kind);
    assert_ne!(kind, DateTimeFormat::Utc);
    assert_eq!(format!("{kind:?}"), "Local");

    let datetime = utc(1);
    assert_eq!(Clone::clone(&datetime), datetime);
    assert_ne!(datetime, utc(2));
    assert_eq!(
        format!("{datetime:?}"),
        "DateTime { ticks: 1, format: Utc }"
    );
}

#[test]
fn scheme_default_ports() {
    assert_eq!(Scheme::Http.default_port(), 80);
    assert_eq!(Scheme::Https.default_port(), 443);
    assert_eq!(Scheme::NetTcp.default_port(), 808);
    assert_eq!(Scheme::SoapUdp.default_port(), 65535);
    assert_eq!(Scheme::NetPipe.default_port(), 65535);
}

#[test]
fn scheme_names_ignore_case() {
    assert_eq!(decode("HTTP://h/").unwrap().scheme, Scheme::Http);
    assert_eq!(decode("Https://h/").unwrap().scheme, Scheme::Https);
    assert_eq!(decode("NET.TCP://h/").unwrap().scheme, Scheme::NetTcp);
    assert_eq!(decode("Soap.Udp://h/").unwrap().scheme, Scheme::SoapUdp);
    assert_eq!(decode("Net.Pipe://h/").unwrap().scheme, Scheme::NetPipe);
}

#[test]
fn decode_checks_input_before_flags() {
    assert_eq!(decode_url(&[], 1), Err(Error::InvalidFormat));
    assert_eq!(
        decode_url(&wide("http://h/"), 1),
        Err(Error::NotImplemented)
    );
}

#[test]
fn encode_rejects_flags() {
    let url = url_of(Scheme::Http, "h", 0, "", "", "", "");
    assert_eq!(encode_url(&url, 1), Err(Error::NotImplemented));
}

#[test]
fn decode_rejects_bad_scheme_and_separators() {
    let bad = [
        "http:x",
        "http:/:/",
        "xhttp://h/",
        "http:://h/",
        "http:///p",
        "http://:80/",
        "ftp:",
        "ftp://h/",
        "http:",
    ];
    for text in bad {
        assert_eq!(decode(text), Err(Error::InvalidFormat), "{text}");
    }
}

/// Wine reads one unit past each of these inputs (ASan heap-buffer-overflow), so the oracle has
/// no row for them. These assert the port's own result.
#[test]
fn decode_truncated_input_is_our_behavior() {
    for text in ["http", "http://", "http://[::1"] {
        assert_eq!(decode(text), Err(Error::InvalidFormat), "{text}");
    }
}

#[test]
fn decode_bracketed_host() {
    assert_eq!(
        decode("http://[::1]:8080/").unwrap(),
        url_of(Scheme::Http, "[::1]", 8080, "8080", "/", "", "")
    );
    assert_eq!(
        decode("http://[a/b]/c").unwrap(),
        url_of(Scheme::Http, "[a/b]", 80, "", "/c", "", "")
    );
    assert_eq!(
        decode("http://[]/").unwrap(),
        url_of(Scheme::Http, "[]", 80, "", "/", "", "")
    );
    assert_eq!(
        decode("http://[::1]x/").unwrap(),
        url_of(Scheme::Http, "[::1]", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://[::1/x]/y").unwrap(),
        url_of(Scheme::Http, "[::1/x]", 80, "", "/y", "", "")
    );
}

#[test]
fn decode_port_text() {
    assert_eq!(
        decode("http://h:/").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/", "", "")
    );
    assert_eq!(
        decode("http://h:a/").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://h:0/").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/", "", "")
    );
    assert_eq!(
        decode("http://h:0080/").unwrap(),
        url_of(Scheme::Http, "h", 80, "0080", "/", "", "")
    );
    assert_eq!(
        decode("https://h:0443/").unwrap(),
        url_of(Scheme::Https, "h", 443, "0443", "/", "", "")
    );
    assert_eq!(
        decode("http://h:65535/").unwrap(),
        url_of(Scheme::Http, "h", 65535, "65535", "/", "", "")
    );
    assert_eq!(
        decode("http://h:6553x").unwrap(),
        url_of(Scheme::Http, "h", 6553, "6553", "", "", "")
    );
    assert_eq!(
        decode("http://h:x80/").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://h:80:81/").unwrap(),
        url_of(Scheme::Http, "h", 80, "80", "", "", "")
    );
    assert_eq!(
        decode("http://a:1b/").unwrap(),
        url_of(Scheme::Http, "a", 1, "1", "", "", "")
    );
    assert_eq!(decode("http://h:65536x"), Err(Error::InvalidFormat));
    assert_eq!(decode("http://h:99999"), Err(Error::InvalidFormat));
}

#[test]
fn decode_path_query_and_fragment() {
    assert_eq!(
        decode("http://h/a#b?c").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a#b", "c", "")
    );
    assert_eq!(
        decode("http://h/a?b#c").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a", "b", "c")
    );
    assert_eq!(
        decode("http://h/a?b?c").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a", "b?c", "")
    );
    assert_eq!(
        decode("http://h/a?b#c#d").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a", "b", "c#d")
    );
    assert_eq!(
        decode("http://h/?a#b#c").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/", "a", "b#c")
    );
    assert_eq!(
        decode("http://h?q").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "q", "")
    );
    assert_eq!(
        decode("http://h#f").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "f")
    );
    assert_eq!(
        decode("http://h/?").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/", "", "")
    );
    assert_eq!(
        decode("http://h/#").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/#", "", "")
    );
    assert_eq!(
        decode("http://h?").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://h#").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
}

#[test]
fn decode_escapes_before_parsing() {
    assert_eq!(
        decode("http://h/a%3Fb").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a", "b", "")
    );
    assert_eq!(
        decode("http://h/a%23b").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/a#b", "", "")
    );
    assert_eq!(
        decode("http://h/%2Fx").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "//x", "", "")
    );
    assert_eq!(
        decode("http://h%2Fx/").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "/x/", "", "")
    );
    assert_eq!(
        decode("http://h%3Ax").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://h%3A80/").unwrap(),
        url_of(Scheme::Http, "h", 80, "80", "/", "", "")
    );
    assert_eq!(
        decode("http://h%3F").unwrap(),
        url_of(Scheme::Http, "h", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://h%5B::1%5D").unwrap(),
        url_of(Scheme::Http, "h[", 80, "", "", "", "")
    );
    assert_eq!(
        decode("http://%5B::1%5D/").unwrap(),
        url_of(Scheme::Http, "[::1]", 80, "", "/", "", "")
    );
    assert_eq!(
        decode("http://h%20").unwrap(),
        url_of(Scheme::Http, "h ", 80, "", "", "", "")
    );
}

#[test]
fn decode_percent_escapes() {
    assert_eq!(decode("http://h/%7e").unwrap().path, wide("/~"));
    assert_eq!(decode("http://h/%7E").unwrap().path, wide("/~"));
    assert_eq!(decode("http://h/%20%2").unwrap().path, wide("/ %2"));
    assert_eq!(decode("http://h/a%0Ab").unwrap().path, wide("/a\nb"));
    assert_eq!(decode("http://h/%00").unwrap().path, wide("/\u{0}"));
    assert_eq!(decode("http://h/%2F").unwrap().path, wide("//"));
    assert_eq!(decode("http://h/%80").unwrap().path, wide("/\u{80}"));
    for literal in ["%2", "%7", "%u", "%", "%G0", "%7G", "%G", "%7g", "%u0041"] {
        assert_eq!(
            decode(&format!("http://h/{literal}")).unwrap().path,
            wide(&format!("/{literal}")),
            "{literal}"
        );
    }
}

#[test]
fn decode_convert_path_utf8() {
    assert_eq!(decode("http://h/%C3%A9").unwrap().path, wide("/\u{e9}"));
    assert_eq!(decode("http://h/%c3%a9").unwrap().path, wide("/\u{e9}"));
    assert_eq!(
        decode("http://h/%C3%A9x%41").unwrap().path,
        wide("/\u{e9}xA")
    );
    assert_eq!(decode("http://h/%C2%80").unwrap().path, wide("/\u{80}"));
    assert_eq!(
        decode("http://h/%F4%8F%BF%BF").unwrap().path,
        wide("/\u{10ffff}")
    );
    assert_eq!(
        decode("http://h/%F0%9F%98%80").unwrap().path,
        wide("/\u{1f600}")
    );
    assert_eq!(
        decode("http://h/%E4%B8%AD%41").unwrap().path,
        wide("/\u{4e2d}A")
    );
    assert_eq!(
        decode("http://h/\u{e9}%C3%A9").unwrap().path,
        wide("/\u{e9}\u{e9}")
    );
    assert_eq!(
        decode_units(&around("http://h/%C3%A9", &[0xD83D], ""))
            .unwrap()
            .path,
        wide("/\u{e9}\u{fffd}")
    );
    assert_eq!(
        decode_units(&around("http://h/", &[0xD83D], "%C3%A9"))
            .unwrap()
            .path,
        wide("/\u{fffd}\u{e9}")
    );
}

#[test]
fn decode_convert_path_errors() {
    let bad = [
        "%A0",
        "%E9",
        "%E9%41",
        "%C3",
        "%C0%80",
        "%ED%A0%80",
        "%F4%90%80%80",
        "%C3%A9%E9",
    ];
    for path in bad {
        assert_eq!(
            decode(&format!("http://h/{path}")),
            Err(Error::QuotaExceeded),
            "{path}"
        );
    }
    assert_eq!(decode("http://h/%A0\u{e9}"), Err(Error::QuotaExceeded));
    assert_eq!(
        decode_units(&around("http://h/", &[0xD83D], "%A0")),
        Err(Error::QuotaExceeded)
    );
}

/// Wine keeps only the low byte of the unit after `%`, so U+0130 followed by `0` decodes to
/// U+0000 there, and `%4` followed by U+0130 decodes to `@`. This port reads ASCII hex digits
/// only, so these assert the literal `%` that the port keeps.
#[test]
fn decode_low_byte_divergence() {
    let path = |units: &[u16]| decode_units(units).unwrap().path;
    assert_eq!(
        path(&around("http://h/%", &[0x0130], "0")),
        around("/%", &[0x0130], "0")
    );
    assert_eq!(
        path(&around("http://h/%4", &[0x0130], "")),
        around("/%4", &[0x0130], "")
    );
    assert_eq!(
        path(&around("http://h/%", &[0x0130], "1")),
        around("/%", &[0x0130], "1")
    );
    assert_eq!(
        path(&around("http://h/%", &[0x0130], "G")),
        around("/%", &[0x0130], "G")
    );
    assert_eq!(
        path(&around("http://h/%", &[0x0130], "")),
        around("/%", &[0x0130], "")
    );
}

#[test]
fn encode_host_forms() {
    assert_eq!(
        encode(&url_of(Scheme::Http, "a/b", 0, "", "", "", "")).unwrap(),
        "http://a%2Fb"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "[::1]", 0, "", "", "", "")).unwrap(),
        "http://%5B::1%5D"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "|", 0, "", "", "", "")).unwrap(),
        "http://%7C"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "", 0, "", "", "", "")).unwrap(),
        "http://"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "", "", "")).unwrap(),
        "http://h"
    );
    let surrogate_host = HttpUrl {
        host: vec![0xD83D],
        ..url_of(Scheme::Http, "", 0, "", "", "", "")
    };
    assert_eq!(encode(&surrogate_host).unwrap(), "http://%EF%BF%BD");
}

#[test]
fn encode_port_rules() {
    let url = |scheme, port, port_str| url_of(scheme, "h", port, port_str, "", "", "");
    assert_eq!(
        encode(&url(Scheme::Http, 8080, "8080")).unwrap(),
        "http://h:8080"
    );
    assert_eq!(
        encode(&url(Scheme::Http, 0, "8080")).unwrap(),
        "http://h:8080"
    );
    assert_eq!(encode(&url(Scheme::Http, 0, "8x")).unwrap(), "http://h:8");
    assert_eq!(encode(&url(Scheme::Http, 80, "")).unwrap(), "http://h");
    assert_eq!(encode(&url(Scheme::Https, 80, "")).unwrap(), "https://h:80");
    assert_eq!(encode(&url(Scheme::Https, 443, "")).unwrap(), "https://h");
    assert_eq!(encode(&url(Scheme::Http, 443, "")).unwrap(), "http://h:443");
    assert_eq!(
        encode(&url(Scheme::Http, 80, "0")),
        Err(Error::InvalidArgument)
    );
    assert_eq!(encode(&url(Scheme::Http, 0, "x80")).unwrap(), "http://h");
    assert_eq!(encode(&url(Scheme::Http, 0, "0080")).unwrap(), "http://h");
    assert_eq!(
        encode(&url(Scheme::Http, 0, "65536")),
        Err(Error::InvalidFormat)
    );
    assert_eq!(
        encode(&url(Scheme::NetTcp, 808, "")).unwrap(),
        "net.tcp://h"
    );
    assert_eq!(encode(&url(Scheme::NetTcp, 0, "")).unwrap(), "net.tcp://h");
    assert_eq!(
        encode(&url(Scheme::SoapUdp, 65535, "")).unwrap(),
        "soap.udp://h"
    );
    assert_eq!(
        encode(&url(Scheme::NetPipe, 0, "")).unwrap(),
        "net.pipe://h"
    );
    assert_eq!(
        encode(&url(Scheme::NetPipe, 80, "")).unwrap(),
        "net.pipe://h:80"
    );
}

#[test]
fn encode_path_query_fragment() {
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a", "b", "c d")).unwrap(),
        "http://h/a?b#c%20d"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a", "", "#")).unwrap(),
        "http://h/a#%23"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "", "", "#")).unwrap(),
        "http://h#%23"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "", "", "x y")).unwrap(),
        "http://h#x%20y"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "", "", "?/")).unwrap(),
        "http://h#?/"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "", "x", "")).unwrap(),
        "http://h?x"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a?b", "", "")).unwrap(),
        "http://h/a%3Fb"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a", "a#b", "")).unwrap(),
        "http://h/a?a%23b"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a", "a/b?c#d", "")).unwrap(),
        "http://h/a?a/b?c%23d"
    );
    assert_eq!(
        encode(&url_of(Scheme::Http, "h", 0, "", "/a?b", "c?d/e", "f?g/h")).unwrap(),
        "http://h/a%3Fb?c?d/e#f?g/h"
    );
}

#[test]
fn encode_byte_escapes() {
    let path = |units: Vec<u16>| {
        encode(&HttpUrl {
            path: units,
            ..url_of(Scheme::Http, "h", 0, "", "", "", "")
        })
        .unwrap()
    };
    assert_eq!(
        path(wide("\"#%<>\\[]^`{|}")),
        "http://h%22%23%25%3C%3E%5C%5B%5D%5E%60%7B%7C%7D"
    );
    assert_eq!(path(wide("/%")), "http://h/%25");
    assert_eq!(path(wide("/\u{1}\u{7f}")), "http://h/%01%7F");
    assert_eq!(path(wide("/\u{80}\u{9f}")), "http://h/%80%9F");
    assert_eq!(path(wide("/\u{a0}")), "http://h/%C2%A0");
    assert_eq!(path(around("/", &[0xD83D], "")), "http://h/%EF%BF%BD");
    assert_eq!(path(wide("/\u{e9}%")), "http://h/%C3%A9%25");
    assert_eq!(path(wide("/\u{4e2d}")), "http://h/%E4%B8%AD");
    assert_eq!(path(wide("/\u{7f}")), "http://h/%7F");
    assert_eq!(
        path(wide("/~!$&'()*+,;=@:-._")),
        "http://h/~!$&'()*+,;=@:-._"
    );
}

#[test]
fn trim_only_removes_wine_whitespace() {
    assert_eq!(trim(b" \t\r\n x \r\n"), b"x");
    assert_eq!(trim(b"\x0b1\x0b"), b"\x0b1\x0b");
    assert_eq!(trim(b" \t\r\n"), b"");
    assert_eq!(trim(b""), b"");
}

#[test]
fn bool_accepts_four_spellings_only() {
    assert_eq!(str_to_bool(b"true"), Ok(true));
    assert_eq!(str_to_bool(b"1"), Ok(true));
    assert_eq!(str_to_bool(b"false"), Ok(false));
    assert_eq!(str_to_bool(b"0"), Ok(false));
    let rejected: &[&[u8]] = &[
        b"TRUE",
        b"True",
        b"",
        b" 1",
        b"1 ",
        b"2",
        b"\x0b1",
        "\u{e9}".as_bytes(),
    ];
    for text in rejected {
        assert_eq!(str_to_bool(text), Err(Error::InvalidFormat), "{text:?}");
    }
}

#[test]
fn int64_bounds_and_trim() {
    assert_eq!(
        str_to_int64(b"-9223372036854775808", i64::MIN, i64::MAX),
        Ok(i64::MIN)
    );
    assert_eq!(
        str_to_int64(b"9223372036854775807", i64::MIN, i64::MAX),
        Ok(i64::MAX)
    );
    assert_eq!(
        str_to_int64(b"9223372036854775808", i64::MIN, i64::MAX),
        Err(Error::NumericOverflow)
    );
    assert_eq!(
        str_to_int64(b"-9223372036854775809", i64::MIN, i64::MAX),
        Err(Error::NumericOverflow)
    );
    assert_eq!(str_to_int64(b"-0", -100, 100), Ok(0));
    assert_eq!(str_to_int64(b"-9", -9, 9), Ok(-9));
    assert_eq!(str_to_int64(b"-10", -9, 9), Err(Error::NumericOverflow));
    assert_eq!(str_to_int64(b"-3", -5, -1), Ok(-3));
    assert_eq!(str_to_int64(b"-2", -5, -1), Ok(-2));
    assert_eq!(str_to_int64(b"-6", -5, -1), Err(Error::NumericOverflow));
    assert_eq!(str_to_int64(b"5", 0, 9), Ok(5));
    assert_eq!(str_to_int64(b"5", 9, 0), Err(Error::NumericOverflow));
    assert_eq!(str_to_int64(b"99x", 0, 9), Err(Error::NumericOverflow));
    assert_eq!(str_to_int64(b"9x", 0, 9), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"1 2", 0, 9), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"-", 0, 9), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"", 0, 9), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"+5", 0, 9), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"\x0b5", -100, 100), Err(Error::InvalidFormat));
    assert_eq!(str_to_int64(b"\t-5\r", -100, 100), Ok(-5));
}

#[test]
fn uint64_bounds_and_trim() {
    assert_eq!(str_to_uint64(b"", 255), Err(Error::InvalidFormat));
    assert_eq!(str_to_uint64(b"-1", 255), Err(Error::InvalidFormat));
    assert_eq!(str_to_uint64(b"1 2", 255), Err(Error::InvalidFormat));
    assert_eq!(str_to_uint64(b"+1", 255), Err(Error::InvalidFormat));
    assert_eq!(str_to_uint64(b"\x0b5", 255), Err(Error::InvalidFormat));
    assert_eq!(str_to_uint64(b"\t 12 \r", 255), Ok(12));
    assert_eq!(str_to_uint64(b"007", 255), Ok(7));
    assert_eq!(str_to_uint64(b"0255", 255), Ok(255));
    assert_eq!(str_to_uint64(b"255", 255), Ok(255));
    assert_eq!(str_to_uint64(b"256", 255), Err(Error::NumericOverflow));
    assert_eq!(str_to_uint64(b"10", 9), Err(Error::NumericOverflow));
    assert_eq!(str_to_uint64(b"0", 0), Ok(0));
    assert_eq!(
        str_to_uint64(b"18446744073709551615", u64::MAX),
        Ok(u64::MAX)
    );
    assert_eq!(
        str_to_uint64(b"18446744073709551616", u64::MAX),
        Err(Error::NumericOverflow)
    );
    assert_eq!(str_to_uint64(b"5", 5), Ok(5));
}

/// Wine computes `max - digit` without a sign check, so a digit above a small `max` passes there:
/// `5 6` and `5 7` return 6 and 7, and `0 1` returns 1. This port reports overflow instead.
#[test]
fn uint64_digit_above_small_max_is_overflow_here() {
    assert_eq!(str_to_uint64(b"6", 5), Err(Error::NumericOverflow));
    assert_eq!(str_to_uint64(b"7", 5), Err(Error::NumericOverflow));
    assert_eq!(str_to_uint64(b"1", 0), Err(Error::NumericOverflow));
}

#[test]
fn base64_decodes_quads_and_padding() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"QUJD", b"ABC"),
        (b"QUJDRA==", b"ABCD"),
        (b"QUI=", b"AB"),
        (b"QQ==", b"A"),
        (b"AB==", &[0x00]),
        (b"+/8=", &[0xfb, 0xff]),
        (b"YWJj", b"abc"),
        (b"MDEy", b"012"),
        (b" QUJD ", b"ABC"),
        (b"QQ=A", b"A"),
        (b"QQ=!", b"A"),
        (b"QUJDQQ==", b"ABCA"),
        (b"QUJDQUJ=", b"ABCAB"),
        (b"QUJDQUJD", b"ABCABC"),
    ];
    for (text, expected) in cases {
        assert_eq!(str_to_bytes(text), Ok(expected.to_vec()), "{text:?}");
    }
}

#[test]
fn base64_rejects_bad_length() {
    let short: &[&[u8]] = &[b"QUJ", b"QUJDRA", b"QUI", b"Q", b" QUJ "];
    for text in short {
        assert_eq!(str_to_bytes(text), Err(Error::InvalidFormat), "{text:?}");
    }
}

/// Invalid base64 characters give an empty result, not an error, as in `decode_base64`.
#[test]
fn base64_invalid_characters_give_empty() {
    let invalid: &[&[u8]] = &[
        b"!QQ=",
        b"Q!==",
        b"QQ!=",
        b"QU!=",
        b"QUJ!",
        b"AAAA!BBB",
        b"QUJDQUJ!",
        b"QQ==QQ==",
        b"QU==QUJD",
        b"====",
        b"Q!Q=",
        b"QUJ!QUJD",
        b"!UJDQUJD",
        b"Q!AAAAAA",
        b"Q!JJ",
        b"QQ!J",
    ];
    for text in invalid {
        assert_eq!(str_to_bytes(text), Ok(Vec::new()), "{text:?}");
    }
}

/// Wine reads past an empty input (ASan flags it), so the oracle has no row. This asserts the
/// port's own result: empty text decodes to nothing.
#[test]
fn base64_empty_input_is_our_behavior() {
    assert_eq!(str_to_bytes(b""), Ok(Vec::new()));
}

#[test]
fn decode_text_named() {
    assert_eq!(
        decode_text(b"&lt;&gt;&quot;&amp;&apos;"),
        Ok(b"<>\"&'".to_vec())
    );
    assert_eq!(decode_text(b"&amp;amp;"), Ok(b"&amp;".to_vec()));
    assert_eq!(decode_text(b"&lt;&lt;&amp;"), Ok(b"<<&".to_vec()));
    let bad: &[&[u8]] = &[b"&lT;", b"&nbsp;", b"&x", b"&lt", b"&", b"&foo;"];
    for text in bad {
        assert_eq!(decode_text(text), Err(Error::InvalidFormat), "{text:?}");
    }
}

#[test]
fn decode_text_numeric() {
    let cases: &[(&[u8], &str)] = &[
        (b"&#0000000000065;", "A"),
        (b"&#x0041;", "A"),
        (b"&#1;", "\u{1}"),
        (b"&#x7;", "\u{7}"),
        (b"&#x00A0;", "\u{a0}"),
        (b"&#0001114111;", "\u{10ffff}"),
        (b"&#xD7FF;", "\u{d7ff}"),
        (b"&#xE000;", "\u{e000}"),
        (b"&#x10FFFF;", "\u{10ffff}"),
        (b"&#x4E2D;", "\u{4e2d}"),
        (b"&#x1F600;", "\u{1f600}"),
        (b"&#xE9;&#xe9;", "\u{e9}\u{e9}"),
        (b"a&#x41;b", "aAb"),
        (b"plain", "plain"),
    ];
    for (text, expected) in cases {
        assert_eq!(
            decode_text(text),
            Ok(expected.as_bytes().to_vec()),
            "{text:?}"
        );
    }
}

#[test]
fn decode_text_bad_references() {
    let bad: &[&[u8]] = &[
        b"&#;",
        b"&#x;",
        b"&#X;",
        b"&#X41;",
        b"&#0x41;",
        b"&#x41g",
        b"&#xg;",
        b"&#x41",
        b"&#65",
        b"&#0;",
        b"&#00;",
        b"&#x0;",
        b"&#x110000;",
        b"&#1114112;",
        b"&#1234567;",
        b"&#12345678;",
        b"&#65534;",
        b"&#65535;",
        b"&#xfffe;",
        b"&#xD800;",
        b"&#x0010FFFF;",
        b"&#x00FFFF;",
        b"&#x0000000041;",
        b"&#xFFFFFF;",
        b"&#x7fffffff;",
        b"x&#0;y",
    ];
    for text in bad {
        assert_eq!(decode_text(text), Err(Error::InvalidFormat), "{text:?}");
    }
}

#[test]
fn codepoint_to_utf8_ranges() {
    assert_eq!(codepoint_to_utf8(0x01), Some(vec![0x01]));
    assert_eq!(codepoint_to_utf8(0x80), Some(vec![0xc2, 0x80]));
    assert_eq!(codepoint_to_utf8(0x7ff), Some(vec![0xdf, 0xbf]));
    assert_eq!(codepoint_to_utf8(0x800), Some(vec![0xe0, 0xa0, 0x80]));
    assert_eq!(codepoint_to_utf8(0xd7ff), Some(vec![0xed, 0x9f, 0xbf]));
    assert_eq!(codepoint_to_utf8(0xd800), None);
    assert_eq!(codepoint_to_utf8(0xdfff), None);
    assert_eq!(codepoint_to_utf8(0xe000), Some(vec![0xee, 0x80, 0x80]));
    assert_eq!(codepoint_to_utf8(0xfffe), None);
    assert_eq!(codepoint_to_utf8(0xffff), None);
    assert_eq!(codepoint_to_utf8(0), None);
    assert_eq!(
        codepoint_to_utf8(0x10000),
        Some(vec![0xf0, 0x90, 0x80, 0x80])
    );
    assert_eq!(
        codepoint_to_utf8(0x10ffff),
        Some(vec![0xf4, 0x8f, 0xbf, 0xbf])
    );
    assert_eq!(codepoint_to_utf8(0x110000), None);
}

#[test]
fn datetime_zones() {
    assert_eq!(str_to_datetime(b"0001-01-01T00:00:00Z"), Ok(utc(0)));
    assert_eq!(str_to_datetime(b"0001-01-01T00:00:00-00:00"), Ok(local(0)));
    assert_eq!(
        str_to_datetime(b"0001-01-01T00:00:00-00:01"),
        Ok(local(600_000_000))
    );
    assert_eq!(
        str_to_datetime(b"0001-01-01T00:00:00+00:01"),
        Err(Error::InvalidFormat)
    );
    assert_eq!(str_to_datetime(b"0001-01-01T00:01:00+00:01"), Ok(local(0)));
    assert_eq!(
        str_to_datetime(b"0000-01-01T00:00:00Z"),
        Err(Error::InvalidFormat)
    );
    assert_eq!(
        str_to_datetime(b"1601-01-01T00:00:00Z"),
        Ok(utc(504_911_232_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00+14:00"),
        Ok(local(633_978_504_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00-14:00"),
        Ok(local(633_979_512_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00+13:59"),
        Ok(local(633_978_504_600_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00-05:30"),
        Ok(local(633_979_206_000_000_000))
    );
    let bad_zones = [
        "", "+14:01", "+15:00", "+05:60", "+05:3", "+05:", "+0", "+05", "+5:30", "*05:30",
        "-14:01", "-14:59", "+00:00Z", "ZZ", "Z.", "+05:30x", "+",
    ];
    for zone in bad_zones {
        assert_eq!(
            str_to_datetime(format!("2010-01-01T00:00:00{zone}").as_bytes()),
            Err(Error::InvalidFormat),
            "{zone}"
        );
    }
    assert_eq!(
        str_to_datetime(b"2010-01-01T23:59:59.9999999+00:01"),
        Ok(local(633_979_871_399_999_999))
    );
    assert_eq!(
        str_to_datetime(b"9999-12-31T23:59:59.9999999+00:01"),
        Ok(local(3_155_378_975_399_999_999))
    );
    assert_eq!(
        str_to_datetime(b"9999-12-31T23:59:59.9999999-00:00"),
        Ok(local(3_155_378_975_999_999_999))
    );
    assert_eq!(
        str_to_datetime(b"9999-12-31T23:59:59.9999999+00:00"),
        Ok(local(3_155_378_975_999_999_999))
    );
    assert_eq!(
        str_to_datetime(b"9999-12-31T23:59:59.9999999-00:01"),
        Err(Error::InvalidFormat)
    );
}

#[test]
fn datetime_fraction_and_clock() {
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.1Z"),
        Ok(utc(633_979_008_001_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.1234567Z"),
        Ok(utc(633_979_008_001_234_567))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.12345678Z"),
        Err(Error::InvalidFormat)
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.Z"),
        Err(Error::InvalidFormat)
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.+00:00"),
        Err(Error::InvalidFormat)
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.5+00:00"),
        Ok(local(633_979_008_005_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T00:00:00.1234567-05:30"),
        Ok(local(633_979_206_001_234_567))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T24:00:00Z"),
        Ok(utc(633_979_872_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T24:00:00.0000000+00:00"),
        Ok(local(633_979_872_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-01-01T24:00:00.5Z"),
        Ok(utc(633_979_872_005_000_000))
    );
    for text in [
        "2010-01-01T24:00:01Z",
        "2010-01-01T24:01:00Z",
        "2010-01-01T25:00:00Z",
        "2010-01-01T23:59:60Z",
        "2010-01-01T23:60:00Z",
        "9999-12-31T24:00:00-00:00",
    ] {
        assert_eq!(
            str_to_datetime(text.as_bytes()),
            Err(Error::InvalidFormat),
            "{text}"
        );
    }
    assert_eq!(
        str_to_datetime(b"9999-12-31T24:00:00Z"),
        Ok(utc(3_155_378_976_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"9999-12-31T24:00:00+00:00"),
        Ok(local(3_155_378_976_000_000_000))
    );
}

#[test]
fn datetime_calendar_days() {
    assert_eq!(
        str_to_datetime(b"1600-02-29T00:00:00Z"),
        Ok(utc(504_645_984_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2000-02-29T00:00:00Z"),
        Ok(utc(630_873_792_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2008-02-29T00:00:00Z"),
        Ok(utc(633_398_400_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2400-02-29T00:00:00Z"),
        Ok(utc(757_101_600_000_000_000))
    );
    for text in [
        "1900-02-29T00:00:00Z",
        "2100-02-29T00:00:00Z",
        "2010-02-29T00:00:00Z",
        "2010-04-31T00:00:00Z",
        "2010-06-31T00:00:00Z",
        "2010-11-31T00:00:00Z",
        "2010-00-01T00:00:00Z",
        "2010-13-01T00:00:00Z",
        "2010-01-00T00:00:00Z",
        "2010-01-32T00:00:00Z",
    ] {
        assert_eq!(
            str_to_datetime(text.as_bytes()),
            Err(Error::InvalidFormat),
            "{text}"
        );
    }
    assert_eq!(
        str_to_datetime(b"2010-04-30T00:00:00Z"),
        Ok(utc(634_081_824_000_000_000))
    );
    assert_eq!(
        str_to_datetime(b"2010-12-31T23:59:59Z"),
        Ok(utc(634_294_367_990_000_000))
    );
}

#[test]
fn datetime_syntax_errors() {
    let bad: &[&[u8]] = &[
        b"2010-01-01 00:00:00Z",
        b"2010/01/01T00:00:00Z",
        b"2010--01-01T00:00:00Z",
        b"2010-1-01T00:00:00Z",
        b"2010-01-01T0:00:00Z",
        b"2010-01-01T00:00:0Z",
        b"20a0-01-01T00:00:00Z",
        b"201-01-01T00:00:00Z",
        b"2010-01-01",
        b"2010-01-01T00:00:00",
        b"T0",
        b"2010-01/01T00:00:00Z",
        b"2010-01-xxT00:00:00Z",
        b"2010-01-01T00-00:00Z",
        b"2010-01-01T00:0x:00Z",
        b"2010-01-01T00:00-00Z",
    ];
    for text in bad {
        assert_eq!(str_to_datetime(text), Err(Error::InvalidFormat), "{text:?}");
    }
    assert_eq!(
        str_to_datetime(b"  2010-01-01T00:00:00Z \t"),
        Ok(utc(633_979_008_000_000_000))
    );
}

#[test]
fn filetime_conversions() {
    assert_eq!(
        datetime_to_filetime(utc(504_911_231_999_999_999)),
        Err(Error::InvalidFormat)
    );
    assert_eq!(datetime_to_filetime(utc(504_911_232_000_000_000)), Ok(0));
    assert_eq!(datetime_to_filetime(utc(504_911_232_000_000_001)), Ok(1));
    assert_eq!(
        datetime_to_filetime(utc(3_155_378_975_999_999_999)),
        Ok(2_650_467_743_999_999_999)
    );
    assert_eq!(datetime_to_filetime(utc(0)), Err(Error::InvalidFormat));
    assert_eq!(datetime_to_filetime(local(504_911_232_000_000_000)), Ok(0));

    assert_eq!(filetime_to_datetime(0), Ok(utc(504_911_232_000_000_000)));
    assert_eq!(filetime_to_datetime(1), Ok(utc(504_911_232_000_000_001)));
    assert_eq!(
        filetime_to_datetime(2_650_467_743_999_999_999),
        Ok(utc(3_155_378_975_999_999_999))
    );
    assert_eq!(
        filetime_to_datetime(2_650_467_744_000_000_000),
        Err(Error::InvalidFormat)
    );
    assert_eq!(filetime_to_datetime(u64::MAX), Err(Error::NumericOverflow));
    assert_eq!(
        filetime_to_datetime(18_446_744_073_709_549_615),
        Err(Error::NumericOverflow)
    );
    assert_eq!(
        filetime_to_datetime(18_446_744_073_709_549_616),
        Err(Error::NumericOverflow)
    );
}
