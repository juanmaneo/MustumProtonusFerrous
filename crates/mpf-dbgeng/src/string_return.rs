/// `S_OK`.
pub const S_OK: i32 = 0;
/// `S_FALSE`: the string did not fit in the caller's buffer.
pub const S_FALSE: i32 = 1;

/// Port of `debug_target_return_string`.
///
/// `s` is a C string without interior NUL bytes. `size` receives `len + 1`
/// when given. When `buffer` is given it receives at most `buffer_size - 1`
/// bytes followed by a NUL, and is left untouched when `buffer_size` is 0.
/// `buffer`, when `Some`, must hold at least `buffer_size` bytes.
pub fn return_string(
    s: &[u8],
    buffer: Option<&mut [u8]>,
    buffer_size: usize,
    size: Option<&mut u32>,
) -> i32 {
    let len = s.len();

    if let Some(size) = size {
        *size = len as u32 + 1;
    }

    if let Some(buf) = buffer {
        if buffer_size != 0 {
            let dst = &mut buf[..buffer_size];
            let dst_len = len.min(buffer_size - 1);
            dst[..dst_len].copy_from_slice(&s[..dst_len]);
            dst[dst_len] = 0;
        }
    }

    if len < buffer_size {
        S_OK
    } else {
        S_FALSE
    }
}
