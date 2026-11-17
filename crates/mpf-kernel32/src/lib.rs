//! kernel32 surface: strings, process, thread, module, file.
//!
//! Mirrors `dlls/kernel32/*.c` in the reference Wine tree. Safe slice-based
//! cores plus thin `unsafe` FFI wrappers with the real Win32 signatures.

pub mod file;
pub mod module;
pub mod process;
pub mod thread;

/// `MulDiv`: `multiplicand * multiplier / divisor`, rounded half away from
/// zero. Returns `-1` for a zero divisor or a result outside `-i32::MAX..=i32::MAX`.
/// A negative divisor negates the multiplicand, which wraps for `i32::MIN`, as
/// in Wine.
pub fn mul_div(multiplicand: i32, multiplier: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        return -1;
    }
    let (multiplicand, divisor) = if divisor < 0 {
        (multiplicand.wrapping_neg(), divisor.wrapping_neg())
    } else {
        (multiplicand, divisor)
    };
    let product = i64::from(multiplicand) * i64::from(multiplier);
    let half = i64::from(divisor / 2);
    let rounded = if (multiplicand < 0) == (multiplier < 0) {
        product + half
    } else {
        product - half
    };
    let result = rounded / i64::from(divisor);
    if (-i64::from(i32::MAX)..=i64::from(i32::MAX)).contains(&result) {
        result as i32
    } else {
        -1
    }
}

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

/// Mirrors `lstrcpyW` (`wcscpy`): copies `src` up to its NUL, terminator
/// included. Returns the copied length, or `None` when `dst` is too small;
/// Win32 would overrun the buffer there, so the port leaves `dst` untouched.
pub fn lstrcpy_w(dst: &mut [u16], src: &[u16]) -> Option<usize> {
    let len = lstrlen_w(src);
    if len >= dst.len() {
        return None;
    }
    dst[..len].copy_from_slice(&src[..len]);
    dst[len] = 0;
    Some(len)
}

/// Mirrors `lstrcatW` (`wcscat`): appends `src` up to its NUL to the string
/// in `dst`. Returns the new length, or `None` when it does not fit with its
/// terminator; `dst` is left untouched in that case.
pub fn lstrcat_w(dst: &mut [u16], src: &[u16]) -> Option<usize> {
    let start = lstrlen_w(dst);
    let len = lstrlen_w(src);
    let end = start + len;
    if end >= dst.len() {
        return None;
    }
    dst[start..end].copy_from_slice(&src[..len]);
    dst[end] = 0;
    Some(end)
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
    fn mul_div_rounds_half_away_from_zero() {
        assert_eq!(mul_div(3, 4, 2), 6);
        assert_eq!(mul_div(1, 1, 2), 1);
        assert_eq!(mul_div(7, 1, 2), 4);
        assert_eq!(mul_div(-1, 1, 2), -1);
        assert_eq!(mul_div(-3, -4, 2), 6);
        assert_eq!(mul_div(100, 3, 4), 75);
        assert_eq!(mul_div(5, 1, -2), -3);
    }

    #[test]
    fn mul_div_reports_zero_divisor_and_out_of_range_as_minus_one() {
        assert_eq!(mul_div(1, 1, 0), -1);
        assert_eq!(mul_div(i32::MAX, 2, 1), -1);
        assert_eq!(mul_div(i32::MIN, 1, 1), -1);
        assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
        assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
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

    #[test]
    fn lstrcpy_copies_with_terminator() {
        let mut dst = [9u16; 8];
        assert_eq!(lstrcpy_w(&mut dst, &w("abc")), Some(3));
        assert_eq!(dst, [b'a' as u16, b'b' as u16, b'c' as u16, 0, 9, 9, 9, 9]);
        assert_eq!(lstrcpy_w(&mut dst, &w("")), Some(0));
        assert_eq!(dst[0], 0);
    }

    #[test]
    fn lstrcpy_rejects_destination_without_room_for_nul() {
        let mut dst = [9u16; 3];
        assert_eq!(lstrcpy_w(&mut dst, &w("abc")), None);
        assert_eq!(dst, [9, 9, 9]);
        assert_eq!(lstrcpy_w(&mut [], &w("")), None);
    }

    #[test]
    fn lstrcpy_stops_at_source_nul() {
        let mut dst = [9u16; 8];
        assert_eq!(lstrcpy_w(&mut dst, &[b'x' as u16, 0, b'y' as u16]), Some(1));
        assert_eq!(dst[..3], [b'x' as u16, 0, 9]);
    }

    #[test]
    fn lstrcat_appends_to_existing_string() {
        let mut dst = [0u16; 8];
        dst[..3].copy_from_slice(&[b'a' as u16, b'b' as u16, 0]);
        assert_eq!(lstrcat_w(&mut dst, &w("cd")), Some(4));
        assert_eq!(
            dst[..5],
            [b'a' as u16, b'b' as u16, b'c' as u16, b'd' as u16, 0]
        );
        assert_eq!(lstrcat_w(&mut dst, &w("")), Some(4));
    }

    #[test]
    fn lstrcat_rejects_overflow_and_unterminated_destination() {
        let mut dst = [0u16; 4];
        dst[..2].copy_from_slice(&[b'a' as u16, b'b' as u16]);
        assert_eq!(lstrcat_w(&mut dst, &w("cd")), None);
        assert_eq!(dst, [b'a' as u16, b'b' as u16, 0, 0]);
        let mut full = [b'z' as u16; 2];
        assert_eq!(lstrcat_w(&mut full, &w("")), None);
    }
}
