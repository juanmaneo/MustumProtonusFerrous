//! kernel32 surface (initial subset).
//!
//! Mirrors `dlls/kernel32/string.c` (`lstrlenW`, `lstrcpynW`) in the
//! reference Wine tree. Safe slice-based cores plus thin `unsafe` FFI
//! wrappers with the real Win32 signatures.

/// Mirrors `lstrlenW` on a slice: counts UTF-16 code units up to (not
/// including) the first NUL.
pub fn lstrlen_w(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

/// # Safety
/// `p` must point to a valid, readable, NUL-terminated UTF-16 string.
#[allow(non_snake_case)] // keeps the Win32 API name
pub unsafe fn lstrlenW(p: *const u16) -> i32 {
    let mut n = 0i32;
    unsafe {
        while *p.add(n as usize) != 0 {
            n += 1;
        }
    }
    n
}

/// Mirrors `lstrcpynW`: copies at most `dst.len() - 1` code units plus a
/// terminating NUL. Returns the length of `src` (up to its NUL).
pub fn lstrcpyn_w(dst: &mut [u16], src: &[u16]) -> usize {
    let src_len = lstrlen_w(src);
    let n = src_len.min(dst.len().saturating_sub(1));
    dst[..n].copy_from_slice(&src[..n]);
    if !dst.is_empty() {
        dst[n] = 0;
    }
    src_len
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        let mut v: Vec<u16> = s.encode_utf16().collect();
        v.push(0);
        v
    }

    #[test]
    fn lstrlen_counts_to_nul() {
        assert_eq!(lstrlen_w(&w("hello")), 5);
        assert_eq!(lstrlen_w(&w("")), 0);
        assert_eq!(lstrlen_w(&[b'a' as u16, b'b' as u16]), 2); // no NUL
    }

    #[test]
    fn lstrlen_w_raw() {
        let v = w("kernel32");
        assert_eq!(unsafe { lstrlenW(v.as_ptr()) }, 8);
    }

    #[test]
    fn lstrcpyn_truncates() {
        let src = w("abcdef");
        let mut dst = [0u16; 4];
        let n = lstrcpyn_w(&mut dst, &src);
        assert_eq!(n, 6); // full source length
        assert_eq!(&dst, &[b'a' as u16, b'b' as u16, b'c' as u16, 0]);
    }

    #[test]
    fn lstrcpyn_handles_empty_and_single_slot_destinations() {
        assert_eq!(lstrcpyn_w(&mut [], &w("abc")), 3);
        let mut dst = [9u16; 1];
        assert_eq!(lstrcpyn_w(&mut dst, &w("abc")), 3);
        assert_eq!(dst, [0]);
    }
}
