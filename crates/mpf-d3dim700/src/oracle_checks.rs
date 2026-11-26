//! Oracle-derived test vectors for the d3dim700 export stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g`, and calling it once per
//! export of `dlls/d3dim700/d3dim700.spec` with the module name from `dlls/d3dim700/Makefile.in`.
//! The harness is `target/canon/d3dim700/oracle.c`. Its `RtlRaiseException` shim records the
//! record and longjmps out, because the verbatim `for (;;)` loop would call a returning recorder
//! forever. All 18 rows ran clean under ASan and UBSan, so no vector is excluded.

use super::*;

/// The 18 rows of the oracle output, in spec order: the export name the stub was called with, then
/// the `StubException` fields it raised (code, flags, number of parameters, module, function).
const ORACLE_ROWS: [(&str, u32, u32, u32, &str, &str); 18] = [
    ("D3DFree", 0x8000_0100, 0x01, 2, "d3dim700.dll", "D3DFree"),
    (
        "D3DMalloc",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "D3DMalloc",
    ),
    (
        "D3DRealloc",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "D3DRealloc",
    ),
    (
        "Direct3DCreateDevice",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "Direct3DCreateDevice",
    ),
    (
        "CreateTexture",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "CreateTexture",
    ),
    (
        "D3DBreakVBLock",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "D3DBreakVBLock",
    ),
    (
        "D3DTextureUpdate",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "D3DTextureUpdate",
    ),
    (
        "DestroyTexture",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "DestroyTexture",
    ),
    (
        "Direct3DCreate",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "Direct3DCreate",
    ),
    (
        "Direct3D_HALCleanUp",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "Direct3D_HALCleanUp",
    ),
    (
        "FlushD3DDevices",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "FlushD3DDevices",
    ),
    ("GetLOD", 0x8000_0100, 0x01, 2, "d3dim700.dll", "GetLOD"),
    (
        "GetPriority",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "GetPriority",
    ),
    (
        "PaletteAssociateNotify",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "PaletteAssociateNotify",
    ),
    (
        "PaletteUpdateNotify",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "PaletteUpdateNotify",
    ),
    ("SetLOD", 0x8000_0100, 0x01, 2, "d3dim700.dll", "SetLOD"),
    (
        "SetPriority",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "SetPriority",
    ),
    (
        "SurfaceFlipNotify",
        0x8000_0100,
        0x01,
        2,
        "d3dim700.dll",
        "SurfaceFlipNotify",
    ),
];

#[test]
fn oracle_stub_export_rows() {
    for (export, code, flags, number_parameters, module, function) in ORACLE_ROWS {
        assert_eq!(
            export_stub(export),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }),
            "{export}"
        );
    }
}
