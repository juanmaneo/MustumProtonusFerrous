//! Edge-case tests for the ported msisys.ocx logic.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the value from include/winerror.h. The harness used the same value.
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111u32 as i32);
}

#[test]
fn returned_hresult_is_failed() {
    // Not oracle-derived: the severity bit is set, so the value is negative and FAILED() is true.
    assert!(dll_get_class_object() < 0);
}

#[test]
fn spec_exports_match_msisys_ocx_spec() {
    // Not oracle-derived: the four lines of dlls/msisys.ocx/msisys.ocx.spec.
    assert_eq!(
        export_kind("DllCanUnloadNow"),
        Some(ExportKind::PrivateWithoutBody)
    );
    assert_eq!(
        export_kind("DllGetClassObject"),
        Some(ExportKind::Implemented)
    );
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::PrivateWithoutBody)
    );
    assert_eq!(
        export_kind("DllUnregisterServer"),
        Some(ExportKind::PrivateWithoutBody)
    );
}

#[test]
fn names_outside_the_spec_are_not_exports() {
    // Not oracle-derived: exact, case-sensitive matches only, with no trimming.
    for name in [
        "",
        "dllgetclassobject",
        "DllGetClassObject ",
        "DllGetClassObject2",
        "DllMain",
    ] {
        assert_eq!(export_kind(name), None, "{name:?}");
    }
}
