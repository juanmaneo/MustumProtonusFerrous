//! Port of the string-pointer functions in Wine's `dlls/comctl32/string.c`.
//!
//! The C code allocates from the process heap. Here each `ReAlloc` is one call to the `alloc`
//! callback with the byte size; the callback returns `false` to fail that call. Frees are not
//! modelled.

/// The length of a NUL-terminated string, without the terminator. The C code reads past the
/// end of a string without a terminator, so the port panics there.
fn nul_len<T: Copy + Default + PartialEq>(s: &[T]) -> usize {
    s.iter()
        .position(|&c| c == T::default())
        .expect("C string without a NUL terminator (UB)")
}

/// `Str_GetPtrA`. With a NULL destination or a zero `max_len`, returns the size of `src` with
/// its NUL. Otherwise copies at most `max_len - 1` bytes, NUL-terminates the copy and returns
/// its length with the NUL.
pub fn str_get_ptr_a(src: Option<&[u8]>, dest: Option<&mut [u8]>, max_len: i32) -> i32 {
    match (src, dest) {
        (Some(src), Some(dest)) if max_len != 0 => copy_a(src, dest, max_len),
        (Some(src), _) => nul_len(src) as i32 + 1,
        (None, _) if max_len == 0 => 0,
        (None, Some(dest)) => {
            dest[0] = 0;
            0
        }
        (None, None) => panic!("Str_GetPtrA: NULL destination"),
    }
}

fn copy_a(src: &[u8], dest: &mut [u8], max_len: i32) -> i32 {
    let max = usize::try_from(max_len).expect("Str_GetPtrA: negative max_len");
    let mut len = nul_len(src) + 1;
    if len >= max {
        len = max;
    }
    dest[..len - 1].copy_from_slice(&src[..len - 1]);
    dest[len - 1] = 0;
    len as i32
}

/// `Str_SetPtrA`: replaces `dest` with a copy of `src`, including its NUL. A failed allocation
/// returns `false` and keeps the old value. A `None` source frees the string.
pub fn str_set_ptr_a(
    dest: &mut Option<Vec<u8>>,
    src: Option<&[u8]>,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> bool {
    match src {
        Some(src) => {
            let len = nul_len(src) + 1;
            if !alloc(len) {
                return false;
            }
            *dest = Some(src[..len].to_vec());
            true
        }
        None => {
            *dest = None;
            true
        }
    }
}

/// `Str_GetPtrW`. With a NULL destination, returns the length of `src`. A zero `max_len`
/// returns 0. Otherwise copies at most `max_len - 1` units, NUL-terminates the copy and returns
/// its length, without the NUL.
pub fn str_get_ptr_w(src: Option<&[u16]>, dest: Option<&mut [u16]>, max_len: i32) -> i32 {
    match (src, dest) {
        (Some(src), None) => nul_len(src) as i32,
        (_, _) if max_len == 0 => 0,
        (Some(src), Some(dest)) => copy_w(src, dest, max_len),
        (None, Some(dest)) => {
            dest[0] = 0;
            0
        }
        (None, None) => panic!("Str_GetPtrW: NULL destination"),
    }
}

fn copy_w(src: &[u16], dest: &mut [u16], max_len: i32) -> i32 {
    let max = usize::try_from(max_len).expect("Str_GetPtrW: negative max_len");
    let mut len = nul_len(src);
    if len >= max {
        len = max - 1;
    }
    dest[..len].copy_from_slice(&src[..len]);
    dest[len] = 0;
    len as i32
}

/// `Str_SetPtrW`: the wide-string version of [`str_set_ptr_a`], which allocates `len * 2` bytes
/// for `len` units including the NUL.
pub fn str_set_ptr_w(
    dest: &mut Option<Vec<u16>>,
    src: Option<&[u16]>,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> bool {
    match src {
        Some(src) => {
            let len = nul_len(src) + 1;
            if !alloc(len * 2) {
                return false;
            }
            *dest = Some(src[..len].to_vec());
            true
        }
        None => {
            *dest = None;
            true
        }
    }
}
