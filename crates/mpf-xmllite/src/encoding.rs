//! Encoding names, code pages and byte-order detection from reader.c and writer.c.

use crate::chars::is_namestartchar;
use crate::{c_str, E_NOTIMPL, MX_E_INPUTEND};

/// `xml_encoding`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlEncoding {
    UsAscii,
    Utf16,
    Utf8,
    Unknown,
}

struct EncodingData {
    name: &'static str,
    encoding: XmlEncoding,
    code_page: u32,
}

/// Sorted by name, which the binary search in `search` relies on.
const ENCODING_MAP: [EncodingData; 3] = [
    EncodingData {
        name: "US-ASCII",
        encoding: XmlEncoding::UsAscii,
        code_page: 20127,
    },
    EncodingData {
        name: "UTF-16",
        encoding: XmlEncoding::Utf16,
        code_page: 1200,
    },
    EncodingData {
        name: "UTF-8",
        encoding: XmlEncoding::Utf8,
        code_page: 65001,
    },
];

fn entry(encoding: XmlEncoding) -> Option<&'static EncodingData> {
    ENCODING_MAP.iter().find(|entry| entry.encoding == encoding)
}

/// `get_encoding_name`. `Unknown` has no name, where the C reads past its table.
pub fn get_encoding_name(encoding: XmlEncoding) -> Option<&'static str> {
    entry(encoding).map(|entry| entry.name)
}

/// `get_code_page`, which fails with `E_NOTIMPL` for `Unknown`.
pub fn get_code_page(encoding: XmlEncoding) -> Result<u32, i32> {
    entry(encoding)
        .map(|entry| entry.code_page)
        .ok_or(E_NOTIMPL)
}

/// `get_encoding_from_codepage`.
pub fn get_encoding_from_codepage(code_page: u32) -> XmlEncoding {
    ENCODING_MAP
        .iter()
        .find(|entry| entry.code_page == code_page)
        .map_or(XmlEncoding::Unknown, |entry| entry.encoding)
}

/// `parse_encoding_name` for a NUL-terminated name, compared as `wcsicmp` does.
pub fn parse_encoding_name(name: &[u16]) -> XmlEncoding {
    let name = c_str(name);
    search(|table| compare(table, name, usize::MAX))
}

/// `parse_encoding_name` for a counted name, compared as `wcsnicmp` does with the length.
/// A name that is a prefix of an encoding name matches it, so `UTF-1` is UTF-16.
pub fn parse_encoding_prefix(name: &[u16]) -> XmlEncoding {
    search(|table| compare(table, name, name.len()))
}

fn search(compare_at: impl Fn(&str) -> i32) -> XmlEncoding {
    let mut min: isize = 0;
    let mut max = ENCODING_MAP.len() as isize - 1;
    while min <= max {
        let n = (min + max) / 2;
        let entry = &ENCODING_MAP[n as usize];
        let c = compare_at(entry.name);
        if c == 0 {
            return entry.encoding;
        }
        if c > 0 {
            max = n - 1;
        } else {
            min = n + 1;
        }
    }
    XmlEncoding::Unknown
}

/// The `wcsnicmp` loop: stops at the first differing unit, or at the end of the table name.
/// A name shorter than `count` reads as NUL.
fn compare(table: &str, name: &[u16], count: usize) -> i32 {
    let table = table.as_bytes();
    for i in 0..count {
        let a = i32::from(table.get(i).copied().unwrap_or(0));
        let b = i32::from(name.get(i).copied().unwrap_or(0));
        let ret = fold(a) - fold(b);
        if ret != 0 || a == 0 {
            return ret;
        }
    }
    0
}

fn fold(unit: i32) -> i32 {
    if (i32::from(b'a')..=i32::from(b'z')).contains(&unit) {
        unit - 0x20
    } else {
        unit
    }
}

/// `readerinput_detectencoding` over the start of the input. Returns the encoding and the
/// number of bytes that a byte order mark consumes. Fewer than three bytes is an error.
pub fn detect_encoding(data: &[u8]) -> Result<(XmlEncoding, usize), i32> {
    if data.len() < 3 {
        return Err(MX_E_INPUTEND);
    }
    if is_utf8(data) {
        return Ok((XmlEncoding::Utf8, 0));
    }
    let first = u16::from_le_bytes([byte(data, 0), byte(data, 1)]);
    if first == u16::from(b'<') {
        let second = u16::from_le_bytes([byte(data, 2), byte(data, 3)]);
        let is_xml =
            second == u16::from(b'?') || second == u16::from(b'!') || is_namestartchar(second);
        let encoding = if is_xml {
            XmlEncoding::Utf16
        } else {
            XmlEncoding::Unknown
        };
        return Ok((encoding, 0));
    }
    if data.starts_with(&[0xef, 0xbb, 0xbf]) {
        Ok((XmlEncoding::Utf8, 3))
    } else if data.starts_with(&[0xff, 0xfe]) {
        Ok((XmlEncoding::Utf16, 2))
    } else {
        Ok((XmlEncoding::Unknown, 0))
    }
}

/// `readerinput_is_utf8`. The shift tests read the second byte as a signed char, as the C
/// does on x86, so they never hold; they are kept for fidelity.
fn is_utf8(data: &[u8]) -> bool {
    if data.starts_with(b"<?") || data.starts_with(b"<!") {
        return true;
    }
    let second = byte(data, 1);
    byte(data, 0) == b'<'
        && ((1..=0x7f).contains(&second)
            || ((second as i8) >> 5) == 6
            || ((second as i8) >> 4) == 0xe
            || ((second as i8) >> 3) == 0x1e)
}

/// A byte of the input. Past the end it reads as 0, where the C reads the buffer past its end.
fn byte(data: &[u8], index: usize) -> u8 {
    data.get(index).copied().unwrap_or(0)
}
