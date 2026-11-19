//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/comsvcs/harness.c`, built with `gcc -fsanitize=address,undefined -g`, with
//! no sanitizer reports). Each row is `(input, output)` in the harness's line format:
//! - `<text>`: `guid_from_string` on ASCII text; output `ok <GUID>` or `fail`.
//! - `u:<hex units>`: `guid_from_string` on UTF-16 units; same outputs.
//! - `n:<text>`: `new_moniker_parse_displayname` on ASCII text.
//! - `nu:<hex units>`: the same on UTF-16 units. Outputs are `syntax` (`MK_E_SYNTAX`),
//!   `clsid <GUID> eaten=<n>` or `progid=<text> eaten=<n>`.
//!
//! The harness stands in for the registry: `CLSIDFromProgID` resolves any non-empty name,
//! which [`fake_progid`] reproduces. No row was excluded for a sanitizer report.

use crate::new_moniker::{parse_displayname, NewMonikerTarget};
use crate::{guid_from_string, Guid};

/// Rows recorded from the verbatim Wine C code: `(input, output)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("12345678-1234-1234-1234-123456789abc", "ok 12345678-1234-1234-1234-123456789ABC"),
    ("{12345678-1234-1234-1234-123456789abc", "fail"),
    ("{12345678-1234-1234-1234-123456789abc}xyz", "ok 12345678-1234-1234-1234-123456789ABC"),
    ("12345678-1234-1234-1234-123456789abcxyz", "ok 12345678-1234-1234-1234-123456789ABC"),
    ("12345678-1234-1234-1234-123456789ab", "fail"),
    ("12345678_1234-1234-1234-123456789abc", "fail"),
    ("12345678-1234-1234-1234_123456789abc", "fail"),
    ("1234567g-1234-1234-1234-123456789abc", "fail"),
    ("12345678-1234-1234-1234-1234567G9abc", "fail"),
    ("{}", "fail"),
    ("{ 12345678-1234-1234-1234-123456789abc}", "fail"),
    ("{12345678-1234-1234-1234-123456789abc]", "fail"),
    ("{00000000-0000-0000-0000-000000000000}", "ok 00000000-0000-0000-0000-000000000000"),
    ("{aBcDeF01-2345-6789-abcd-EF0123456789}", "ok ABCDEF01-2345-6789-ABCD-EF0123456789"),
    ("u:FF10 0030 0030 0030 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030", "fail"),
    ("u:0030 0030 0030 0030 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 002D 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030 0030 0000 0030", "ok 00000000-0000-0000-0000-000000000000"),
    ("n:new:{12345678-1234-1234-1234-123456789abc}", "clsid 12345678-1234-1234-1234-123456789ABC eaten=42"),
    ("n:NEW:12345678-1234-1234-1234-123456789ABC", "clsid 12345678-1234-1234-1234-123456789ABC eaten=40"),
    ("n:new:Some.Prog", "progid=Some.Prog eaten=13"),
    ("n:new:", "syntax"),
    ("n:ne:{12345678-1234-1234-1234-123456789abc}", "syntax"),
    ("n:new", "syntax"),
    ("n:new:{12345678-1234-1234-1234-123456789abc", "progid={12345678-1234-1234-1234-123456789abc eaten=41"),
    ("nu:006E 0065 0077 003A 0046 006F 006F 0000 0042 0061 0072", "progid=Foo eaten=7"),
    ("nu:FF4E 0065 0077 003A 0053 006F 006D 0065", "syntax"),
];

#[test]
fn oracle_rows_match_wine() {
    for (input, expected) in ROWS {
        assert_eq!(replay(input), *expected, "input: {input}");
    }
}

fn replay(input: &str) -> String {
    if let Some(hex) = input.strip_prefix("nu:") {
        display_output(&units_from_hex(hex))
    } else if let Some(text) = input.strip_prefix("n:") {
        display_output(&units_from_ascii(text))
    } else if let Some(hex) = input.strip_prefix("u:") {
        guid_output(&units_from_hex(hex))
    } else {
        guid_output(&units_from_ascii(input))
    }
}

fn guid_output(units: &[u16]) -> String {
    match guid_from_string(units) {
        Some(guid) => format!("ok {}", guid_text(&guid)),
        None => "fail".to_string(),
    }
}

fn display_output(units: &[u16]) -> String {
    match parse_displayname(units, fake_progid) {
        None => "syntax".to_string(),
        Some((NewMonikerTarget::Clsid(clsid), eaten)) => {
            format!("clsid {} eaten={eaten}", guid_text(&clsid))
        }
        // The harness escapes non-printable units. No row contains one.
        Some((NewMonikerTarget::ProgId { name, .. }, eaten)) => {
            format!("progid={} eaten={eaten}", String::from_utf16_lossy(name))
        }
    }
}

/// `CLSIDFromProgID` as the harness stubs it: any non-empty name resolves, to 0x5a bytes.
fn fake_progid(text: &[u16]) -> Option<Guid> {
    (!text.is_empty()).then_some(Guid {
        data1: 0x5a5a_5a5a,
        data2: 0x5a5a,
        data3: 0x5a5a,
        data4: [0x5a; 8],
    })
}

fn guid_text(guid: &Guid) -> String {
    let tail: String = guid.data4[2..]
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect();
    format!(
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{tail}",
        guid.data1, guid.data2, guid.data3, guid.data4[0], guid.data4[1]
    )
}

fn units_from_hex(text: &str) -> Vec<u16> {
    text.split_whitespace()
        .map(|unit| u16::from_str_radix(unit, 16).expect("hex unit"))
        .collect()
}

fn units_from_ascii(text: &str) -> Vec<u16> {
    text.bytes().map(u16::from).collect()
}
