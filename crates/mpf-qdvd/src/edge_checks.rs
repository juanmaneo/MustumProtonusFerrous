//! Edge-case tests for the ported qdvd logic.
//!
//! These cover the sign boundary of `SUCCEEDED`, the field-by-field `IsEqualGUID`, both sides of
//! the class factory gate, the `DllGetClassObject` lookup, and the forwarders' single call. Not
//! every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
}

#[test]
fn succeeded_sign_boundary() {
    // Not oracle-derived: SUCCEEDED is `hr >= 0`, so the boundary sits between 0 and -1.
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(i32::MIN));
    assert!(!succeeded(E_NOINTERFACE));
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: each field of IDvdControl2 is changed on its own.
    assert!(is_equal_guid(&IID_IDVDCONTROL2, &IID_IDVDCONTROL2));
    let changed = [
        Guid {
            data1: 0x33bc_7431,
            ..IID_IDVDCONTROL2
        },
        Guid {
            data2: 0xeec1,
            ..IID_IDVDCONTROL2
        },
        Guid {
            data3: 0x11d3,
            ..IID_IDVDCONTROL2
        },
        Guid {
            data4: [0x82, 0x01, 0x00, 0xa0, 0xc9, 0xd7, 0x48, 0x43],
            ..IID_IDVDCONTROL2
        },
    ];
    for guid in &changed {
        assert!(!is_equal_guid(guid, &IID_IDVDCONTROL2));
    }
}

#[test]
fn class_factory_query_interface_is_exact() {
    // Partly oracle-derived. IUnknown, IClassFactory and IDvdControl2 are oracle rows. The
    // IDvdGraphBuilder and IDvdInfo2 cases are not, and are checked here.
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!(class_factory_query_interface(&IID_ICLASSFACTORY), S_OK);
    assert_eq!(
        class_factory_query_interface(&IID_IDVDGRAPHBUILDER),
        E_NOINTERFACE
    );
    assert_eq!(class_factory_query_interface(&IID_IDVDINFO2), E_NOINTERFACE);
}

#[test]
fn create_instance_gate_outer_and_iid_combinations() {
    // Not oracle-derived for (true, IClassFactory) and (true, IDvdControl2). The other
    // combinations are oracle rows.
    assert!(class_factory_create_instance_gate(false, &IID_IDVDGRAPHBUILDER).is_none());
    assert!(class_factory_create_instance_gate(true, &IID_IUNKNOWN).is_none());
    assert!(class_factory_create_instance_gate(true, &IID_IDVDGRAPHBUILDER) == Some(E_NOINTERFACE));
    assert!(class_factory_create_instance_gate(true, &IID_ICLASSFACTORY) == Some(E_NOINTERFACE));
    assert!(class_factory_create_instance_gate(true, &IID_IDVDCONTROL2) == Some(E_NOINTERFACE));
}

#[test]
fn dll_get_class_object_lookup_is_exact() {
    // Not oracle-derived: a CLSID one field away from each known class is unavailable, and the
    // CLSID check runs before the IID check.
    let near_graph_builder = Guid {
        data4: [0x8e, 0x00, 0x00, 0xc0, 0x4f, 0xd7, 0xc0, 0x8a],
        ..CLSID_DVDGRAPHBUILDER
    };
    let near_navigator = Guid {
        data1: 0x9b8c_4621,
        ..CLSID_DVDNAVIGATOR
    };
    assert!(
        dll_get_class_object(&near_graph_builder, &IID_IUNKNOWN) == Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert!(
        dll_get_class_object(&near_navigator, &IID_IDVDCONTROL2) == Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert!(
        dll_get_class_object(&CLSID_DVDNAVIGATOR, &IID_ICLASSFACTORY)
            == Ok(ClassFactory::Navigator)
    );
    assert!(dll_get_class_object(&CLSID_DVDGRAPHBUILDER, &IID_IDVDINFO2) == Err(E_NOINTERFACE));
}

#[test]
fn forwarders_call_the_resource_function_once() {
    // Not oracle-derived: each forwarder calls its resource function exactly once and returns
    // the result unchanged.
    let calls = Cell::new(0);
    let hr = dll_register_server(|| {
        calls.set(calls.get() + 1);
        E_NOTIMPL
    });
    assert_eq!(calls.get(), 1);
    assert_eq!(hr, E_NOTIMPL);

    let calls = Cell::new(0);
    let hr = dll_unregister_server(|| {
        calls.set(calls.get() + 1);
        S_FALSE
    });
    assert_eq!(calls.get(), 1);
    assert_eq!(hr, S_FALSE);
}

#[test]
fn graph_builder_refuses_other_iids_without_addref() {
    // Not oracle-derived: a refused query leaves the reference count unchanged, for the class
    // IID and for IDvdInfo2.
    let mut builder = graph::GraphBuilder::create();
    assert!(builder.inner_query_interface(&CLSID_DVDGRAPHBUILDER) == Err(E_NOINTERFACE));
    assert!(builder.dvd_query_interface(&IID_IDVDINFO2) == Err(E_NOINTERFACE));
    assert_eq!(builder.refcount(), 1);
}

#[test]
fn navigator_query_interface_refuses_other_iids() {
    // Not oracle-derived: the class factory IID and a one-field near miss of IDvdInfo2 are refused.
    assert!(navigator_query_interface(&IID_ICLASSFACTORY) == Err(E_NOINTERFACE));
    let near_info2 = Guid {
        data1: 0x3415_1511,
        ..IID_IDVDINFO2
    };
    assert!(navigator_query_interface(&near_info2) == Err(E_NOINTERFACE));
}
