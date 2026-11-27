//! Edge-case tests for the ported d3dcompiler_42 logic.
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
    // Not oracle-derived: the thirteen export lines of dlls/d3dcompiler_42/d3dcompiler_42.spec,
    // in order.
    assert_eq!(EXPORTS.len(), 13);
    assert_eq!(
        EXPORTS,
        [
            ("D3DAssemble", ExportKind::Private),
            ("DebugSetMute", ExportKind::Stub),
            ("D3DCompile", ExportKind::Stdcall),
            ("D3DDisassemble10Effect", ExportKind::Stub),
            ("D3DDisassemble", ExportKind::Stdcall),
            ("D3DGetDebugInfo", ExportKind::Stdcall),
            ("D3DGetInputAndOutputSignatureBlob", ExportKind::Stdcall),
            ("D3DGetInputSignatureBlob", ExportKind::Stdcall),
            ("D3DGetOutputSignatureBlob", ExportKind::Stdcall),
            ("D3DPreprocess", ExportKind::Stdcall),
            ("D3DReflect", ExportKind::Stdcall),
            ("D3DReturnFailure1", ExportKind::Stub),
            ("D3DStripShader", ExportKind::Stdcall),
        ]
    );
}

#[test]
fn export_kind_rejects_unknown_names() {
    // Not oracle-derived: lookups outside the spec. Names are case-sensitive.
    assert_eq!(export_kind("d3dcompile"), None);
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind("D3DCompileFromMemory"), None);
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
        [
            "DebugSetMute",
            "D3DDisassemble10Effect",
            "D3DReturnFailure1"
        ]
    );
    for (name, record) in [
        ("DebugSetMute", debug_set_mute()),
        ("D3DDisassemble10Effect", d3d_disassemble10_effect()),
        ("D3DReturnFailure1", d3d_return_failure1()),
    ] {
        assert_eq!(record.function, name);
        assert_eq!(record.module, "d3dcompiler_42.dll");
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
