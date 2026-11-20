//! `SHAnsiToAnsi` and `SHUnicodeToUnicode` in Wine's `dlls/shcore/main.c`.

use crate::str_len;

/// `SHAnsiToAnsi`: copies `src` into `dest` and NUL-terminates it. Returns the copied length plus
/// one when the whole source fit, and 0 otherwise. See [sh_unicode_to_unicode].
pub fn sh_ansi_to_ansi(src: Option<&[u8]>, dest: &mut [u8]) -> usize {
    copy_terminated(src, dest)
}

/// `SHUnicodeToUnicode`: the `WCHAR` form of [sh_ansi_to_ansi], with the same return value.
pub fn sh_unicode_to_unicode(src: Option<&[u16]>, dest: &mut [u16]) -> usize {
    copy_terminated(src, dest)
}

fn copy_terminated<T>(src: Option<&[T]>, dest: &mut [T]) -> usize
where
    T: Copy + Default + PartialEq,
{
    let Some(src) = src else {
        return 0;
    };
    if dest.is_empty() {
        return 0;
    }
    let zero = T::default();
    let copied = str_len(src).min(dest.len() - 1);
    dest[..copied].copy_from_slice(&src[..copied]);
    dest[copied] = zero;
    if src.get(copied) == Some(&zero) {
        copied + 1
    } else {
        0
    }
}
