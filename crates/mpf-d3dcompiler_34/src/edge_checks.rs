//! Edge-case tests for the ported d3dcompiler_34 logic.
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
    // Not oracle-derived: the ten lines of dlls/d3dcompiler_34/d3dcompiler_34.spec, in order.
    // An `@ stdcall NAME(args) TARGET` line maps NAME to TARGET, and a line without a target
    // maps the name to itself.
    assert_eq!(
        EXPORTS,
        [
            ("D3DCompileFromMemory", ExportKind::Stdcall, "D3DCompile"),
            ("D3DDisassembleCode", ExportKind::Stdcall, "D3DDisassemble"),
            (
                "D3DDisassembleEffect",
                ExportKind::Stub,
                "D3DDisassembleEffect"
            ),
            (
                "D3DGetCodeDebugInfo",
                ExportKind::Stdcall,
                "D3DGetDebugInfo"
            ),
            (
                "D3DGetInputAndOutputSignatureBlob",
                ExportKind::Stdcall,
                "D3DGetInputAndOutputSignatureBlob"
            ),
            (
                "D3DGetInputSignatureBlob",
                ExportKind::Stdcall,
                "D3DGetInputSignatureBlob"
            ),
            (
                "D3DGetOutputSignatureBlob",
                ExportKind::Stdcall,
                "D3DGetOutputSignatureBlob"
            ),
            (
                "D3DPreprocessFromMemory",
                ExportKind::Stdcall,
                "D3DPreprocess"
            ),
            ("D3DReflectCode", ExportKind::Stdcall, "D3DReflect"),
            ("DebugSetMute", ExportKind::Stub, "DebugSetMute"),
        ]
    );
}

#[test]
fn export_lookups_follow_the_table() {
    // Not oracle-derived: lookups over the table above. Names are case-sensitive, and a name the
    // spec does not export gives None.
    assert_eq!(
        export_kind("D3DCompileFromMemory"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("DebugSetMute"), Some(ExportKind::Stub));
    assert_eq!(export_kind("D3DDisassembleEffect"), Some(ExportKind::Stub));
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind("D3DCompile"), None);
    assert_eq!(export_kind(""), None);

    assert_eq!(
        implementation_name("D3DCompileFromMemory"),
        Some("D3DCompile")
    );
    assert_eq!(implementation_name("D3DReflectCode"), Some("D3DReflect"));
    assert_eq!(implementation_name("DebugSetMute"), Some("DebugSetMute"));
    assert_eq!(implementation_name("D3DCompile"), None);
    assert_eq!(implementation_name(""), None);
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
fn stub_helpers_name_their_export() {
    // Not oracle-derived: the helpers must pass the spec's export name and the DLL's MODULE name
    // to the shared stub body. The oracle rows cover the same records.
    assert_eq!(
        d3d_disassemble_effect(),
        wine_spec_unimplemented_stub("d3dcompiler_34.dll", "D3DDisassembleEffect")
    );
    assert_eq!(
        debug_set_mute(),
        wine_spec_unimplemented_stub("d3dcompiler_34.dll", "DebugSetMute")
    );
    // Each stub export in the table has a helper with the same name.
    for (name, kind, _) in EXPORTS {
        if kind == ExportKind::Stub {
            assert_eq!(
                wine_spec_unimplemented_stub("d3dcompiler_34.dll", name).function,
                name
            );
        }
    }
}
