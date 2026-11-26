//! Edge-case tests for the ported xaudio2_7 logic.
//!
//! These cover the sign boundary of `FAILED`, GUID field comparisons, the check order in the
//! create-instance gates, and the callback table shapes that the oracle rows do not reach. Each
//! expectation that is not oracle-derived says so in a comment.
//!
//! Through the public API the callback table keeps its NULL slots at the tail: `register` fills
//! the first NULL slot, and `unregister` shifts non-NULL entries left, so a NULL slot is never
//! followed by a non-NULL one. The tests only build tables that this API can produce.

use super::*;

fn guid_copy(g: &Guid) -> Guid {
    Guid {
        data1: g.data1,
        data2: g.data2,
        data3: g.data3,
        data4: g.data4,
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/xaudio2.h. The oracle rows
    // print the same values for E_NOTIMPL, E_NOINTERFACE, E_INVALIDARG, the two CLASS_E values
    // and XAUDIO2_E_INVALID_CALL.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(CLASS_E_NOAGGREGATION, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(XAUDIO2_E_INVALID_CALL, 0x8896_0001);
}

#[test]
fn failed_tests_the_sign_bit() {
    // Not oracle-derived: FAILED(hr) is `(hr) < 0` on the signed HRESULT, so the top bit decides.
    assert!(!failed(0));
    assert!(!failed(S_FALSE));
    assert!(!failed(0x7fff_ffff));
    assert!(failed(0x8000_0000));
    assert!(failed(E_NOTIMPL));
    assert!(failed(0xffff_ffff));
}

#[test]
fn registration_status_passes_failures_through() {
    // Not oracle-derived: a failed hr is returned unchanged, whatever its low bits. A success
    // (sign bit clear) returns 0, including the largest positive HRESULT.
    assert_eq!(get_registration_properties_status(0x8000_0000), 0x8000_0000);
    assert_eq!(get_registration_properties_status(0x8000_1234), 0x8000_1234);
    assert_eq!(get_registration_properties_status(0x7fff_ffff), 0);
    assert_eq!(get_registration_properties_status(0xffff_ffff), 0xffff_ffff);
}

#[test]
fn calc_frames_ignore_their_argument() {
    // Not oracle-derived: the C bodies ignore the frame count, so 0 and the largest value give 0.
    // Oracle rows 19 and 20 cover 480 only.
    assert_eq!(calc_input_frames(0), 0);
    assert_eq!(calc_input_frames(u32::MAX), 0);
    assert_eq!(calc_output_frames(0), 0);
    assert_eq!(calc_output_frames(u32::MAX), 0);
}

#[test]
fn guid_equality_checks_every_field() {
    // Not oracle-derived: a GUID is equal only when every field matches, so each field is
    // changed on its own and must make the comparison fail.
    assert!(is_equal_guid(&guid_copy(&CLSID_XAUDIO2), &CLSID_XAUDIO2));

    let mut g = guid_copy(&CLSID_XAUDIO2);
    g.data1 ^= 1;
    assert!(!is_equal_guid(&g, &CLSID_XAUDIO2));

    let mut g = guid_copy(&CLSID_XAUDIO2);
    g.data2 ^= 1;
    assert!(!is_equal_guid(&g, &CLSID_XAUDIO2));

    let mut g = guid_copy(&CLSID_XAUDIO2);
    g.data3 ^= 1;
    assert!(!is_equal_guid(&g, &CLSID_XAUDIO2));

    let mut g = guid_copy(&CLSID_XAUDIO2);
    g.data4[0] ^= 1;
    assert!(!is_equal_guid(&g, &CLSID_XAUDIO2));

    let mut g = guid_copy(&CLSID_XAUDIO2);
    g.data4[7] ^= 1;
    assert!(!is_equal_guid(&g, &CLSID_XAUDIO2));
}

#[test]
fn class_object_rejects_null_and_interface_guids() {
    // Not oracle-derived: an all-zero CLSID and an IID that is not a class are not available.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert!(class_object(&zero) == Err(CLASS_E_CLASSNOTAVAILABLE));
    assert!(class_object(&IID_IXAUDIO2) == Err(CLASS_E_CLASSNOTAVAILABLE));
}

#[test]
fn aggregation_gate_is_the_pouter_check() {
    // Not oracle-derived: direct check of the gate that oracle rows 08 and 10 reach through the
    // create-instance wrappers.
    assert!(aggregation_gate(false) == Ok(()));
    assert!(aggregation_gate(true) == Err(CLASS_E_NOAGGREGATION));
}

#[test]
fn create_instance_checks_outer_before_class() {
    // Not oracle-derived: the verbatim xapocf_CreateInstance tests pOuter before the class, so
    // an aggregated request for an unknown class is CLASS_E_NOAGGREGATION, not E_INVALIDARG.
    assert!(xapo_create_instance_class(&CLSID_FX_REVERB, true) == Err(CLASS_E_NOAGGREGATION));
    assert!(xapo_create_instance_class(&CLSID_XAUDIO2, true) == Err(CLASS_E_NOAGGREGATION));
}

#[test]
fn create_instance_rejects_non_xapo_classes() {
    // Not oracle-derived: DllGetClassObject only hands out XAudio2 and the two XAPO factories,
    // so this is defensive. The class dispatch only knows the two XAPO CLSIDs.
    assert!(xapo_create_instance_class(&CLSID_XAUDIO2, false) == Err(E_INVALIDARG));
    assert!(xapo_create_instance_class(&IID_IXAPO, false) == Err(E_INVALIDARG));
}

#[test]
fn xapo_query_interface_rejects_other_iids() {
    // Not oracle-derived: the IClassFactory IID (00000001-0000-0000-C000-000000000046) and the
    // XAudio2 CLSID are not interfaces of the XAPO wrapper.
    let class_factory = Guid {
        data1: 1,
        data2: 0,
        data3: 0,
        data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
    };
    assert!(xapo_query_interface(&class_factory) == Err(E_NOINTERFACE));
    assert!(xapo_query_interface(&CLSID_XAUDIO2) == Err(E_NOINTERFACE));
}

#[test]
fn xaudio2_query_interface_accepts_only_iunknown_and_ixaudio2() {
    // Not oracle-derived: the two accepted IIDs and the XAPO IID that is refused.
    assert!(xaudio2_query_interface(&IID_IUNKNOWN) == Ok(()));
    assert!(xaudio2_query_interface(&IID_IXAPO) == Err(E_NOINTERFACE));
}

#[test]
fn register_reuses_the_null_tail_before_appending() {
    // Not oracle-derived: after an unregister leaves a NULL tail, the next register fills that
    // tail slot, and only a register with no NULL slot left appends.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(Some(2));
    e.register_for_callbacks(Some(3));
    e.unregister_for_callbacks(Some(3));
    assert_eq!(e.callbacks(), &[Some(1), Some(2), None]);
    assert!(e.register_for_callbacks(Some(4)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2), Some(4)]);
    assert!(e.register_for_callbacks(Some(5)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2), Some(4), Some(5)]);
}

