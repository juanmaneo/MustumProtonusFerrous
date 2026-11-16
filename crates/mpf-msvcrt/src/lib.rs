//! Partial `msvcrt` byte-string and UTF-16 routines based on Wine's
//! `dlls/msvcrt/string.c` and `dlls/msvcrt/wcs.c`.
//!
//! The subset includes byte-string comparison, length, search, copy,
//! concatenation, and mutation routines alongside UTF-16 equivalents. Slice-based APIs treat
//! the first NUL as the string end (or the slice end if no NUL is present) and
//! avoid pointer-style out-of-bounds reads. Case-insensitive comparison uses
//! ASCII-only folding; locale-aware behavior and `MSVCRT_CHECK_PMT`
//! invalid-parameter handling are not ported.

/// Replaces all bytes before the first NUL and returns the count changed.
///
/// This slice-based equivalent of Wine's `_strset` reports a count instead of
/// returning the C string pointer.
pub fn strset(s: &mut [u8], replacement: u8) -> usize {
    let length = strnlen(s, s.len());
    s[..length].fill(replacement);
    length
}

/// Replaces at most `count` bytes before the first NUL and returns the count.
///
/// This slice-based equivalent of Wine's `_strnset` reports a count instead of
/// returning the C pointer advanced past the changed prefix.
pub fn strnset(s: &mut [u8], replacement: u8, count: usize) -> usize {
    let length = strnlen(s, count);
    s[..length].fill(replacement);
    length
}

/// Reverses the bytes before the first NUL and returns their count.
///
/// This slice-based equivalent of Wine's `_strrev` reports a count instead of
/// returning the C string pointer.
pub fn strrev(s: &mut [u8]) -> usize {
    let length = strnlen(s, s.len());
    s[..length].reverse();
    length
}

/// Converts ASCII lowercase bytes to uppercase before the first NUL.
pub fn strupr(s: &mut [u8]) -> usize {
    let length = strnlen(s, s.len());
    s[..length].make_ascii_uppercase();
    length
}

/// Converts ASCII uppercase bytes to lowercase before the first NUL.
pub fn strlwr(s: &mut [u8]) -> usize {
    let length = strnlen(s, s.len());
    s[..length].make_ascii_lowercase();
    length
}

/// Replaces all UTF-16 units before the first NUL and returns the count changed.
pub fn wcsset(s: &mut [u16], replacement: u16) -> usize {
    let length = wcslen(s);
    s[..length].fill(replacement);
    length
}

/// Replaces at most `count` UTF-16 units before the first NUL.
pub fn wcsnset(s: &mut [u16], replacement: u16, count: usize) -> usize {
    let length = wcsnlen(s, count);
    s[..length].fill(replacement);
    length
}

/// Reverses the UTF-16 code units before the first NUL and returns their count.
///
/// Like Wine's `_wcsrev`, this reverses code units, not Unicode scalar values.
pub fn wcsrev(s: &mut [u16]) -> usize {
    let length = wcslen(s);
    s[..length].reverse();
    length
}

/// Converts ASCII lowercase UTF-16 units to uppercase before the first NUL.
pub fn wcsupr(s: &mut [u16]) -> usize {
    let length = wcslen(s);
    for unit in &mut s[..length] {
        if (b'a' as u16..=b'z' as u16).contains(unit) {
            *unit -= 32;
        }
    }
    length
}

/// Converts ASCII uppercase UTF-16 units to lowercase before the first NUL.
pub fn wcslwr(s: &mut [u16]) -> usize {
    let length = wcslen(s);
    for unit in &mut s[..length] {
        *unit = to_lower(*unit);
    }
    length
}

/// Duplicates an optional NUL-terminated UTF-16 string.
///
/// `None` models a null pointer. For a bounded, unterminated slice, the result
/// is safely terminated rather than reading beyond the available input.
pub fn wcsdup(source: Option<&[u16]>) -> Option<Vec<u16>> {
    source.map(|source| {
        let length = wcslen(source);
        let mut duplicate = source[..length].to_vec();
        duplicate.push(0);
        duplicate
    })
}

/// Duplicates an optional NUL-terminated byte string.
///
/// `None` models a null pointer. A bounded, unterminated slice is copied in
/// full and explicitly terminated, avoiding an out-of-bounds pointer read.
pub fn strdup(source: Option<&[u8]>) -> Option<Vec<u8>> {
    source.map(|source| {
        let length = strnlen(source, source.len());
        let mut duplicate = source[..length].to_vec();
        duplicate.push(0);
        duplicate
    })
}

/// Length of a byte string before its first NUL, bounded by `max_len`.
///
/// This slice-based equivalent of Wine's `strnlen` cannot read past the
/// supplied slice, so an unterminated slice ends at its available length.
pub fn strnlen(s: &[u8], max_len: usize) -> usize {
    s.iter()
        .take(max_len)
        .position(|&byte| byte == 0)
        .unwrap_or(s.len().min(max_len))
}

/// Length of a UTF-16 string before its first NUL, bounded by `max_len`.
///
/// This slice-based equivalent of Wine's `wcsnlen` cannot read beyond the
/// supplied slice, so an unterminated slice ends at its available length.
pub fn wcsnlen(s: &[u16], max_len: usize) -> usize {
    s.iter()
        .take(max_len)
        .position(|&unit| unit == 0)
        .unwrap_or(s.len().min(max_len))
}

