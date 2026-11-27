/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = gdiplus.dll` in `dlls/gdiplus/Makefile.in`.
const GDIPLUS_DLL_NAME: &str = "gdiplus.dll";

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

/// `GdipCreateBitmapFromDirectDrawSurface`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_create_bitmap_from_direct_draw_surface() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipCreateBitmapFromDirectDrawSurface")
}

/// `GdipEnumerateMetafileDestPoints`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_enumerate_metafile_dest_points() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipEnumerateMetafileDestPoints")
}

/// `GdipEnumerateMetafileDestPointsI`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_enumerate_metafile_dest_points_i() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipEnumerateMetafileDestPointsI")
}

/// `GdipEnumerateMetafileSrcRectDestPoint`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_enumerate_metafile_src_rect_dest_point() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipEnumerateMetafileSrcRectDestPoint")
}

/// `GdipEnumerateMetafileSrcRectDestPointI`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_enumerate_metafile_src_rect_dest_point_i() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipEnumerateMetafileSrcRectDestPointI")
}

/// `GdipEnumerateMetafileSrcRectDestPointsI`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_enumerate_metafile_src_rect_dest_points_i() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipEnumerateMetafileSrcRectDestPointsI")
}

/// `GdipGetCustomLineCapStrokeCaps`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_get_custom_line_cap_stroke_caps() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipGetCustomLineCapStrokeCaps")
}

/// `GdipGetEncoderParameterList`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_get_encoder_parameter_list() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipGetEncoderParameterList")
}

/// `GdipSetPenUnit`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_set_pen_unit() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipSetPenUnit")
}

/// `GdipFindNextImageItem`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_find_next_image_item() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipFindNextImageItem")
}

/// `GdipConvertToEmfPlusToStream`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_convert_to_emf_plus_to_stream() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipConvertToEmfPlusToStream")
}

/// `GdipPlayTSClientRecord`, exported as `@ stub` in `dlls/gdiplus/gdiplus.spec`.
pub fn gdip_play_ts_client_record() -> StubException {
    wine_spec_unimplemented_stub(GDIPLUS_DLL_NAME, "GdipPlayTSClientRecord")
}
