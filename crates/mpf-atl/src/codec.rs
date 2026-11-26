use crate::error::Error;
use crate::script::{MINUS, PLUS};
use crate::text::is_space;

/// `wctoint` digit zeros: ASCII `0`, then the start of each Unicode decimal digit block.
const DIGIT_ZEROS: [u16; 18] = [
    0x30, 0x660, 0x6f0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xc66, 0xce6, 0xd66, 0xe50, 0xed0,
    0xf20, 0x1040, 0x17e0, 0x1810, 0xff10,
];

fn decimal_digit(c: u16) -> Option<i64> {
    DIGIT_ZEROS
        .into_iter()
        .find(|&zero| (zero..=zero + 9).contains(&c))
        .map(|zero| i64::from(c - zero))
}

/// `_wcstoi64_l` in base 10: skips leading whitespace and takes one sign. Accumulation
/// saturates at the i64 range, which is what the C overflow checks produce.
fn wcstoi64_decimal(text: &[u16]) -> i64 {
    let start = text.iter().take_while(|&&c| is_space(c)).count();
    let (negative, digits) = match text.get(start) {
        Some(&MINUS) => (true, &text[start + 1..]),
        Some(&PLUS) => (false, &text[start + 1..]),
        _ => (false, &text[start..]),
    };
    let mut value = 0i64;
    for &c in digits {
        let Some(digit) = decimal_digit(c) else {
            break;
        };
        let digit = if negative { -digit } else { digit };
        value = value.saturating_mul(10).saturating_add(digit);
    }
    value
}

/// `wcstoul(s, NULL, 10)` on a 32-bit `unsigned long`: clamps, then truncates.
pub(crate) fn wcstoul_decimal(text: &[u16]) -> u32 {
    let value = wcstoi64_decimal(text);
    if value > i64::from(u32::MAX) {
        u32::MAX
    } else if value < -i64::from(u32::MAX) {
        1
    } else {
        value as u32
    }
}

/// `xdigit_to_int`: ASCII hex digits only.
fn hex_digit(c: u16) -> Option<u8> {
    match c {
        0x30..=0x39 => Some((c - 0x30) as u8),
        0x41..=0x46 => Some((c - 0x41 + 10) as u8),
        0x61..=0x66 => Some((c - 0x61 + 10) as u8),
        _ => None,
    }
}

/// The bytes of a `b` value: each pair of hex digits is one byte. An odd length or a
/// non-hex digit fails.
pub(crate) fn decode_binary(word: &[u16]) -> Result<Vec<u8>, Error> {
    word.chunks(2)
        .map(|pair| {
            let high = hex_digit(pair[0]).ok_or(Error::Fail)?;
            let low = hex_digit(*pair.get(1).ok_or(Error::Fail)?).ok_or(Error::Fail)?;
            Ok((high << 4) | low)
        })
        .collect()
}
