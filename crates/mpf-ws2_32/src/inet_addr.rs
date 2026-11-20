//! Legacy IPv4 address parsing, matching Wine's `inet_addr` implementation.

/// The value returned by `inet_addr` when a string cannot be parsed.
pub const INADDR_NONE: u32 = u32::MAX;

/// The error raised when the C API would set `WSAEFAULT` for a null string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InetAddrError {
    /// No input pointer was supplied.
    NullInput,
}

/// Parses an IPv4 string using Wine's legacy `inet_addr` rules.
///
/// Accepts one to four decimal, octal, or hexadecimal components. As in the C API, malformed
/// text returns [`INADDR_NONE`], which is also a possible result for `255.255.255.255`. A missing
/// input is represented separately because Wine sets `WSAEFAULT` in that case. Bytes after the
/// first NUL are ignored.
pub fn inet_addr(input: Option<&[u8]>) -> Result<u32, InetAddrError> {
    let input = input.ok_or(InetAddrError::NullInput)?;
    let c_end = input
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(input.len());
    let input = &input[..c_end];

    if input == b" " {
        return Ok(0);
    }

    let mut parts = [0_u64; 4];
    let mut cursor = 0;
    let mut last_part = 0;

    for (index, part) in parts.iter_mut().enumerate() {
        if !input.get(cursor).is_some_and(u8::is_ascii_digit) {
            return Ok(INADDR_NONE);
        }

        let (value, consumed) = parse_base_zero(&input[cursor..]);
        *part = value;
        last_part = index;
        cursor += consumed;

        match input.get(cursor).copied() {
            None => break,
            Some(byte) if is_ascii_space(byte) => break,
            Some(b'.') => cursor += 1,
            Some(_) => return Ok(INADDR_NONE),
        }

        if index == 3 {
            return Ok(INADDR_NONE);
        }
    }

    if last_part == 0 {
        parts[1] = parts[0] & 0x00ff_ffff;
        parts[0] >>= 24;
    }
    if last_part <= 1 {
        parts[2] = parts[1] & 0x0000_ffff;
        parts[1] >>= 16;
    }
    if last_part <= 2 {
        parts[3] = parts[2] & 0x0000_00ff;
        parts[2] >>= 8;
    }

    if parts.iter().any(|&part| part > 255) {
        return Ok(INADDR_NONE);
    }

    Ok(u32::from_le_bytes([
        parts[0] as u8,
        parts[1] as u8,
        parts[2] as u8,
        parts[3] as u8,
    ]))
}

/// Parses the digits at the start of `text`, which must begin with an ASCII digit, and returns the
/// value with the number of bytes consumed. `strtoul` with base 0 is the model for the prefixes.
fn parse_base_zero(text: &[u8]) -> (u64, usize) {
    let (base, mut cursor) = if text.starts_with(b"0x") || text.starts_with(b"0X") {
        if text.get(2).is_some_and(u8::is_ascii_hexdigit) {
            (16_u64, 2)
        } else {
            (8, 0)
        }
    } else if text.first() == Some(&b'0') {
        (8, 0)
    } else {
        (10, 0)
    };

    let mut value = 0_u64;
    while let Some(&byte) = text.get(cursor) {
        let Some(digit) = digit_value(byte) else {
            break;
        };
        if u64::from(digit) >= base {
            break;
        }
        value = value.saturating_mul(base).saturating_add(u64::from(digit));
        cursor += 1;
    }

    (value, cursor)
}

fn digit_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn is_ascii_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}
