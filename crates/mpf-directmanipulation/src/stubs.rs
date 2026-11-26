//! Return codes of the exports that only log a FIXME in Wine. Each export's Wine body is either
//! `return E_NOTIMPL;` or, for four of them, `return S_OK;`.

use crate::hresult::{HResult, E_NOTIMPL, S_OK};

/// Export name (the C function name) and its return value in `directmanipulation.c`.
const STUBS: [(&str, HResult); 61] = [
    ("update_manager_RegisterWaitHandleCallback", E_NOTIMPL),
    ("update_manager_UnregisterWaitHandleCallback", E_NOTIMPL),
    ("update_manager_Update", E_NOTIMPL),
    ("primary_SetSnapInterval", E_NOTIMPL),
    ("primary_SetSnapPoints", E_NOTIMPL),
    ("primary_SetSnapType", E_NOTIMPL),
    ("primary_SetSnapCoordinate", E_NOTIMPL),
    ("primary_SetZoomBoundaries", E_NOTIMPL),
    ("primary_SetHorizontalAlignment", E_NOTIMPL),
    ("primary_SetVerticalAlignment", E_NOTIMPL),
    ("primary_GetInertiaEndTransform", E_NOTIMPL),
    ("primary_GetCenterPoint", E_NOTIMPL),
    ("content_GetContentRect", E_NOTIMPL),
    ("content_SetContentRect", S_OK),
    ("content_GetViewport", E_NOTIMPL),
    ("content_GetTag", E_NOTIMPL),
    ("content_SetTag", E_NOTIMPL),
    ("content_GetOutputTransform", E_NOTIMPL),
    ("content_GetContentTransform", E_NOTIMPL),
    ("content_SyncContentTransform", E_NOTIMPL),
    ("viewport_Enable", E_NOTIMPL),
    ("viewport_Disable", E_NOTIMPL),
    ("viewport_SetContact", E_NOTIMPL),
    ("viewport_ReleaseContact", E_NOTIMPL),
    ("viewport_ReleaseAllContacts", E_NOTIMPL),
    ("viewport_GetStatus", E_NOTIMPL),
    ("viewport_GetTag", E_NOTIMPL),
    ("viewport_SetTag", E_NOTIMPL),
    ("viewport_GetViewportRect", E_NOTIMPL),
    ("viewport_SetViewportRect", S_OK),
    ("viewport_ZoomToRect", E_NOTIMPL),
    ("viewport_SetViewportTransform", E_NOTIMPL),
    ("viewport_SyncDisplayTransform", E_NOTIMPL),
    ("viewport_AddContent", E_NOTIMPL),
    ("viewport_RemoveContent", E_NOTIMPL),
    ("viewport_SetViewportOptions", S_OK),
    ("viewport_AddConfiguration", E_NOTIMPL),
    ("viewport_RemoveConfiguration", E_NOTIMPL),
    ("viewport_ActivateConfiguration", S_OK),
    ("viewport_SetManualGesture", E_NOTIMPL),
    ("viewport_SetChaining", E_NOTIMPL),
    ("viewport_AddEventHandler", E_NOTIMPL),
    ("viewport_RemoveEventHandler", E_NOTIMPL),
    ("viewport_SetInputMode", E_NOTIMPL),
    ("viewport_SetUpdateMode", E_NOTIMPL),
    ("viewport_Stop", E_NOTIMPL),
    ("viewport_Abandon", E_NOTIMPL),
    ("viewport_AddBehavior", E_NOTIMPL),
    ("viewport_RemoveBehavior", E_NOTIMPL),
    ("viewport_RemoveAllBehaviors", E_NOTIMPL),
    ("direct_manip_Activate", E_NOTIMPL),
    ("direct_manip_Deactivate", E_NOTIMPL),
    ("direct_manip_RegisterHitTestTarget", E_NOTIMPL),
    ("direct_manip_ProcessInput", E_NOTIMPL),
    ("direct_manip_CreateContent", E_NOTIMPL),
    ("direct_manip_CreateBehavior", E_NOTIMPL),
    ("compositor_AddContent", E_NOTIMPL),
    ("compositor_RemoveContent", E_NOTIMPL),
    ("compositor_Flush", E_NOTIMPL),
    ("compositor_AddContentWithCrossProcessChaining", E_NOTIMPL),
    ("provider_GetNextFrameInfo", E_NOTIMPL),
];

/// Return value of a stub export, or `None` for a name that is not a stub.
pub fn stub_return(export: &str) -> Option<HResult> {
    STUBS
        .iter()
        .find(|(name, _)| *name == export)
        .map(|&(_, hr)| hr)
}
