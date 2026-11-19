//! `converter.c`: the half-float decoder and the nearest-palette-entry search that the converters
//! use.

/// `float_16_to_32`: decodes an IEEE 754 half-precision value. Exponent 31 decodes like the other
/// exponents, so no infinity or NaN comes out, as in Wine.
pub fn float_16_to_32(value: u16) -> f32 {
    let sign: f32 = if (value & 0x8000) != 0 { -1.0 } else { 1.0 };
    let exponent = (value & 0x7c00) >> 10;
    let mantissa = value & 0x3ff;
    if exponent == 0 {
        if mantissa == 0 {
            sign * 0.0
        } else {
            sign * 2f32.powi(-14) * (f32::from(mantissa) / 1024.0)
        }
    } else {
        sign * 2f32.powi(i32::from(exponent) - 15) * (1.0 + f32::from(mantissa) / 1024.0)
    }
}

/// `rgb_to_palette_index`: the first entry that equals `bgr` exactly, or else the entry with the
/// smallest squared distance, where ties go to the earlier entry. `bgr` holds blue, green and red,
/// as the C does. An empty palette gives 0.
pub fn rgb_to_palette_index(bgr: [u8; 3], colors: &[u32]) -> usize {
    let mut best_diff = u32::MAX;
    let mut best_index = 0;
    for (index, &color) in colors.iter().enumerate() {
        let diff = squared_difference(bgr[2], (color >> 16) as u8)
            + squared_difference(bgr[1], (color >> 8) as u8)
            + squared_difference(bgr[0], color as u8);
        if diff == 0 {
            return index;
        }
        if diff < best_diff {
            best_diff = diff;
            best_index = index;
        }
    }
    best_index
}

fn squared_difference(a: u8, b: u8) -> u32 {
    u32::from(a.abs_diff(b)).pow(2)
}
