//! Oracle-derived test vectors for the ported d3dcompiler_34 stub exception.
//!
//! Vectors were captured by compiling the verbatim body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/d3dcompiler_34/`, and running the harness `stub_oracle.c`. The shim
//! `RtlRaiseException` records the raised record and longjmps out, because the C body loops on
//! it. All 16 printed rows raised once, and ASan and UBSan reported nothing, so no row is
//! excluded.
//!
//! Only two of the ten spec names are `@ stub`. Rows 03-10 pass the other eight names through
//! the same stub body to show that the raised record does not depend on the name.

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

const ROWS: [Row; 16] = [
    // row 01 D3DDisassembleEffect
    (
        "d3dcompiler_34.dll",
        "D3DDisassembleEffect",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DDisassembleEffect",
    ),
    // row 02 DebugSetMute
    (
        "d3dcompiler_34.dll",
        "DebugSetMute",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "DebugSetMute",
    ),
    // row 03 D3DCompileFromMemory
    (
        "d3dcompiler_34.dll",
        "D3DCompileFromMemory",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DCompileFromMemory",
    ),
    // row 04 D3DDisassembleCode
    (
        "d3dcompiler_34.dll",
        "D3DDisassembleCode",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DDisassembleCode",
    ),
    // row 05 D3DGetCodeDebugInfo
    (
        "d3dcompiler_34.dll",
        "D3DGetCodeDebugInfo",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DGetCodeDebugInfo",
    ),
    // row 06 D3DGetInputAndOutputSignatureBlob
    (
        "d3dcompiler_34.dll",
        "D3DGetInputAndOutputSignatureBlob",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DGetInputAndOutputSignatureBlob",
    ),
    // row 07 D3DGetInputSignatureBlob
    (
        "d3dcompiler_34.dll",
        "D3DGetInputSignatureBlob",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DGetInputSignatureBlob",
    ),
    // row 08 D3DGetOutputSignatureBlob
    (
        "d3dcompiler_34.dll",
        "D3DGetOutputSignatureBlob",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DGetOutputSignatureBlob",
    ),
    // row 09 D3DPreprocessFromMemory
    (
        "d3dcompiler_34.dll",
        "D3DPreprocessFromMemory",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DPreprocessFromMemory",
    ),
    // row 10 D3DReflectCode
    (
        "d3dcompiler_34.dll",
        "D3DReflectCode",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DReflectCode",
    ),
    // row 11 empty function name
    (
        "d3dcompiler_34.dll",
        "",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
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
        "d3dcompiler_34.dll",
        "ThisExportNameIsDeliberatelyLongToCheckNoTruncationAtAllXXXXXX",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "ThisExportNameIsDeliberatelyLongToCheckNoTruncationAtAllXXXXXX",
    ),
    // row 16 D3DDisassembleEffect again
    (
        "d3dcompiler_34.dll",
        "D3DDisassembleEffect",
        0x8000_0100,
        0x1,
        2,
        "d3dcompiler_34.dll",
        "D3DDisassembleEffect",
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
    // row 01 and row 02: the two @ stub exports of the spec.
    assert_eq!(
        d3d_disassemble_effect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_34.dll",
            function: "D3DDisassembleEffect",
        }
    );
    assert_eq!(
        debug_set_mute(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_34.dll",
            function: "DebugSetMute",
        }
    );
}
