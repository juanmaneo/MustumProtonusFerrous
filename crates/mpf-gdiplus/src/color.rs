/// `ARGB2COLORREF` from `dlls/gdiplus/gdiplus.c`. The alpha byte is dropped.
pub fn argb2colorref(color: u32) -> u32 {
    ((color & 0xff) << 16) + (color & 0xff00) + ((color & 0xff_0000) >> 16)
}

/// `color_over` from `dlls/gdiplus/gdiplus_private.h`: `fg` composited over `bg`, both
/// non-premultiplied.
///
/// C writes `(a << 24)` with `a` a `BYTE` promoted to `int`, which is undefined when `a >= 0x80`.
/// The port packs the bits as unsigned, which is the value GCC produces.
pub fn color_over(bg: u32, fg: u32) -> u32 {
    let fg_alpha = (fg >> 24) as u8;
    if fg_alpha == 0xff {
        return fg;
    }
    if fg_alpha == 0 {
        return bg;
    }
    let bg_alpha = ((bg >> 24) * (0xff - u32::from(fg_alpha)) / 0xff) as u8;
    if bg_alpha == 0 {
        return fg;
    }
    let a = bg_alpha.wrapping_add(fg_alpha);
    let weights = (u32::from(bg_alpha), u32::from(fg_alpha), u32::from(a));
    pack(
        a,
        blend_channel(bg, fg, 16, weights),
        blend_channel(bg, fg, 8, weights),
        blend_channel(bg, fg, 0, weights),
    )
}

/// `color_over_fgpremult` from `dlls/gdiplus/gdiplus_private.h`: `fg` is premultiplied, `bg` and
/// the result are not. The channel bytes truncate to `BYTE` as the C assignments do.
pub fn color_over_fgpremult(bg: u32, fg: u32) -> u32 {
    let fg_alpha = (fg >> 24) as u8;
    if fg_alpha == 0 {
        return bg;
    }
    let bg_alpha = ((bg >> 24) * (0xff - u32::from(fg_alpha)) / 0xff) as u8;
    let a = bg_alpha.wrapping_add(fg_alpha);
    let weights = (u32::from(bg_alpha), 0xff, u32::from(a));
    pack(
        a,
        blend_channel(bg, fg, 16, weights),
        blend_channel(bg, fg, 8, weights),
        blend_channel(bg, fg, 0, weights),
    )
}

/// `convert_32bppARGB_to_32bppPARGB` from `dlls/gdiplus/gdiplus.c`. Each row of `width` pixels
/// starts `src_stride` bytes (or `dst_stride` bytes) after the previous one.
pub fn convert_32bpp_argb_to_32bpp_pargb(
    width: usize,
    height: usize,
    dst: &mut [u8],
    dst_stride: usize,
    src: &[u8],
    src_stride: usize,
) {
    for y in 0..height {
        let src_row = y * src_stride;
        let dst_row = y * dst_stride;
        for x in 0..width {
            let src_px = src_row + x * 4;
            let dst_px = dst_row + x * 4;
            let alpha = u32::from(src[src_px + 3]);
            dst[dst_px] = premultiply(src[src_px], alpha);
            dst[dst_px + 1] = premultiply(src[src_px + 1], alpha);
            dst[dst_px + 2] = premultiply(src[src_px + 2], alpha);
            dst[dst_px + 3] = src[src_px + 3];
        }
    }
}

fn premultiply(channel: u8, alpha: u32) -> u8 {
    ((u32::from(channel) * alpha + 127) / 255) as u8
}

/// The packing at the end of `color_over` and `color_over_fgpremult`.
fn pack(a: u8, r: u8, g: u8, b: u8) -> u32 {
    (u32::from(a) << 24) | (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
}

/// One colour byte of `(bg_c * bg_alpha + fg_c * fg_alpha) / a`, truncated to `BYTE`. `weights`
/// is `(bg_alpha, fg_alpha, a)`.
fn blend_channel(bg: u32, fg: u32, shift: u32, weights: (u32, u32, u32)) -> u8 {
    let (bg_alpha, fg_alpha, a) = weights;
    ((((bg >> shift) & 0xff) * bg_alpha + ((fg >> shift) & 0xff) * fg_alpha) / a) as u8
}
