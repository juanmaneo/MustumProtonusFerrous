//! The `ICU_ESCAPE` and `ICU_DECODE` transforms of `WinHttpCrackUrl`, and the percent-encoding
//! that `WinHttpCreateUrl` applies to path and extra text.

use crate::{at, c_str, ERROR_INVALID_PARAMETER, SLASH};

const PERCENT: u16 = b'%' as u16;
const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

fn need_escape(ch: u16) -> bool {
    !(32..127).contains(&ch) || b" \"#%<>[\\]^`{|}~".contains(&(ch as u8))
}

/// Percent-encodes `src`. The flag is false when a unit is above 0xff, which Wine rejects; the
/// text is still produced at the length `WinHttpCreateUrl` budgets for, so the caller can check
/// the buffer before it reports the error.
pub(crate) fn escape_string(src: &[u16]) -> (Vec<u16>, bool) {
    let mut out = Vec::with_capacity(src.len());
    let mut ok = true;
    for &ch in src {
        ok &= ch <= 0xff;
        if need_escape(ch) {
            out.push(PERCENT);
            out.push(u16::from(HEX_DIGITS[usize::from((ch >> 4) & 0xf)]));
            out.push(u16::from(HEX_DIGITS[usize::from(ch & 0xf)]));
        } else {
            out.push(ch);
        }
    }
    (out, ok)
}

/// `escape_url`: percent-encodes the text from the last slash on. A URL without a slash is
/// returned unchanged.
pub fn escape_url(url: &[u16]) -> Result<Vec<u16>, u32> {
    let Some(slash) = c_str(url).iter().rposition(|&unit| unit == SLASH) else {
        return Ok(url.to_vec());
    };
    let (escaped, ok) = escape_string(&url[slash..]);
    if !ok {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let mut out = url[..slash].to_vec();
    out.extend(escaped);
    Ok(out)
}

/// `decode_url`: replaces each `%XX` with the unit it names. The result is never longer than
/// `url`, and a `%` that is not followed by two hex digits is kept as it is.
pub fn decode_url(url: &[u16]) -> Vec<u16> {
    let mut out = Vec::with_capacity(url.len());
    let mut i = 0;
    while i < url.len() {
        match escape_at(url, i) {
            Some(unit) => {
                out.push(unit);
                i += 3;
            }
            None => {
                out.push(url[i]);
                i += 1;
            }
        }
    }
    out
}

fn escape_at(url: &[u16], i: usize) -> Option<u16> {
    if at(url, i) != PERCENT {
        return None;
    }
    Some((hex_value(at(url, i + 1))? << 4) | hex_value(at(url, i + 2))?)
}

fn hex_value(unit: u16) -> Option<u16> {
    match unit {
        0x30..=0x39 => Some(unit - 0x30),
        0x41..=0x46 => Some(unit - 0x41 + 10),
        0x61..=0x66 => Some(unit - 0x61 + 10),
        _ => None,
    }
}