/// First code-unit offset containing `needle`, including the terminating NUL.
///
/// Returns `None` when the character is absent from the NUL-terminated string.
pub fn wcschr(s: &[u16], needle: u16) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    let end = wcslen(s);
    s[..=end.min(s.len().saturating_sub(1))]
        .iter()
        .position(|&unit| unit == needle)
}

/// Copies a byte string, including its terminator, into the destination.
///
/// Returns `None` without modifying the destination if it is too small. An
/// unterminated source slice is copied in full and explicitly terminated.
pub fn strcpy(destination: &mut [u8], source: &[u8]) -> Option<()> {
    let length = strnlen(source, source.len());
    let required = length + 1;
    let destination = destination.get_mut(..required)?;
    destination[..length].copy_from_slice(&source[..length]);
    destination[length] = 0;
    Some(())
}

/// Appends a byte string, including its terminator, to the destination.
///
/// Returns `None` without modifying the destination if it lacks a terminator or
/// capacity. An unterminated source slice is copied in full.
pub fn strcat(destination: &mut [u8], source: &[u8]) -> Option<()> {
    let end = destination.iter().position(|&byte| byte == 0)?;
    let source_len = strnlen(source, source.len());
    let terminator = end + source_len;
    let required = terminator + 1;
    let destination = destination.get_mut(end..required)?;
    destination[..source_len].copy_from_slice(&source[..source_len]);
    destination[source_len] = 0;
    Some(())
}

/// Appends at most `count` bytes and NUL-terminates the destination.
///
/// Returns `None` without modifying the destination when it is unterminated or
/// lacks capacity for the appended bytes and terminator.
pub fn strncat(destination: &mut [u8], source: &[u8], count: usize) -> Option<()> {
    let end = destination.iter().position(|&byte| byte == 0)?;
    let append_len = strnlen(source, count);
    let required = end + append_len + 1;
    let destination = destination.get_mut(end..required)?;
    destination[..append_len].copy_from_slice(&source[..append_len]);
    destination[append_len] = 0;
    Some(())
}

/// Copies at most `count` bytes and NUL-pads the remaining destination.
///
/// Returns `None` without modifying the destination if it has fewer than
/// `count` bytes. Like Wine's `strncpy`, exactly `count` bytes are written.
pub fn strncpy(destination: &mut [u8], source: &[u8], count: usize) -> Option<()> {
    let destination = destination.get_mut(..count)?;
    let mut copied = 0;
    while copied < count {
        let byte = source.get(copied).copied().unwrap_or(0);
        destination[copied] = byte;
        copied += 1;
        if byte == 0 {
            break;
        }
    }
    destination[copied..].fill(0);
    Some(())
}

/// Copies a UTF-16 string, including its terminator, into the destination.
///
/// Returns `None` without modifying the destination if it is too small. An
/// unterminated source slice is copied in full and explicitly terminated.
pub fn wcscpy(destination: &mut [u16], source: &[u16]) -> Option<()> {
    let length = wcslen(source);
    let required = length + 1;
    let destination = destination.get_mut(..required)?;
    destination[..length].copy_from_slice(&source[..length]);
    destination[length] = 0;
    Some(())
}

/// Appends a UTF-16 string, including its terminator, to the destination.
///
/// Returns `None` without modifying the destination when either slice lacks
/// the required terminator/capacity. An unterminated source is copied in full.
pub fn wcscat(destination: &mut [u16], source: &[u16]) -> Option<()> {
    let end = destination.iter().position(|&unit| unit == 0)?;
    let source_len = wcslen(source);
    let terminator = end + source_len;
    let required = terminator + 1;
    let destination = destination.get_mut(end..required)?;
    destination[..source_len].copy_from_slice(&source[..source_len]);
    destination[source_len] = 0;
    Some(())
}

/// Appends at most `count` UTF-16 units and NUL-terminates the destination.
///
/// Returns `None` without modifying the destination when it is unterminated or
/// lacks capacity for the appended text and terminator.
pub fn wcsncat(destination: &mut [u16], source: &[u16], count: usize) -> Option<()> {
    let end = destination.iter().position(|&unit| unit == 0)?;
    let append_len = wcsnlen(source, count);
    let required = end + append_len + 1;
    let destination = destination.get_mut(end..required)?;
    destination[..append_len].copy_from_slice(&source[..append_len]);
    destination[append_len] = 0;
    Some(())
}

/// Copies at most `count` UTF-16 units and NUL-pads the remaining destination.
///
/// Returns `None` without modifying the destination if it has fewer than
/// `count` code units. Like Wine's `wcsncpy`, exactly `count` units are written.
pub fn wcsncpy(destination: &mut [u16], source: &[u16], count: usize) -> Option<()> {
    let destination = destination.get_mut(..count)?;
    let mut copied = 0;
    while copied < count {
        let unit = source.get(copied).copied().unwrap_or(0);
        destination[copied] = unit;
        copied += 1;
        if unit == 0 {
            break;
        }
    }
    destination[copied..].fill(0);
    Some(())
}

/// Finds a UTF-16 substring and returns its first code-unit offset.
///
/// Matches Wine's `wcsstr`: an empty needle matches any non-empty haystack,
/// while an empty haystack returns `None`.
pub fn wcsstr(haystack: &[u16], needle: &[u16]) -> Option<usize> {
    let haystack_len = wcslen(haystack);
    let needle_len = wcslen(needle);
    (0..haystack_len).find(|&start| {
        start + needle_len <= haystack_len
            && haystack[start..start + needle_len] == needle[..needle_len]
    })
}

