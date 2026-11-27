//! Oracle-derived test vectors for the ported d3dcompiler_47 stub exception.
//!
//! Vectors were captured by compiling the verbatim body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516, Wine tag `wine-11.22`) with
//! `gcc -std=gnu11 -fsanitize=address,undefined -g` under `target/canon/d3dcompiler_47/`, and
//! running the harness `harness.c`. The shim `RtlRaiseException` records the raised record and
//! longjmps out, because the C body loops on it. All 15 printed rows raised once, and ASan and
//! UBSan reported nothing, so no row is excluded.
//!
//! `dlls/d3dcompiler_47` has no local C sources (`PARENTSRC = ../d3dcompiler_43`), so the
//! harness covers only the stub body. Rows 01-10 are the ten `@ stub` names of the spec. Rows
//! 11-15 pass empty, other and long names through the same body.

use super::*;

/// One printed row: the inputs, then `code`, `flags`, `nparams`, `p0` and `p1` as the harness
/// printed them.
type Row = (
    &'static str,
    &'static str,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

const ROWS: [Row; 15] = [
    // row 01 D3DCompressShaders
    (
        "d3dcompiler_47.dll",
        "D3DCompressShaders",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DCompressShaders",
    ),
    // row 02 D3DDecompressShaders
    (
        "d3dcompiler_47.dll",
        "D3DDecompressShaders",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DDecompressShaders",
    ),
    // row 03 D3DDisassemble10Effect
    (
        "d3dcompiler_47.dll",
        "D3DDisassemble10Effect",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DDisassemble10Effect",
    ),
    // row 04 D3DDisassemble11Trace
    (
        "d3dcompiler_47.dll",
        "D3DDisassemble11Trace",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DDisassemble11Trace",
    ),
    // row 05 D3DDisassembleRegion
    (
        "d3dcompiler_47.dll",
        "D3DDisassembleRegion",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DDisassembleRegion",
    ),
    // row 06 D3DGetTraceInstructionOffsets
    (
        "d3dcompiler_47.dll",
        "D3DGetTraceInstructionOffsets",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DGetTraceInstructionOffsets",
    ),
    // row 07 D3DReflectLibrary
    (
        "d3dcompiler_47.dll",
        "D3DReflectLibrary",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DReflectLibrary",
    ),
    // row 08 D3DReturnFailure1
    (
        "d3dcompiler_47.dll",
        "D3DReturnFailure1",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DReturnFailure1",
    ),
    // row 09 D3DSetBlobPart
    (
        "d3dcompiler_47.dll",
        "D3DSetBlobPart",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "D3DSetBlobPart",
    ),
    // row 10 DebugSetMute
    (
        "d3dcompiler_47.dll",
        "DebugSetMute",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "DebugSetMute",
    ),
    // row 11 empty function name
    (
        "d3dcompiler_47.dll",
        "",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "",
    ),
    // row 12 empty module name
    ("", "DebugSetMute", 0x8000_0100, 0x1, 2, "", "DebugSetMute"),
    // row 13 empty module and function names
    ("", "", 0x8000_0100, 0x1, 2, "", ""),
    // row 14 another module name
    (
        "d3dcompiler_43.dll",
        "DebugSetMute",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_43.dll",
        "DebugSetMute",
    ),
    // row 15 a long function name
    (
        "d3dcompiler_47.dll",
        "ThisExportNameIsDeliberatelyLongToCheckNoTruncationAtAllXXXXXX",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_47.dll",
        "ThisExportNameIsDeliberatelyLongToCheckNoTruncationAtAllXXXXXX",
    ),
];

#[test]
fn oracle_stub_exception_rows() {
    for &(module, function, code, flags, number_parameters, p0, p1) in &ROWS {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module: p0,
                function: p1,
            },
            "oracle row for {module:?} {function:?}"
        );
    }
}

#[test]
fn oracle_stub_export_rows() {
    // rows 01-10: each @ stub export of the spec raises the record of the shared body with the
    // DLL's MODULE name, as the harness printed for the same function name.
    for &(_, function, code, flags, number_parameters, _, _) in &ROWS[..10] {
        assert_eq!(
            export_stub(function),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module: "d3dcompiler_47.dll",
                function,
            }),
            "oracle export row for {function:?}"
        );
    }
}
