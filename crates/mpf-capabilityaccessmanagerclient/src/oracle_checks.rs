//! Oracle-derived checks for the ported `capabilityaccessmanagerclient` logic.
//!
//! Expectations come from the 24 rows printed by the harness in `target/canon/capabilityaccessmanagerclient/`,
//! which compiles the verbatim Wine bodies of `capability.c` and `main.c` with
//! `gcc -fsanitize=address,undefined -g`. The run was clean, so no vector is excluded.
//! Each test quotes the printed row it encodes. Reference counts are read from
//! `ref_count`, the LONG that the harness prints.

use super::*;
use crate::com::{
    IID_IACTIVATIONFACTORY, IID_IAGILEOBJECT, IID_IAPPCAPABILITY, IID_IAPPCAPABILITYSTATICS,
    IID_IINSPECTABLE, IID_IUNKNOWN,
};

/// `IAppCapability2`, which Wine does not implement. Used only as an unsupported IID.
const IID_IAPPCAPABILITY2: Guid = Guid {
    data1: 0x11C7_CCB6,
    data2: 0xC74F,
    data3: 0x50A3,
    data4: [0xB9, 0x60, 0x88, 0x00, 0x87, 0x67, 0xD9, 0x39],
};

const CLASS_NAME: &str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn app_capability(name: &str) -> AppCapability {
    CapabilityStatics::default().create(&wide(name)).unwrap()
}

/// R01 QI_IUnknown=0x00000000/ActivationFactory/ref2 QI_IInspectable=0x00000000/ActivationFactory/ref3 QI_IAgileObject=0x00000000/ActivationFactory/ref4 QI_IActivationFactory=0x00000000/ActivationFactory/ref5
#[test]
fn oracle_r01_factory_query_interface_base_iids() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        s.query_interface(&IID_IUNKNOWN),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 2);
    assert_eq!(
        s.query_interface(&IID_IINSPECTABLE),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 3);
    assert_eq!(
        s.query_interface(&IID_IAGILEOBJECT),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 4);
    assert_eq!(
        s.query_interface(&IID_IACTIVATIONFACTORY),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 5);
}

/// R02 QI_IAppCapabilityStatics=0x00000000/AppCapabilityStatics/ref2 QI_IAppCapability=0x80004002/NULL/ref2 QI_IAppCapability2=0x80004002/NULL/ref2
#[test]
fn oracle_r02_factory_query_interface_statics_and_unsupported() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        s.query_interface(&IID_IAPPCAPABILITYSTATICS),
        Ok(Interface::AppCapabilityStatics)
    );
    assert_eq!(s.ref_count, 2);
    assert_eq!(s.query_interface(&IID_IAPPCAPABILITY), Err(E_NOINTERFACE));
    assert_eq!(s.ref_count, 2);
    assert_eq!(s.query_interface(&IID_IAPPCAPABILITY2), Err(E_NOINTERFACE));
    assert_eq!(s.ref_count, 2);
}

/// R03 factory_AddRef=2 statics_AddRef=3 statics_Release=2 factory_Release=1 factory_Release=0
#[test]
fn oracle_r03_refcount_is_shared_between_faces() {
    let mut s = CapabilityStatics::default();
    assert_eq!(s.add_ref(), 2);
    assert_eq!(s.add_ref(), 3);
    assert_eq!(s.release(), 2);
    assert_eq!(s.release(), 1);
    assert_eq!(s.release(), 0);
}

/// R04 factory_GetIids=0x80004001 factory_GetRuntimeClassName=0x80004001 factory_GetTrustLevel=0x80004001 factory_ActivateInstance=0x80004001
#[test]
fn oracle_r04_factory_stubs_return_not_implemented() {
    let s = CapabilityStatics::default();
    assert_eq!(s.get_iids(), E_NOTIMPL);
    assert_eq!(s.get_runtime_class_name(), E_NOTIMPL);
    assert_eq!(s.get_trust_level(), E_NOTIMPL);
    assert_eq!(s.activate_instance(), E_NOTIMPL);
}

/// R05 statics_QI_IAppCapabilityStatics=0x00000000/AppCapabilityStatics/ref2 statics_QI_IAppCapability=0x80004002/NULL/ref2
#[test]
fn oracle_r05_statics_face_query_interface() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        s.query_interface(&IID_IAPPCAPABILITYSTATICS),
        Ok(Interface::AppCapabilityStatics)
    );
    assert_eq!(s.ref_count, 2);
    assert_eq!(s.query_interface(&IID_IAPPCAPABILITY), Err(E_NOINTERFACE));
    assert_eq!(s.ref_count, 2);
}

