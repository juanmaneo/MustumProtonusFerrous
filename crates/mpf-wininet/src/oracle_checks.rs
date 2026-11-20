//! Rows from the Wine oracle, the verbatim C driver built with ASan and UBSan. The oracle has 216
//! rows, generated under `target/canon/wininet` (gitignored). These 25 cover each mode. The rest
//! are in `edge_checks.rs`.
//!
//! Full RFC 850 weekday names are left out of these rows, because the C code reads an
//! uninitialized unit after the weekday. `Wednesday` also writes past its buffer, and UBSan flags
//! it. Its two oracle rows are dropped.

use crate::cookie::{is_domain_legal, CookieDomainError};
use crate::date::parse_http_date;
use crate::has_keep_alive;
use crate::proxy_bypass::domain_matches;
use crate::wide::{until_nul, wcstoul_dec};

/// `(input line, expected output)`, in the driver's format. `\uXXXX` escapes are UTF-16 units.
const ROWS: &[(&str, &str)] = &[
    (
        r"date|Sun, 06 Nov 1994 08:49:37 GMT",
        "ok 124285853770000000",
    ),
    (
        r"date|Sun, 06-Nov-1994 08:49:37 GMT",
        "ok 124285853770000000",
    ),
    (r"date|Sun Nov 6 08:49:37 1994", "ok 124285853770000000"),
    (r"date|Sun Nov 6 08:49:37 1994 EST", "fail"),
    (r"date|0", "ok 0"),
    (
        r"date|Sat, 29 Feb 2019 00:00:00 GMT",
        "ok 131958720000000000",
    ),
    (r"date|Thu, 01 Jan 1601 00:00:00 GMT", "ok 0"),
    (
        r"date|Thu, 01 Jan 30827 00:00:00 GMT",
        "ok 9222834528000000000",
    ),
    (r"date|Thu, 01 Jan 2020 24:00:00 GMT", "fail"),
    (r"date|thu, 01 jan 2020 10:00:00 gmt", "fail"),
    (
        r"date|Sun, \u0660\u0666 Nov 1994 08:49:37 GMT",
        "ok 124285853770000000",
    ),
    (r"match|www.example.com|*.example.com", "1"),
    (r"match|example.com|*.example.com", "0"),
    (r"match|<local>|<local>", "1"),
    (r"match|www.ab.c|*.b.c", "0"),
    (r"keep|HTTP/1.1|", "1"),
    (r"keep|HTTP/1.1|keep-alive, Close", "0"),
    (r"keep|http/1.1|Keep-Alive", "1"),
    (r"legal|example.com|www.example.com", "legal"),
    (r"legal|.example.com|www.example.com", "invalid_name"),
    (r"legal|example.org|www.example.com", "invalid_parameter"),
    (r"legal|com|example.com", "rejected"),
    (r"ulong|  42abc", "42 4"),
    (r"ulong|-5", "4294967291 2"),
    (r"ulong|99999999999", "4294967295 11"),
];

/// Runs one driver line through the port and formats the result as the C driver prints it.
pub(crate) fn run(line: &str) -> String {
    let mut fields = line.splitn(3, '|');
    let mode = fields.next().unwrap_or_default();
    let first = fields.next().unwrap_or_default();
    let second = fields.next().unwrap_or_default();
    match mode {
        "date" => match parse_http_date(&decode(first)) {
            Some(ticks) => format!("ok {ticks}"),
            None => "fail".to_owned(),
        },
        "match" => {
            let server = decode(first);
            let domain = decode(second);
            u8::from(domain_matches(&server, until_nul(&domain))).to_string()
        }
        "legal" | "legalx" => {
            let domain = decode(first);
            let full = decode(second);
            let result = if mode == "legal" {
                is_domain_legal(until_nul(&domain), until_nul(&full))
            } else {
                is_domain_legal(&domain, &full)
            };
            match result {
                Ok(()) => "legal".to_owned(),
                Err(CookieDomainError::InvalidName) => "invalid_name".to_owned(),
                Err(CookieDomainError::InvalidParameter) => "invalid_parameter".to_owned(),
                Err(CookieDomainError::Rejected) => "rejected".to_owned(),
            }
        }
        "keep" => u8::from(has_keep_alive(&decode(first), &decode(second))).to_string(),
        "ulong" => {
            let (value, end) = wcstoul_dec(&decode(first));
            format!("{value} {end}")
        }
        _ => "error".to_owned(),
    }
}

/// Decodes a row argument: ASCII, with `\uXXXX` escapes for UTF-16 units.
pub(crate) fn decode(text: &str) -> Vec<u16> {
    let mut parts = text.split("\\u");
    let mut units: Vec<u16> = parts.next().unwrap_or_default().encode_utf16().collect();
    for part in parts {
        let (hex, rest) = part.split_at(4);
        units.push(u16::from_str_radix(hex, 16).expect("escapes have four hex digits"));
        units.extend(rest.encode_utf16());
    }
    units
}

#[test]
fn oracle_rows_match_the_port() {
    for &(line, expected) in ROWS {
        assert_eq!(run(line), expected, "{line}");
    }
}
