//! Byte-swap and string helpers from `dlls/mapi32/util.c`.

use crate::c_str;

/// `SwapPlong`. Byte-swaps each element in place.
pub fn swap_plong(data: &mut [u32]) {
    for v in data {
        *v = v.swap_bytes();
    }
}

/// `SwapPword`. Byte-swaps each element in place.
pub fn swap_pword(data: &mut [u16]) {
    for v in data {
        *v = v.swap_bytes();
    }
}

/// `UFromSz`. Reads the leading decimal digits, and stops at the first other character. The
/// arithmetic wraps at 32 bits, like the C `ULONG`.
pub fn uf_from_sz(text: &[u8]) -> u32 {
    text.iter()
        .take_while(|b| b.is_ascii_digit())
        .fold(0, |acc, &b| {
            acc.wrapping_mul(10).wrapping_add(u32::from(b - b'0'))
        })
}

/// `CbOfEncoded`. The encoded size of a string, in bytes. `None` is a NULL pointer, which gives 0.
pub fn cb_of_encoded(text: Option<&[u8]>) -> u32 {
    text.map_or(0, |text| {
        let len = c_str(text).len();
        ((((len | 3) >> 2) + 1) * 3) as u32
    })
}

/// `MNLS_lstrlenW`. The number of units before the first NUL. A slice without a NUL counts whole.
pub fn lstrlen_w(text: &[u16]) -> usize {
    text.iter().position(|&c| c == 0).unwrap_or(text.len())
}

/// `MNLS_lstrcpyW`. Copies `src` with its NUL into `dest`, and returns the size in bytes. Returns
/// `None` when the copy does not fit in `dest`, or when `src` has no NUL. In those cases the C
/// code writes or reads past the end of a buffer.
pub fn lstrcpy_w(dest: &mut [u16], src: &[u16]) -> Option<usize> {
    let units = lstrlen_w(src) + 1;
    dest.get_mut(..units)?.copy_from_slice(src.get(..units)?);
    Some(units * 2)
}
