//! The 23 rows below were observed by compiling the verbatim conversion helpers from
//! Wine's `dlls/comdlg32/colordlg.c` with ASan and UBSan.

use crate::color::{hsl_to_rgb, rgb_to_hsl};

const RGB_ROWS: &[(u8, u32, u16)] = &[
    (b'L', 0x0000_0000, 0),
    (b'S', 0x0000_0000, 0),
    (b'H', 0x0000_0000, 160),
    (b'L', 0x0080_8080, 120),
    (b'S', 0x0080_8080, 0),
    (b'H', 0x0080_8080, 160),
    (b'L', 0x0000_00ff, 120),
    (b'S', 0x0000_00ff, 240),
    (b'H', 0x0000_00ff, 0),
    (b'L', 0x0000_ff00, 120),
    (b'S', 0x0000_ff00, 240),
    (b'H', 0x0000_ff00, 80),
    (b'L', 0x00ff_0000, 120),
    (b'S', 0x00ff_0000, 240),
    (b'H', 0x00ff_0000, 160),
    (b'L', 0x0040_2080, 75),
    (b'S', 0x0040_2080, 144),
    (b'H', 0x0040_2080, 227),
    (b'H', 0x0020_4080, 13),
    (b'S', 0x0080_a0d0, 110),
];

const HSL_ROWS: &[(i32, i32, i32, u32)] = &[
    (0, 240, 120, 0x0000_00ff),
    (120, 120, 80, 8_355_627),
    (0, 0, 240, 16_777_215),
];

#[test]
fn rgb_to_hsl_matches_all_oracle_rows() {
    for (index, &(component, color, expected)) in RGB_ROWS.iter().enumerate() {
        assert_eq!(rgb_to_hsl(component, color), expected, "R{:02}", index + 1);
    }
}

#[test]
fn hsl_to_rgb_matches_all_oracle_rows() {
    for (index, &(hue, saturation, luminosity, expected)) in HSL_ROWS.iter().enumerate() {
        assert_eq!(
            hsl_to_rgb(hue, saturation, luminosity),
            expected,
            "R{:02}",
            index + 19
        );
    }
}
