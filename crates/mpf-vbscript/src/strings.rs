//! `Global_Left`, `Global_Right`, `Global_Mid` and `Global_ChrW` in `dlls/vbscript/global.c`. A
//! string is a slice of UTF-16 units, and each result is a slice of its argument.

use crate::VbsError;

/// A count argument. A negative count is `IllegalFuncCall`.
fn count(len: i32) -> Result<usize, VbsError> {
    if len < 0 {
        return Err(VbsError::IllegalFuncCall);
    }
    Ok(len as usize)
}

/// `Left`: the first `len` units, or the whole string if it is shorter.
pub fn left(s: &[u16], len: i32) -> Result<&[u16], VbsError> {
    let n = count(len)?.min(s.len());
    Ok(&s[..n])
}

/// `Right`: the last `len` units, or the whole string if it is shorter.
pub fn right(s: &[u16], len: i32) -> Result<&[u16], VbsError> {
    let n = count(len)?.min(s.len());
    Ok(&s[s.len() - n..])
}

/// `Mid`. `start` is 1-based and must be positive. `len` of `None` takes the rest of the string.
/// A `start` past the end gives an empty string.
pub fn mid(s: &[u16], start: i32, len: Option<i32>) -> Result<&[u16], VbsError> {
    if start <= 0 {
        return Err(VbsError::IllegalFuncCall);
    }
    let from = (start as usize - 1).min(s.len());
    let rest = s.len() - from;
    let n = match len {
        None => rest,
        Some(len) => count(len)?.min(rest),
    };
    Ok(&s[from..from + n])
}

/// `ChrW`. C accepts any value that fits in a `short` or an `unsigned short`, and stores the low
/// 16 bits.
pub fn chr_w(c: i32) -> Result<u16, VbsError> {
    if (i32::from(i16::MIN)..=i32::from(u16::MAX)).contains(&c) {
        Ok(c as u16)
    } else {
        Err(VbsError::IllegalFuncCall)
    }
}
