//! Edge-case tests for the ported gameinput stubs.
//!
//! The oracle covers the observed inputs. These tests cover the generic out-slot
//! type and the HRESULT properties. Expectations not taken from the oracle are marked.

use super::*;

#[test]
fn stubs_accept_any_out_slot_type() {
    // Not oracle-derived: the C harness only uses `void **`/`v0_IGameInput **`.
    // The generic port must behave the same for any pointee type.
    let mut byte_slot: Option<u8> = Some(7);
    assert_eq!(game_input_create(Some(&mut byte_slot)) as u32, 0x8000_4001);
    assert_eq!(byte_slot, Some(7));

    let mut unit_slot = ();
    assert_eq!(
        dll_get_class_object(Some(&mut unit_slot)) as u32,
        0x8004_0111
    );
    assert_eq!(game_input_create::<()>(None) as u32, 0x8000_4001);
    assert_eq!(dll_get_class_object::<()>(None) as u32, 0x8004_0111);
}

#[test]
fn stub_results_are_failure_hresults() {
    // Not oracle-derived: `FAILED(hr)` is `hr < 0`, and both codes are error codes.
    // The C callers test this macro, so the sign bit matters even though the oracle only prints the values.
    assert!(game_input_create::<usize>(None) < 0);
    assert!(dll_get_class_object::<usize>(None) < 0);
}

#[test]
fn stub_codes_are_distinct() {
    // Not oracle-derived: the two codes must differ, or a caller could not tell
    // "not implemented" from "class not available".
    assert_ne!(E_NOTIMPL, CLASS_E_CLASSNOTAVAILABLE);
}

#[test]
fn hresult_constants_match_winerror() {
    // Not oracle-derived: checked against `winerror.h` in target/repos/wine/include.
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
}
