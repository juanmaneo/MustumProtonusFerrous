//! `eval_like` from `query.c`: the WQL `LIKE` match. `%` matches any run of characters and `_`
//! matches one. A doubled backslash in the pattern matches one backslash, but only while text is
//! left. Case is folded with [towupper]. `[` has no special meaning: Wine's FIXME says character
//! ranges are not supported, so `[` is a literal here too.

use crate::wide::{at, rest, towupper};

const PERCENT: u16 = b'%' as u16;
const UNDERSCORE: u16 = b'_' as u16;
const BACKSLASH: u16 = b'\\' as u16;

/// Whether `text` matches `pattern`.
pub fn eval_like(text: &[u16], pattern: &[u16]) -> bool {
    let mut text = text;
    let mut pattern = pattern;
    while !text.is_empty() {
        if at(pattern, 0) == BACKSLASH && at(pattern, 1) == BACKSLASH {
            pattern = rest(pattern);
        }
        if at(pattern, 0) == PERCENT {
            while at(pattern, 0) == PERCENT {
                pattern = rest(pattern);
            }
            if pattern.is_empty() {
                return true;
            }
            while !eval_like(text, pattern) && !text.is_empty() {
                text = rest(text);
            }
            return !text.is_empty();
        }
        if at(pattern, 0) != UNDERSCORE && towupper(at(text, 0)) != towupper(at(pattern, 0)) {
            return false;
        }
        text = rest(text);
        pattern = rest(pattern);
    }
    while at(pattern, 0) == PERCENT {
        pattern = rest(pattern);
    }
    pattern.is_empty()
}
