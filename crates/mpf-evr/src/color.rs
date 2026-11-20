/// `DXVA2_AYUVSample16`: a 16-bit AYUV sample. Each field holds the 8-bit value in its top byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ayuv16 {
    pub cr: u16,
    pub cb: u16,
    pub y: u16,
    pub alpha: u16,
}

/// `video_mixer_rgb_to_ycbcr`: converts an opaque `COLORREF`, laid out as `0x00bbggrr`, to an
/// AYUV sample with the SDTV ITU-R BT.601 coefficients. The integer arithmetic and the
/// truncating division follow the C body.
pub fn rgb_to_ycbcr(rgb: u32) -> Ayuv16 {
    let r = (rgb & 0xff) as i32;
    let g = ((rgb >> 8) & 0xff) as i32;
    let b = ((rgb >> 16) & 0xff) as i32;

    let y = (77 * r + 150 * g + 29 * b + 128) / 256 + 16;
    let cb = (-44 * r - 87 * g + 131 * b + 128) / 256 + 128;
    let cr = (131 * r - 110 * g - 21 * b + 128) / 256 + 128;

    Ayuv16 {
        cr: (cr * 0x100) as u16,
        cb: (cb * 0x100) as u16,
        y: (y * 0x100) as u16,
        alpha: 0xffff,
    }
}
