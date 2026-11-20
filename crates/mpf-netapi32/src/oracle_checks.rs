//! Rows from the Wine oracle, the verbatim C driver built with ASan and UBSan. The oracle has 255
//! rows, generated under `target/canon/netapi32` (gitignored). These 25 cover each mode. The rest
//! are in `edge_checks.rs`. Seven sanitizer-flagged cases are left out, as described in the crate
//! docs.

use crate::dav::{http_from_unc, unc_from_http, DavError};
use crate::mac::format_mac;
use crate::nbt::{encode_name, name_request};
use crate::netbios_status_to_api_status;
use crate::samba::{
    access_mask_to_samba, ace_flags_to_samba, ace_type_to_samba, sd_control_to_samba,
};
use crate::server::validate_servername;
use crate::wide::until_nul;
use crate::ERROR_INSUFFICIENT_BUFFER;

/// `(input line, expected output)`, in the driver's format. `\uXXXX` and `\xHH` are escapes.
const ROWS: &[(&str, &str)] = &[
    (r"status|9", "2105"),
    (r"status|4294967295", "2136"),
    (r"enc|\xff\x80|S", "2050504941434143414341434143414341434143414341434143414341434143415300"),
    (r"pkt|ABC|1234|0021|1|271|", "1234011000010000000000002045424543454443414341434143414341434143414341434143414341434143410000210001"),
    (r"pkt|ABC|1234|0020|0|270|", "none"),
    (r"pkt|ABC|1234|0020|0|271|SS", "12340100000100000000000020454245434544434143414341434143414341434143414341434143414341434153530000200001"),
    (r"dav2http|\\server@-5\foo|512", "ok:29:http://server:4294967291/foo"),
    (r"dav2http|\\server@80foo\bar|512", "ok:22:http://server/foo/bar"),
    (r"dav2http|\\server\u0660|512", "ok:15:http://server\\u0660"),
    (r"dav2http|\\server|13", "err:122:14"),
    (r"dav2http|\\server@0\foo|512", "err:87"),
    (r"http2unc|http://server:80x|512", "ok:21:\\\\server\\DavWWWRootx"),
    (r"http2unc|https://server:80/x|512", "ok:29:\\\\server@SSL@80\\DavWWWRoot\\x"),
    (r"http2unc|http://server:|512", "ok:21:\\\\server@\\DavWWWRoot"),
    (r"http2unc|http://server//x|512", "err:67"),
    (r"http2unc|ftp://x|512", "err:87"),
    (r"validate|<null>", "0"),
    (r"validate|\\", "123"),
    (r"mac|001122aabbcc|13", "001122AABBCC"),
    (r"mac|<null>|1", ""),
    (r"sdctl|ffff", "32575"),
    (r"acetype|4", "0"),
    (r"aceflags|223", "221"),
    (r"mask|12345678", "0"),
    (r"bogus|x", "error"),
];

/// Runs one driver line through the port and formats the result as the C driver prints it.
pub(crate) fn run(line: &str) -> String {
    let fields: Vec<&str> = line.split('|').collect();
    let field = |i: usize| fields.get(i).copied().unwrap_or("");
    match field(0) {
        "status" => netbios_status_to_api_status(parse_dec(field(1)) as u32).to_string(),
        "enc" => {
            let name = bytes_of(&decode(field(1)));
            let scope = bytes_of(&decode(field(2)));
            hex(&encode_name(&name, &scope))
        }
        "pkt" => {
            let name = bytes_of(&decode(field(1)));
            let scope = bytes_of(&decode(field(6)));
            let packet = name_request(
                &name,
                parse_hex(field(2)) as u16,
                parse_hex(field(3)) as u16,
                parse_dec(field(4)) != 0,
                &scope,
                parse_dec(field(5)) as usize,
            );
            match packet {
                Some(bytes) => hex(&bytes),
                None => "none".to_owned(),
            }
        }
        "dav2http" | "http2unc" => run_dav(field(0), field(1), field(2)),
        "validate" => {
            let text = (field(1) != "<null>").then(|| decode(field(1)));
            validate_servername(text.as_deref()).to_string()
        }
        "mac" => {
            let address = (field(1) != "<null>").then(|| hex_bytes(field(1)));
            escape(&format_mac(
                address.as_deref(),
                parse_dec(field(2)) as usize,
            ))
        }
        "sdctl" => sd_control_to_samba(parse_hex(field(1)) as u16).to_string(),
        "acetype" => ace_type_to_samba(parse_dec(field(1)) as u8).to_string(),
        "aceflags" => ace_flags_to_samba(parse_dec(field(1)) as u8).to_string(),
        "mask" => access_mask_to_samba(parse_hex(field(1)) as u32).to_string(),
        _ => "error".to_owned(),
    }
}

