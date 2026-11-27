//! Edge-case tests for the ported d3dcompiler_38 logic.
//!
//! These cover the export table boundaries and the stub helpers. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the eleven export lines of dlls/d3dcompiler_38/d3dcompiler_38.spec,
    // in order. The forward targets are the words after the closing parenthesis.
    assert_eq!(EXPORTS.len(), 11);
    assert_eq!(
        EXPORTS,
        [
            ("D3DCompileFromMemory", ExportKind::Forward("D3DCompile")),
            ("D3DDisassembleCode", ExportKind::Forward("D3DDisassemble")),
            ("D3DDisassembleEffect", ExportKind::Stub),
            (
                "D3DGetCodeDebugInfo",
                ExportKind::Forward("D3DGetDebugInfo")
            ),
            ("D3DGetInputAndOutputSignatureBlob", ExportKind::Stdcall),
            ("D3DGetInputSignatureBlob", ExportKind::Stdcall),
            ("D3DGetOutputSignatureBlob", ExportKind::Stdcall),
            (
                "D3DPreprocessFromMemory",
                ExportKind::Forward("D3DPreprocess")
            ),
            ("D3DReflectCode", ExportKind::Forward("D3DReflect")),
            ("D3DReturnFailure1", ExportKind::Stub),
            ("DebugSetMute", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_rejects_unknown_names() {
    // Not oracle-derived: lookups outside the spec. Names are case-sensitive, and the forward
    // targets are not exports of this DLL.
    assert_eq!(export_kind("d3dcompile"), None);
    assert_eq!(export_kind("D3DCompile"), None);
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_exports_match_the_stub_table() {
    // Not oracle-derived: every stub entry in EXPORTS has a helper that raises the same record
    // as the stub body with its own name. The oracle covers the three names; this checks that
    // the table and the helpers name the same exports.
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        stubs,
        ["D3DDisassembleEffect", "D3DReturnFailure1", "DebugSetMute"]
    );
    for (name, record) in [
        ("D3DDisassembleEffect", d3d_disassemble_effect()),
        ("D3DReturnFailure1", d3d_return_failure1()),
        ("DebugSetMute", debug_set_mute()),
    ] {
        assert_eq!(record.function, name);
        assert_eq!(record.module, "d3dcompiler_38.dll");
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
    }
}

#[test]
fn stub_record_is_never_a_return_value() {
    // Not oracle-derived: the stub body loops in `for (;;)`, so a stub export has no return
    // value to model. The record is the non-continuable stub exception, never a zero code.
    let record = debug_set_mute();
    assert_ne!(record.code, 0);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
}
