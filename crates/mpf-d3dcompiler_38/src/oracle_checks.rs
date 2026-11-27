//! Oracle-derived test vectors for the ported d3dcompiler_38 logic.
//!
//! Vectors were printed by `target/canon/d3dcompiler_38/harness.c`, compiled with
//! `gcc -fsanitize=address,undefined -g`. The harness reads `d3dcompiler_38.spec` and
//! `Makefile.in`, and calls the verbatim `__wine_spec_unimplemented_stub` body from
//! `dlls/ntdll/exception.c`. Its `RtlRaiseException` records the raised record and longjmps out,
//! because the stub loops forever. The 15 rows are: 11 spec exports (rows 01 to 11, where the
//! `@ stdcall` rows come from the spec and the `@ stub` rows from the stub body) and 4 variant
//! inputs (rows 12 to 15). ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

#[test]
fn oracle_export_rows() {
    // row 01 export=D3DCompileFromMemory kind=stdcall forward=D3DCompile
    assert_eq!(
        export_kind("D3DCompileFromMemory"),
        Some(ExportKind::Forward("D3DCompile"))
    );
    // row 02 export=D3DDisassembleCode kind=stdcall forward=D3DDisassemble
    assert_eq!(
        export_kind("D3DDisassembleCode"),
        Some(ExportKind::Forward("D3DDisassemble"))
    );
    // row 03 export=D3DDisassembleEffect kind=stub (see oracle_stub_rows)
    assert_eq!(export_kind("D3DDisassembleEffect"), Some(ExportKind::Stub));
    // row 04 export=D3DGetCodeDebugInfo kind=stdcall forward=D3DGetDebugInfo
    assert_eq!(
        export_kind("D3DGetCodeDebugInfo"),
        Some(ExportKind::Forward("D3DGetDebugInfo"))
    );
    // row 05 export=D3DGetInputAndOutputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetInputAndOutputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 06 export=D3DGetInputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetInputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 07 export=D3DGetOutputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetOutputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 08 export=D3DPreprocessFromMemory kind=stdcall forward=D3DPreprocess
    assert_eq!(
        export_kind("D3DPreprocessFromMemory"),
        Some(ExportKind::Forward("D3DPreprocess"))
    );
    // row 09 export=D3DReflectCode kind=stdcall forward=D3DReflect
    assert_eq!(
        export_kind("D3DReflectCode"),
        Some(ExportKind::Forward("D3DReflect"))
    );
    // row 10 export=D3DReturnFailure1 kind=stub (see oracle_stub_rows)
    assert_eq!(export_kind("D3DReturnFailure1"), Some(ExportKind::Stub));
    // row 11 export=DebugSetMute kind=stub (see oracle_stub_rows)
    assert_eq!(export_kind("DebugSetMute"), Some(ExportKind::Stub));
}

#[test]
fn oracle_stub_rows() {
    // row 03 export=D3DDisassembleEffect kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_38.dll p1=D3DDisassembleEffect raises=1
    assert_eq!(
        d3d_disassemble_effect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_38.dll",
            function: "D3DDisassembleEffect",
        }
    );
    // row 10 export=D3DReturnFailure1 kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_38.dll p1=D3DReturnFailure1 raises=1
    assert_eq!(
        d3d_return_failure1(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_38.dll",
            function: "D3DReturnFailure1",
        }
    );
    // row 11 export=DebugSetMute kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_38.dll p1=DebugSetMute raises=1
    assert_eq!(
        debug_set_mute(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_38.dll",
            function: "DebugSetMute",
        }
    );
}

#[test]
fn oracle_stub_variant_rows() {
    // row 12 variant module=d3dcompiler_38.dll function="" -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_38.dll p1= raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("d3dcompiler_38.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_38.dll",
            function: "",
        }
    );
    // row 13 variant module="" function="" -> code=0x80000100 flags=0x1 nargs=2 p0= p1= raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );
    // row 14 variant module=other.dll function=Foo -> code=0x80000100 flags=0x1 nargs=2 p0=other.dll p1=Foo raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Foo"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );
    // row 15 variant module=d3dcompiler_43.dll function=D3DCompile -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_43.dll p1=D3DCompile raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("d3dcompiler_43.dll", "D3DCompile"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_43.dll",
            function: "D3DCompile",
        }
    );
}
