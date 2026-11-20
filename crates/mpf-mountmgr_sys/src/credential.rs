//! `check_credential_string` from `cred.c`.

/// Checks that `size` bytes at `offset` of `buf` hold a NUL-terminated UTF-16 string.
///
/// The length must be non-zero and even, and the range must lie within `buf`. Its
/// last code unit must be zero. An `offset + size` that overflows `u32` is rejected.
pub fn check_credential_string(buf: &[u8], size: u32, offset: u32) -> bool {
    let Some(end) = offset.checked_add(size) else {
        return false;
    };
    let end = end as usize;
    if size == 0 || !size.is_multiple_of(2) || end > buf.len() {
        return false;
    }
    buf[end - 2..end] == [0, 0]
}