/// R06 statics_GetIids=0x80004001 statics_GetRuntimeClassName=0x80004001 statics_GetTrustLevel=0x80004001 statics_RequestAccessForCapabilitiesAsync=0x80004001 statics_RequestAccessForCapabilitiesForUserAsync=0x80004001 statics_CreateWithProcessIdForUser=0x80004001
#[test]
fn oracle_r06_statics_stubs_return_not_implemented() {
    let s = CapabilityStatics::default();
    assert_eq!(s.get_iids(), E_NOTIMPL);
    assert_eq!(s.get_runtime_class_name(), E_NOTIMPL);
    assert_eq!(s.get_trust_level(), E_NOTIMPL);
    assert_eq!(s.request_access_for_capabilities_async(), E_NOTIMPL);
    assert_eq!(
        s.request_access_for_capabilities_for_user_async(),
        E_NOTIMPL
    );
    assert_eq!(s.create_with_process_id_for_user(), E_NOTIMPL);
}

/// R07 statics_Create=0x00000000 cap_name=len11:nonexistent cap_ref=1
#[test]
fn oracle_r07_create_nonexistent() {
    let name = wide("nonexistent");
    let mut cap = CapabilityStatics::default().create(&name).unwrap();
    assert_eq!(cap.get_capability_name(), Ok(name.as_slice()));
    assert_eq!(cap.release(), 0);
}

/// R08 statics_Create=0x00000000 cap_name=len5:ab\0cd cap_ref=1
#[test]
fn oracle_r08_create_keeps_embedded_nul() {
    let mut cap = app_capability("ab\0cd");
    assert_eq!(
        cap.get_capability_name(),
        Ok(&[0x61, 0x62, 0x00, 0x63, 0x64][..])
    );
    assert_eq!(cap.release(), 0);
}

/// R09 statics_Create=0x00000000 cap_name=len0: cap_ref=1
#[test]
fn oracle_r09_create_empty_name() {
    let empty: &[u16] = &[];
    let mut cap = CapabilityStatics::default().create(empty).unwrap();
    assert_eq!(cap.get_capability_name(), Ok(empty));
    assert_eq!(cap.release(), 0);
}

/// R10 cap_QI_IUnknown=0x00000000/AppCapability/ref2 cap_QI_IInspectable=0x00000000/AppCapability/ref3 cap_QI_IAgileObject=0x00000000/AppCapability/ref4 cap_QI_IAppCapability=0x00000000/AppCapability/ref5
#[test]
fn oracle_r10_capability_query_interface_base_iids() {
    let mut cap = app_capability("nonexistent");
    assert_eq!(
        cap.query_interface(&IID_IUNKNOWN),
        Ok(Interface::AppCapability)
    );
    assert_eq!(cap.ref_count, 2);
    assert_eq!(
        cap.query_interface(&IID_IINSPECTABLE),
        Ok(Interface::AppCapability)
    );
    assert_eq!(cap.ref_count, 3);
    assert_eq!(
        cap.query_interface(&IID_IAGILEOBJECT),
        Ok(Interface::AppCapability)
    );
    assert_eq!(cap.ref_count, 4);
    assert_eq!(
        cap.query_interface(&IID_IAPPCAPABILITY),
        Ok(Interface::AppCapability)
    );
    assert_eq!(cap.ref_count, 5);
    for expected in [4, 3, 2, 1, 0] {
        assert_eq!(cap.release(), expected);
    }
}

/// R11 cap_QI_IAppCapabilityStatics=0x80004002/NULL/ref1 cap_QI_IAppCapability2=0x80004002/NULL/ref1
#[test]
fn oracle_r11_capability_query_interface_unsupported() {
    let mut cap = app_capability("nonexistent");
    assert_eq!(
        cap.query_interface(&IID_IAPPCAPABILITYSTATICS),
        Err(E_NOINTERFACE)
    );
    assert_eq!(cap.ref_count, 1);
    assert_eq!(
        cap.query_interface(&IID_IAPPCAPABILITY2),
        Err(E_NOINTERFACE)
    );
    assert_eq!(cap.ref_count, 1);
}

/// R12 cap_AddRef=2 cap_Release=1 cap_Release=0
#[test]
fn oracle_r12_capability_add_ref_and_release() {
    let mut cap = app_capability("nonexistent");
    assert_eq!(cap.add_ref(), 2);
    assert_eq!(cap.release(), 1);
    assert_eq!(cap.release(), 0);
}

/// R13 cap_get_CapabilityName=0x00000000 name=len11:nonexistent cap_get_CapabilityName_empty=0x00000000 name=len0:
#[test]
fn oracle_r13_get_capability_name() {
    let name = wide("nonexistent");
    let cap = CapabilityStatics::default().create(&name).unwrap();
    assert_eq!(cap.get_capability_name(), Ok(name.as_slice()));
    let empty: &[u16] = &[];
    let cap_empty = CapabilityStatics::default().create(empty).unwrap();
    assert_eq!(cap_empty.get_capability_name(), Ok(empty));
}

