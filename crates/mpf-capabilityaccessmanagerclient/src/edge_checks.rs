//! Boundary and branch checks for the ported logic.
//!
//! Expectations marked "not oracle-derived" do not come from a printed oracle row. They
//! follow from the C source, the IDL, or the wrap-around that `InterlockedIncrement` and
//! `InterlockedDecrement` perform.

use super::*;
use crate::com::{
    is_base_interface, IID_IACTIVATIONFACTORY, IID_IAGILEOBJECT, IID_IAPPCAPABILITY,
    IID_IAPPCAPABILITYSTATICS, IID_IINSPECTABLE, IID_IUNKNOWN,
};

const CLASS_NAME: &str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// Not oracle-derived: direct check of the base-IID test. The oracle rows exercise the same IIDs through `QueryInterface`.
#[test]
fn base_interface_test_accepts_only_the_three_base_iids() {
    assert!(is_base_interface(&IID_IUNKNOWN));
    assert!(is_base_interface(&IID_IINSPECTABLE));
    assert!(is_base_interface(&IID_IAGILEOBJECT));
    assert!(!is_base_interface(&IID_IACTIVATIONFACTORY));
    assert!(!is_base_interface(&IID_IAPPCAPABILITY));
    assert!(!is_base_interface(&IID_IAPPCAPABILITYSTATICS));
}

/// Not oracle-derived: a GUID that differs from IUnknown only in its last byte is not a base IID.
#[test]
fn near_miss_iid_is_rejected_by_query_interface() {
    let near = Guid {
        data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x47],
        ..IID_IUNKNOWN
    };
    assert!(!is_base_interface(&near));
    let mut s = CapabilityStatics::default();
    assert_eq!(s.query_interface(&near), Err(E_NOINTERFACE));
    assert_eq!(s.ref_count, 1);
}

/// Not oracle-derived: the capability's QueryInterface does not list the factory IIDs,
/// which the oracle does not probe on the capability.
#[test]
fn capability_rejects_factory_iids_and_keeps_count() {
    let mut cap = AppCapability::create(&[]).unwrap();
    assert_eq!(
        cap.query_interface(&IID_IACTIVATIONFACTORY),
        Err(E_NOINTERFACE)
    );
    assert_eq!(
        cap.query_interface(&IID_IAPPCAPABILITYSTATICS),
        Err(E_NOINTERFACE)
    );
    assert_eq!(cap.ref_count, 1);
}

/// Not oracle-derived: `InterlockedIncrement` wraps. The `ULONG` return shows the raw bits.
#[test]
fn statics_count_wraps_past_i32_max() {
    let mut s = CapabilityStatics {
        ref_count: i32::MAX,
    };
    assert_eq!(s.add_ref(), 0x8000_0000);
    assert_eq!(s.release(), 0x7FFF_FFFF);
}

/// Not oracle-derived: releasing an object whose count is already 0 wraps to `ULONG` -1.
#[test]
fn capability_release_below_zero_wraps_to_u32_max() {
    let mut cap = AppCapability::create(&[]).unwrap();
    cap.ref_count = 0;
    assert_eq!(cap.release(), u32::MAX);
}

/// Not oracle-derived: the stub methods must not change the count, which no oracle row prints.
#[test]
fn statics_stubs_leave_count_unchanged() {
    let mut s = CapabilityStatics::default();
    assert_eq!(s.activate_instance(), E_NOTIMPL);
    assert_eq!(s.request_access_for_capabilities_async(), E_NOTIMPL);
    assert_eq!(s.ref_count, 1);
    assert_eq!(s.add_ref(), 2);
}

/// Not oracle-derived: the `IAppCapability` stub methods must not change the count either.
#[test]
fn capability_stubs_leave_count_unchanged() {
    let cap = AppCapability::create(&[]).unwrap();
    assert_eq!(cap.request_access_async(), E_NOTIMPL);
    assert_eq!(cap.get_user(), E_NOTIMPL);
    assert_eq!(cap.ref_count, 1);
}

/// Not oracle-derived: values from `windows.security.authorization.appcapabilityaccess.idl`.
/// The oracle prints only `Allowed`.
#[test]
fn access_status_values_match_idl() {
    assert_eq!(AppCapabilityAccessStatus::DeniedBySystem as i32, 0);
    assert_eq!(AppCapabilityAccessStatus::NotDeclaredByApp as i32, 1);
    assert_eq!(AppCapabilityAccessStatus::DeniedByUser as i32, 2);
    assert_eq!(AppCapabilityAccessStatus::UserPromptRequired as i32, 3);
    assert_eq!(AppCapabilityAccessStatus::Allowed as i32, 4);
}

/// Not oracle-derived: the class name matches only when the buffer is exactly the name
/// with no trailing NUL, which `wcscmp` also accepts.
#[test]
fn activation_accepts_exact_name_without_trailing_nul() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide(CLASS_NAME)),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 2);
}

/// Not oracle-derived: a longer name with no NUL to stop it is a different class.
#[test]
fn activation_rejects_longer_name_without_nul() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide(&format!("{CLASS_NAME}X"))),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
}

/// Not oracle-derived: a truncated class name is a different class.
#[test]
fn activation_rejects_truncated_name() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide(&CLASS_NAME[..CLASS_NAME.len() - 1])),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
}

/// Not oracle-derived: a leading NUL ends the buffer before the class name starts, so the comparison sees an empty name.
#[test]
fn activation_rejects_leading_nul() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide(&format!("\0{CLASS_NAME}"))),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
}
