/// `DXVA2_AYUVSample16`: a 16-bit AYUV sample. Each field holds the 8-bit value in its top byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ayuv16 {
    pub cr: u16,
    pub cb: u16,
    pub y: u16,
    pub alpha: u16,
}

/// `video_processor_get_background_color`: the opaque `D3DCOLOR` of a YCbCr background.
///
/// The arithmetic follows the C body step for step, in `f32` up to the BT.601 weights, which are
/// evaluated in `f64`. The C `(BYTE)` conversion truncates in range. Out of range, C is undefined
/// and Rust saturates, so the result is the clamped channel rather than a wrapped one.
pub fn background_color(ayuv: &Ayuv16) -> u32 {
    let y = (i32::from(ayuv.y >> 8) - 16) as f32;
    let cb = (i32::from(ayuv.cb >> 8) - 128) as f32;
    let cr = (i32::from(ayuv.cr >> 8) - 128) as f32;

    let y = 255.0f32 * y / 219.0;
    let cb = 255.0f32 * cb / 224.0;
    let cr = 255.0f32 * cr / 224.0;

    let r = (f64::from(y) + 1.402 * f64::from(cr)) as u8;
    let g = (f64::from(y) - 0.344 * f64::from(cb) - 0.714 * f64::from(cr)) as u8;
    let b = (f64::from(y) + 1.772 * f64::from(cb)) as u8;

    0xff00_0000 | (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
}
