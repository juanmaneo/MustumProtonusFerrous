//! Port of `str_to_uint64`, `read_isspace` and `wide_text_to_ulonglong` from
//! Wine's `dlls/wsdapi/soap.c`: the decimal parser for WS-Discovery message numbers.

use crate::{until_nul, Error};

/// `MAX_UINT64`, the bound that `wide_text_to_ulonglong` passes.
const MAX_UINT64: u64 = u64::MAX;

/// `read_isspace`: the blanks that the parser trims at both ends.
fn is_space(ch: u8) -> bool {
    matches!(ch, b' ' | b'\t' | b'\r' | b'\n')
}

/// `str_to_uint64`: parses `text` as decimal digits, with blanks trimmed at both
/// ends, and fails if the value is above `max`.
pub fn parse_uint64(text: &[u8], max: u64) -> Result<u64, Error> {
    let start = text
        .iter()
        .position(|&c| !is_space(c))
        .unwrap_or(text.len());
    let end = text[start..]
        .iter()
        .rposition(|&c| !is_space(c))
        .map_or(start, |i| start + i + 1);
    let digits = &text[start..end];

    if digits.is_empty() {
        return Err(Error::InvalidFormat);
    }

    let mut ret: u64 = 0;
    for &ch in digits {
        if !ch.is_ascii_digit() {
            return Err(Error::InvalidFormat);
        }
        let val = u64::from(ch - b'0');
        // Wine's `max - val` wraps for a `max` below 9. Keep the wrap so that the
        // check matches Wine's.
        if ret > max / 10 || ret * 10 > max.wrapping_sub(val) {
            return Err(Error::NumericOverflow);
        }
        ret = ret * 10 + val;
    }
    Ok(ret)
}

/// `wide_text_to_ulonglong`: parses the UTF-16 `text`, cut at its first NUL, as a
/// decimal `u64`. Empty text fails with `Error::Fail`.
///
/// Wine converts the text to UTF-8 first. Each non-ASCII unit becomes bytes of
/// 0x80 or more, which are neither digits nor blanks, so one such byte gives the
/// same result. The first non-digit or the first overflow decides the error, and
/// both come at the same position. So each non-ASCII unit maps to the byte 0x80.
pub fn parse_wide_uint64(text: &[u16]) -> Result<u64, Error> {
    let text = until_nul(text);
    if text.is_empty() {
        return Err(Error::Fail);
    }
    let bytes: Vec<u8> = text
        .iter()
        .map(|&u| if u < 0x80 { u as u8 } else { 0x80 })
        .collect();
    parse_uint64(&bytes, MAX_UINT64)
}
