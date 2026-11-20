//! Rows from the Wine oracle: the verbatim `button.c` helpers, built with ASan and UBSan. The
//! driver lives under `target/canon/comctl32_v6` (gitignored). Each row gives the input style,
//! extended style and image list alignment, then the expected type, centering, DrawText flags
//! and button alignment. No vector was flagged by the sanitizers.

use crate::{bs_to_dt, button_centers_text, button_type, il_align_to_bs};

/// `(style, ex_style, align, type, centers_text, dt_flags, bs_align)`.
const ROWS: &[(u32, u32, u32, u32, bool, u32, u32)] = &[
    (0x0000, 0, 0, 0, true, 293, 3328),
    (0x0001, 0, 1, 1, true, 293, 3584),
    (0x0002, 0, 2, 2, false, 292, 1792),
    (0x0003, 0, 3, 3, false, 292, 2816),
    (0x0004, 0, 4, 4, false, 292, 3840),
    (0x0005, 0, 0, 5, false, 292, 3328),
    (0x0006, 0, 1, 6, false, 292, 3584),
    (0x0007, 0, 2, 7, false, 288, 1792),
    (0x0008, 0, 3, 8, false, 292, 2816),
    (0x0009, 0, 4, 9, false, 292, 3840),
    (0x000a, 0, 0, 10, false, 292, 3328),
    (0x000b, 0, 1, 11, false, 292, 3584),
    (0x000c, 0, 2, 12, true, 293, 1792),
    (0x000d, 0, 3, 13, true, 293, 2816),
    (0x000e, 0, 4, 14, false, 292, 3840),
    (0x000f, 0, 0, 15, false, 292, 3328),
    (0x0100, 0, 0, 0, true, 292, 3328),
    (0x0202, 0, 1, 2, false, 294, 3584),
    (0x2303, 0, 2, 3, false, 277, 1792),
    (0x2c07, 0x1000, 3, 7, false, 274, 2816),
    (0x1804, 0, 4, 4, false, 297, 3840),
    (0x040c, 0x1000, 99, 12, true, 290, 3328),
    (0x2401, 0, 5, 1, true, 273, 3328),
    (0x0a0b, 0x1000, 2, 11, false, 298, 1792),
    (0x3f05, 0x1000, 99, 5, false, 278, 3328),
];

#[test]
fn oracle_rows_match_wine_helpers() {
    for &(style, ex_style, align, kind, centers, dt, bs) in ROWS {
        let window_style = style as i32;
        assert_eq!(button_type(window_style), kind, "type of {style:#x}");
        assert_eq!(
            button_centers_text(window_style),
            centers,
            "centers of {style:#x}"
        );
        assert_eq!(
            bs_to_dt(style, ex_style),
            dt,
            "dt of {style:#x}/{ex_style:#x}"
        );
        assert_eq!(il_align_to_bs(align), bs, "bs of align {align}");
    }
}
