//! `Global_Escape` and `Global_Unescape` in `dlls/vbscript/global.c`. Both stop at the first NUL,
//! as the C loops over `*ptr` do.

use crate::{hex_value, unit_at};

const PERCENT: u16 = b'%' as u16;
const U: u16 = b'u' as u16;

fn until_nul(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}

/// `is_ecma_nonblank`: the characters that `escape` keeps as they are.
fn is_nonblank(c: u8) -> bool {
    matches!(
        c,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'@' | b'*' | b'_' | b'+' | b'-' | b'.' | b'/'
    )
}

/// `int_to_hex`: the uppercase digit for a nibble.
fn upper_hex(n: u16) -> u16 {
    if n < 10 {
        u16::from(b'0') + n
    } else {
        u16::from(b'A') + n - 10
    }
}

/// `Escape`. Units above `0xff` become `%uXXXX`, the non-blank ASCII characters stay, and every
/// other unit becomes `%XX`.
pub fn escape(src: &[u16]) -> Vec<u16> {
    let mut out = Vec::new();
    for &c in until_nul(src) {
        if c > 0xff {
            out.extend([
                PERCENT,
                U,
                upper_hex(c >> 12),
                upper_hex((c >> 8) & 0xf),
                upper_hex((c >> 4) & 0xf),
                upper_hex(c & 0xf),
            ]);
        } else if is_nonblank(c as u8) {
            out.push(c);
        } else {
            out.extend([PERCENT, upper_hex(c >> 4), upper_hex(c & 0xf)]);
        }
    }
    out
}

/// The value of `digits` hex digits starting at `start`, or `None` if one is not hex. A unit past
/// the end is NUL, which is not hex.
fn hex_run(src: &[u16], start: usize, digits: usize) -> Option<u32> {
    let mut value = 0;
    for i in start..start + digits {
        value = (value << 4) | hex_value(unit_at(src, i))?;
    }
    Some(value)
}

/// `Unescape`. Decodes `%XX` and `%uXXXX`. Any other `%` is copied as it is.
pub fn unescape(src: &[u16]) -> Vec<u16> {
    let src = until_nul(src);
    let mut out = Vec::new();
    let mut i = 0;
    while i < src.len() {
        if src[i] == PERCENT {
            if let Some(unit) = hex_run(src, i + 1, 2) {
                out.push(unit as u16);
                i += 3;
                continue;
            }
            if unit_at(src, i + 1) == U {
                if let Some(unit) = hex_run(src, i + 2, 4) {
                    out.push(unit as u16);
                    i += 6;
                    continue;
                }
            }
        }
        out.push(src[i]);
        i += 1;
    }
    out
}
