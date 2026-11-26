/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// `MODULE` from `dlls/d3d8thk/Makefile.in`: the module name `winebuild` passes to each stub.
const D3D8THK_DLL_NAME: &str = "d3d8thk.dll";

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

/// `OsThunkD3dContextCreate`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_d3d_context_create() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkD3dContextCreate")
}

/// `OsThunkD3dContextDestroy`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_d3d_context_destroy() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkD3dContextDestroy")
}

/// `OsThunkD3dContextDestroyAll`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_d3d_context_destroy_all() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkD3dContextDestroyAll")
}

/// `OsThunkD3dDrawPrimitives2`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_d3d_draw_primitives2() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkD3dDrawPrimitives2")
}

/// `OsThunkD3dValidateTextureStageState`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_d3d_validate_texture_stage_state() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkD3dValidateTextureStageState")
}

/// `OsThunkDdAddAttachedSurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_add_attached_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdAddAttachedSurface")
}

/// `OsThunkDdAlphaBlt`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_alpha_blt() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdAlphaBlt")
}

/// `OsThunkDdAttachSurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_attach_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdAttachSurface")
}

/// `OsThunkDdBeginMoCompFrame`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_begin_mo_comp_frame() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdBeginMoCompFrame")
}

/// `OsThunkDdBlt`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_blt() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdBlt")
}

/// `OsThunkDdCanCreateD3DBuffer`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_can_create_d3d_buffer() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCanCreateD3DBuffer")
}

/// `OsThunkDdCanCreateSurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_can_create_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCanCreateSurface")
}

/// `OsThunkDdColorControl`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_color_control() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdColorControl")
}

/// `OsThunkDdCreateD3DBuffer`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_d3d_buffer() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateD3DBuffer")
}

/// `OsThunkDdCreateDirectDrawObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_direct_draw_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateDirectDrawObject")
}

/// `OsThunkDdCreateMoComp`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_mo_comp() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateMoComp")
}

/// `OsThunkDdCreateSurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateSurface")
}

/// `OsThunkDdCreateSurfaceEx`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_surface_ex() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateSurfaceEx")
}

/// `OsThunkDdCreateSurfaceObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_create_surface_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdCreateSurfaceObject")
}

/// `OsThunkDdDeleteDirectDrawObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_delete_direct_draw_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdDeleteDirectDrawObject")
}

/// `OsThunkDdDeleteSurfaceObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_delete_surface_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdDeleteSurfaceObject")
}

/// `OsThunkDdDestroyD3DBuffer`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_destroy_d3d_buffer() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdDestroyD3DBuffer")
}

/// `OsThunkDdDestroyMoComp`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_destroy_mo_comp() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdDestroyMoComp")
}

/// `OsThunkDdDestroySurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_destroy_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdDestroySurface")
}

/// `OsThunkDdEndMoCompFrame`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_end_mo_comp_frame() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdEndMoCompFrame")
}

/// `OsThunkDdFlip`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_flip() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdFlip")
}

/// `OsThunkDdFlipToGDISurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_flip_to_gdi_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdFlipToGDISurface")
}

/// `OsThunkDdGetAvailDriverMemory`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_avail_driver_memory() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetAvailDriverMemory")
}

/// `OsThunkDdGetBltStatus`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_blt_status() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetBltStatus")
}

/// `OsThunkDdGetDC`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_dc() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetDC")
}

/// `OsThunkDdGetDriverInfo`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_driver_info() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetDriverInfo")
}

/// `OsThunkDdGetDriverState`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_driver_state() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetDriverState")
}

/// `OsThunkDdGetDxHandle`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_dx_handle() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetDxHandle")
}

/// `OsThunkDdGetFlipStatus`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_flip_status() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetFlipStatus")
}

/// `OsThunkDdGetInternalMoCompInfo`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_internal_mo_comp_info() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetInternalMoCompInfo")
}

/// `OsThunkDdGetMoCompBuffInfo`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_mo_comp_buff_info() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetMoCompBuffInfo")
}

/// `OsThunkDdGetMoCompFormats`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_mo_comp_formats() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetMoCompFormats")
}

/// `OsThunkDdGetMoCompGuids`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_mo_comp_guids() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetMoCompGuids")
}

/// `OsThunkDdGetScanLine`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_get_scan_line() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdGetScanLine")
}

/// `OsThunkDdLock`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_lock() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdLock")
}

/// `OsThunkDdLockD3D`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_lock_d3d() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdLockD3D")
}

/// `OsThunkDdQueryDirectDrawObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_query_direct_draw_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdQueryDirectDrawObject")
}

/// `OsThunkDdQueryMoCompStatus`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_query_mo_comp_status() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdQueryMoCompStatus")
}

/// `OsThunkDdReenableDirectDrawObject`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_reenable_direct_draw_object() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdReenableDirectDrawObject")
}

/// `OsThunkDdReleaseDC`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_release_dc() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdReleaseDC")
}

/// `OsThunkDdRenderMoComp`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_render_mo_comp() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdRenderMoComp")
}

/// `OsThunkDdResetVisrgn`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_reset_visrgn() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdResetVisrgn")
}

/// `OsThunkDdSetColorKey`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_set_color_key() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdSetColorKey")
}

/// `OsThunkDdSetExclusiveMode`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_set_exclusive_mode() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdSetExclusiveMode")
}

/// `OsThunkDdSetGammaRamp`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_set_gamma_ramp() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdSetGammaRamp")
}

/// `OsThunkDdSetOverlayPosition`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_set_overlay_position() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdSetOverlayPosition")
}

/// `OsThunkDdUnattachSurface`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_unattach_surface() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdUnattachSurface")
}

/// `OsThunkDdUnlock`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_unlock() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdUnlock")
}

/// `OsThunkDdUnlockD3D`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_unlock_d3d() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdUnlockD3D")
}

/// `OsThunkDdUpdateOverlay`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_update_overlay() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdUpdateOverlay")
}

/// `OsThunkDdWaitForVerticalBlank`, `@ stub` in `dlls/d3d8thk/d3d8thk.spec`.
pub fn os_thunk_dd_wait_for_vertical_blank() -> StubException {
    wine_spec_unimplemented_stub(D3D8THK_DLL_NAME, "OsThunkDdWaitForVerticalBlank")
}