/// The DAV conversion named by `mode`, with the buffer capacity from `cap`.
fn run_dav(mode: &str, text: &str, cap: &str) -> String {
    let input = decode(text);
    let capacity = parse_dec(cap) as usize;
    let result = if mode == "dav2http" {
        http_from_unc(&input, capacity)
    } else {
        unc_from_http(&input, capacity)
    };
    match result {
        Ok(units) => format!("ok:{}:{}", units.len(), escape(&units)),
        Err(DavError::InsufficientBuffer { required }) => {
            format!("err:{ERROR_INSUFFICIENT_BUFFER}:{required}")
        }
        Err(error) => format!("err:{}", error.win32_code()),
    }
}

/// The unit and length of the escape at the start of `s`, if there is one.
fn escape_at(s: &[u8]) -> Option<(u16, usize)> {
    let digits = match s {
        [b'\\', b'u', ..] => 4,
        [b'\\', b'x', ..] => 2,
        _ => return None,
    };
    let hex = s.get(2..2 + digits)?;
    let unit = hex.iter().try_fold(0u16, |value, &c| {
        Some((value << 4) | char::from(c).to_digit(16)? as u16)
    })?;
    Some((unit, 2 + digits))
}

/// The units of a driver text argument: `\uXXXX` and `\xHH` are one unit each, and every other
/// byte is one unit. A malformed escape is read as its bytes.
pub(crate) fn decode(text: &str) -> Vec<u16> {
    let bytes = text.as_bytes();
    let mut units = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match escape_at(&bytes[i..]) {
            Some((unit, used)) => {
                units.push(unit);
                i += used;
            }
            None => {
                units.push(u16::from(bytes[i]));
                i += 1;
            }
        }
    }
    units
}

/// The text result: units 0x20 to 0x7e as themselves, others as `\uXXXX`, up to the first NUL.
pub(crate) fn escape(units: &[u16]) -> String {
    let mut out = String::new();
    for &unit in until_nul(units) {
        if (0x20..=0x7e).contains(&unit) {
            out.push(char::from(unit as u8));
        } else {
            out.push_str(&format!("\\u{unit:04X}"));
        }
    }
    out
}

/// The bytes that a C `char` array holds for these units: each unit keeps its low byte.
pub(crate) fn bytes_of(units: &[u16]) -> Vec<u8> {
    units.iter().map(|&unit| unit as u8).collect()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// The bytes of hex pairs. A trailing odd digit is dropped, as `strlen(hex) / 2` drops it.
pub(crate) fn hex_bytes(text: &str) -> Vec<u8> {
    let (pairs, _) = text.as_bytes().as_chunks::<2>();
    pairs
        .iter()
        .map(|pair| (hex_value(pair[0]) << 4) | hex_value(pair[1]))
        .collect()
}

fn hex_value(c: u8) -> u8 {
    char::from(c).to_digit(16).unwrap_or(0) as u8
}

/// `strtoul(text, NULL, 10)` for the plain numbers in the rows. Other text gives 0.
pub(crate) fn parse_dec(text: &str) -> u64 {
    text.parse().unwrap_or(0)
}

/// `strtoul(text, NULL, 16)` for the plain numbers in the rows. Other text gives 0.
pub(crate) fn parse_hex(text: &str) -> u64 {
    u64::from_str_radix(text, 16).unwrap_or(0)
}

#[test]
fn oracle_rows_match_the_port() {
    for &(line, expected) in ROWS {
        assert_eq!(run(line), expected, "{line}");
    }
}