/// Counts the initial bytes that are not in `reject`.
///
/// Stops at the first NUL in `s`, matching Wine's `strcspn`.
pub fn strcspn(s: &[u8], reject: &[u8]) -> usize {
    let length = strnlen(s, s.len());
    let reject_len = strnlen(reject, reject.len());
    s[..length]
        .iter()
        .position(|byte| reject[..reject_len].contains(byte))
        .unwrap_or(length)
}

/// Counts the initial bytes that are in `accept`.
///
/// Stops at the first NUL in `s`, matching Wine's `strspn`.
pub fn strspn(s: &[u8], accept: &[u8]) -> usize {
    let length = strnlen(s, s.len());
    let accept_len = strnlen(accept, accept.len());
    s[..length]
        .iter()
        .position(|byte| !accept[..accept_len].contains(byte))
        .unwrap_or(length)
}

/// Compares exactly `count` bytes as unsigned binary data.
///
/// Returns `None` if either bounded slice is shorter than `count`; like Wine's
/// `memcmp`, the comparison itself does not stop at NUL and returns -1, 0, or 1.
pub fn memcmp(a: &[u8], b: &[u8], count: usize) -> Option<i32> {
    let a = a.get(..count)?;
    let b = b.get(..count)?;
    Some(
        a.iter()
            .zip(b)
            .find_map(|(&left, &right)| {
                (left != right).then_some(i32::from(left > right) - i32::from(left < right))
            })
            .unwrap_or(0),
    )
}

/// Finds `needle` in the first `count` bytes, including NUL as ordinary data.
///
/// `needle` is narrowed to one byte, matching Wine's `memchr` conversion.
pub fn memchr(s: &[u8], needle: i32, count: usize) -> Option<usize> {
    s.get(..count)?
        .iter()
        .position(|&byte| byte == needle as u8)
}

/// Finds the first occurrence of `needle`, including the NUL terminator.
///
/// `needle` is narrowed to one byte, matching Wine's C `char` comparison.
pub fn strchr(s: &[u8], needle: i32) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    let end = strnlen(s, s.len());
    s[..=end.min(s.len().saturating_sub(1))]
        .iter()
        .position(|&byte| byte == needle as u8)
}

/// Finds the last occurrence of `needle`, including the NUL terminator.
///
/// `needle` is narrowed to one byte, matching Wine's C `char` comparison.
pub fn strrchr(s: &[u8], needle: i32) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    let end = strnlen(s, s.len());
    s[..=end.min(s.len().saturating_sub(1))]
        .iter()
        .rposition(|&byte| byte == needle as u8)
}

/// Finds a byte substring and returns its first offset.
///
/// Like Wine's `strstr`, an empty needle matches at offset zero, even for an
/// empty haystack.
pub fn strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let haystack_len = strnlen(haystack, haystack.len());
    let needle_len = strnlen(needle, needle.len());
    if needle_len == 0 {
        return Some(0);
    }
    (0..haystack_len).find(|&start| {
        start + needle_len <= haystack_len
            && haystack[start..start + needle_len] == needle[..needle_len]
    })
}

/// Finds the first byte in `s` that is also in `accept`.
///
/// Returns the byte offset, analogous to Wine's pointer-returning `strpbrk`.
pub fn strpbrk(s: &[u8], accept: &[u8]) -> Option<usize> {
    let length = strnlen(s, s.len());
    let accept_len = strnlen(accept, accept.len());
    s[..length]
        .iter()
        .position(|byte| accept[..accept_len].contains(byte))
}

/// Counts the initial UTF-16 units that are not in `reject`.
///
/// Stops at the first NUL in `s`, matching Wine's `wcscspn`.
pub fn wcscspn(s: &[u16], reject: &[u16]) -> usize {
    let length = wcslen(s);
    let reject_len = wcslen(reject);
    s[..length]
        .iter()
        .position(|unit| reject[..reject_len].contains(unit))
        .unwrap_or(length)
}

/// Counts the initial UTF-16 units that are in `accept`.
///
/// Stops at the first NUL in `s`, matching Wine's `wcsspn`.
pub fn wcsspn(s: &[u16], accept: &[u16]) -> usize {
    let length = wcslen(s);
    let accept_len = wcslen(accept);
    s[..length]
        .iter()
        .position(|unit| !accept[..accept_len].contains(unit))
        .unwrap_or(length)
}

/// Last code-unit offset containing `needle`, including the terminating NUL.
///
/// Returns `None` when the character is absent from the NUL-terminated string.
pub fn wcsrchr(s: &[u16], needle: u16) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    let end = wcslen(s);
    s[..=end.min(s.len().saturating_sub(1))]
        .iter()
        .rposition(|&unit| unit == needle)
}

/// Length of the string before its first NUL, or the slice length.
pub fn wcslen(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

fn at(s: &[u16], i: usize) -> u16 {
    s.get(i).copied().unwrap_or(0)
}

fn byte_at(s: &[u8], i: usize) -> u8 {
    s.get(i).copied().unwrap_or(0)
}

fn lower_byte(byte: u8) -> u8 {
    if byte.is_ascii_uppercase() {
        byte + (b'a' - b'A')
    } else {
        byte
    }
}

fn compare_bytes(a: &[u8], b: &[u8], count: usize, fold: fn(u8) -> u8) -> i32 {
    for i in 0..count {
        let left = fold(byte_at(a, i));
        let right = fold(byte_at(b, i));
        if left != right || left == 0 {
            return i32::from(left) - i32::from(right);
        }
    }
    0
}

/// Compares NUL-terminated byte strings, returning -1, 0, or 1 like Wine's
/// Win64/UCRT `strcmp` implementation.
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    compare_bytes(a, b, usize::MAX, |byte| byte).signum()
}

