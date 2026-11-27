//! Oracle-derived test vectors for the ported d3dcompiler_42 logic.
//!
//! Vectors were printed by `target/canon/d3dcompiler_42/harness.c` (saved as `vectors.txt`),
//! compiled with `gcc -fsanitize=address,undefined -g`. The harness reads
//! `d3dcompiler_42.spec` and `Makefile.in`, and calls the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` (lines 504 to 516,
//! extracted into `stub_verbatim.inc`). Its `RtlRaiseException` records the raised record and
//! longjmps out, because the stub loops forever. The 16 rows are: 13 spec exports (rows 01 to
//! 13) and 3 variant inputs (rows 14 to 16). ASan and UBSan reported nothing, so no vector is
//! excluded.

use super::*;

#[test]
fn oracle_export_rows() {
    // row 01 export=D3DAssemble kind=private forward=-
    assert_eq!(export_kind("D3DAssemble"), Some(ExportKind::Private));
    // row 03 export=D3DCompile kind=stdcall forward=-
    assert_eq!(export_kind("D3DCompile"), Some(ExportKind::Stdcall));
    // row 05 export=D3DDisassemble kind=stdcall forward=-
    assert_eq!(export_kind("D3DDisassemble"), Some(ExportKind::Stdcall));
    // row 06 export=D3DGetDebugInfo kind=stdcall forward=-
    assert_eq!(export_kind("D3DGetDebugInfo"), Some(ExportKind::Stdcall));
    // row 07 export=D3DGetInputAndOutputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetInputAndOutputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 08 export=D3DGetInputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetInputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 09 export=D3DGetOutputSignatureBlob kind=stdcall forward=-
    assert_eq!(
        export_kind("D3DGetOutputSignatureBlob"),
        Some(ExportKind::Stdcall)
    );
    // row 10 export=D3DPreprocess kind=stdcall forward=-
    assert_eq!(export_kind("D3DPreprocess"), Some(ExportKind::Stdcall));
    // row 11 export=D3DReflect kind=stdcall forward=-
    assert_eq!(export_kind("D3DReflect"), Some(ExportKind::Stdcall));
    // row 13 export=D3DStripShader kind=stdcall forward=-
    assert_eq!(export_kind("D3DStripShader"), Some(ExportKind::Stdcall));
}

#[test]
fn oracle_stub_rows() {
    // row 02 export=DebugSetMute kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_42.dll p1=DebugSetMute raises=1
    assert_eq!(
        debug_set_mute(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_42.dll",
            function: "DebugSetMute",
        }
    );
    // row 04 export=D3DDisassemble10Effect kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_42.dll p1=D3DDisassemble10Effect raises=1
    assert_eq!(
        d3d_disassemble10_effect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_42.dll",
            function: "D3DDisassemble10Effect",
        }
    );
    // row 12 export=D3DReturnFailure1 kind=stub -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_42.dll p1=D3DReturnFailure1 raises=1
    assert_eq!(
        d3d_return_failure1(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_42.dll",
            function: "D3DReturnFailure1",
        }
    );
}

#[test]
fn oracle_stub_variant_rows() {
    // row 14 variant module=d3dcompiler_42.dll function="" -> code=0x80000100 flags=0x1 nargs=2 p0=d3dcompiler_42.dll p1= raises=1
    assert_eq!(
        wine_spec_unimplemented_stub("d3dcompiler_42.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "d3dcompiler_42.dll",
            function: "",
        }
    );
    // row 15 variant module="" function="" -> code=0x80000100 flags=0x1 nargs=2 p0= p1= raises=1
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
    // row 16 variant module=other.dll function=Foo -> code=0x80000100 flags=0x1 nargs=2 p0=other.dll p1=Foo raises=1
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
}
