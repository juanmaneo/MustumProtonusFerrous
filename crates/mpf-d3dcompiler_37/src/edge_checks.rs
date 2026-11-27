//! Edge-case tests for the d3dcompiler_37 export table and stub records.
//!
//! The DLL has no code of its own to exercise beyond the stub records, so these tests check the
//! table, the lookups, and the record shape. Not every expectation is oracle-derived; the ones
//! that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of d3dcompiler_37.spec. The code
    // and flags are the same for any names.
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
    // Not oracle-derived: the module from MODULE in dlls/d3dcompiler_37/Makefile.in, and each
    // helper's function name from the `@ stub` line in d3dcompiler_37.spec.
    for (record, function) in [
        (debug_set_mute(), "DebugSetMute"),
        (d3d_disassemble_effect(), "D3DDisassembleEffect"),
    ] {
        assert_eq!(record.module, "d3dcompiler_37.dll");
        assert_eq!(record.function, function);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(record.number_parameters, 2);
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the ten export lines of dlls/d3dcompiler_37/d3dcompiler_37.spec, in
    // order, and the C function each `@ stdcall` line names after its `)`.
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
                ExportKind::Stdcall("D3DPreprocess"),
            ),
            ("D3DReflectCode", ExportKind::Stdcall("D3DReflect")),
            ("DebugSetMute", ExportKind::Stub),
        ]
    );
}

#[test]
fn export_kind_finds_every_export_and_rejects_others() {
    // Not oracle-derived: lookups that hit and miss the table. Names are case-sensitive, and an
    // empty name, a trailing space, or a C function name that the spec does not export is a miss.
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
    assert_eq!(export_kind("d3dreflectcode"), None);
    assert_eq!(export_kind("D3DReflect"), None);
    assert_eq!(export_kind("DebugSetMute "), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn export_stub_returns_a_record_only_for_stub_exports() {
    // Not oracle-derived: the two `@ stub` exports give a record with their own name. The
    // `stdcall` exports and names the spec does not export give None, never a plain value.
    for (name, kind) in EXPORTS {
        match kind {
            ExportKind::Stub => {
                let record = export_stub(name).expect("every stub export gives a record");
                assert_eq!(record.function, name);
                assert_eq!(record.module, "d3dcompiler_37.dll");
            }
            ExportKind::Stdcall(_) => assert_eq!(export_stub(name), None),
        }
    }
    assert_eq!(export_stub("D3DCompile"), None);
    assert_eq!(export_stub("d3ddisassembleeffect"), None);
    assert_eq!(export_stub(""), None);
}
