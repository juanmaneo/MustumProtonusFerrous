//! `palette.c`: the fixed palettes that `PaletteImpl_InitializePredefined` builds, with or without
//! the transparent entry that its `add` flag asks for.

use crate::{HResult, E_INVALIDARG};

/// The predefined `WICBitmapPaletteType` values. `Custom` and `MedianCut` are not here, because
/// `PaletteImpl_InitializePredefined` rejects them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteType {
    BlackWhite,
    Halftone8,
    Halftone27,
    Halftone64,
    Halftone125,
    Halftone216,
    Halftone252,
    Halftone256,
    Gray4,
    Gray16,
    Gray256,
}

const LEVELS_2: [u32; 2] = [0x00, 0xff];
const LEVELS_3: [u32; 3] = [0x00, 0x80, 0xff];
const LEVELS_4: [u32; 4] = [0x00, 0x55, 0xaa, 0xff];
const LEVELS_5: [u32; 5] = [0x00, 0x40, 0x80, 0xbf, 0xff];
const LEVELS_6: [u32; 6] = [0x00, 0x33, 0x66, 0x99, 0xcc, 0xff];
const LEVELS_7: [u32; 7] = [0x00, 0x2b, 0x55, 0x80, 0xaa, 0xd5, 0xff];
const LEVELS_8: [u32; 8] = [0x00, 0x24, 0x49, 0x6d, 0x92, 0xb6, 0xdb, 0xff];
const HALFTONE_EXTRAS: [u32; 8] = [
    0xffc0_c0c0,
    0xff80_8080,
    0xff80_0000,
    0xff00_8000,
    0xff00_0080,
    0xff80_8000,
    0xff80_0080,
    0xff00_8080,
];

impl PaletteType {
    /// The `WICBitmapPaletteType` switch of `PaletteImpl_InitializePredefined`. Every value that is
    /// not one of the eleven predefined types is `E_INVALIDARG`.
    pub fn from_wic(palette_type: u32) -> Result<Self, HResult> {
        match palette_type {
            2 => Ok(Self::BlackWhite),
            3 => Ok(Self::Halftone8),
            4 => Ok(Self::Halftone27),
            5 => Ok(Self::Halftone64),
            6 => Ok(Self::Halftone125),
            7 => Ok(Self::Halftone216),
            8 => Ok(Self::Halftone252),
            9 => Ok(Self::Halftone256),
            0xa => Ok(Self::Gray4),
            0xb => Ok(Self::Gray16),
            0xc => Ok(Self::Gray256),
            _ => Err(E_INVALIDARG),
        }
    }
}

/// The colours of a predefined palette, as ARGB. With `add`, the cube and halftone palettes gain a
/// transparent entry: Halftone256 replaces its last colour, and the others append one. The
/// black-white and gray palettes ignore `add`.
pub fn predefined_palette(palette_type: PaletteType, add: bool) -> Vec<u32> {
    match palette_type {
        PaletteType::BlackWhite => vec![0xff00_0000, 0xffff_ffff],
        PaletteType::Gray4 => vec![0xff00_0000, 0xff55_5555, 0xffaa_aaaa, 0xffff_ffff],
        PaletteType::Gray16 => (0..16).map(|level| gray(level * 0x11)).collect(),
        PaletteType::Gray256 => (0..256).map(gray).collect(),
        PaletteType::Halftone8 => {
            with_extras(cube(&LEVELS_2, &LEVELS_2, &LEVELS_2), &HALFTONE_EXTRAS, add)
        }
        PaletteType::Halftone27 => {
            with_extras(cube(&LEVELS_3, &LEVELS_3, &LEVELS_3), &[0xffc0_c0c0], add)
        }
        PaletteType::Halftone64 => {
            with_extras(cube(&LEVELS_4, &LEVELS_4, &LEVELS_4), &HALFTONE_EXTRAS, add)
        }
        PaletteType::Halftone125 => {
            with_extras(cube(&LEVELS_5, &LEVELS_5, &LEVELS_5), &[0xffc0_c0c0], add)
        }
        PaletteType::Halftone216 => {
            with_extras(cube(&LEVELS_6, &LEVELS_6, &LEVELS_6), &HALFTONE_EXTRAS, add)
        }
        PaletteType::Halftone252 => with_extras(cube(&LEVELS_6, &LEVELS_7, &LEVELS_6), &[], add),
        PaletteType::Halftone256 => {
            let mut colors = cube(&LEVELS_8, &LEVELS_8, &LEVELS_4);
            if add {
                colors[255] = 0;
            }
            colors
        }
    }
}

/// Every red, green and blue combination of the given levels, with red varying slowest.
fn cube(reds: &[u32], greens: &[u32], blues: &[u32]) -> Vec<u32> {
    let mut colors = Vec::new();
    for &red in reds {
        for &green in greens {
            for &blue in blues {
                colors.push(0xff00_0000 | (red << 16) | (green << 8) | blue);
            }
        }
    }
    colors
}

fn gray(level: u32) -> u32 {
    0xff00_0000 | (level << 16) | (level << 8) | level
}

fn with_extras(mut colors: Vec<u32>, extras: &[u32], add: bool) -> Vec<u32> {
    colors.extend_from_slice(extras);
    if add {
        colors.push(0);
    }
    colors
}
