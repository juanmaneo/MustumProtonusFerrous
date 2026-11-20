/// A parsed Wine `GUID`, retaining the integer fields and byte array used by its C struct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Uuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Parses a Bluetooth UUID using Wine's `dbus.c` `parse_uuid` rules.
///
/// The C helper first measures the string through its first NUL byte and requires exactly 36
/// bytes, then applies `%08x-%hx-%hx-%2hhx...` conversions. Consequently this accepts normal
/// UUID text and retains `sscanf`'s permissive handling of signs, leading whitespace in numeric
/// fields, and a final partially parsed hex pair.
pub fn parse_uuid(text: &[u8]) -> Option<Uuid> {
    let nul = text
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(text.len());
    let text = &text[..nul];
    if text.len() != 36 {
        return None;
    }

    let mut at = 0;
    let data1 = scan_hex(text, &mut at, Some(8), 32)?;
    if !take_dash(text, &mut at) {
        return None;
    }
    let data2 = scan_hex(text, &mut at, None, 16)? as u16;
    if !take_dash(text, &mut at) {
        return None;
    }
    let data3 = scan_hex(text, &mut at, None, 16)? as u16;
    if !take_dash(text, &mut at) {
        return None;
    }

    let mut data4 = [0; 8];
    for byte in &mut data4[..2] {
        *byte = scan_hex(text, &mut at, Some(2), 8)? as u8;
    }
    if !take_dash(text, &mut at) {
        return None;
    }
    for byte in &mut data4[2..] {
        *byte = scan_hex(text, &mut at, Some(2), 8)? as u8;
    }

    Some(Uuid {
        data1,
        data2,
        data3,
        data4,
    })
}

fn take_dash(text: &[u8], at: &mut usize) -> bool {
    if text.get(*at) != Some(&b'-') {
        return false;
    }
    *at += 1;
    true
}

/// One `sscanf` integer conversion: skips leading white space, takes an optional sign and `0x`
/// prefix, then hex digits up to `width` characters (the sign and prefix count toward it). The value
/// is truncated to `bits`, as the C `%hx` and `%2hhx` stores are.
fn scan_hex(text: &[u8], at: &mut usize, width: Option<usize>, bits: u32) -> Option<u32> {
    while text.get(*at).is_some_and(|byte| is_scan_space(*byte)) {
        *at += 1;
    }
    let end = width.map_or(text.len(), |width| at.saturating_add(width).min(text.len()));
    let mut cursor = *at;
    let negative = match text.get(cursor) {
        Some(b'-') => {
            cursor += 1;
            true
        }
        Some(b'+') => {
            cursor += 1;
            false
        }
        _ => false,
    };
    if cursor + 1 < end && text[cursor] == b'0' && matches!(text[cursor + 1], b'x' | b'X') {
        cursor += 2;
    }

    let mut value = 0u32;
    let first_digit = cursor;
    while cursor < end {
        let Some(digit) = hex_digit(text[cursor]) else {
            break;
        };
        value = value.wrapping_mul(16).wrapping_add(u32::from(digit));
        cursor += 1;
    }
    if cursor == first_digit {
        return None;
    }
    *at = cursor;
    if negative {
        value = value.wrapping_neg();
    }
    Some(if bits == 32 {
        value
    } else {
        value & ((1u32 << bits) - 1)
    })
}

fn is_scan_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
