//! Oracle-derived test vectors for the ported d3d8thk export table.
//!
//! The rows come from the harness in `target/canon/d3d8thk/`. It compiles the verbatim body of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c` lines 504-516, Wine 9f8c4ae) with
//! `gcc -fsanitize=address,undefined -g`, and calls it once for each of the 56 `@ stub` names in
//! `dlls/d3d8thk/d3d8thk.spec`. The harness replaces `RtlRaiseException` with a recorder that
//! longjmps out, because the C loop never returns. ASan and UBSan reported nothing, so no row is
//! excluded.
//!
//! The brief's 15-25 rows do not apply here: a stub has one row per export, so there are 56.

use super::*;

/// `(code, flags, number_parameters, module, function)` for each export, in spec order.
const ORACLE_ROWS: [(u32, u32, u32, &str, &str); 56] = [
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkD3dContextCreate",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkD3dContextDestroy",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkD3dContextDestroyAll",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkD3dDrawPrimitives2",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkD3dValidateTextureStageState",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdAddAttachedSurface",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdAlphaBlt"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdAttachSurface",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdBeginMoCompFrame",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdBlt"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCanCreateD3DBuffer",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCanCreateSurface",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdColorControl"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCreateD3DBuffer",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCreateDirectDrawObject",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdCreateMoComp"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCreateSurface",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCreateSurfaceEx",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdCreateSurfaceObject",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdDeleteDirectDrawObject",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdDeleteSurfaceObject",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdDestroyD3DBuffer",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdDestroyMoComp",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdDestroySurface",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdEndMoCompFrame",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdFlip"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdFlipToGDISurface",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetAvailDriverMemory",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdGetBltStatus"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdGetDC"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetDriverInfo",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetDriverState",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdGetDxHandle"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetFlipStatus",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetInternalMoCompInfo",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetMoCompBuffInfo",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetMoCompFormats",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdGetMoCompGuids",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdGetScanLine"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdLock"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdLockD3D"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdQueryDirectDrawObject",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdQueryMoCompStatus",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdReenableDirectDrawObject",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdReleaseDC"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdRenderMoComp"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdResetVisrgn"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdSetColorKey"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdSetExclusiveMode",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdSetGammaRamp"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdSetOverlayPosition",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdUnattachSurface",
    ),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdUnlock"),
    (0x8000_0100, 0x01, 2, "d3d8thk.dll", "OsThunkDdUnlockD3D"),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdUpdateOverlay",
    ),
    (
        0x8000_0100,
        0x01,
        2,
        "d3d8thk.dll",
        "OsThunkDdWaitForVerticalBlank",
    ),
];

#[test]
fn oracle_stub_rows_match_export_table() {
    for (&(code, flags, number_parameters, module, function), &(name, _)) in
        ORACLE_ROWS.iter().zip(EXPORTS.iter())
    {
        assert_eq!(name, function);
        assert_eq!(
            export_stub(function),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            })
        );
    }
}
