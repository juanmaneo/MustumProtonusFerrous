//! Port of `char_traits<char>` and `char_traits<wchar_t>` from Wine's `dlls/msvcp60/string.c`.
//!
//! Wine's functions take raw pointers and rely on a zero unit to end a string. Here they take
//! slices. `char` is [`u8`] and `wchar_t` is [`u16`], so one generic body serves both widths.

/// A character unit: `char` (`u8`) or `wchar_t` (`u16`).
pub trait Unit: Copy + Ord + Default {}

impl Unit for u8 {}
impl Unit for u16 {}

/// The `_Invalid_parameter` that Wine raises in `_Copy_s`.
pub struct InvalidParameter;

/// `char_traits::compare`: the sign of the first differing unit among the first `count`, or 0.
pub fn compare<T: Unit>(s1: &[T], s2: &[T], count: usize) -> i32 {
    s1[..count].cmp(&s2[..count]) as i32
}

/// `char_traits::length`: the number of units before the first zero unit. A slice with no zero
/// unit ends at its own length, where Wine would read past it.
pub fn length<T: Unit>(s: &[T]) -> usize {
    s.iter().position(|&c| c == T::default()).unwrap_or(s.len())
}

/// The C string at the start of `s`: the units before its first zero unit.
pub fn cstr<T: Unit>(s: &[T]) -> &[T] {
    &s[..length(s)]
}

/// `char_traits::_Copy_s`. `dest` is the buffer, and its length is Wine's `size`. The copy
/// succeeds only when `dest` and `src` both hold `count` units. Otherwise it reports an invalid
/// parameter, zeroes `dest[0]` if the buffer is non-empty, and copies nothing.
///
/// A `src` shorter than `count` is reported invalid, where Wine would read past its end.
pub fn copy_s<T: Unit>(
    dest: Option<&mut [T]>,
    src: Option<&[T]>,
    count: usize,
) -> Result<(), InvalidParameter> {
    match (dest, src) {
        (Some(d), Some(s)) if d.len() >= count && s.len() >= count => {
            d[..count].copy_from_slice(&s[..count]);
            Ok(())
        }
        (Some(d), _) => {
            if let Some(first) = d.first_mut() {
                *first = T::default();
            }
            Err(InvalidParameter)
        }
        (None, _) => Err(InvalidParameter),
    }
}

/// `char_traits::copy`, which is `_Copy_s(dest, count, src, count)`. Wine passes no buffer size
/// here, so `dest` is limited to `count` units. A `dest` shorter than `count` is reported invalid,
/// where Wine would overrun it.
pub fn copy<T: Unit>(
    dest: Option<&mut [T]>,
    src: Option<&[T]>,
    count: usize,
) -> Result<(), InvalidParameter> {
    copy_s(
        dest.map(|d| {
            let size = count.min(d.len());
            &mut d[..size]
        }),
        src,
        count,
    )
}

/// `char_traits::find`: the position of the first unit of `s` equal to `c`.
pub fn find<T: Unit>(s: &[T], c: T) -> Option<usize> {
    s.iter().position(|&x| x == c)
}

/// `char_traits::assign`: sets the first `num` units of `s` to `c`.
pub fn assign<T: Unit>(s: &mut [T], num: usize, c: T) {
    s[..num].fill(c);
}
