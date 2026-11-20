//! `MSSTYLES_GetNextLong`, which `MSSTYLES_GetNextInteger` wraps.

/// Reads the next decimal number in `text[start..end]`.
///
/// Skips to the first digit before `end`. A `-` seen on the way negates the
/// value, wherever it is. Returns the value and the index just past its digits,
/// or `None` when no digit lies before `end`. `end` must not exceed `text.len()`.
pub(crate) fn next_number(text: &[u16], start: usize, end: usize) -> Option<(i32, usize)> {
    let mut cur = start;
    let mut negative = false;
    while cur < end && !is_digit(text[cur]) {
        negative |= text[cur] == u16::from(b'-');
        cur += 1;
    }
    if cur >= end {
        return None;
    }
    let mut total: i32 = 0;
    while cur < end && is_digit(text[cur]) {
        total = total
            .wrapping_mul(10)
            .wrapping_add(i32::from(text[cur] - u16::from(b'0')));
        cur += 1;
    }
    if negative {
        total = total.wrapping_neg();
    }
    Some((total, cur))
}

fn is_digit(unit: u16) -> bool {
    matches!(unit, 0x30..=0x39)
}
