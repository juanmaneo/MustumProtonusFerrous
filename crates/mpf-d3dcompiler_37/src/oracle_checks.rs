//! Oracle-derived test vectors for the d3dcompiler_37 export stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g`. The harness is
//! `target/canon/d3dcompiler_37/oracle.c`. Its `RtlRaiseException` shim records the record and
//! longjmps out, because the verbatim `for (;;)` loop would call a returning recorder forever.
//! All 18 rows ran clean under ASan and UBSan, so no vector is excluded.
//!
//! Rows 1 and 2 are the two `@ stub` exports of `dlls/d3dcompiler_37/d3dcompiler_37.spec`. Rows 3
//! to 18 call the generic function with other inputs, to check that the two strings pass through.
//! In every printed row the `p0` and `p1` fields equal the module and function arguments, so the
//! vectors below omit them.

use super::*;

/// The 18 rows of the oracle output: the module and function the stub was called with, then the
/// `StubException` fields it raised (code, flags, number of parameters).
const ORACLE_ROWS: [(&str, &str, u32, u32, u32); 18] = [
    ("d3dcompiler_37.dll", "DebugSetMute", 0x8000_0100, 0x01, 2),
    (
        "d3dcompiler_37.dll",
        "D3DDisassembleEffect",
        0x8000_0100,
        0x01,
        2,
    ),
    ("d3dcompiler_37.dll", "", 0x8000_0100, 0x01, 2),
    ("", "", 0x8000_0100, 0x01, 2),
    ("", "DebugSetMute", 0x8000_0100, 0x01, 2),
    ("d3dcompiler_43.dll", "DebugSetMute", 0x8000_0100, 0x01, 2),
    ("D3DCOMPILER_37.DLL", "debugsetmute", 0x8000_0100, 0x01, 2),
    (
        "d3dcompiler_37.dll",
        "D3DDisassembleEffect ",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DCompileFromMemory",
        0x8000_0100,
        0x01,
        2,
    ),
    ("d3dcompiler_37.dll", "D3DCompile", 0x8000_0100, 0x01, 2),
    (
        "d3dcompiler_37.dll",
        "D3DDisassembleCode",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DGetCodeDebugInfo",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DGetInputAndOutputSignatureBlob",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DGetInputSignatureBlob",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DGetOutputSignatureBlob",
        0x8000_0100,
        0x01,
        2,
    ),
    (
        "d3dcompiler_37.dll",
        "D3DPreprocessFromMemory",
        0x8000_0100,
        0x01,
        2,
    ),
    ("d3dcompiler_37.dll", "D3DReflectCode", 0x8000_0100, 0x01, 2),
    ("a.dll", "x", 0x8000_0100, 0x01, 2),
];

#[test]
fn oracle_stub_rows() {
    for (module, function, code, flags, number_parameters) in ORACLE_ROWS {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_export_stub_rows() {
    // Rows 1 and 2 are the two `@ stub` exports, called the way winebuild calls them.
    for (module, function, code, flags, number_parameters) in &ORACLE_ROWS[..2] {
        let record = export_stub(function).expect("row is a stub export");
        assert_eq!(record.module, *module);
        assert_eq!(record.function, *function);
        assert_eq!(record.code, *code);
        assert_eq!(record.flags, *flags);
        assert_eq!(record.number_parameters, *number_parameters);
    }
}
