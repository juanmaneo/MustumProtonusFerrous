//! Integer HSL and RGB conversions used by the common color dialog.

fn hsl_to_x(hue: i32, saturation: i32, luminosity: i32) -> i32 {
    let mut result;
    let max_rgb = (256 * 120.min(luminosity)) / 120;

    if hue < 80 {
        result = 0;
    } else if hue < 120 {
        result = (hue - 80) * max_rgb;
        result /= 40;
    } else if hue < 200 {
        result = max_rgb;
    } else {
        result = (240 - hue) * max_rgb;
        result /= 40;
    }

    result -= max_rgb / 2;
    result = max_rgb / 2 + (saturation * result) / 240;

    if luminosity > 120 && result < 256 {
        result += ((luminosity - 120) * (256 - result)) / 120;
    }

    result.min(255)
}

/// Converts Wine's HSL values (each on a 0..240 scale) to a Windows `COLORREF`.
///
/// As in Wine, the packed return value stores red in the low byte, then green and blue.
pub fn hsl_to_rgb(hue: i32, saturation: i32, luminosity: i32) -> u32 {
    let red_hue = if hue > 80 { hue - 80 } else { hue + 160 };
    let green_hue = if hue > 160 { hue - 160 } else { hue + 80 };
    let red = hsl_to_x(red_hue, saturation, luminosity);
    let green = hsl_to_x(green_hue, saturation, luminosity);
    let blue = hsl_to_x(hue, saturation, luminosity);

    (red as u8 as u32) | ((green as u8 as u32) << 8) | ((blue as u8 as u32) << 16)
}

/// Extracts hue (`b'H'`), saturation (`b'S'`), or luminosity (`b'L'`) from a `COLORREF`.
///
/// Matches Wine's integer arithmetic and returns zero for any other component selector.
pub fn rgb_to_hsl(component: u8, color: u32) -> u16 {
    let red = (color & 0xff) as i32;
    let green = ((color >> 8) & 0xff) as i32;
    let blue = ((color >> 16) & 0xff) as i32;
    let maximum = red.max(blue).max(green) as u16;
    let minimum = red.min(blue).min(green) as u16;
    let sum = maximum + minimum;
    let difference = maximum - minimum;

    match component {
        b'L' => (sum * 120) / 255,
        b'S' => {
            if sum == 0 {
                0
            } else if minimum == 0 || maximum == 255 {
                240
            } else {
                let mut saturation = difference * 240;
                saturation /= if sum > 255 { 510 - sum } else { sum };
                saturation
            }
        }
        b'H' => {
            if difference == 0 {
                160
            } else if maximum == red as u16 {
                let mut hue = 40 * (green - blue) / i32::from(difference);
                if hue < 0 {
                    hue += 240;
                }
                hue as u16
            } else if maximum == green as u16 {
                (40 * (blue - red) / i32::from(difference) + 80) as u16
            } else {
                (40 * (red - green) / i32::from(difference) + 160) as u16
            }
        }
        _ => 0,
    }
}
