//! Replays of Wine's oracle rows. The oracle compiles the verbatim `url.c` and `reader.c` with
//! `-fsanitize=address,undefined` (`target/canon/webservices/harness_url.c` and
//! `harness_reader.c`). Each row below is a vector line and the output line that Wine printed for
//! it. `gen_oracle.py` writes the rows from `sweep.jsonl`, so they are not edited by hand.

use crate::datetime::{
    datetime_to_filetime, filetime_to_datetime, str_to_datetime, DateTime, DateTimeFormat,
};
use crate::text::{
    codepoint_to_utf8, decode_text, str_to_bool, str_to_bytes, str_to_int64, str_to_uint64,
};
use crate::url::{decode_url, encode_url, HttpUrl, Scheme};
use crate::Error;
use std::fmt::Debug;
use std::str::FromStr;

const SCHEMES: [Scheme; 5] = [
    Scheme::Http,
    Scheme::Https,
    Scheme::NetTcp,
    Scheme::SoapUdp,
    Scheme::NetPipe,
];

/// Each vector line and the output line that Wine printed for it.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("dec http://a%20b/a\\u0022\\u005c", "ok scheme=0 host=\"a\\u0020b\" port=80 port_str=\"\" path=\"/a\\u0022\\u005c\" query=\"\" frag=\"\""),
    ("dec http://h:65536/", "err 803d0000"),
    ("enc 0|example.com|80||/a b|q=1|f", "ok \"http://example.com/a%20b?q=1#f\""),
    ("enc 0|h\\u00e9|0||/\\u00e9|||", "ok \"http://h%C3%A9/%C3%A9\""),
    ("enc 0|h|81|80|||", "err 80070057"),
    ("bool true", "ok true"),
    ("bool 2", "err 803d0000"),
    ("i64 -9223372036854775808 9223372036854775807 -9223372036854775808", "ok -9223372036854775808"),
    ("i64 -128 127 -129", "err 803d0002"),
    ("u64 255 0255", "ok 255"),
    ("u64 18446744073709551615 18446744073709551616", "err 803d0002"),
    ("b64 QUJDRA==", "ok 41424344"),
    ("b64 QUJ", "err 803d0000"),
    ("ent a&lt;b&amp;c", "ok 613c622663"),
    ("ent &#x41", "err 803d0000"),
    ("cp 65536", "ok f0908080"),
    ("cp 0", "err invalid"),
    ("dtp 2010-01-01T00:00:00.1234567+05:30", "ok 633978810001234567 1"),
    ("dtp 2010-02-29T00:00:00Z", "err 803d0000"),
    ("d2f 633979008000000000", "ok 129067776000000000"),
    ("d2f 0", "err 803d0000"),
    ("f2d 0", "ok 504911232000000000 0"),
    ("f2d 17941832841709551616", "err 803d0002"),
    ("frob 1", "unknown"),
    ("", ""),
];

/// Runs one vector line the way the C harness does, and returns the line it prints.
fn replay(line: &str) -> String {
    if line.is_empty() {
        return String::new();
    }
    let (command, arg) = line.split_once(' ').unwrap_or((line, ""));
    match command {
        "dec" => outcome(decode_url(&units(arg), 0).map(|url| describe(&url))),
        "enc" => encode_row(arg),
        "bool" => outcome(str_to_bool(arg.as_bytes()).map(|value| value.to_string())),
        "i64" => {
            let fields: Vec<&str> = arg.splitn(3, ' ').collect();
            let (min, max): (i64, i64) = (number(fields[0]), number(fields[1]));
            outcome(str_to_int64(fields[2].as_bytes(), min, max).map(|value| value.to_string()))
        }
        "u64" => {
            let (max, text) = arg.split_once(' ').unwrap_or(("", ""));
            outcome(str_to_uint64(text.as_bytes(), number(max)).map(|value| value.to_string()))
        }
        "b64" => outcome(str_to_bytes(arg.as_bytes()).map(|bytes| hex(&bytes))),
        "ent" => outcome(decode_text(arg.as_bytes()).map(|bytes| hex(&bytes))),
        "cp" => match codepoint_to_utf8(number(arg)) {
            Some(bytes) => format!("ok {}", hex(&bytes)),
            None => String::from("err invalid"),
        },
        "dtp" => outcome(str_to_datetime(arg.as_bytes()).map(describe_datetime)),
        "d2f" => {
            let datetime = DateTime {
                ticks: number(arg),
                format: DateTimeFormat::Utc,
            };
            outcome(datetime_to_filetime(datetime).map(|filetime| filetime.to_string()))
        }
        "f2d" => outcome(filetime_to_datetime(number(arg)).map(describe_datetime)),
        _ => String::from("unknown"),
    }
}

fn number<T: FromStr>(text: &str) -> T
where
    T::Err: Debug,
{
    text.parse().expect("a number in the vector")
}

/// `enc`: the fields are scheme, host, port, port text, path, query and fragment.
fn encode_row(arg: &str) -> String {
    let fields: Vec<&str> = arg.split('|').collect();
    let url = HttpUrl {
        scheme: SCHEMES[number::<usize>(fields[0])],
        host: units(fields[1]),
        port: number(fields[2]),
        port_as_string: units(fields[3]),
        path: units(fields[4]),
        query: units(fields[5]),
        fragment: units(fields[6]),
    };
    outcome(encode_url(&url, 0).map(|out| quoted(&out)))
}

/// The harness escapes: each `\uXXXX` is one UTF-16 unit, and every other byte is one unit.
fn units(raw: &str) -> Vec<u16> {
    let mut pieces = raw.split("\\u");
    let head = pieces.next().unwrap_or("");
    let mut out: Vec<u16> = head.bytes().map(u16::from).collect();
    for piece in pieces {
        let (hex_digits, rest) = piece.split_at(4);
        out.push(u16::from_str_radix(hex_digits, 16).expect("four hex digits"));
        out.extend(rest.bytes().map(u16::from));
    }
    out
}

/// The `dec` line: the scheme number, then the quoted parts.
fn describe(url: &HttpUrl) -> String {
    format!(
        "scheme={} host={} port={} port_str={} path={} query={} frag={}",
        url.scheme as u8,
        quoted(&url.host),
        url.port,
        quoted(&url.port_as_string),
        quoted(&url.path),
        quoted(&url.query),
        quoted(&url.fragment),
    )
}

fn describe_datetime(datetime: DateTime) -> String {
    format!("{} {}", datetime.ticks, datetime.format as u8)
}

/// The harness `put_str`: units from 0x21 to 0x7e, other than `"` and `\`, print as they are.
fn quoted(units: &[u16]) -> String {
    let mut out = String::from("\"");
    for &unit in units {
        if (0x21..=0x7e).contains(&unit) && unit != 0x22 && unit != 0x5c {
            out.push(char::from(unit as u8));
        } else {
            out += &format!("\\u{unit:04x}");
        }
    }
    out.push('"');
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn outcome(result: Result<String, Error>) -> String {
    match result {
        Ok(text) => format!("ok {text}"),
        Err(error) => format!("err {:08x}", error.hresult() as u32),
    }
}

#[test]
fn rows_match_wine() {
    for (command, output) in ROWS {
        assert_eq!(replay(command), *output, "row: {command}");
    }
}
