//! Replays rows of the Wine oracle (`target/canon/inetcomm`) against this crate. The rows are the
//! output of the verbatim Wine C functions under ASan and UBSan, run through `driver.c`. Each row is
//! `(line, output)`: `line` is `<mode>|<payload>` with the driver's payload escapes, and `output` is
//! the escaped line the driver prints. `run` mirrors `driver.c`, so `edge_checks` can use it too.

use crate::codec::{decode_base64, decode_quoted_printable};
use crate::header::{self, ContentEncoding, Param};
use crate::mhtml::parse_mhtml_url;
use crate::smtp::{parse_status, RESPONSE_ERROR};

/// Rows recorded from the verbatim Wine C code: `(line, output)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    (r"unfold|  abc\r\n", "abc"),
    (r"unfold|  \r\n \r\n", " "),
    (r#"unquote|"a\\"b""#, r#"a"b"#),
    (r#"unquote|"ab"cd"#, "ab"),
    (r#"params|a;b=1;c="x;y""#, "a|b=1|c=x;y"),
    (r"params|a;;x=1", "a|x=1"),
    (r"params|a;b=x\|y", r"a|b=x\|y"),
    (r#"field|Content-Type: text/plain; charset="utf-8"\r\n\r\n"#, r"Content-Type|text/plain|charset=utf-8|@43"),
    (r"field|X-Foo: one\r\n two\r\n\r\n", "X-Foo|one two|@18"),
    (r"field|Foo\r\nSubject: x\r\n\r\n", r"Foo\r\nSubject|x|@17"),
    (r"field|content-type:\r\n\r\n", "content-type||@15"),
    (r"field|no colon here\r\n\r\n", "none"),
    (r"ctype|a/b/c", "a|b/c"),
    (r"ctype|noslash", "none"),
    (r"cenc|Quoted-Printable", "quoted-printable"),
    (r"cenc|binary", "unchanged"),
    (r"block|a\r\r\n\r\n", "6 6"),
    (r"block|Subject: x\r\n\r\nbody", "14 14"),
    (r"b64|QUI=", "4142"),
    (r"b64|a+/z", "6beff3"),
    (r"qp|a=\nbc", "613d63"),
    (r"mhtml|mhtml:http://x/y.mht!http://x/y.mht/a.png", "ok 14 21"),
    (r"mhtml|mhtml:a!X-USC:b", "ok 1 8"),
    (r"smtp|421 busy", "ok 421 1 800ccc67"),
    (r"smtp|250-SIZE 100", "ok 250 0 000ccc74"),
];

#[test]
fn oracle_rows_match_wine() {
    for &(line, output) in ROWS {
        assert_eq!(run(line), output, "driver line: {line}");
    }
}

/// Runs one driver line the way `driver.c` does, and returns the line it prints without the newline.
pub(crate) fn run(line: &str) -> String {
    let Some((mode, payload)) = line.split_once('|') else {
        return "error".to_string();
    };
    let bytes = unescape(payload);
    match mode {
        "unfold" => emit(&header::unfold(&bytes)),
        "unquote" => emit(&header::unquote(&bytes)),
        "params" => {
            let (main, params) = header::split_params(&bytes);
            format!("{}{}", emit(main), emit_params(&params))
        }
        "field" => match header::read_field(&bytes) {
            None => "none".to_string(),
            Some(field) => format!(
                "{}|{}{}|@{}",
                emit(field.name),
                emit(&field.value),
                emit_params(&field.params),
                bytes.len() - field.rest.len()
            ),
        },
        "ctype" => match header::content_type(&bytes) {
            None => "none".to_string(),
            Some((primary, sub)) => format!("{}|{}", emit(&primary), emit(&sub)),
        },
        "cenc" => encoding_name(header::content_encoding(&bytes)).to_string(),
        "block" => {
            let len = header::header_block_len(&bytes);
            format!("{len} {len}")
        }
        "b64" => hex(&decode_base64(&bytes)),
        "qp" => hex(&decode_quoted_printable(&bytes)),
        "mhtml" => {
            let units: Vec<u16> = bytes.iter().map(|&b| u16::from(b)).collect();
            match parse_mhtml_url(&units) {
                None => "fail".to_string(),
                Some(url) => {
                    let location = match url.location {
                        Some(location) => (units.len() - location.len()).to_string(),
                        None => "-1".to_string(),
                    };
                    format!("ok {} {location}", url.mhtml.len())
                }
            }
        }
        "smtp" => match parse_status(&bytes) {
            None => format!("error {RESPONSE_ERROR:08x}"),
            Some(status) => format!(
                "ok {} {} {:08x}",
                status.code,
                u8::from(status.done),
                status.hr
            ),
        },
        _ => "error".to_string(),
    }
}

fn encoding_name(encoding: Option<ContentEncoding>) -> &'static str {
    match encoding {
        None => "unchanged",
        Some(ContentEncoding::Base64) => "base64",
        Some(ContentEncoding::QuotedPrintable) => "quoted-printable",
        Some(ContentEncoding::SevenBit) => "7bit",
        Some(ContentEncoding::EightBit) => "8bit",
    }
}

/// `driver.c` `unescape`: a backslash starts one of `\r \n \t \\ \|` or `\xHH`. Any other backslash
/// is kept, and the byte after it is read as usual.
fn unescape(payload: &str) -> Vec<u8> {
    let bytes = payload.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let (byte, width) = match bytes[i] {
            b'\\' => escape(&bytes[i + 1..]).unwrap_or((b'\\', 1)),
            c => (c, 1),
        };
        out.push(byte);
        i += width;
    }
    out
}

/// The byte of the escape whose backslash is just before `rest`, and the escape's width counted
/// from the backslash.
fn escape(rest: &[u8]) -> Option<(u8, usize)> {
    let byte = match *rest.first()? {
        b'r' => b'\r',
        b'n' => b'\n',
        b't' => b'\t',
        b'\\' => b'\\',
        b'|' => b'|',
        b'x' => {
            let hi = hex_value(*rest.get(1)?)?;
            let lo = hex_value(*rest.get(2)?)?;
            return Some(((hi << 4) | lo, 4));
        }
        _ => return None,
    };
    Some((byte, 2))
}

fn hex_value(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// `driver.c` `emit`: escapes `\r \n \t \\ |`, and writes every other byte below 0x20 or at 0x7f and
/// up as `\xHH`.
fn emit(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &c in bytes {
        match c {
            b'\r' => out.push_str(r"\r"),
            b'\n' => out.push_str(r"\n"),
            b'\t' => out.push_str(r"\t"),
            b'\\' => out.push_str(r"\\"),
            b'|' => out.push_str(r"\|"),
            0x20..=0x7e => out.push(char::from(c)),
            _ => out.push_str(&format!(r"\x{c:02x}")),
        }
    }
    out
}

/// `driver.c` `emit_params`: each parameter as `|name=value`.
fn emit_params(params: &[Param]) -> String {
    let mut out = String::new();
    for param in params {
        out.push_str(&format!("|{}={}", emit(&param.name), emit(&param.value)));
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