#[test]
fn register_null_matches_an_existing_null_slot() {
    // Not oracle-derived: registering a NULL callback reuses a NULL slot rather than appending.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(None);
    assert_eq!(e.callbacks(), &[Some(1), None]);
    assert!(e.register_for_callbacks(None) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), None]);
}

#[test]
fn unregister_on_empty_table_does_nothing() {
    // Not oracle-derived: the ncbs == 0 case, which the C code returns from early.
    let mut e = Engine::default();
    e.unregister_for_callbacks(Some(1));
    assert!(e.callbacks().is_empty());
    e.unregister_for_callbacks(None);
    assert!(e.callbacks().is_empty());
}

#[test]
fn unregister_last_slot_keeps_the_table_length() {
    // Not oracle-derived: removing the last entry clears it to NULL without shrinking ncbs.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(Some(2));
    e.unregister_for_callbacks(Some(2));
    assert_eq!(e.callbacks(), &[Some(1), None]);
}

#[test]
fn unregister_shifts_until_the_null_tail() {
    // Not oracle-derived: a longer table shifts every following entry left and stops at the
    // first NULL slot, which is then cleared as the vacated slot.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(Some(2));
    e.register_for_callbacks(Some(3));
    e.register_for_callbacks(Some(4));
    e.unregister_for_callbacks(Some(3));
    assert_eq!(e.callbacks(), &[Some(1), Some(2), Some(4), None]);
    e.unregister_for_callbacks(Some(1));
    assert_eq!(e.callbacks(), &[Some(2), Some(4), None, None]);
}

#[test]
fn unregister_null_matches_the_first_null_slot() {
    // Not oracle-derived: NULL matches the NULL tail slot, so nothing shifts and the table is
    // unchanged.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(Some(2));
    e.register_for_callbacks(None);
    e.unregister_for_callbacks(None);
    assert_eq!(e.callbacks(), &[Some(1), Some(2), None]);
}

#[test]
fn unregister_unknown_callback_leaves_table_unchanged() {
    // Not oracle-derived: a callback that is not registered leaves every slot as it was,
    // including a NULL tail.
    let mut e = Engine::default();
    e.register_for_callbacks(Some(1));
    e.register_for_callbacks(Some(2));
    e.unregister_for_callbacks(Some(7));
    assert_eq!(e.callbacks(), &[Some(1), Some(2)]);
    e.unregister_for_callbacks(Some(1));
    assert_eq!(e.callbacks(), &[Some(2), None]);
    e.unregister_for_callbacks(Some(7));
    assert_eq!(e.callbacks(), &[Some(2), None]);
}

#[test]
fn mastering_latch_is_per_engine_and_ignores_callbacks() {
    // Not oracle-derived: callback registration does not touch the mastering latch, and a
    // fresh engine can create one mastering voice.
    let mut e = Engine::default();
    assert!(!e.mastering_in_use());
    e.register_for_callbacks(Some(1));
    e.unregister_for_callbacks(Some(1));
    assert!(!e.mastering_in_use());
    assert!(e.create_mastering_voice() == S_OK);
    assert!(e.mastering_in_use());
    assert!(Engine::default().create_mastering_voice() == S_OK);
}

#[test]
fn engine_default_is_empty() {
    // Not oracle-derived: a new engine has no callback slots and no mastering voice. The oracle
    // rows start from a zeroed IXAudio2Impl, which is the same state.
    let e = Engine::default();
    assert!(e.callbacks().is_empty());
    assert!(!e.mastering_in_use());
}
