//! Edge-case tests for the ported wimgapi logic.
//!
//! These cover the last-error slot, the unread arguments, and the stub and export tables. Not
//! every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(NULL_HANDLE, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WIMGAPI_DLL_NAME, "wimgapi.dll");
}

#[test]
fn default_slot_starts_at_zero() {
    // Not oracle-derived: a new thread's last error is 0.
    assert_eq!(Wimgapi::default().last_error(), 0);
}

#[test]
fn register_message_callback_keeps_the_last_error() {
    // Not oracle-derived beyond rows 01 and 02: the C body has no SetLastError call, so any
    // earlier value survives, including a value that is not the sentinel.
    for earlier in [0, 5, ERROR_CALL_NOT_IMPLEMENTED, u32::MAX] {
        let mut w = Wimgapi::with_last_error(earlier);
        assert_eq!(w.wim_register_message_callback(usize::MAX, 1, 2), 0);
        assert_eq!(w.last_error(), earlier);
    }
}

#[test]
fn get_mounted_images_overwrites_the_last_error_and_ignores_length() {
    // Not oracle-derived beyond rows 03 and 04: the body overwrites any earlier error.
    let mut w = Wimgapi::with_last_error(5);
    let mut len = u32::MAX;
    assert_eq!(w.wim_get_mounted_images(usize::MAX, Some(&mut len)), 0);
    assert_eq!(w.last_error(), 120);
    assert_eq!(len, u32::MAX);
}

#[test]
fn create_file_overwrites_the_last_error_and_ignores_arguments() {
    // Not oracle-derived beyond rows 05 and 06: an empty path and extreme flag values change
    // nothing, and the out-parameter is never written.
    let mut w = Wimgapi::with_last_error(0);
    let mut res = 0;
    let handle = w.wim_create_file(
        Some(&[]),
        u32::MAX,
        u32::MAX,
        u32::MAX,
        u32::MAX,
        Some(&mut res),
    );
    assert_eq!(handle, NULL_HANDLE);
    assert_eq!(w.last_error(), 120);
    assert_eq!(res, 0);
}

#[test]
fn last_error_is_per_instance() {
    // Not oracle-derived: two slots do not share state.
    let mut a = Wimgapi::default();
    let b = Wimgapi::default();
    a.wim_get_mounted_images(0, None);
    assert_eq!(a.last_error(), 120);
    assert_eq!(b.last_error(), 0);
}

#[test]
fn stub_record_carries_the_arguments() {
    // Not oracle-derived: the record is built from its two arguments, even for an ordinal-style name.
    let e = wine_spec_unimplemented_stub("x.dll", "17");
    assert_eq!(
        e,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "x.dll",
            function: "17",
        }
    );
}

#[test]
fn stub_table_matches_the_spec_table() {
    // Not oracle-derived: STUB_EXPORTS and EXPORTS list the same `@ stub` names in the same order.
    let from_exports: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let from_stubs: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(from_exports.len(), 37);
    assert_eq!(from_exports, from_stubs);
    for (name, helper) in STUB_EXPORTS {
        let e = helper();
        assert_eq!(e.function, *name);
        assert_eq!(e.module, WIMGAPI_DLL_NAME);
    }
}

#[test]
fn export_kinds() {
    // Not oracle-derived: classification from wimgapi.spec.
    assert_eq!(EXPORTS.len(), 42);
    assert_eq!(export_kind("WIMCreateFile"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("WIMGetMountedImages"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("WIMRegisterMessageCallback"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("DllMain"), Some(ExportKind::Private));
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Private));
    assert_eq!(export_kind("WIMApplyImage"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("WIMUnregisterMessageCallback"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("wimcreatefile"), None);
    assert_eq!(export_kind(""), None);
}
