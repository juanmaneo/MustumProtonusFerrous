//! Edge-case tests for the ported qedit logic.
//!
//! These cover the HRESULT sign boundary, the GUID and class table lookups, the reference count
//! steps, the IID and outer gates, and the registration paths that the oracle rows do not reach.
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::oracle_checks::{registered_calls, run_registration, Call, IID_IFILTERMAPPER2};
use super::*;
use crate::com::{failed, is_equal_guid};
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/unknwn.idl and
    // include/axextend.idl.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(REGDB_E_CLASSNOTREG as u32, 0x8004_0154);
    assert_eq!(MERIT_DO_NOT_USE, 0x0020_0000);
    assert_eq!(REG_PINFLAG_B_OUTPUT, 0x8);
}

#[test]
fn failed_depends_on_the_sign_bit() {
    // Not oracle-derived: FAILED is true for negative HRESULTs only.
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(!failed(0));
    assert!(!failed(1));
    assert!(!failed(i32::MAX));
}

#[test]
fn guid_equality_compares_every_byte() {
    // Not oracle-derived: IsEqualGUID compares all 16 bytes.
    assert!(is_equal_guid(&CLSID_AMTIMELINE, &CLSID_AMTIMELINE));
    let mut other = CLSID_AMTIMELINE;
    other.data4[7] ^= 1;
    assert!(!is_equal_guid(&CLSID_AMTIMELINE, &other));
    assert!(!is_equal_guid(&GUID_NULL, &IID_IUNKNOWN));
}

#[test]
fn class_table_maps_each_clsid_to_its_class() {
    // Not oracle-derived: every class in the table maps back to itself, and the IIDs and
    // GUID_NULL are not classes.
    for class in [
        ClassId::AmTimeline,
        ClassId::MediaDet,
        ClassId::NullRenderer,
        ClassId::SampleGrabber,
    ] {
        assert_eq!(ClassId::from_clsid(class.clsid()), Some(class));
    }
    assert_eq!(ClassId::from_clsid(&IID_IUNKNOWN), None);
    assert_eq!(ClassId::from_clsid(&GUID_NULL), None);
}

#[test]
fn every_class_accepts_iunknown_and_reports_its_class() {
    // Not oracle-derived: the oracle queried IUnknown only on MediaDet, and the class of the
    // factory only through the create closure.
    for class in [
        ClassId::AmTimeline,
        ClassId::MediaDet,
        ClassId::NullRenderer,
        ClassId::SampleGrabber,
    ] {
        let factory = dll_get_class_object(class.clsid(), &IID_IUNKNOWN, true).unwrap();
        assert_eq!(factory.class(), class);
        assert_eq!(factory.ref_count(), 1);
    }
}

#[test]
fn query_interface_adds_a_reference_for_iclassfactory_on_other_classes() {
    // Not oracle-derived: the oracle queried IClassFactory on SampleGrabber only.
    let mut factory = dll_get_class_object(&CLSID_MEDIADET, &IID_ICLASSFACTORY, true).unwrap();
    assert_eq!(factory.query_interface(&IID_ICLASSFACTORY), S_OK);
    assert_eq!(factory.ref_count(), 2);
}

#[test]
fn release_steps_down_to_zero() {
    // Not oracle-derived: the oracle released from 1 only. This takes a second release from 2.
    let mut factory = dll_get_class_object(&CLSID_NULLRENDERER, &IID_ICLASSFACTORY, true).unwrap();
    assert_eq!(factory.add_ref(), 2);
    assert_eq!(factory.release(), 1);
    assert_eq!(factory.release(), 0);
}

#[test]
fn create_instance_gate_checks_outer_against_iunknown() {
    // Not oracle-derived: a non-NULL outer with IUnknown and a NULL outer with IClassFactory both
    // reach the create step. The oracle covered the outer gate with IClassFactory only.
    let factory = ClassFactory::new(ClassId::MediaDet);
    let seen = Cell::new(None);
    let create = |class| {
        seen.set(Some(class));
        S_OK
    };
    assert_eq!(factory.create_instance(true, &IID_IUNKNOWN, create), S_OK);
    assert_eq!(seen.get(), Some(ClassId::MediaDet));
    seen.set(None);
    assert_eq!(
        factory.create_instance(false, &IID_ICLASSFACTORY, create),
        S_OK
    );
    assert_eq!(seen.get(), Some(ClassId::MediaDet));
}

#[test]
fn registration_continues_on_a_non_failure_resources_hresult() {
    // Not oracle-derived: the C code tests FAILED(resources), so S_FALSE continues to the mapper.
    assert_eq!(
        run_registration(true, S_FALSE, S_OK),
        (S_OK, registered_calls(), 1, 1)
    );
    assert_eq!(
        run_registration(false, S_FALSE, S_OK),
        (
            S_OK,
            vec![
                Call::Unregister(CLSID_SAMPLEGRABBER),
                Call::Unregister(CLSID_NULLRENDERER)
            ],
            1,
            1
        )
    );
}

#[test]
fn iid_gate_rejects_other_interfaces_before_clsid_lookup() {
    // Not oracle-derived: an unknown CLSID with a non-factory IID reports E_NOINTERFACE, and a
    // known CLSID with IFilterMapper2 does too. The oracle covered both with IFilterMapper2.
    assert_eq!(
        dll_get_class_object(&CLSID_MEDIADET, &IID_IFILTERMAPPER2, false),
        Err(E_NOINTERFACE)
    );
}
