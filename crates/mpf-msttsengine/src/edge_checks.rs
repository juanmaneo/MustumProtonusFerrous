//! Boundary and branch tests for the ported msttsengine logic.
//!
//! A test whose doc comment says "Not oracle-derived" checks a branch or boundary that no harness row reaches.
//! Its expectation is read from the reference C, not printed by the oracle.

use super::*;
use crate::oracle_checks::{run_dll_main, FakeToken};
use std::cell::Cell;

/// Not oracle-derived: `DllMain` for a reason other than `DLL_PROCESS_ATTACH` returns `TRUE` and makes no thread
/// library call. Reason 0 is `DLL_PROCESS_DETACH`, from `include/winnt.h`.
#[test]
fn dll_main_other_reason_makes_no_call() {
    assert_eq!(run_dll_main(0), (true, 0));
}

/// Not oracle-derived: `class_factory_QueryInterface` also accepts `IClassFactory`, as `main.c` shows. The harness
/// queries only `IUnknown` on the factory.
#[test]
fn class_factory_query_interface_accepts_iclassfactory() {
    assert!(class_factory_query_interface(&IID_CLASS_FACTORY).is_ok());
}

/// Not oracle-derived: `class_factory_LockServer` ignores its argument, so `FALSE` also gives `S_OK`. The harness
/// passes only `TRUE`.
#[test]
fn class_factory_lock_server_false() {
    assert_eq!(class_factory_lock_server(false), S_OK);
}

/// Not oracle-derived: `ttsengine_QueryInterface` for `IUnknown` and for `ISpTTSEngine` gives `SpTtsEngine` and
/// takes one reference each time.
#[test]
fn ttsengine_query_interface_takes_reference() {
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_TTS_ENGINE, true).unwrap();
    assert!(engine.query_interface(&IID_UNKNOWN) == Ok(Interface::SpTtsEngine));
    assert_eq!(engine.ref_count, 2);
    assert!(engine.query_interface(&IID_SP_TTS_ENGINE) == Ok(Interface::SpTtsEngine));
    assert_eq!(engine.ref_count, 3);
}

/// Not oracle-derived: `ttsengine_QueryInterface` for an IID it does not serve gives `E_NOINTERFACE` and takes no
/// reference.
#[test]
fn ttsengine_query_interface_unknown_iid_takes_no_reference() {
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_TTS_ENGINE, true).unwrap();
    assert_eq!(
        engine.query_interface(&IID_CLASS_FACTORY).err(),
        Some(E_NOINTERFACE)
    );
    assert_eq!(engine.ref_count, 1);
}

/// Not oracle-derived: `ttsengine_create` checks the allocation before the IID, so a failed `malloc` gives
/// `E_OUTOFMEMORY` even for an IID that would otherwise fail with `E_NOINTERFACE`.
#[test]
fn ttsengine_create_allocation_checked_before_interface() {
    assert_eq!(
        ttsengine_create::<FakeToken>(&IID_CLASS_FACTORY, false).err(),
        Some(E_OUTOFMEMORY)
    );
}

/// Not oracle-derived: `class_factory_CreateInstance` rejects a non-NULL outer before it allocates, so a failed
/// `malloc` is not reached and the result is `CLASS_E_NOAGGREGATION`.
#[test]
fn class_factory_create_instance_outer_checked_before_allocation() {
    assert_eq!(
        class_factory_create_instance::<FakeToken>(true, &IID_SP_TTS_ENGINE, false).err(),
        Some(CLASS_E_NOAGGREGATION)
    );
}

/// Not oracle-derived: `DllGetClassObject` for `CLSID_TTSEngineCom` returns the factory's `QueryInterface` result,
/// so an IID the factory does not serve gives `E_NOINTERFACE`.
#[test]
fn dll_get_class_object_unsupported_iid() {
    assert_eq!(
        dll_get_class_object(&CLSID_TTS_ENGINE_COM, &IID_SP_TTS_ENGINE).err(),
        Some(E_NOINTERFACE)
    );
}

/// Not oracle-derived: `InterlockedIncrement` wraps, so `AddRef` at `i32::MAX` gives a negative count, which
/// `ttsengine_AddRef` returns as `0x80000000`.
#[test]
fn ttsengine_add_ref_wraps_like_interlocked_increment() {
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_TTS_ENGINE, true).unwrap();
    engine.ref_count = i32::MAX;
    assert_eq!(engine.add_ref(), 0x8000_0000);
}

/// Not oracle-derived: `ttsengine_Release` above zero keeps the token held. Releasing to zero with no token set has
/// nothing to release.
#[test]
fn ttsengine_release_without_token() {
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_TTS_ENGINE, true).unwrap();
    assert_eq!(engine.add_ref(), 2);
    assert_eq!(engine.release(), 1);
    assert_eq!(engine.release(), 0);
}

/// Not oracle-derived: `ttsengine_Release` above zero does not release a stored token. The token is released
/// only when the count reaches zero.
#[test]
fn ttsengine_release_above_zero_keeps_token() {
    let refs = Cell::new(0);
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_TTS_ENGINE, true).unwrap();
    assert_eq!(
        engine.set_object_token(Some(FakeToken { refs: &refs })),
        S_OK
    );
    assert_eq!(engine.add_ref(), 2);
    assert_eq!(engine.release(), 1);
    assert_eq!(refs.get(), 1);
    assert_eq!(engine.release(), 0);
    assert_eq!(refs.get(), 0);
}

/// Not oracle-derived: `SetObjectToken(NULL)` gives `E_INVALIDARG` before it checks whether a token is already
/// set, so a NULL argument does not change the stored token.
#[test]
fn set_object_token_null_checked_before_already_set() {
    let refs = Cell::new(0);
    let (mut engine, _) = ttsengine_create::<FakeToken>(&IID_SP_OBJECT_WITH_TOKEN, true).unwrap();
    assert_eq!(
        engine.set_object_token(Some(FakeToken { refs: &refs })),
        S_OK
    );
    assert_eq!(engine.set_object_token(None), E_INVALIDARG);
    assert_eq!(refs.get(), 1);
}

/// Not oracle-derived: `DllRegisterServer` and `DllUnregisterServer` return whatever the OS call returns.
#[test]
fn dll_register_servers_return_os_result() {
    assert_eq!(dll_register_server(|| S_OK), S_OK);
    assert_eq!(dll_unregister_server(|| E_NOINTERFACE), E_NOINTERFACE);
}

/// Not oracle-derived: `IsEqualGUID` compares all sixteen bytes, so a GUID that differs only in its last byte is
/// not equal.
#[test]
fn is_equal_guid_compares_every_byte() {
    let near = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47],
    };
    assert!(is_equal_guid(&IID_UNKNOWN, &IID_UNKNOWN));
    assert!(!is_equal_guid(&IID_UNKNOWN, &near));
}

/// Not oracle-derived: the export names come from `msttsengine.spec`, in spec order. Matching is case-sensitive.
#[test]
fn exports_match_spec_names() {
    assert_eq!(
        EXPORTS,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
    assert!(is_export("DllGetClassObject"));
    assert!(!is_export("dllgetclassobject"));
    assert!(!is_export("DllMain"));
}
