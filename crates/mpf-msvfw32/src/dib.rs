//! DIB sizing: `get_size_image` from `msvideo_main.c` and `num_colours` from `drawdib.c`.

/// `get_size_image`: the byte size of a DIB, with each row rounded up to a multiple of 4 bytes.
///
/// `width * depth` and `abs(height)` wrap instead of overflowing, as the 32-bit `DWORD` results
/// do in Wine's gcc build.
pub const fn get_size_image(width: i32, height: i32, depth: u16) -> u32 {
    let bits = width.wrapping_mul(depth as i32) as u32;
    let bytes = bits.wrapping_add(7) / 8;
    let stride = bytes.wrapping_add(3) & !3;
    stride.wrapping_mul(height.wrapping_abs() as u32)
}

/// `num_colours`: the palette entry count of a DIB header, given its `biClrUsed` and
/// `biBitCount`. A non-zero `biClrUsed` wins. Otherwise depths up to 8 bits have `1 << bit_count`
/// entries, and deeper formats have none.
pub const fn num_colours(clr_used: u32, bit_count: u16) -> i32 {
    if clr_used != 0 {
        clr_used as i32
    } else if bit_count <= 8 {
        1 << bit_count
    } else {
        0
    }
}
