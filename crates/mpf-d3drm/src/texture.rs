//! Image helpers of Wine's `dlls/d3drm/texture.c`: the palettised 8-bit form, the 32-bit BGRX form
//! and the 8-bit copy. The source is 24-bit BGR rows, read bottom-up when `flip` is set.

/// The palettised form of `d3drm_image_palettise`.
#[derive(Debug, PartialEq, Eq)]
pub struct Palettised {
    /// Bytes per row: the width rounded up to a multiple of 4.
    pub pitch: usize,
    /// `pitch * h` palette indices, top-down. The padding bytes are 0xff.
    pub indices: Vec<u8>,
    /// The colours in order of first use, as `[red, green, blue]`.
    pub palette: Vec<[u8; 3]>,
}

/// The image that [`load_rgb24`] makes.
#[derive(Debug, PartialEq, Eq)]
pub enum Rgb24Image {
    /// The source has at most 256 colours, so it is palettised.
    Palette(Palettised),
    /// The source expands to 32-bit BGRX rows of `pitch` bytes each.
    Bgrx { pitch: usize, pixels: Vec<u8> },
}

/// The failures of [`load_rgb24`] and [`load_indexed`].
#[derive(Debug, PartialEq, Eq)]
pub enum ImageError {
    /// `D3DRMERR_BADALLOC`: the image size overflows `usize`.
    TooLarge,
    /// The source is shorter than the image. Wine reads past its input here.
    ShortSource,
}

/// `d3drm_image_palettise`. Returns `None` when the source has more than 256 colours, when it is
/// shorter than `w * h * 3` bytes, or when that size overflows `usize`. A zero height gives an
/// empty image, and a zero width gives an empty palette.
pub fn palettise(src: &[u8], w: usize, h: usize, flip: bool) -> Option<Palettised> {
    let pitch = w.checked_add(3)? & !3;
    if src.len() < w.checked_mul(h)?.checked_mul(3)? {
        return None;
    }
    let mut indices = vec![0xff; pitch * h];
    let mut palette: Vec<[u8; 3]> = Vec::new();
    for y in 0..h {
        let row = source_row(y, h, flip) * w * 3;
        for x in 0..w {
            let pixel = &src[row + 3 * x..row + 3 * x + 3];
            let colour = [pixel[2], pixel[1], pixel[0]];
            let index = match palette.iter().position(|&entry| entry == colour) {
                Some(index) => index,
                None if palette.len() == 256 => return None,
                None => {
                    palette.push(colour);
                    palette.len() - 1
                }
            };
            indices[y * pitch + x] = index as u8;
        }
    }
    Some(Palettised {
        pitch,
        indices,
        palette,
    })
}

/// `d3drm_image_load_32`. The source is palettised when it has at most 256 colours. Otherwise it
/// expands to BGRX rows with the alpha byte set to 0xff.
pub fn load_rgb24(src: &[u8], w: usize, h: usize, flip: bool) -> Result<Rgb24Image, ImageError> {
    let pitch = w.checked_mul(4).ok_or(ImageError::TooLarge)?;
    if pitch.checked_mul(h).is_none() {
        return Err(ImageError::TooLarge);
    }
    if src.len() < w * h * 3 {
        return Err(ImageError::ShortSource);
    }
    match palettise(src, w, h, flip) {
        Some(palettised) => Ok(Rgb24Image::Palette(palettised)),
        None => Ok(Rgb24Image::Bgrx {
            pitch,
            pixels: expand_bgrx(src, w, h, flip),
        }),
    }
}

/// `d3drm_image_load_8` without its palette copy. The port takes no 256-entry palette input, so
/// only the rows are copied, bottom-up when `flip` is set.
pub fn load_indexed(src: &[u8], w: usize, h: usize, flip: bool) -> Result<Vec<u8>, ImageError> {
    let len = w.checked_mul(h).ok_or(ImageError::TooLarge)?;
    if src.len() < len {
        return Err(ImageError::ShortSource);
    }
    let mut pixels = vec![0; len];
    for y in 0..h {
        let row = source_row(y, h, flip) * w;
        pixels[y * w..(y + 1) * w].copy_from_slice(&src[row..row + w]);
    }
    Ok(pixels)
}

fn expand_bgrx(src: &[u8], w: usize, h: usize, flip: bool) -> Vec<u8> {
    let pitch = w * 4;
    let mut pixels = vec![0; pitch * h];
    for y in 0..h {
        let row = source_row(y, h, flip) * w * 3;
        let dst_row = &mut pixels[y * pitch..(y + 1) * pitch];
        for (x, dst) in dst_row.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            let pixel = &src[row + 3 * x..row + 3 * x + 3];
            dst.copy_from_slice(&[pixel[0], pixel[1], pixel[2], 0xff]);
        }
    }
    pixels
}

/// The source row that output row `y` is copied from.
fn source_row(y: usize, h: usize, flip: bool) -> usize {
    if flip {
        h - 1 - y
    } else {
        y
    }
}
