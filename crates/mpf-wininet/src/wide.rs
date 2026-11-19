//! UTF-16 helpers with the C string rules that the Wine code relies on.
//!
//! Case folding and white space are ASCII only, as in the C locale of the oracle's shim. msvcrt
//! also folds and classifies code points above U+00FF, which is not reproduced.

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
    c == u16::from(b' ') || (0x09..=0x0d).contains(&c)
}

fn lower(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 0x20
    } else {
        c
    }
}

/// `wcsicmp(a, b) == 0` for an ASCII literal `b`. Pass `until_nul(a)` when `a` is a C string.
pub(crate) fn eq_ci(a: &[u16], b: &str) -> bool {
    a.iter().copied().map(lower).eq(b.encode_utf16().map(lower))
}

/// `wcscmp(a, b) == 0` for an ASCII literal `b`. Pass `until_nul(a)` when `a` is a C string.
pub(crate) fn eq_exact(a: &[u16], b: &str) -> bool {
    a.iter().copied().eq(b.encode_utf16())
}

/// `wcsnicmp(a, b, a.len()) == 0`. The units fold equal, and the comparison ends early at a NUL
/// unit of `a`, as `wcsnicmp` does. The lengths must match.
pub(crate) fn nicmp_eq(a: &[u16], b: &[u16]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (&x, &y) in a.iter().zip(b) {
        if lower(x) != lower(y) {
            return false;
        }
        if x == 0 {
            break;
        }
    }
    true
}

/// The Unicode decimal digit runs that msvcrt's `wctoint` accepts, sorted.
const ZEROS: [u16; 17] = [
    0x660, 0x6f0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xc66, 0xce6, 0xd66, 0xe50, 0xed0, 0xf20,
    0x1040, 0x17e0, 0x1810, 0xff10,
];

fn digit10(c: u16) -> Option<u16> {
    if (u16::from(b'0')..=u16::from(b'9')).contains(&c) {
        return Some(c - u16::from(b'0'));
    }
    ZEROS
        .iter()
        .find(|&&zero| (zero..=zero + 9).contains(&c))
        .map(|&zero| c - zero)
}

/// `wcstoul(s, &end, 10)` with msvcrt's rules. Returns the value, clamped the way `_wcstoul_l`
/// clamps it, and the index after the last digit. The index is 0 when no digit follows the white
/// space, which is the C code's "no conversion" case.
pub(crate) fn wcstoul_dec(s: &[u16]) -> (u32, usize) {
    let mut i = 0;
    while is_space(at(s, i)) {
        i += 1;
    }
    let negative = at(s, i) == u16::from(b'-');
    if negative || at(s, i) == u16::from(b'+') {
        i += 1;
    }
    let mut value: i64 = 0;
    let mut end = 0;
    while let Some(digit) = digit10(at(s, i)) {
        let digit = i64::from(digit);
        value = value
            .saturating_mul(10)
            .saturating_add(if negative { -digit } else { digit });
        i += 1;
        end = i;
    }
    let max = i64::from(u32::MAX);
    let clamped = if value > max {
        u32::MAX
    } else if value < -max {
        1
    } else {
        value as u32
    };
    (clamped, end)
}

/// The position of the first unit `c` in `s`, if any. `c` is an ASCII byte.
pub(crate) fn find(s: &[u16], c: u8) -> Option<usize> {
    s.iter().position(|&unit| unit == u16::from(c))
}
