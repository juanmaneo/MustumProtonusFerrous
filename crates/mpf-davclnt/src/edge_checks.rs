//! Edge-case tests for the ported davclnt logic.
//!
//! Not every expectation is oracle-derived. Each one that is not says so in a comment.

use super::*;

#[test]
fn register_ignores_callback_and_version() {
    // not oracle-derived: the oracle passed NULL and one non-NULL callback, with versions 0 and 1.
    // The C body never reads `cb` or `version`, so extreme values give the same handle.
    assert_eq!(dav_register_auth_callback(0, 0), 0xdead_beef);
    assert_eq!(
        dav_register_auth_callback(usize::MAX, u32::MAX),
        0xdead_beef
    );
}

#[test]
fn unregister_accepts_any_handle() {
    // not oracle-derived: the oracle passed only 0xdeadbeef. The C body ignores `handle`.
    dav_unregister_auth_callback(0);
    dav_unregister_auth_callback(u32::MAX);
}

#[test]
fn dll_main_reason_branches() {
    // not oracle-derived for reasons 2, 3 and u32::MAX: the oracle captured only 0 and 1.
    // Read from `libs/winecrt0/dll_main.c`: only DLL_PROCESS_ATTACH disables thread
    // library calls, and every reason returns TRUE.
    for reason in [0, 2, 3, u32::MAX] {
        let outcome = dll_main(reason);
        assert!(outcome.ret);
        assert_eq!(
            outcome.disable_thread_library_calls,
            reason == DLL_PROCESS_ATTACH
        );
    }
}

#[test]
fn can_unload_now_is_s_false() {
    // S_FALSE from winerror.h. The oracle row confirms the value 1.
    assert_eq!(dll_can_unload_now(), S_FALSE);
    assert_eq!(dll_can_unload_now(), 1);
}

#[test]
fn stub_export_names_are_unique() {
    // not oracle-derived: the 19 names come from the `@ stub` lines of `davclnt.spec`.
    assert_eq!(STUB_EXPORTS.len(), 19);
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(!STUB_EXPORTS[..i].contains(name), "duplicate stub {name}");
    }
}

#[test]
fn stub_record_passes_arguments_through() {
    // not oracle-derived: the harness used only spec names. The C stub forwards its
    // arguments unchanged, so an empty or unknown name is reported as given.
    assert_eq!(
        unimplemented_stub("", ""),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            module: "",
            function: "",
        }
    );
    let unknown = export_stub("NoSuchExport");
    assert_eq!(unknown.module, MODULE_FILE_NAME);
    assert_eq!(unknown.function, "NoSuchExport");
}

#[test]
fn authn_scheme_constants() {
    // not oracle-derived: values copied from `davclnt.h`; the oracle did not print them.
    assert_eq!(DAV_AUTHN_SCHEME_BASIC, 0x1);
    assert_eq!(DAV_AUTHN_SCHEME_NTLM, 0x2);
    assert_eq!(DAV_AUTHN_SCHEME_PASSPORT, 0x4);
    assert_eq!(DAV_AUTHN_SCHEME_DIGEST, 0x8);
    assert_eq!(DAV_AUTHN_SCHEME_NEGOTIATE, 0x10);
    assert_eq!(DAV_AUTHN_SCHEME_CERT, 0x0001_0000);
    assert_eq!(DAV_AUTHN_SCHEME_FBA, 0x0010_0000);
}

#[test]
fn auth_next_step_discriminants() {
    // not oracle-derived: the enum order and values come from `davclnt.h`.
    assert_eq!(AuthNextStep::DefaultBehavior as u32, 0);
    assert_eq!(AuthNextStep::RetryRequest as u32, 1);
    assert_eq!(AuthNextStep::CancelRequest as u32, 2);
}
