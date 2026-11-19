//! `gifformat.c`: the palette that a GIF frame gets, with its transparent entry, and the copy that
//! turns the interlaced rows of a frame back into display order.

use crate::{HResult, WicRect, E_INVALIDARG};

/// `GRAPHICS_EXT_FUNC_CODE`: the function code of the graphic control extension.
pub const GRAPHICS_EXT_FUNC_CODE: u8 = 0xf9;

/// A `GifColorType` entry of a colour map.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// An `ExtensionBlock`. Its `ByteCount` is the length of `bytes`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExtensionBlock {
    pub function: u8,
    pub bytes: Vec<u8>,
}

/// `copy_palette`: writes the first `count` colours as opaque ARGB. With no map, index 0 is black,
/// index 1 is white and the rest are black. A graphic control extension with the transparency flag
/// then clears the alpha of its transparent entry. Writes stop at the end of `colors`, and map
/// entries past the end of `colormap` are not written.
pub fn copy_palette(
    colormap: Option<&[Rgb]>,
    extensions: &[ExtensionBlock],
    count: usize,
    colors: &mut [u32],
) {
    match colormap {
        Some(map) => {
            for (color, entry) in colors.iter_mut().take(count).zip(map) {
                *color = 0xff00_0000
                    | (u32::from(entry.red) << 16)
                    | (u32::from(entry.green) << 8)
                    | u32::from(entry.blue);
            }
        }
        None => {
            for (index, color) in colors.iter_mut().take(count).enumerate() {
                *color = if index == 1 { 0xffff_ffff } else { 0xff00_0000 };
            }
        }
    }
    let transparent = extensions.iter().find(|block| {
        block.function == GRAPHICS_EXT_FUNC_CODE
            && block.bytes.len() == 8
            && (block.bytes[3] & 1) != 0
    });
    if let Some(block) = transparent {
        if let Some(color) = colors.get_mut(usize::from(block.bytes[6])) {
            *color &= 0x00ff_ffff;
        }
    }
}

/// `copy_interlaced_pixels`: copies `rect`, or the whole frame without one, from the interlaced
/// rows of `src` into `dst` in display order, `dst_stride` bytes apart. Wine's `srcstride` is
/// signed, and here it is unsigned.
pub fn copy_interlaced_pixels(
    src: &[u8],
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    rect: Option<WicRect>,
    dst_stride: u32,
    dst: &mut [u8],
) -> Result<(), HResult> {
    let [x, y, width, height] = match rect {
        None => [0, 0, u64::from(src_width), u64::from(src_height)],
        Some(rect) => rect.unsigned()?,
    };
    if x + width > u64::from(src_width)
        || y + height > u64::from(src_height)
        || u64::from(dst_stride) < width
    {
        return Err(E_INVALIDARG);
    }
    let needed = if height == 0 {
        0
    } else {
        u64::from(dst_stride) * (height - 1) + width
    };
    if needed > dst.len() as u64 {
        return Err(E_INVALIDARG);
    }
    let source_offset = |row: u64| {
        interlaced_row(row, u64::from(src_height))
            .saturating_mul(u64::from(src_stride))
            .saturating_add(x)
    };
    if (y..y + height).any(|row| source_offset(row).saturating_add(width) > src.len() as u64) {
        return Err(E_INVALIDARG);
    }
    let len = width as usize;
    for (index, row) in (y..y + height).enumerate() {
        let from = source_offset(row) as usize;
        let to = index * dst_stride as usize;
        dst[to..to + len].copy_from_slice(&src[from..from + len]);
    }
    Ok(())
}

/// The source row that display row `row` is read from. A GIF frame stores its rows in four passes:
/// every eighth row, then the rows 4 mod 8, the rows 2 mod 4, and the odd rows.
fn interlaced_row(row: u64, height: u64) -> u64 {
    if row.is_multiple_of(8) {
        row / 8
    } else if row.is_multiple_of(4) {
        height.div_ceil(8) + row / 8
    } else if row.is_multiple_of(2) {
        height.div_ceil(4) + row / 4
    } else {
        height.div_ceil(2) + row / 2
    }
}
