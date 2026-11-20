//! `lstrcpynW` and `MSSTYLES_GetNextToken`.

use crate::{is_space, unit_at};

const COMMA: u16 = b',' as u16;

/// Copies from `text[from..]` into a buffer of `n` units, as `lstrcpynW` does.
///
/// At most `n - 1` units are copied, and the copy stops at a NUL or at the end
/// of `text`. The terminator is not part of the result, and `n == 0` copies
/// nothing.
pub(crate) fn copy_units(text: &[u16], from: usize, n: usize) -> Vec<u16> {
    text.get(from..)
        .unwrap_or_default()
        .iter()
        .take(n.saturating_sub(1))
        .take_while(|&&unit| unit != 0)
        .copied()
        .collect()
}

/// Reads the next comma-separated token from `text[start..end]`.
///
/// Leading whitespace and commas are skipped. Trailing whitespace is trimmed by
/// looking at the unit at `end`, so a comma-less token followed by whitespace
/// loses its last unit, as Wine's `MSSTYLES_GetNextToken` does. Returns the
/// token, copied into a buffer of `buf_size` units, and the index of the comma
/// or `end` that ends it.
pub(crate) fn next_token(
    text: &[u16],
    start: usize,
    end: usize,
    buf_size: usize,
) -> Option<(Vec<u16>, usize)> {
    let mut cur = start;
    while cur < end && (is_space(text[cur]) || text[cur] == COMMA) {
        cur += 1;
    }
    if cur >= end {
        return None;
    }
    let first = cur;
    while cur < end && text[cur] != COMMA {
        cur += 1;
    }
    let mut last = cur;
    while is_space(unit_at(text, last)) {
        last -= 1;
    }
    let token = copy_units(text, first, buf_size.min(last - first + 1));
    Some((token, cur))
}
