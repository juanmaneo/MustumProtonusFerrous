//! Boundary and branch tests for the ported dpnhpast logic.
//!
//! Each test says whether its expectations are oracle-derived. The ones marked
//! "not oracle-derived" come from reading the Wine C source, not from a printed row.

use super::ResourceStep::{NotLoaded, OutOfMemory, Registered, RegistrarFailed};
use super::*;
use std::ffi::c_void;

#[test]
fn hresult_constants_match_the_oracle_hex() {
    // Oracle-derived: the printed hex across rows 00 to 22.
    assert_eq!(S_OK as u32, 0x0000_0000);
    assert_eq!(S_FALSE as u32, 0x0000_0001);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
}

#[test]
fn succeeded_boundaries() {
    // Not oracle-derived: SUCCEEDED(hr) is `(HRESULT)(hr) >= 0` in winerror.h.
    // S_FALSE succeeding is oracle-derived (rows 15 and 21 continue past it).
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(i32::MIN));
    assert!(!succeeded(E_FAIL));
}

#[test]
fn stub_results_ignore_the_guid_arguments() {
    // Not oracle-derived: the oracle passes one fixed GUID. The C stubs never read pIID, rclsid or riid.
    let zeros = [0u8; 16];
    let ones = [0xffu8; 16];
    assert_eq!(direct_play_nat_help_create(Some(&zeros), None), E_NOTIMPL);
    assert_eq!(direct_play_nat_help_create(Some(&ones), None), E_NOTIMPL);
    assert_eq!(dll_get_class_object(&zeros, &ones, None), E_FAIL);
    assert_eq!(dll_get_class_object(&ones, &zeros, None), E_FAIL);
}

#[test]
fn stubs_leave_a_null_out_slot_null() {
    // Not oracle-derived for a NULL slot: the oracle only checks a non-null sentinel (rows 02, 03, 05).
    let mut slot: *mut c_void = std::ptr::null_mut();
    assert_eq!(
        direct_play_nat_help_create(Some(&[0u8; 16]), Some(&mut slot)),
        E_NOTIMPL
    );
    assert!(slot.is_null());
    assert_eq!(
        dll_get_class_object(&[0u8; 16], &[0u8; 16], Some(&mut slot)),
        E_FAIL
    );
    assert!(slot.is_null());
}

#[test]
fn not_loaded_stops_and_keeps_the_previous_result() {
    // Not oracle-derived beyond rows 10 and 11, which only cover NotLoaded first.
    // The rule is `if (!str) return FALSE;` in register_resource, before any result update.
    assert_eq!(
        dll_register_server(&[Registered(E_FAIL), NotLoaded]),
        E_FAIL
    );
    assert_eq!(
        dll_register_server(&[Registered(S_FALSE), NotLoaded]),
        S_FALSE
    );
    assert_eq!(
        dll_register_server(&[Registered(S_OK), NotLoaded, Registered(E_FAIL)]),
        S_OK
    );
}

#[test]
fn first_failed_registration_stops_with_its_hresult() {
    // Not oracle-derived for these HRESULTs: the rule is `return SUCCEEDED(info->result)`.
    assert_eq!(
        dll_register_server(&[Registered(E_NOINTERFACE), Registered(S_OK)]),
        E_NOINTERFACE
    );
    assert_eq!(
        dll_register_server(&[Registered(E_OUTOFMEMORY), Registered(E_FAIL)]),
        E_OUTOFMEMORY
    );
    assert_eq!(
        dll_unregister_server(&[Registered(E_FAIL), Registered(S_OK)]),
        E_FAIL
    );
}

#[test]
fn out_of_memory_stops_after_a_success() {
    // Not oracle-derived beyond row 17 (S then O). Later steps are never looked at.
    assert_eq!(
        dll_register_server(&[OutOfMemory, Registered(S_OK)]),
        E_OUTOFMEMORY
    );
    assert_eq!(
        dll_unregister_server(&[Registered(S_OK), OutOfMemory, Registered(E_FAIL)]),
        E_OUTOFMEMORY
    );
}

#[test]
fn registrar_failure_carries_its_own_hresult() {
    // Not oracle-derived: the oracle only has E_NOINTERFACE. A failing AtlCreateRegistrar
    // stores its own HRESULT in info->result and stops enumeration.
    assert_eq!(
        dll_register_server(&[RegistrarFailed(E_FAIL), Registered(S_OK)]),
        E_FAIL
    );
    assert_eq!(dll_unregister_server(&[RegistrarFailed(E_FAIL)]), E_FAIL);
}

#[test]
fn unregister_follows_the_same_loop_as_register() {
    // Not oracle-derived beyond rows 19 to 22: these sequences are not in the oracle.
    let sequences: &[&[ResourceStep]] = &[
        &[],
        &[NotLoaded],
        &[Registered(S_FALSE), Registered(E_FAIL), Registered(S_OK)],
        &[OutOfMemory],
        &[Registered(S_OK), NotLoaded, Registered(E_FAIL)],
    ];
    for steps in sequences {
        assert_eq!(dll_unregister_server(steps), dll_register_server(steps));
    }
}
