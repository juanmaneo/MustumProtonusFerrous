//! Port of the search and comparison members of `basic_string<char>` and `basic_string<wchar_t>`
//! in Wine's `dlls/msvcp60/string.c`.
//!
//! A string is a slice, and its size is the slice length. Wine's `npos` is [`NPOS`]. Where Wine
//! raises `_Xran` for a position past the end, the comparisons return `None`. A needle is the
//! slice of exactly the `len` units that Wine's `(const _Elem *, size_type len)` pair names.

use crate::char_traits::{self, Unit};
use crate::NPOS;

/// `find(s, pos, n)`: the first position at or after `pos` where `needle` occurs in `this`.
pub fn find<T: Unit>(this: &[T], needle: &[T], pos: usize) -> usize {
    let (size, len) = (this.len(), needle.len());
    if len == 0 && pos <= size {
        return pos;
    }
    if pos >= size || len > size {
        return NPOS;
    }
    (pos..=size - len)
        .find(|&i| this[i..i + len] == *needle)
        .unwrap_or(NPOS)
}

/// `rfind(s, pos, n)`: the last position at or before `pos` where `needle` occurs in `this`.
pub fn rfind<T: Unit>(this: &[T], needle: &[T], pos: usize) -> usize {
    let (size, len) = (this.len(), needle.len());
    if len == 0 {
        return pos.min(size);
    }
    if len > size {
        return NPOS;
    }
    (0..=pos.min(size - len))
        .rev()
        .find(|&i| this[i..i + len] == *needle)
        .unwrap_or(NPOS)
}

/// `find_first_of(s, off, n)`: the first position at or after `off` holding a unit of `needle`.
pub fn find_first_of<T: Unit>(this: &[T], needle: &[T], off: usize) -> usize {
    (off..this.len())
        .find(|&i| char_traits::find(needle, this[i]).is_some())
        .unwrap_or(NPOS)
}

/// `find_first_not_of(s, off, n)`: the first position at or after `off` holding no unit of
/// `needle`.
pub fn find_first_not_of<T: Unit>(this: &[T], needle: &[T], off: usize) -> usize {
    (off..this.len())
        .find(|&i| char_traits::find(needle, this[i]).is_none())
        .unwrap_or(NPOS)
}

/// `find_last_of(s, off, n)`: the last position at or before `off` holding a unit of `needle`.
pub fn find_last_of<T: Unit>(this: &[T], needle: &[T], off: usize) -> usize {
    find_last_where(this, needle, off, true)
}

/// `find_last_not_of(s, off, n)`: the last position at or before `off` holding no unit of
/// `needle`. An empty needle has no such position, as in Wine.
pub fn find_last_not_of<T: Unit>(this: &[T], needle: &[T], off: usize) -> usize {
    find_last_where(this, needle, off, false)
}

fn find_last_where<T: Unit>(this: &[T], needle: &[T], off: usize, member: bool) -> usize {
    if needle.is_empty() || this.is_empty() {
        return NPOS;
    }
    (0..=off.min(this.len() - 1))
        .rev()
        .find(|&i| char_traits::find(needle, this[i]).is_some() == member)
        .unwrap_or(NPOS)
}

/// Compares the first `num` units of `tail` with `s`. When they match, the shorter sequence
/// orders first.
fn compare_tail<T: Unit>(tail: &[T], num: usize, s: &[T]) -> i32 {
    let num = num.min(tail.len());
    match char_traits::compare(tail, s, num.min(s.len())) {
        0 => num.cmp(&s.len()) as i32,
        ans => ans,
    }
}

/// `compare_substr(pos, num, other)`: compares `num` units from `pos` with `other`. Wine's
/// `compare_substr_cstr_len(pos, num, str, count)` is this function with `other` cut to its first
/// `count` units.
pub fn compare_substr<T: Unit>(this: &[T], pos: usize, num: usize, other: &[T]) -> Option<i32> {
    Some(compare_tail(this.get(pos..)?, num, other))
}

/// `compare_substr_substr(pos, num, other, off, count)`: compares `num` units from `pos` with
/// the `count` units of `other` from `off`.
pub fn compare_substr_substr<T: Unit>(
    this: &[T],
    pos: usize,
    num: usize,
    other: &[T],
    off: usize,
    count: usize,
) -> Option<i32> {
    let tail = other.get(off..)?;
    compare_substr(this, pos, num, &tail[..count.min(tail.len())])
}

/// `compare(other)`: compares the whole string with `other`.
pub fn compare<T: Unit>(this: &[T], other: &[T]) -> i32 {
    compare_tail(this, this.len(), other)
}

/// `compare_cstr(str)`: compares the whole string with the C string `other`, which ends at its
/// first zero unit.
pub fn compare_cstr<T: Unit>(this: &[T], other: &[T]) -> i32 {
    compare_tail(this, this.len(), char_traits::cstr(other))
}

/// `operator==` on two strings.
pub fn equal<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) == 0
}

/// `operator!=` on two strings.
pub fn not_equal<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) != 0
}

/// `operator<` on two strings.
pub fn lower<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) < 0
}

/// `operator<=` on two strings.
pub fn leq<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) <= 0
}

/// `operator>` on two strings.
pub fn greater<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) > 0
}

/// `operator>=` on two strings.
pub fn geq<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare(left, right) >= 0
}

/// `operator==` of a string and a C string.
pub fn equal_str_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) == 0
}

/// `operator!=` of a string and a C string.
pub fn not_equal_str_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) != 0
}

/// `operator<` of a string and a C string.
pub fn lower_bstr_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) < 0
}

/// `operator<=` of a string and a C string.
pub fn leq_bstr_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) <= 0
}

/// `operator>` of a string and a C string.
pub fn greater_bstr_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) > 0
}

/// `operator>=` of a string and a C string.
pub fn geq_bstr_cstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(left, right) >= 0
}

/// `operator==` of a C string and a string. The string is compared with the C string on its
/// left, so the relation of `compare_cstr` is reversed.
pub fn equal_cstr_str<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) == 0
}

/// `operator!=` of a C string and a string.
pub fn not_equal_cstr_str<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) != 0
}

/// `operator<` of a C string and a string.
pub fn lower_cstr_bstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) > 0
}

/// `operator<=` of a C string and a string.
pub fn leq_cstr_bstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) >= 0
}

/// `operator>` of a C string and a string.
pub fn greater_cstr_bstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) < 0
}

/// `operator>=` of a C string and a string.
pub fn geq_cstr_bstr<T: Unit>(left: &[T], right: &[T]) -> bool {
    compare_cstr(right, left) <= 0
}
