//! Edge-case tests for the ported d3dcompiler_47 logic.
//!
//! These cover the constants, the export table and its lookups, and the stub helpers. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 29 lines of dlls/d3dcompiler_47/d3dcompiler_47.spec, in order.
    // A line without a target maps the name to itself. `-private` gives Private, and `@ stub`
    // gives Stub.
    use ExportKind::{Private, Stdcall, Stub};
    assert_eq!(
        EXPORTS,
        [
            ("D3DAssemble", Private),
            ("D3DCompile", Stdcall),
            ("D3DCompile2", Stdcall),
            ("D3DCompileFromFile", Stdcall),
            ("D3DCompressShaders", Stub),
            ("D3DCreateBlob", Stdcall),
            ("D3DCreateFunctionLinkingGraph", Stdcall),
            ("D3DCreateLinker", Stdcall),
            ("D3DDecompressShaders", Stub),
            ("D3DDisassemble", Stdcall),
            ("D3DDisassemble10Effect", Stub),
            ("D3DDisassemble11Trace", Stub),
            ("D3DDisassembleRegion", Stub),
            ("D3DGetBlobPart", Stdcall),
            ("D3DGetDebugInfo", Stdcall),
            ("D3DGetInputAndOutputSignatureBlob", Stdcall),
            ("D3DGetInputSignatureBlob", Stdcall),
            ("D3DGetOutputSignatureBlob", Stdcall),
            ("D3DGetTraceInstructionOffsets", Stub),
            ("D3DLoadModule", Stdcall),
            ("D3DPreprocess", Stdcall),
            ("D3DReadFileToBlob", Stdcall),
            ("D3DReflect", Stdcall),
            ("D3DReflectLibrary", Stub),
            ("D3DReturnFailure1", Stub),
            ("D3DSetBlobPart", Stub),
            ("D3DStripShader", Stdcall),
            ("D3DWriteBlobToFile", Stdcall),
            ("DebugSetMute", Stub),
        ]
    );
}

#[test]
fn export_kind_follows_the_table() {
    // Not oracle-derived: lookups over the table above. Names are case-sensitive, and a name the
    // spec does not declare gives None.
    assert_eq!(export_kind("D3DAssemble"), Some(ExportKind::Private));
    assert_eq!(export_kind("D3DCompile2"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("D3DCompressShaders"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DebugSetMute"), Some(ExportKind::Stub));
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind("D3DCompile2VKD3D"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the code and flags are the same for any names, and the two names are
    // passed through without change. The oracle harness only passed the fixed spec names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_stub_only_answers_for_stub_exports() {
    // Not oracle-derived: `export_stub` returns a record only for `@ stub` lines. Stdcall and
    // private exports, and names the spec does not declare, give None.
    for (name, kind) in EXPORTS {
        let record = export_stub(name);
        if kind == ExportKind::Stub {
            let record = record.expect("every @ stub export raises a record");
            assert_eq!(record.module, "d3dcompiler_47.dll");
            assert_eq!(record.function, name);
            assert_eq!(record.code, EXCEPTION_WINE_STUB);
        } else {
            assert_eq!(record, None, "{name} is not a stub export");
        }
    }
    assert_eq!(export_stub("D3DCompile2VKD3D"), None);
    assert_eq!(export_stub(""), None);
}
