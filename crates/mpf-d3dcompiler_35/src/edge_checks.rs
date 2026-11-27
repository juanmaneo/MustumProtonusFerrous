//! Edge-case tests for the ported d3dcompiler_35 logic.
//!
//! These cover the version boundary of the size gate, the export table and its lookups, and the
//! stub record. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h, include/winnt.h and
    // dlls/d3dcompiler_35/Makefile.in.
    assert_eq!(D3D_COMPILER_VERSION, 35);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn size_gate_switches_on_at_version_46() {
    // Not oracle-derived: the C `#if D3D_COMPILER_VERSION >= 46` boundary. The oracle only built
    // versions 35, 46 and 0, so 45 and 47 are checked here by the same rule.
    assert_eq!(disassemble_precheck(45, 0), None);
    assert_eq!(disassemble_precheck(47, 0), Some(E_INVALIDARG));
    assert_eq!(disassemble_precheck(D3D_COMPILER_VERSION, 0), None);
    assert_eq!(disassemble_precheck(0, 0), None);
}

#[test]
fn size_gate_rejects_only_the_empty_input() {
    // Not oracle-derived: at version 46 and above, only size 0 fails, and one byte is enough.
    assert_eq!(disassemble_precheck(46, 1), None);
    assert_eq!(disassemble_precheck(u32::MAX, usize::MAX), None);
    assert_eq!(disassemble_precheck(u32::MAX, 0), Some(E_INVALIDARG));
}

#[test]
fn reflect_forward_uses_the_version_for_every_nonzero_build() {
    // Not oracle-derived: the C `#if !D3D_COMPILER_VERSION` gives D3DReflect for every nonzero
    // version, and it forwards that version unchanged.
    for version in [1, 34, 35, 43, 47, u32::MAX] {
        assert_eq!(
            reflect_forward(version),
            ReflectForward {
                entry: "D3DReflect",
                vkd3d_version: version,
            }
        );
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the two fixed names of d3dcompiler_35.spec.
    // The code, flags and count are the same for any names.
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
fn export_table_matches_spec_order() {
    // Not oracle-derived: the ten lines of dlls/d3dcompiler_35/d3dcompiler_35.spec, in order.
    // The Stdcall payload is the C function that implements the export, from the spec's
    // `name(args) impl` form, or the export name when the form has no `impl`.
    assert_eq!(
        EXPORTS,
        [
            ("D3DCompileFromMemory", ExportKind::Stdcall("D3DCompile")),
            ("D3DDisassembleCode", ExportKind::Stdcall("D3DDisassemble")),
            ("D3DDisassembleEffect", ExportKind::Stub),
            (
                "D3DGetCodeDebugInfo",
                ExportKind::Stdcall("D3DGetDebugInfo")
            ),
            (
                "D3DGetInputAndOutputSignatureBlob",
                ExportKind::Stdcall("D3DGetInputAndOutputSignatureBlob"),
            ),
            (
                "D3DGetInputSignatureBlob",
                ExportKind::Stdcall("D3DGetInputSignatureBlob"),
            ),
            (
                "D3DGetOutputSignatureBlob",
                ExportKind::Stdcall("D3DGetOutputSignatureBlob"),
            ),
            (
                "D3DPreprocessFromMemory",
                ExportKind::Stdcall("D3DPreprocess")
            ),
            ("D3DReflectCode", ExportKind::Stdcall("D3DReflect")),
            ("DebugSetMute", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_looks_names_up_case_sensitively() {
    // Not oracle-derived: lookups over the table above. Names are case-sensitive, and names the
    // spec does not export give None.
    assert_eq!(
        export_kind("D3DCompileFromMemory"),
        Some(ExportKind::Stdcall("D3DCompile"))
    );
    assert_eq!(export_kind("D3DDisassembleEffect"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DebugSetMute"), Some(ExportKind::Stub));
    assert_eq!(export_kind("D3DCompile"), None);
    assert_eq!(export_kind("debugsetmute"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_helpers_match_their_spec_names() {
    // Not oracle-derived: the two stub helpers must raise the records for the two `@ stub` names
    // in EXPORTS. The oracle rows cover the field values; this checks the names against the table.
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(stubs, ["D3DDisassembleEffect", "DebugSetMute"]);
    assert_eq!(d3d_disassemble_effect().function, stubs[0]);
    assert_eq!(debug_set_mute().function, stubs[1]);
    assert_eq!(d3d_disassemble_effect().module, "d3dcompiler_35.dll");
    assert_eq!(debug_set_mute().module, "d3dcompiler_35.dll");
}
