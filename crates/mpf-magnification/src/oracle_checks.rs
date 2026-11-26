//! Oracle-derived test vectors for the ported magnification logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `MagInitialize` and
//! `MagUninitialize` (`dlls/magnification/main.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/magnification/`, and running the harness. It printed 21 rows, one per export of
//! `magnification.spec` in spec order. ASan and UBSan reported nothing, so no vector is excluded.
//! The harness replaces `RtlRaiseException` with a recorder that longjmps back to the caller,
//! because the verbatim stub loops on it.

use super::*;

type StubRow = (usize, fn() -> StubException, &'static str);

/// The 19 stub rows: the row number the harness printed, the helper, and the export name. Each
/// row raised `code=0x80000100 flags=0x1 nargs=2 p0=magnification.dll p1=<name> raises=1`.
const STUB_ROWS: [StubRow; 19] = [
    (
        1,
        mag_set_fullscreen_use_bitmap_smoothing,
        "MagSetFullscreenUseBitmapSmoothing",
    ),
    (
        2,
        mag_set_lens_use_bitmap_smoothing,
        "MagSetLensUseBitmapSmoothing",
    ),
    (3, mag_get_color_effect, "MagGetColorEffect"),
    (
        4,
        mag_get_fullscreen_color_effect,
        "MagGetFullscreenColorEffect",
    ),
    (5, mag_get_fullscreen_transform, "MagGetFullscreenTransform"),
    (
        6,
        mag_get_image_scaling_callback,
        "MagGetImageScalingCallback",
    ),
    (7, mag_get_input_transform, "MagGetInputTransform"),
    (8, mag_get_window_filter_list, "MagGetWindowFilterList"),
    (9, mag_get_window_source, "MagGetWindowSource"),
    (10, mag_get_window_transform, "MagGetWindowTransform"),
    (12, mag_set_color_effect, "MagSetColorEffect"),
    (
        13,
        mag_set_fullscreen_color_effect,
        "MagSetFullscreenColorEffect",
    ),
    (
        14,
        mag_set_fullscreen_transform,
        "MagSetFullscreenTransform",
    ),
    (
        15,
        mag_set_image_scaling_callback,
        "MagSetImageScalingCallback",
    ),
    (16, mag_set_input_transform, "MagSetInputTransform"),
    (17, mag_set_window_filter_list, "MagSetWindowFilterList"),
    (18, mag_set_window_source, "MagSetWindowSource"),
    (19, mag_set_window_transform, "MagSetWindowTransform"),
    (20, mag_show_system_cursor, "MagShowSystemCursor"),
];

#[test]
fn oracle_stub_rows() {
    for (row, helper, name) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "magnification.dll",
                function: name,
            },
            "row {row:02}"
        );
    }
}

#[test]
fn oracle_mag_initialize_row() {
    // row 11 MagInitialize last_error_before=0 -> ret=0 last_error=120
    let mut last_error = 0;
    assert!(!mag_initialize(&mut last_error));
    assert_eq!(last_error, 120);
}

#[test]
fn oracle_mag_uninitialize_row() {
    // row 21 MagUninitialize last_error_before=85 -> ret=1 last_error=85
    // The C body never calls SetLastError, and the port takes no last-error argument.
    assert!(mag_uninitialize());
}
