//! Edge-case tests for the ported updspapi logic.
//!
//! These cover the argument handling of `UpdSpSetDynamicStringA`, the stub record, and the export
//! table. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn set_dynamic_string_a_ignores_embedded_nul_bytes() {
    // Not oracle-derived: the C body never reads its string arguments, so bytes that would end a C
    // string early make no difference. The return value stays 1.
    assert_eq!(
        upd_sp_set_dynamic_string_a(0, Some(b"a\0b"), Some(&[0, 0])),
        1
    );
    assert_eq!(upd_sp_set_dynamic_string_a(usize::MAX, Some(&[0]), None), 1);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of updspapi.spec. The code and
    // flags are the same for any names.
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
fn stub_helpers_match_the_stub_exports_in_the_table() {
    // Not oracle-derived: every `@ stub` line of updspapi.spec has one helper, and the helper's
    // function name is the export name.
    let helpers = [
        ("UpdSpSetDynamicStringExA", upd_sp_set_dynamic_string_ex_a()),
        ("UpdSpSetDynamicStringExW", upd_sp_set_dynamic_string_ex_w()),
        ("UpdSpSetDynamicStringW", upd_sp_set_dynamic_string_w()),
        ("UpdSpStringTableEnum", upd_sp_string_table_enum()),
    ];
    for (name, record) in helpers {
        assert_eq!(record.module, "updspapi.dll");
        assert_eq!(record.function, name);
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, helpers.len());
}

#[test]
fn export_table_matches_spec_counts() {
    // Not oracle-derived: the 78 export lines of dlls/updspapi/updspapi.spec, of which one has no
    // forward target, four are `@ stub`, and the other 73 forward to setupapi.
    assert_eq!(EXPORTS.len(), 78);
    let implemented = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Implemented)
        .count();
    let forwards = EXPORTS
        .iter()
        .filter(|(_, kind)| matches!(kind, ExportKind::Forward(_)))
        .count();
    assert_eq!(implemented, 1);
    assert_eq!(forwards, 73);
    for (_, kind) in EXPORTS.iter() {
        if let ExportKind::Forward(target) = kind {
            assert!(target.starts_with("setupapi."), "{target}");
        }
    }
}

#[test]
fn export_kind_lookups() {
    // Not oracle-derived: spot checks of the spec lines for each kind, and of names the spec does
    // not export. Names are case-sensitive.
    assert_eq!(
        export_kind("UpdSpSetDynamicStringA"),
        Some(ExportKind::Implemented)
    );
    assert_eq!(
        export_kind("UpdSpSetDynamicStringExA"),
        Some(ExportKind::Stub)
    );
    assert_eq!(
        export_kind("UpdSpCloseFileQueue"),
        Some(ExportKind::Forward("setupapi.SetupCloseFileQueue"))
    );
    assert_eq!(
        export_kind("UpdSpTermDefaultQueueCallback"),
        Some(ExportKind::Forward(
            "setupapi.SetupTermDefaultQueueCallback"
        ))
    );
    assert_eq!(export_kind("updspsetdynamicstringa"), None);
    assert_eq!(export_kind("SetupCloseFileQueue"), None);
    assert_eq!(export_kind(""), None);
}
