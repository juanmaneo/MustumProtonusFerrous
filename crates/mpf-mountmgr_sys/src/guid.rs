//! `parse_uuid` from `dbus.c`.

/// The fields of a Windows `GUID`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Parses an 8-4-4-4-12 UUID, or a `XXXX-XXXX` FAT volume serial.
///
/// The input is read up to its first NUL, as `strlen` reads a C string.
/// Returns `None` when the form does not match.
pub fn parse_uuid(input: &[u8]) -> Option<Guid> {
    let s = match input.iter().position(|&b| b == 0) {
        Some(end) => &input[..end],
        None => input,
    };
    if s.len() == 36 && s[8] == b'-' && s[13] == b'-' && s[18] == b'-' && s[23] == b'-' {
        let (data1, data2, data3) = scan_fields(s)?;
        let mut data4 = [0u8; 8];
        for (byte, &at) in data4.iter_mut().zip(&DATA4_PAIRS) {
            *byte = hex_pair(s, at)?;
        }
        return Some(Guid {
            data1,
            data2,
            data3,
            data4,
        });
    }
    if s.len() == 9 && s[4] == b'-' {
        let (data2, data3) = scan_short(s)?;
        return Some(Guid {
            data2,
            data3,
            ..Guid::default()
        });
    }
    None
}

/// The first index of each Data4 byte pair in the 36-character form, skipping the `-` at 23.
const DATA4_PAIRS: [usize; 8] = [19, 21, 24, 26, 28, 30, 32, 34];

/// `sscanf(str, "%x-%hx-%hx-", ...) == 3`. The trailing `-` cannot change the count.
fn scan_fields(s: &[u8]) -> Option<(u32, u16, u16)> {
    let (data1, pos) = scan_hex(s, 0)?;
    let pos = expect_dash(s, pos)?;
    let (data2, pos) = scan_hex(s, pos)?;
    let pos = expect_dash(s, pos)?;
    let (data3, _) = scan_hex(s, pos)?;
    Some((data1, data2 as u16, data3 as u16))
}

/// `sscanf(str, "%hx-%hx", ...) == 2` for the FAT serial form.
fn scan_short(s: &[u8]) -> Option<(u16, u16)> {
    let (data2, pos) = scan_hex(s, 0)?;
    let pos = expect_dash(s, pos)?;
    let (data3, _) = scan_hex(s, pos)?;
    Some((data2 as u16, data3 as u16))
}

/// Reads one `%x` conversion as glibc does: white space, an optional sign, an
/// optional `0x` prefix, then one or more hex digits. Returns the value, wrapped
/// to 32 bits, and the index after it.
fn scan_hex(s: &[u8], mut pos: usize) -> Option<(u32, usize)> {
    while s.get(pos).is_some_and(|&b| is_space(b)) {
        pos += 1;
    }
    let negative = match s.get(pos) {
        Some(b'-') => {
            pos += 1;
            true
        }
        Some(b'+') => {
            pos += 1;
            false
        }
        _ => false,
    };
    if s.get(pos) == Some(&b'0') && matches!(s.get(pos + 1), Some(b'x' | b'X')) {
        pos += 2;
    }
    let start = pos;
    let mut value = 0u32;
    while let Some(digit) = s.get(pos).and_then(|&b| hex_digit(b)) {
        value = value.wrapping_mul(16).wrapping_add(u32::from(digit));
        pos += 1;
    }
    if pos == start {
        return None;
    }
    Some((
        if negative {
            value.wrapping_neg()
        } else {
            value
        },
        pos,
    ))
}

fn expect_dash(s: &[u8], pos: usize) -> Option<usize> {
    (s.get(pos) == Some(&b'-')).then_some(pos + 1)
}

fn hex_pair(s: &[u8], at: usize) -> Option<u8> {
    Some((hex_digit(s[at])? << 4) | hex_digit(s[at + 1])?)
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
}
