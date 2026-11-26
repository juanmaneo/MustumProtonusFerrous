//! Edge-case tests for the ported uiribbon logic.
//!
//! These cover the GUID boundaries, the reference-count transitions, the untouched-output
//! rules and the stub table. Not every expectation is oracle-derived; the ones that aren't say
//! so in a comment.

use super::*;

fn c_name(method: UIFrameworkMethod) -> &'static str {
    match method {
        UIFrameworkMethod::Initialize => "Initialize",
        UIFrameworkMethod::Destroy => "Destroy",
        UIFrameworkMethod::LoadUI => "LoadUI",
        UIFrameworkMethod::GetView => "GetView",
        UIFrameworkMethod::GetUICommandProperty => "GetUICommandProperty",
        UIFrameworkMethod::SetUICommandProperty => "SetUICommandProperty",
        UIFrameworkMethod::InvalidateUICommand => "InvalidateUICommand",
        UIFrameworkMethod::FlushPendingInvalidations => "FlushPendingInvalidations",
        UIFrameworkMethod::SetModes => "SetModes",
    }
}

fn with_data4_last(guid: &Guid, last: u8) -> Guid {
    Guid {
        data1: guid.data1,
        data2: guid.data2,
        data3: guid.data3,
        data4: [0x88, 0x45, 0xc3, 0x3e, 0x65, 0xf2, 0xb9, last],
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/unknwn.idl and
    // include/uiribbon.idl. The harness uses the same GUID bytes, but the value check is here.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOTIMPL, 0x8000_4001u32 as i32);
    assert_eq!(E_NOINTERFACE, 0x8000_4002u32 as i32);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111u32 as i32);
    assert_eq!(IID_IUIFRAMEWORK.data1, 0xf4f0_385d);
    assert_eq!(IID_IUIFRAMEWORK.data4[7], 0xc5);
    assert_eq!(CLSID_UIRIBBONFRAMEWORK.data1, 0x9267_49fa);
    assert_eq!(CLSID_UIRIBBONFRAMEWORK.data4[7], 0x57);
}

#[test]
fn clsid_compares_every_byte() {
    // Not oracle-derived: `IsEqualGUID` compares all 16 bytes. The oracle covers one near miss
    // (Data1 XOR 1). These cover the last byte, and the Data2 and Data3 fields.
    let last_byte = with_data4_last(&CLSID_UIRIBBONFRAMEWORK, 0x58);
    let (hr, factory) = dll_get_class_object(&last_byte, &IID_ICLASSFACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
    assert!(factory.is_none());

    let data2 = Guid {
        data2: 0x2616,
        ..with_data4_last(&CLSID_UIRIBBONFRAMEWORK, 0x57)
    };
    let (hr, _) = dll_get_class_object(&data2, &IID_ICLASSFACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);

    let data3 = Guid {
        data3: 0x4988,
        ..with_data4_last(&CLSID_UIRIBBONFRAMEWORK, 0x57)
    };
    let (hr, _) = dll_get_class_object(&data3, &IID_ICLASSFACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
}

#[test]
fn iid_compares_every_byte() {
    // Not oracle-derived: an IID one byte away from IClassFactory is rejected by the IID check
    // before the CLSID lookup, so the result is E_NOINTERFACE, not CLASS_E_CLASSNOTAVAILABLE.
    let near = Guid {
        data1: 0x0000_0001,
        data2: 0x0000,
        data3: 0x0000,
        data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47],
    };
    let (hr, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &near);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(factory.is_none());
}

#[test]
fn factory_release_reaches_zero_only_after_last_reference() {
    // Not oracle-derived: the reference transitions of the factory, read from `XFCF_AddRef` and
    // `XFCF_Release`. The caller drops the factory at the 0.
    let (_, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_IUNKNOWN);
    let mut factory = factory.expect("object");
    assert_eq!(factory.add_ref(), 2);
    assert_eq!(factory.release(), 1);
    assert_eq!(factory.release(), 0);
}

#[test]
fn factory_query_interface_takes_a_reference_only_on_success() {
    // Not oracle-derived: a failed query leaves the count alone. The oracle covers one failure
    // (R11). This checks the count after a sequence of successes and failures.
    let (_, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_ICLASSFACTORY);
    let mut factory = factory.expect("object");
    let mut obj = None;
    assert_eq!(factory.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(
        factory.query_interface(&IID_IUIFRAMEWORK, &mut obj),
        E_NOINTERFACE
    );
    assert_eq!(factory.query_interface(&IID_ICLASSFACTORY, &mut obj), S_OK);
    assert_eq!(factory.release(), 2);
    assert_eq!(factory.release(), 1);
}

#[test]
fn create_instance_with_unsupported_iid_drops_the_object() {
    // Not oracle-derived beyond R16 and R18: the creation reference is released after the
    // failed query, so the object is dropped and no framework is returned.
    let (_, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_ICLASSFACTORY);
    let factory = factory.expect("object");
    let (hr, framework) = factory.create_instance(&IID_ICLASSFACTORY);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(framework.is_none());
}

#[test]
fn framework_query_interface_failure_keeps_the_count() {
    // Not oracle-derived: R21 covers one failed query. This checks the count after a failure
    // with a count above one.
    let (_, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_ICLASSFACTORY);
    let (_, framework) = factory.expect("object").create_instance(&IID_IUIFRAMEWORK);
    let mut framework = framework.expect("object");
    assert_eq!(framework.add_ref(), 2);
    let mut obj = None;
    assert_eq!(framework.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(
        framework.query_interface(&IID_ICLASSFACTORY, &mut obj),
        E_NOINTERFACE
    );
    assert_eq!(framework.release(), 2);
    assert_eq!(framework.release(), 1);
    assert_eq!(framework.release(), 0);
}

#[test]
fn lock_server_has_no_effect_on_the_count() {
    // Not oracle-derived: `XFCF_LockServer` ignores its argument and does not change the count.
    // The C body has no reference change, so this follows from the source.
    let (_, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_ICLASSFACTORY);
    let mut factory = factory.expect("object");
    assert_eq!(factory.lock_server(true), S_OK);
    assert_eq!(factory.lock_server(false), S_OK);
    assert_eq!(factory.release(), 0);
}

#[test]
fn every_ui_framework_method_returns_e_notimpl() {
    // Not oracle-derived: the table covers all nine `IUIFramework` vtable entries that the C
    // code defines. The oracle covers each method once (R24 to R32).
    let names: [&str; 9] = UI_FRAMEWORK_METHODS.map(c_name);
    assert_eq!(
        names,
        [
            "Initialize",
            "Destroy",
            "LoadUI",
            "GetView",
            "GetUICommandProperty",
            "SetUICommandProperty",
            "InvalidateUICommand",
            "FlushPendingInvalidations",
            "SetModes",
        ]
    );
    for method in UI_FRAMEWORK_METHODS {
        assert_eq!(ui_framework_method(method), E_NOTIMPL);
    }
}

#[test]
fn ui_framework_methods_are_distinct() {
    // Not oracle-derived: a duplicate entry in the table would hide a missing method.
    for (i, a) in UI_FRAMEWORK_METHODS.iter().enumerate() {
        for b in &UI_FRAMEWORK_METHODS[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
