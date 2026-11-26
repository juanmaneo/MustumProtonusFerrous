/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = magnification.dll` in
/// `dlls/magnification/Makefile.in`.
const MAGNIFICATION_DLL_NAME: &str = "magnification.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `MagSetFullscreenUseBitmapSmoothing`, exported as `@ stub`.
pub fn mag_set_fullscreen_use_bitmap_smoothing() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetFullscreenUseBitmapSmoothing")
}

/// `MagSetLensUseBitmapSmoothing`, exported as `@ stub`.
pub fn mag_set_lens_use_bitmap_smoothing() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetLensUseBitmapSmoothing")
}

/// `MagGetColorEffect`, exported as `@ stub`.
pub fn mag_get_color_effect() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetColorEffect")
}

/// `MagGetFullscreenColorEffect`, exported as `@ stub`.
pub fn mag_get_fullscreen_color_effect() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetFullscreenColorEffect")
}

/// `MagGetFullscreenTransform`, exported as `@ stub`.
pub fn mag_get_fullscreen_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetFullscreenTransform")
}

/// `MagGetImageScalingCallback`, exported as `@ stub`.
pub fn mag_get_image_scaling_callback() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetImageScalingCallback")
}

/// `MagGetInputTransform`, exported as `@ stub`.
pub fn mag_get_input_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetInputTransform")
}

/// `MagGetWindowFilterList`, exported as `@ stub`.
pub fn mag_get_window_filter_list() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetWindowFilterList")
}

/// `MagGetWindowSource`, exported as `@ stub`.
pub fn mag_get_window_source() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetWindowSource")
}

/// `MagGetWindowTransform`, exported as `@ stub`.
pub fn mag_get_window_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagGetWindowTransform")
}

/// `MagSetColorEffect`, exported as `@ stub`.
pub fn mag_set_color_effect() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetColorEffect")
}

/// `MagSetFullscreenColorEffect`, exported as `@ stub`.
pub fn mag_set_fullscreen_color_effect() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetFullscreenColorEffect")
}

/// `MagSetFullscreenTransform`, exported as `@ stub`.
pub fn mag_set_fullscreen_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetFullscreenTransform")
}

/// `MagSetImageScalingCallback`, exported as `@ stub`.
pub fn mag_set_image_scaling_callback() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetImageScalingCallback")
}

/// `MagSetInputTransform`, exported as `@ stub`.
pub fn mag_set_input_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetInputTransform")
}

/// `MagSetWindowFilterList`, exported as `@ stub`.
pub fn mag_set_window_filter_list() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetWindowFilterList")
}

/// `MagSetWindowSource`, exported as `@ stub`.
pub fn mag_set_window_source() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetWindowSource")
}

/// `MagSetWindowTransform`, exported as `@ stub`.
pub fn mag_set_window_transform() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagSetWindowTransform")
}

/// `MagShowSystemCursor`, exported as `@ stub`.
pub fn mag_show_system_cursor() -> StubException {
    wine_spec_unimplemented_stub(MAGNIFICATION_DLL_NAME, "MagShowSystemCursor")
}