/// Compares at most `count` bytes, returning -1, 0, or 1 like Wine's
/// Win64/UCRT `strncmp` implementation.
pub fn strncmp(a: &[u8], b: &[u8], count: usize) -> i32 {
    compare_bytes(a, b, count, |byte| byte).signum()
}

/// Case-insensitive byte-string comparison using Wine's C-locale ASCII fold.
pub fn stricmp(a: &[u8], b: &[u8]) -> i32 {
    compare_bytes(a, b, usize::MAX, lower_byte)
}

/// Case-insensitive comparison of at most `count` bytes.
pub fn strnicmp(a: &[u8], b: &[u8], count: usize) -> i32 {
    compare_bytes(a, b, count, lower_byte)
}

fn to_lower(c: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&c) {
        c + 32
    } else {
        c
    }
}

/// `wcscmp`: compares two NUL-terminated strings.
pub fn wcscmp(a: &[u16], b: &[u16]) -> i32 {
    wcsncmp(a, b, usize::MAX)
}

/// `wcsncmp`: compares at most `n` code units.
pub fn wcsncmp(a: &[u16], b: &[u16], n: usize) -> i32 {
    compare(a, b, n, |c| c)
}

/// `_wcsicmp`: case-insensitive `wcscmp`.
pub fn wcsicmp(a: &[u16], b: &[u16]) -> i32 {
    wcsnicmp(a, b, usize::MAX)
}

/// `_wcsnicmp`: case-insensitive `wcsncmp`.
pub fn wcsnicmp(a: &[u16], b: &[u16], n: usize) -> i32 {
    compare(a, b, n, to_lower)
}

