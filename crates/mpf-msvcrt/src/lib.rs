//! msvcrt wide-string subset: the comparison and length routines from
//! Wine's `dlls/msvcrt/wcs.c`.
//!
//! Strings are NUL-terminated UTF-16 slices. Comparison returns the first
//! code-unit difference, as Wine's `c1 - c2` does. The case-insensitive
//! variants lower-case with ASCII rules only; locale-aware `_towlower_l`
//! and the `MSVCRT_CHECK_PMT` invalid-parameter path are not ported here.

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
