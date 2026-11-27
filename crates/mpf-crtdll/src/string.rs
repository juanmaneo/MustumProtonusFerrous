/// `_swab` from `dlls/msvcrt/string.c`. Each pair of bytes from `src` is copied to `dst` with its
/// two bytes swapped. A `len` of 1 or less, including every negative `len`, copies nothing, and an
/// odd `len` leaves the last byte of `dst` untouched.
///
/// The C function does not check its buffers. This port panics when `src` or `dst` holds fewer
/// than `len` bytes, where the C function would read or write past the end of a buffer.
pub fn swab(src: &[u8], dst: &mut [u8], len: i32) {
    if len <= 1 {
        return;
    }
    let bytes = 2 * (((len as u32) >> 1) as usize);
    let (pairs, _) = src[..bytes].as_chunks::<2>();
    let (outs, _) = dst[..bytes].as_chunks_mut::<2>();
    for (pair, out) in pairs.iter().zip(outs) {
        out[0] = pair[1];
        out[1] = pair[0];
    }
}