fn compare(a: &[u16], b: &[u16], n: usize, fold: fn(u16) -> u16) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut i = 0;
    let (mut c1, mut c2);
    loop {
        c1 = fold(at(a, i));
        c2 = fold(at(b, i));
        i += 1;
        if i == n || c1 == 0 || c1 != c2 {
            break;
        }
    }
    i32::from(c1) - i32::from(c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn binary_memory_search_and_comparison_are_count_bounded() {
        assert_eq!(memcmp(b"a\0b", b"a\0b", 3), Some(0));
        assert_eq!(memcmp(b"a\0b", b"a\0c", 3), Some(-1));
        assert_eq!(memcmp(b"a\0c", b"a\0b", 3), Some(1));
        assert_eq!(memcmp(&[0x80], &[0x7f], 1), Some(1));
        assert_eq!(memcmp(b"x", b"y", 0), Some(0));
        assert_eq!(memcmp(b"x", b"y", 2), None);
        assert_eq!(memcmp(b"xy", b"x", 2), None);
        assert_eq!(memcmp(b"xy", b"xz", 2), Some(-1));

        assert_eq!(memchr(b"a\0b", 0, 3), Some(1));
        assert_eq!(memchr(&[0x80, 0xff], 0x1ff, 2), Some(1));
        assert_eq!(memchr(b"abc", b'b' as i32, 1), None);
        assert_eq!(memchr(b"abc", b'a' as i32, 0), None);
        assert_eq!(memchr(b"a", b'a' as i32, 2), None);
        assert_eq!(memchr(b"", 0, 0), None);
    }

    #[test]
    fn byte_character_and_substring_searches_match_wine_boundaries() {
        let text = b"banana\0ignored";
        assert_eq!(strchr(text, b'a' as i32), Some(1));
        assert_eq!(strrchr(text, b'a' as i32), Some(5));
        assert_eq!(strchr(text, b'z' as i32), None);
        assert_eq!(strrchr(text, b'z' as i32), None);
        assert_eq!(strchr(text, 0), Some(6));
        assert_eq!(strrchr(text, 0), Some(6));
        assert_eq!(strchr(text, 0x161), Some(1));
        assert_eq!(strchr(b"", 0), None);
        assert_eq!(strrchr(b"", 0), None);
        assert_eq!(strchr(&[0], 0), Some(0));
        assert_eq!(strrchr(&[0], 0), Some(0));
        assert_eq!(strchr(b"x", b'x' as i32), Some(0));
        assert_eq!(strchr(b"x", 0), None);

        assert_eq!(strstr(b"the quick brown fox", b"quick"), Some(4));
        assert_eq!(strstr(b"the quick brown fox", b"fox"), Some(16));
        assert_eq!(strstr(b"the quick", b"missing"), None);
        assert_eq!(strstr(b"text", b""), Some(0));
        assert_eq!(strstr(b"", b""), Some(0));
        assert_eq!(strstr(b"", b"x"), None);
        assert_eq!(strstr(b"abababc", b"ababc"), Some(2));
        assert_eq!(strstr(b"ab", b"b"), Some(1));
    }

    #[test]
    fn byte_comparisons_match_wine_ordering_and_c_locale_folding() {
        assert_eq!(strcmp(b"same\0tail", b"same"), 0);
        assert_eq!(strcmp(b"a", b"z"), -1);
        assert_eq!(strcmp(b"z", b"a"), 1);
        assert_eq!(strcmp(&[0x80, 0], &[0x7f, 0]), 1);
        assert_eq!(strcmp(b"a", b"ab"), -1);
        assert_eq!(strcmp(b"ab", b"a"), 1);

        assert_eq!(strncmp(b"abc", b"abd", 2), 0);
        assert_eq!(strncmp(b"abc", b"abd", 3), -1);
        assert_eq!(strncmp(b"abd", b"abc", 3), 1);
        assert_eq!(strncmp(b"a", b"b", 0), 0);
        assert_eq!(strncmp(&[0xff], &[0], 1), 1);

        assert_eq!(stricmp(b"HeLLo", b"hello"), 0);
        assert_eq!(stricmp(b"A", b"b"), i32::from(b'a') - i32::from(b'b'));
        assert_eq!(strnicmp(b"ABcd", b"abXY", 2), 0);
        assert_eq!(
            strnicmp(b"ABcd", b"abXY", 3),
            i32::from(b'c') - i32::from(b'x')
        );
        assert_eq!(strnicmp(b"A", b"a", 0), 0);
        assert_eq!(stricmp(&[0xc0, 0], &[0xe0, 0]), -32);
    }

    #[test]
    fn ascii_case_conversion_mutates_only_string_prefix() {
        let mut lower = *b"HeLLo\0tail";
        assert_eq!(strlwr(&mut lower), 5);
        assert_eq!(&lower, b"hello\0tail");
        let mut upper = *b"HeLLo\0tail";
        assert_eq!(strupr(&mut upper), 5);
        assert_eq!(&upper, b"HELLO\0tail");
        let mut non_ascii = [0xc0, b'A', 0];
        assert_eq!(strlwr(&mut non_ascii), 2);
        assert_eq!(non_ascii, [0xc0, b'a', 0]);
        let mut empty = [0, b'x'];
        assert_eq!(strupr(&mut empty), 0);
        assert_eq!(empty, [0, b'x']);
        let mut unterminated = *b"abc";
        assert_eq!(strlwr(&mut unterminated), 3);
        assert_eq!(unterminated, *b"abc");

        let mut wide_lower = w("HeLLo");
        assert_eq!(wcslwr(&mut wide_lower), 5);
        assert_eq!(wide_lower, w("hello"));
        let mut wide_upper = w("HeLLo");
        assert_eq!(wcsupr(&mut wide_upper), 5);
        assert_eq!(wide_upper, w("HELLO"));
        let mut wide_non_ascii = [0x00c9, b'A' as u16, 0];
        assert_eq!(wcslwr(&mut wide_non_ascii), 2);
        assert_eq!(wide_non_ascii, [0x00c9, b'a' as u16, 0]);
        let mut wide_unterminated = [b'a' as u16, b'B' as u16];
        assert_eq!(wcsupr(&mut wide_unterminated), 2);
        assert_eq!(wide_unterminated, [b'A' as u16, b'B' as u16]);
    }

    #[test]
    fn byte_set_and_reverse_mutate_only_the_nul_terminated_prefix() {
        let mut value = *b"wine\0tail";
        assert_eq!(strset(&mut value, b'x'), 4);
        assert_eq!(&value, b"xxxx\0tail");

        let mut bounded = *b"wine\0";
        assert_eq!(strnset(&mut bounded, b'y', 2), 2);
        assert_eq!(&bounded, b"yyne\0");
        assert_eq!(strnset(&mut bounded, b'z', 8), 4);
        assert_eq!(&bounded, b"zzzz\0");

        let mut empty = [0, b'a'];
        assert_eq!(strset(&mut empty, b'q'), 0);
        assert_eq!(strnset(&mut empty, b'q', 2), 0);
        assert_eq!(empty, [0, b'a']);
        let mut no_nul = *b"abc";
        assert_eq!(strset(&mut no_nul, b'd'), 3);
        assert_eq!(no_nul, *b"ddd");
        assert_eq!(strnset(&mut no_nul, b'e', 0), 0);

        let mut reversed = *b"wine\0tail";
        assert_eq!(strrev(&mut reversed), 4);
        assert_eq!(&reversed, b"eniw\0tail");
        let mut single = *b"x\0";
        assert_eq!(strrev(&mut single), 1);
        assert_eq!(&single, b"x\0");
        let mut empty = [0, b'x'];
        assert_eq!(strrev(&mut empty), 0);
        assert_eq!(empty, [0, b'x']);
        let mut unterminated = *b"abc";
        assert_eq!(strrev(&mut unterminated), 3);
        assert_eq!(unterminated, *b"cba");
    }

    #[test]
    fn wcsset_and_wcsnset_replace_only_bounded_non_nul_units() {
        let mut value = w("wine");
        assert_eq!(wcsset(&mut value, b'x' as u16), 4);
        assert_eq!(
            value,
            [b'x' as u16, b'x' as u16, b'x' as u16, b'x' as u16, 0]
        );

        let mut bounded = w("wine");
        assert_eq!(wcsnset(&mut bounded, b'y' as u16, 2), 2);
        assert_eq!(
            bounded,
            [b'y' as u16, b'y' as u16, b'n' as u16, b'e' as u16, 0]
        );
        assert_eq!(wcsnset(&mut bounded, b'z' as u16, 8), 4);
        assert_eq!(
            bounded,
            [b'z' as u16, b'z' as u16, b'z' as u16, b'z' as u16, 0]
        );

        let mut empty = [0, b'a' as u16];
        assert_eq!(wcsset(&mut empty, b'q' as u16), 0);
        assert_eq!(wcsnset(&mut empty, b'q' as u16, 2), 0);
        assert_eq!(empty, [0, b'a' as u16]);
        let mut no_nul = [b'a' as u16, b'b' as u16];
        assert_eq!(wcsset(&mut no_nul, b'c' as u16), 2);
        assert_eq!(no_nul, [b'c' as u16, b'c' as u16]);
        assert_eq!(wcsnset(&mut no_nul, b'd' as u16, 0), 0);
    }

    #[test]
    fn wcsrev_reverses_only_the_string_prefix() {
        let mut value = w("wine");
        assert_eq!(wcsrev(&mut value), 4);
        assert_eq!(value, w("eniw"));

        let mut empty = [0, b'x' as u16];
        assert_eq!(wcsrev(&mut empty), 0);
        assert_eq!(empty, [0, b'x' as u16]);

        let mut unterminated = [b'a' as u16, b'b' as u16, b'c' as u16];
        assert_eq!(wcsrev(&mut unterminated), 3);
        assert_eq!(unterminated, [b'c' as u16, b'b' as u16, b'a' as u16]);
    }

    #[test]
    fn wcsdup_copies_through_the_terminator_and_handles_null() {
        assert_eq!(wcsdup(None), None);
        assert_eq!(wcsdup(Some(&w("wine"))), Some(w("wine")));
        assert_eq!(wcsdup(Some(&[b'a' as u16, b'b' as u16])), Some(w("ab")));
        assert_eq!(wcsdup(Some(&[])), Some(vec![0]));
    }

    #[test]
    fn strdup_copies_through_terminator_and_handles_null() {
        assert_eq!(strdup(None), None);
        assert_eq!(strdup(Some(b"wine\0ignored")), Some(b"wine\0".to_vec()));
        assert_eq!(strdup(Some(b"ab")), Some(b"ab\0".to_vec()));
        assert_eq!(strdup(Some(b"")), Some(vec![0]));
    }

    #[test]
    fn strnlen_stops_at_nul_or_limit() {
        assert_eq!(strnlen(b"string\0ignored", 20), 6);
        assert_eq!(strnlen(b"string\0", 3), 3);
        assert_eq!(strnlen(b"string", 20), 6);
        assert_eq!(strnlen(b"string\0", 0), 0);
        assert_eq!(strnlen(b"", 4), 0);
    }

    #[test]
    fn wcsnlen_stops_at_nul_or_limit() {
        assert_eq!(wcsnlen(&w("string"), 20), 6);
        assert_eq!(wcsnlen(&w("string"), 3), 3);
        assert_eq!(wcsnlen(&[b'a' as u16, b'b' as u16], 20), 2);
        assert_eq!(wcsnlen(&w("string"), 0), 0);
        assert_eq!(wcsnlen(&[], 4), 0);
    }

    #[test]
    fn byte_copy_and_concat_match_bounded_c_string_semantics() {
        let mut copied = [0xff; 6];
        assert_eq!(strcpy(&mut copied, b"wine\0ignored"), Some(()));
        assert_eq!(&copied, &[b'w', b'i', b'n', b'e', 0, 0xff]);
        let mut exact = [0xff; 2];
        assert_eq!(strcpy(&mut exact, b"x"), Some(()));
        assert_eq!(&exact, b"x\0");
        let mut too_small = [0xff; 1];
        assert_eq!(strcpy(&mut too_small, b"x"), None);
        assert_eq!(too_small, [0xff]);
        assert_eq!(strcpy(&mut [], b""), None);

        let mut destination = [0xff; 8];
        destination[..3].copy_from_slice(b"ab\0");
        assert_eq!(strcat(&mut destination, b"cd\0ignored"), Some(()));
        assert_eq!(&destination[..5], b"abcd\0");
        assert_eq!(strcat(&mut destination, b""), Some(()));
        assert_eq!(&destination[..5], b"abcd\0");
        assert_eq!(strcat(&mut [], b"x"), None);
        let mut unterminated = *b"ab";
        assert_eq!(strcat(&mut unterminated, b"c"), None);
        assert_eq!(unterminated, *b"ab");
        let mut no_space = *b"a\0";
        assert_eq!(strcat(&mut no_space, b"b"), None);
        assert_eq!(no_space, *b"a\0");

        let mut appended = [0xff; 8];
        appended[..3].copy_from_slice(b"ab\0");
        assert_eq!(strncat(&mut appended, b"cdef", 2), Some(()));
        assert_eq!(&appended[..5], b"abcd\0");
        assert_eq!(strncat(&mut appended, b"ef", 8), Some(()));
        assert_eq!(&appended[..7], b"abcdef\0");
        let mut exact_append = *b"x\0";
        assert_eq!(strncat(&mut exact_append, b"", 0), Some(()));
        assert_eq!(exact_append, *b"x\0");
        assert_eq!(strncat(&mut [], b"x", 1), None);
        let mut unterminated_destination = *b"x";
        assert_eq!(strncat(&mut unterminated_destination, b"y", 1), None);
        let mut too_small = *b"x\0";
        assert_eq!(strncat(&mut too_small, b"y", 1), None);
        assert_eq!(too_small, *b"x\0");

        let mut buffer = [0xff; 6];
        assert_eq!(strncpy(&mut buffer, b"1234567", 6), Some(()));
        assert_eq!(&buffer, b"123456");
        let mut padded = [0xff; 6];
        assert_eq!(strncpy(&mut padded, b"abc", 6), Some(()));
        assert_eq!(&padded, b"abc\0\0\0");
        let mut unchanged = [0xff; 2];
        assert_eq!(strncpy(&mut unchanged, b"x", 3), None);
        assert_eq!(unchanged, [0xff; 2]);
        assert_eq!(strncpy(&mut unchanged, b"x", 0), Some(()));
        assert_eq!(strncpy(&mut unchanged, b"x", 2), Some(()));
        assert_eq!(&unchanged, b"x\0");
        assert_eq!(strncpy(&mut [0xff; 3], b"x\0y", 3), Some(()));
    }

    #[test]
    fn wcscpy_copies_strings_and_rejects_small_buffers() {
        let mut destination = [0xffff; 6];
        assert_eq!(wcscpy(&mut destination, &w("wine")), Some(()));
        assert_eq!(&destination[..5], &w("wine"));
        assert_eq!(&destination[5..], &[0xffff]);

        let mut exact = [0xffff; 2];
        assert_eq!(wcscpy(&mut exact, &[b'x' as u16]), Some(()));
        assert_eq!(exact, [b'x' as u16, 0]);
        let mut too_small = [0xffff; 1];
        assert_eq!(wcscpy(&mut too_small, &w("x")), None);
        assert_eq!(too_small, [0xffff]);
        assert_eq!(wcscpy(&mut [], &[]), None);
    }

    #[test]
    fn wcscat_appends_strings_and_rejects_invalid_destination() {
        let mut destination = [0xffff; 8];
        destination[..3].copy_from_slice(&w("ab"));
        assert_eq!(wcscat(&mut destination, &w("cd")), Some(()));
        assert_eq!(&destination[..5], &w("abcd"));

        assert_eq!(wcscat(&mut destination, &[]), Some(()));
        assert_eq!(&destination[..5], &w("abcd"));
        assert_eq!(wcscat(&mut [], &w("x")), None);
        let mut unterminated = [b'a' as u16, b'b' as u16];
        assert_eq!(wcscat(&mut unterminated, &w("c")), None);
        assert_eq!(unterminated, [b'a' as u16, b'b' as u16]);
        let mut too_small = [b'a' as u16, 0];
        assert_eq!(wcscat(&mut too_small, &w("b")), None);
        assert_eq!(too_small, [b'a' as u16, 0]);
    }

    #[test]
    fn wcsncat_appends_with_a_terminator_and_checks_capacity() {
        let mut destination = [0xffff; 12];
        destination[..5].copy_from_slice(&w("base"));
        assert_eq!(wcsncat(&mut destination, &w("tail"), 2), Some(()));
        assert_eq!(&destination[..7], &w("baseta"));

        assert_eq!(wcsncat(&mut destination, &w("il"), 8), Some(()));
        assert_eq!(&destination[..9], &w("basetail"));

        let mut exact = w("x");
        assert_eq!(wcsncat(&mut exact, &w(""), 0), Some(()));
        assert_eq!(exact, w("x"));
        assert_eq!(wcsncat(&mut [], &w("x"), 1), None);
        assert_eq!(wcsncat(&mut [b'x' as u16], &w("y"), 1), None);
        let mut too_small = [b'x' as u16, 0];
        assert_eq!(wcsncat(&mut too_small, &w("y"), 1), None);
        assert_eq!(too_small, [b'x' as u16, 0]);
    }

    #[test]
    fn wcsncpy_copies_truncates_pads_and_checks_capacity() {
        let mut buffer = [0xffff; 6];
        assert_eq!(wcsncpy(&mut buffer, &w("1234567"), 6), Some(()));
        assert_eq!(
            buffer,
            [
                b'1' as u16,
                b'2' as u16,
                b'3' as u16,
                b'4' as u16,
                b'5' as u16,
                b'6' as u16
            ]
        );

        let mut padded = [0xffff; 6];
        assert_eq!(wcsncpy(&mut padded, &w("abc"), 6), Some(()));
        assert_eq!(padded, [b'a' as u16, b'b' as u16, b'c' as u16, 0, 0, 0]);

        let mut unchanged = [0xffff; 2];
        assert_eq!(wcsncpy(&mut unchanged, &w("x"), 3), None);
        assert_eq!(unchanged, [0xffff; 2]);
        assert_eq!(wcsncpy(&mut unchanged, &w("x"), 0), Some(()));
        assert_eq!(wcsncpy(&mut unchanged, &[b'x' as u16], 2), Some(()));
        assert_eq!(unchanged, [b'x' as u16, 0]);
    }

    #[test]
    fn substring_search_matches_wine_edges_and_offsets() {
        let text = w("the quick brown fox");
        assert_eq!(wcsstr(&text, &w("quick")), Some(4));
        assert_eq!(wcsstr(&text, &w("fox")), Some(16));
        assert_eq!(wcsstr(&text, &w("missing")), None);
        assert_eq!(wcsstr(&text, &w("")), Some(0));
        assert_eq!(wcsstr(&w(""), &w("")), None);
        assert_eq!(wcsstr(&w(""), &w("x")), None);
        assert_eq!(wcsstr(&[b'a' as u16, b'b' as u16], &[b'b' as u16]), Some(1));
    }

    #[test]
    fn character_searches_find_first_last_and_terminator() {
        let text = w("banana");
        assert_eq!(wcschr(&text, b'a' as u16), Some(1));
        assert_eq!(wcsrchr(&text, b'a' as u16), Some(5));
        assert_eq!(wcschr(&text, b'z' as u16), None);
        assert_eq!(wcsrchr(&text, b'z' as u16), None);
        assert_eq!(wcschr(&text, 0), Some(6));
        assert_eq!(wcsrchr(&text, 0), Some(6));
        assert_eq!(wcschr(&[], 0), None);
        assert_eq!(wcsrchr(&[], 0), None);
        assert_eq!(wcschr(&[0], 0), Some(0));
        assert_eq!(wcsrchr(&[0], 0), Some(0));
        assert_eq!(wcschr(&[b'x' as u16], b'x' as u16), Some(0));
        assert_eq!(wcschr(&[b'x' as u16], 0), None);
    }

    #[test]
    fn byte_spans_and_search_use_unsigned_byte_membership() {
        let text = b"abcde\0ignored";
        assert_eq!(strcspn(text, b"xyz"), 5);
        assert_eq!(strcspn(text, b"xd"), 3);
        assert_eq!(strcspn(text, b""), 5);
        assert_eq!(strcspn(b"", b"a"), 0);
        assert_eq!(strcspn(&[0x80, 0xff, 0], &[0xff, 0]), 1);
        assert_eq!(strcspn(b"abc", b"b\0c"), 1);

        assert_eq!(strspn(text, b"abc"), 3);
        assert_eq!(strspn(text, b"abcde"), 5);
        assert_eq!(strspn(text, b""), 0);
        assert_eq!(strspn(b"", b"abc"), 0);
        assert_eq!(strspn(&[0x80, 0xff, 0], &[0x80, 0xff, 0]), 2);
        assert_eq!(strspn(b"abc", b"a\0bc"), 1);

        assert_eq!(strpbrk(text, b"xd"), Some(3));
        assert_eq!(strpbrk(text, b""), None);
        assert_eq!(strpbrk(b"", b"a"), None);
        assert_eq!(strpbrk(b"abc", b"z"), None);
        assert_eq!(strpbrk(&[0x80, 0xff, 0], &[0xff]), Some(1));
        assert_eq!(strpbrk(b"ab", b"b"), Some(1));
    }

    #[test]
    fn spans_count_prefixes_using_wine_string_boundaries() {
        let text = w("abcde");
        assert_eq!(wcscspn(&text, &w("xyz")), 5);
        assert_eq!(wcscspn(&text, &w("xd")), 3);
        assert_eq!(wcscspn(&text, &w("")), 5);
        assert_eq!(wcscspn(&w(""), &w("a")), 0);
        assert_eq!(wcscspn(&[b'a' as u16, b'b' as u16], &[b'b' as u16]), 1);

        assert_eq!(wcsspn(&text, &w("abc")), 3);
        assert_eq!(wcsspn(&text, &w("abcde")), 5);
        assert_eq!(wcsspn(&text, &w("")), 0);
        assert_eq!(wcsspn(&w(""), &w("abc")), 0);
        assert_eq!(wcsspn(&[b'a' as u16, b'b' as u16], &[b'a' as u16]), 1);
    }

    #[test]
    fn length_stops_at_nul() {
        assert_eq!(wcslen(&w("hello")), 5);
        assert_eq!(wcslen(&[0]), 0);
        assert_eq!(wcslen(&[b'a' as u16, b'b' as u16]), 2);
    }

    #[test]
    fn cmp_orders_and_equal() {
        assert_eq!(wcscmp(&w("a"), &w("z")), -(b'z' as i32 - b'a' as i32));
        assert_eq!(wcscmp(&w("z"), &w("a")), b'z' as i32 - b'a' as i32);
        assert_eq!(wcscmp(&w("f"), &w("f")), 0);
    }

    #[test]
    fn cmp_prefix_is_shorter_first() {
        assert!(wcscmp(&w("ab"), &w("abc")) < 0);
        assert!(wcscmp(&w("abc"), &w("ab")) > 0);
    }

    #[test]
    fn ncmp_limits_compared_units() {
        assert_eq!(wcsncmp(&w("abc"), &w("abd"), 2), 0);
        assert!(wcsncmp(&w("abc"), &w("abd"), 3) < 0);
        assert_eq!(wcsncmp(&w("abc"), &w("xyz"), 0), 0);
    }

    #[test]
    fn icmp_folds_ascii_case() {
        assert_eq!(wcsicmp(&w("HeLLo"), &w("hello")), 0);
        assert!(wcsicmp(&w("A"), &w("b")) < 0);
        assert!(wcsicmp(&w("b"), &w("A")) > 0);
    }

    #[test]
    fn nicmp_folds_and_limits() {
        assert_eq!(wcsnicmp(&w("ABcd"), &w("abXY"), 2), 0);
        assert!(wcsnicmp(&w("ABcd"), &w("abXY"), 3) < 0);
        assert_eq!(wcsnicmp(&w("A"), &w("a"), 0), 0);
    }

    #[test]
    fn fold_leaves_non_ascii_alone() {
        assert_eq!(to_lower(b'@' as u16), b'@' as u16);
        assert_eq!(to_lower(b'[' as u16), b'[' as u16);
        assert_eq!(to_lower(b'Z' as u16), b'z' as u16);
        assert_eq!(to_lower(0x00C9), 0x00C9);
    }

    #[test]
    fn short_slice_without_nul_is_treated_as_nul_terminated() {
        assert_eq!(wcscmp(&[b'a' as u16], &[b'a' as u16, 0]), 0);
    }
}
