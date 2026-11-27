//! Edge-case tests for the ported `samlib.spec` logic.
//!
//! These cover the constants, the stub record, the export table and the lookup. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_record_passes_names_through_unchanged() {
    // Not oracle-derived: the code, flags and parameter count are the same for any names.
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
fn helpers_build_the_same_record_as_the_wrapper() {
    // Not oracle-derived: each helper is wine_spec_unimplemented_stub with the module
    // samlib.dll and its own export name.
    assert_eq!(
        sam_connect(),
        wine_spec_unimplemented_stub("samlib.dll", "SamConnect")
    );
    assert_eq!(
        sami_set_dsrm_password_owf(),
        wine_spec_unimplemented_stub("samlib.dll", "SamiSetDSRMPasswordOWF")
    );
}

#[test]
fn export_table_lists_every_spec_export_as_stub() {
    // Not oracle-derived: the 65 lines of dlls/samlib/samlib.spec, all `@ stub`, in order.
    assert_eq!(EXPORTS.len(), 65);
    assert!(EXPORTS.iter().all(|&(_, kind)| kind == ExportKind::Stub));
    assert_eq!(EXPORTS[0], ("SamAddMemberToAlias", ExportKind::Stub));
    assert_eq!(EXPORTS[64], ("SamiSetDSRMPasswordOWF", ExportKind::Stub));
    assert_eq!(export_kind("SamConnect"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("SamiSetDSRMPasswordOWF"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("samconnect"), None);
    assert_eq!(export_kind("SamConnect2"), None);
    assert_eq!(export_kind(""), None);
}