/// R14 cap_CheckAccess=0x00000000 status=4
#[test]
fn oracle_r14_check_access_is_allowed() {
    let cap = app_capability("nonexistent");
    assert_eq!(cap.check_access(), Ok(AppCapabilityAccessStatus::Allowed));
}

/// R15 cap_add_AccessChanged_handler=0x00000000 token=0xdeadbeef cap_add_AccessChanged_null=0x00000000 token=0xdeadbeef
#[test]
fn oracle_r15_add_access_changed_returns_dummy_cookie() {
    let cap = app_capability("nonexistent");
    let cookie = EventRegistrationToken { value: 0xdead_beef };
    assert_eq!(cap.add_access_changed(), Ok(cookie));
    assert_eq!(cap.add_access_changed(), Ok(cookie));
}

/// R16 cap_remove_AccessChanged=0x00000000 cap_remove_AccessChanged_zero=0x00000000
#[test]
fn oracle_r16_remove_access_changed_is_ok() {
    let cap = app_capability("nonexistent");
    assert_eq!(
        cap.remove_access_changed(EventRegistrationToken { value: 0x1234 }),
        S_OK
    );
    assert_eq!(
        cap.remove_access_changed(EventRegistrationToken { value: 0 }),
        S_OK
    );
}

/// R17 cap_get_User=0x80004001 user=untouched cap_RequestAccessAsync=0x80004001 operation=NULL
#[test]
fn oracle_r17_get_user_and_request_access_async_are_stubs() {
    let cap = app_capability("nonexistent");
    assert_eq!(cap.get_user(), E_NOTIMPL);
    assert_eq!(cap.request_access_async(), E_NOTIMPL);
}

/// R18 cap_GetIids=0x80004001 cap_GetRuntimeClassName=0x80004001 cap_GetTrustLevel=0x80004001
#[test]
fn oracle_r18_capability_inspectable_stubs() {
    let cap = app_capability("nonexistent");
    assert_eq!(cap.get_iids(), E_NOTIMPL);
    assert_eq!(cap.get_runtime_class_name(), E_NOTIMPL);
    assert_eq!(cap.get_trust_level(), E_NOTIMPL);
}

/// R19 DllGetClassObject=0x80040111 out=untouched
#[test]
fn oracle_r19_dll_get_class_object() {
    assert_eq!(dll_get_class_object(), CLASS_E_CLASSNOTAVAILABLE);
}

/// R20 DllGetActivationFactory_class=0x00000000/ActivationFactory/ref2
#[test]
fn oracle_r20_activation_factory_for_class_name() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide(CLASS_NAME)),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 2);
}

/// R21 DllGetActivationFactory_nonexistent=0x80040111/NULL/ref1 DllGetActivationFactory_null=0x80040111/NULL/ref1
/// The NULL HSTRING is modelled as an empty raw buffer.
#[test]
fn oracle_r21_activation_factory_unknown_and_null() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut s, &wide("nonexistent")),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
    assert_eq!(
        dll_get_activation_factory(&mut s, &[]),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
}

/// R22 DllGetActivationFactory_embedded_nul=0x00000000/ActivationFactory/ref2
#[test]
fn oracle_r22_activation_factory_ignores_text_after_nul() {
    let mut s = CapabilityStatics::default();
    let name = wide("Windows.Security.Authorization.AppCapabilityAccess.AppCapability\0junk");
    assert_eq!(
        dll_get_activation_factory(&mut s, &name),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 2);
}

/// R23 DllGetActivationFactory_prefix=0x80040111/NULL/ref1 DllGetActivationFactory_lowercase=0x80040111/NULL/ref1
#[test]
fn oracle_r23_activation_factory_prefix_and_lowercase_rejected() {
    let mut s = CapabilityStatics::default();
    assert_eq!(
        dll_get_activation_factory(
            &mut s,
            &wide("Windows.Security.Authorization.AppCapabilityAccess")
        ),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
    assert_eq!(
        dll_get_activation_factory(
            &mut s,
            &wide("windows.security.authorization.appcapabilityaccess.appcapability")
        ),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(s.ref_count, 1);
}

/// R24 DllGetActivationFactory_first=0x00000000/ActivationFactory/ref2 DllGetActivationFactory_second=0x00000000/ActivationFactory/ref3
#[test]
fn oracle_r24_activation_factory_each_call_adds_a_reference() {
    let mut s = CapabilityStatics::default();
    let name = wide(CLASS_NAME);
    assert_eq!(
        dll_get_activation_factory(&mut s, &name),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 2);
    assert_eq!(
        dll_get_activation_factory(&mut s, &name),
        Ok(Interface::ActivationFactory)
    );
    assert_eq!(s.ref_count, 3);
}
