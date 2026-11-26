use crate::stub::*;

/// The helper that builds the stub record for one export.
pub type StubHelper = fn() -> StubException;

/// The `d3d8thk.spec` export table in spec order. Every export is `@ stub`, so each name is paired
/// with the helper that raises its record.
pub const EXPORTS: [(&str, StubHelper); 56] = [
    ("OsThunkD3dContextCreate", os_thunk_d3d_context_create),
    ("OsThunkD3dContextDestroy", os_thunk_d3d_context_destroy),
    (
        "OsThunkD3dContextDestroyAll",
        os_thunk_d3d_context_destroy_all,
    ),
    ("OsThunkD3dDrawPrimitives2", os_thunk_d3d_draw_primitives2),
    (
        "OsThunkD3dValidateTextureStageState",
        os_thunk_d3d_validate_texture_stage_state,
    ),
    (
        "OsThunkDdAddAttachedSurface",
        os_thunk_dd_add_attached_surface,
    ),
    ("OsThunkDdAlphaBlt", os_thunk_dd_alpha_blt),
    ("OsThunkDdAttachSurface", os_thunk_dd_attach_surface),
    ("OsThunkDdBeginMoCompFrame", os_thunk_dd_begin_mo_comp_frame),
    ("OsThunkDdBlt", os_thunk_dd_blt),
    (
        "OsThunkDdCanCreateD3DBuffer",
        os_thunk_dd_can_create_d3d_buffer,
    ),
    ("OsThunkDdCanCreateSurface", os_thunk_dd_can_create_surface),
    ("OsThunkDdColorControl", os_thunk_dd_color_control),
    ("OsThunkDdCreateD3DBuffer", os_thunk_dd_create_d3d_buffer),
    (
        "OsThunkDdCreateDirectDrawObject",
        os_thunk_dd_create_direct_draw_object,
    ),
    ("OsThunkDdCreateMoComp", os_thunk_dd_create_mo_comp),
    ("OsThunkDdCreateSurface", os_thunk_dd_create_surface),
    ("OsThunkDdCreateSurfaceEx", os_thunk_dd_create_surface_ex),
    (
        "OsThunkDdCreateSurfaceObject",
        os_thunk_dd_create_surface_object,
    ),
    (
        "OsThunkDdDeleteDirectDrawObject",
        os_thunk_dd_delete_direct_draw_object,
    ),
    (
        "OsThunkDdDeleteSurfaceObject",
        os_thunk_dd_delete_surface_object,
    ),
    ("OsThunkDdDestroyD3DBuffer", os_thunk_dd_destroy_d3d_buffer),
    ("OsThunkDdDestroyMoComp", os_thunk_dd_destroy_mo_comp),
    ("OsThunkDdDestroySurface", os_thunk_dd_destroy_surface),
    ("OsThunkDdEndMoCompFrame", os_thunk_dd_end_mo_comp_frame),
    ("OsThunkDdFlip", os_thunk_dd_flip),
    ("OsThunkDdFlipToGDISurface", os_thunk_dd_flip_to_gdi_surface),
    (
        "OsThunkDdGetAvailDriverMemory",
        os_thunk_dd_get_avail_driver_memory,
    ),
    ("OsThunkDdGetBltStatus", os_thunk_dd_get_blt_status),
    ("OsThunkDdGetDC", os_thunk_dd_get_dc),
    ("OsThunkDdGetDriverInfo", os_thunk_dd_get_driver_info),
    ("OsThunkDdGetDriverState", os_thunk_dd_get_driver_state),
    ("OsThunkDdGetDxHandle", os_thunk_dd_get_dx_handle),
    ("OsThunkDdGetFlipStatus", os_thunk_dd_get_flip_status),
    (
        "OsThunkDdGetInternalMoCompInfo",
        os_thunk_dd_get_internal_mo_comp_info,
    ),
    (
        "OsThunkDdGetMoCompBuffInfo",
        os_thunk_dd_get_mo_comp_buff_info,
    ),
    ("OsThunkDdGetMoCompFormats", os_thunk_dd_get_mo_comp_formats),
    ("OsThunkDdGetMoCompGuids", os_thunk_dd_get_mo_comp_guids),
    ("OsThunkDdGetScanLine", os_thunk_dd_get_scan_line),
    ("OsThunkDdLock", os_thunk_dd_lock),
    ("OsThunkDdLockD3D", os_thunk_dd_lock_d3d),
    (
        "OsThunkDdQueryDirectDrawObject",
        os_thunk_dd_query_direct_draw_object,
    ),
    (
        "OsThunkDdQueryMoCompStatus",
        os_thunk_dd_query_mo_comp_status,
    ),
    (
        "OsThunkDdReenableDirectDrawObject",
        os_thunk_dd_reenable_direct_draw_object,
    ),
    ("OsThunkDdReleaseDC", os_thunk_dd_release_dc),
    ("OsThunkDdRenderMoComp", os_thunk_dd_render_mo_comp),
    ("OsThunkDdResetVisrgn", os_thunk_dd_reset_visrgn),
    ("OsThunkDdSetColorKey", os_thunk_dd_set_color_key),
    ("OsThunkDdSetExclusiveMode", os_thunk_dd_set_exclusive_mode),
    ("OsThunkDdSetGammaRamp", os_thunk_dd_set_gamma_ramp),
    (
        "OsThunkDdSetOverlayPosition",
        os_thunk_dd_set_overlay_position,
    ),
    ("OsThunkDdUnattachSurface", os_thunk_dd_unattach_surface),
    ("OsThunkDdUnlock", os_thunk_dd_unlock),
    ("OsThunkDdUnlockD3D", os_thunk_dd_unlock_d3d),
    ("OsThunkDdUpdateOverlay", os_thunk_dd_update_overlay),
    (
        "OsThunkDdWaitForVerticalBlank",
        os_thunk_dd_wait_for_vertical_blank,
    ),
];

/// The stub record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, stub)| stub())
}
