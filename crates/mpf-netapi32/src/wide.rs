//! UTF-16 helpers with the C string rules that the Wine code relies on.
//!
//! Case folding and white space are ASCII only, as in the C locale of the oracle's shim.

pub(crate) const BACKSLASH: u16 = b'\\' as u16;
pub(crate) const SLASH: u16 = b'/' as u16;
pub(crate) const COLON: u16 = b':' as u16;
pub(crate) const AT: u16 = b'@' as u16;

/// The units before the first NUL, as a C string reads them.
pub(crate) fn until_nul(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}

/// The unit at `i`, or NUL past the end, as reading a terminated string gives.
pub(crate) fn at(s: &[u16], i: usize) -> u16 {
    s.get(i).copied().unwrap_or(0)
}

/// `iswspace` in the C locale, for ASCII.
pub(crate) fn is_space(c: u16) -> bool {
    c == b' ' as u16 || (0x09..=0x0d).contains(&c)
}

pub(crate) fn is_ascii_digit(c: u16) -> bool {
    (b'0' as u16..=b'9' as u16).contains(&c)
}

fn lower(c: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

/// `wcsicmp(a, b) == 0` for an ASCII literal `b`. Pass `until_nul(a)` when `a` is a C string.
pub(crate) fn eq_ci(a: &[u16], b: &str) -> bool {
    a.iter().copied().map(lower).eq(b.encode_utf16().map(lower))
}

/// The Unicode decimal digit runs that msvcrt's `wctoint` accepts, sorted.
const DIGIT_ZEROS: [u16; 17] = [
    0x660, 0x6f0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xc66, 0xce6, 0xd66, 0xe50, 0xed0, 0xf20,
    0x1040, 0x17e0, 0x1810, 0xff10,
];

/// The value of a base-10 digit: an ASCII digit, or one of the Unicode decimal digits in
/// `DIGIT_ZEROS`.
pub(crate) fn decimal_digit(c: u16) -> Option<u16> {
    if is_ascii_digit(c) {
        return Some(c - b'0' as u16);
    }
    DIGIT_ZEROS
        .iter()
        .find(|&&zero| (zero..=zero + 9).contains(&c))
        .map(|&zero| c - zero)
}

/// `wcstol(s, &end, 10)` with msvcrt's rules. Returns the value, clamped to `i32` as `wcstol`
/// clamps it, and the index after the last digit. The index is 0 when no digit follows the white
/// space and sign, which is the C code's "no conversion" case.
pub(crate) fn wcstol_dec(s: &[u16]) -> (i32, usize) {
    let mut i = 0;
    while is_space(at(s, i)) {
        i += 1;
    }
    let mut sign = 1;
    if at(s, i) == b'-' as u16 {
        sign = -1;
        i += 1;
    } else if at(s, i) == b'+' as u16 {
        i += 1;
    }
    let mut value: i64 = 0;
    let mut end = 0;
    while let Some(digit) = decimal_digit(at(s, i)) {
        value = value
            .saturating_mul(10)
            .saturating_add(sign * i64::from(digit));
        i += 1;
        end = i;
    }
    let clamped = value.clamp(i64::from(i32::MIN), i64::from(i32::MAX));
    (clamped as i32, end)
}
