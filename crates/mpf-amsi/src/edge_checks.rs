//! Edge-case tests for the ported amsi logic.
//!
//! These cover the export table, the handle and result constants, out-parameter
//! overwrite behaviour, and the stub exception record. Expectations that are not
//! oracle-derived say so in a comment.

use super::*;

#[test]
fn export_table_matches_amsi_spec() {
    // Not oracle-derived: classification read from dlls/amsi/amsi.spec.
    for name in [
        "AmsiCloseSession",
        "AmsiInitialize",
        "AmsiOpenSession",
        "AmsiScanBuffer",
        "AmsiScanString",
        "AmsiUninitialize",
    ] {
        assert_eq!(export_kind(name), Some(ExportKind::Implemented));
    }
    for name in [
        "AmsiUacInitialize",
        "AmsiUacScan",
        "AmsiUacUninitialize",
        "DllGetClassObject",
    ] {
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
    for name in [
        "DllCanUnloadNow",
        "DllRegisterServer",
        "DllUnregisterServer",
    ] {
        assert_eq!(export_kind(name), Some(ExportKind::PrivateWithoutBody));
    }
}

#[test]
fn export_table_rejects_unknown_and_near_miss_names() {
    // Not oracle-derived. Matching is exact and case-sensitive; DllMain is not in amsi.spec.
    for name in [
        "",
        "amsiinitialize",
        "AmsiInitialize ",
        "AmsiUac",
        "DllMain",
    ] {
        assert_eq!(export_kind(name), None);
    }
}

#[test]
fn export_kind_derived_impls() {
    // Calls the derived impls directly; assert_eq! only formats on failure.
    let kind = ExportKind::Stub;
    assert_eq!(Clone::clone(&kind), kind);
    assert!(format!("{kind:?}").contains("Stub"));
    assert!(format!("{:?}", ExportKind::PrivateWithoutBody).contains("PrivateWithoutBody"));
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/amsi.idl and include/winerror.h.
    assert_eq!(S_OK, 0);
    assert_eq!(AMSI_RESULT_NOT_DETECTED, 0x01);
    assert_eq!(FAKE_HANDLE, 0xdead_beef);
    assert_eq!(AMSI_DLL_NAME, "amsi.dll");
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
}

#[test]
fn out_pointers_are_overwritten_regardless_of_prior_value() {
    // Not oracle-derived: the prior values 0 and the maximum are boundaries that the
    // harness did not capture. The C bodies store unconditionally, so they must be overwritten.
    for prior in [0, FAKE_HANDLE, usize::MAX] {
        let mut ctx = prior;
        assert_eq!(amsi_initialize(&mut ctx), S_OK);
        assert_eq!(ctx, FAKE_HANDLE);
        let mut sess = prior;
        assert_eq!(amsi_open_session(&mut sess), S_OK);
        assert_eq!(sess, FAKE_HANDLE);
    }
    for prior in [0, AMSI_RESULT_NOT_DETECTED, u32::MAX] {
        let mut res = prior;
        assert_eq!(amsi_scan_buffer(&mut res), S_OK);
        assert_eq!(res, AMSI_RESULT_NOT_DETECTED);
        let mut res = prior;
        assert_eq!(amsi_scan_string(&mut res), S_OK);
        assert_eq!(res, AMSI_RESULT_NOT_DETECTED);
    }
}

#[test]
fn fake_handle_is_non_null() {
    // Not oracle-derived. A NULL handle would look like "no context" to callers.
    assert_ne!(FAKE_HANDLE, 0);
}

#[test]
fn stub_exception_keeps_empty_and_long_names() {
    // Not oracle-derived: boundary values for the string arguments.
    let e = wine_spec_unimplemented_stub("", "");
    assert_eq!((e.module, e.function), ("", ""));
    let long = "X".repeat(4096);
    let e = wine_spec_unimplemented_stub(&long, &long);
    assert_eq!(e.module.len(), 4096);
    assert_eq!(e.function.len(), 4096);
}

#[test]
fn stub_exception_derived_impls() {
    // Calls the derived impls directly; assert_eq! only formats on failure.
    let e = wine_spec_unimplemented_stub(AMSI_DLL_NAME, "DllGetClassObject");
    assert_eq!(Clone::clone(&e), e);
    assert!(format!("{e:?}").contains("DllGetClassObject"));
}
