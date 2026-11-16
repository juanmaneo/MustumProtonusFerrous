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
