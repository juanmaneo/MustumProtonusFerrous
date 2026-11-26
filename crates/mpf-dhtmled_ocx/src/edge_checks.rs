//! Edge-case tests for the ported dhtmled.ocx logic.
//!
//! These cover every branch and match arm. Expectations that are not oracle rows are
//! marked "source-derived": they follow from the Wine C code by reading it, not from a
//! harness run.

use super::*;

const ALL_IIDS: [Iid; 23] = [
    Iid::IUnknown,
    Iid::IDispatch,
    Iid::IDHTMLEdit,
    Iid::IOleObject,
    Iid::IProvideClassInfo,
    Iid::IProvideClassInfo2,
    Iid::IPersistStorage,
    Iid::IPersistStreamInit,
    Iid::IOleControl,
    Iid::IViewObject,
    Iid::IViewObject2,
    Iid::IViewObjectEx,
    Iid::IOleWindow,
    Iid::IOleInPlaceObject,
    Iid::IOleInPlaceObjectWindowless,
    Iid::IOleInPlaceActiveObject,
    Iid::IConnectionPointContainer,
    Iid::IConnectionPoint,
    Iid::IDataObject,
    Iid::IServiceProvider,
    Iid::IClassFactory,
    Iid::DHTMLEditEvents,
    Iid::Other,
];

#[test]
fn qi_table_covers_every_iid() {
    // Source-derived: the if/else chain in dhtml_edit_qi, one expectation per IID.
    let expected = [
        Some(Target::DHTMLEdit),
        Some(Target::DHTMLEdit),
        Some(Target::DHTMLEdit),
        Some(Target::OleObject),
        Some(Target::ProvideClassInfo2),
        Some(Target::ProvideClassInfo2),
        Some(Target::PersistStorage),
        Some(Target::PersistStreamInit),
        Some(Target::OleControl),
        Some(Target::ViewObjectEx),
        Some(Target::ViewObjectEx),
        Some(Target::ViewObjectEx),
        Some(Target::OleInPlaceObjectWindowless),
        Some(Target::OleInPlaceObjectWindowless),
        Some(Target::OleInPlaceObjectWindowless),
        Some(Target::OleInPlaceActiveObject),
        Some(Target::ConnectionPointContainer),
        None,
        Some(Target::DataObject),
        Some(Target::ServiceProvider),
        None,
        None,
        None,
    ];
    for (iid, want) in ALL_IIDS.iter().zip(expected) {
        assert_eq!(dhtml_edit_qi(*iid), want, "{iid:?}");
    }
}

#[test]
fn object_ref_counts() {
    // Source-derived: ref starts at 1, addref/release move it by one.
    let mut refs = ObjectRef::initial();
    assert_eq!(refs.count(), 1);
    assert_eq!(refs.add_ref(), 2);
    assert_eq!(refs.release(), 1);
    assert_eq!(refs.release(), 0);
}

#[test]
fn create_failure_for_unlisted_iids() {
    // Source-derived: create frees the object when qi fails.
    for iid in [Iid::IConnectionPoint, Iid::DHTMLEditEvents, Iid::Other] {
        assert_eq!(dhtml_edit_create(iid), Err(E_NOINTERFACE));
    }
}

#[test]
fn class_factory_constants_and_lock() {
    // Source-derived: AddRef and Release return constants; LockServer always returns S_OK.
    assert_eq!(class_factory_add_ref(), 2);
    assert_eq!(class_factory_release(), 1);
    assert_eq!(class_factory_lock_server(true), S_OK);
    assert_eq!(class_factory_lock_server(false), S_OK);
    assert_eq!(class_factory_query_interface(Iid::IUnknown), S_OK);
}

#[test]
fn dll_main_other_reasons() {
    // Source-derived: any other reason (e.g. DLL_THREAD_ATTACH = 2) does nothing.
    assert_eq!(dll_main(2, false), DllMainAction::Nothing);
    assert_eq!(dll_main(3, true), DllMainAction::Nothing);
    // ATTACH ignores reserved.
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH, true),
        DllMainAction::DisableThreadLibraryCalls
    );
}

#[test]
fn dll_get_class_object_clsid_and_iid() {
    // Source-derived: the matched CLSID hands the factory's QueryInterface result through.
    assert_eq!(dll_get_class_object(Clsid::DHTMLEdit, Iid::IUnknown), S_OK);
    assert_eq!(
        dll_get_class_object(Clsid::DHTMLEdit, Iid::IDHTMLEdit),
        E_NOINTERFACE
    );
    assert_eq!(
        dll_get_class_object(Clsid::Other, Iid::IUnknown),
        CLASS_E_CLASSNOTAVAILABLE
    );
}

#[test]
fn mul_div_rounding_and_signs() {
    // Source-derived: half away from zero, with the sign of c folded into a.
    assert_eq!(mul_div(3, 1, 2), 2);
    assert_eq!(mul_div(-3, 1, 2), -2);
    assert_eq!(mul_div(3, -1, 2), -2);
    assert_eq!(mul_div(-3, -1, 2), 2);
    assert_eq!(mul_div(0, 0, 5), 0);
    assert_eq!(mul_div(0, -5, 5), 0);
    // Negative divisor: a is negated first.
    assert_eq!(mul_div(1, 1, -2), -1);
    assert_eq!(mul_div(-1, -1, -2), -1);
    assert_eq!(mul_div(1, -1, 2), -1);
}

#[test]
fn mul_div_overflow_boundaries() {
    // Source-derived: results outside +/-2147483647 become -1.
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(i32::MIN, 2, 1), -1);
    assert_eq!(mul_div(2_147_483_647, 1, 1), 2_147_483_647);
    assert_eq!(mul_div(-2_147_483_647, 1, 1), -2_147_483_647);
    // Not oracle-derived: the C negation here is UB and row R19 was excluded. The value
    // follows from the 64-bit arithmetic in mul_div (2147483648 > i32::MAX, so -1).
    assert_eq!(mul_div(i32::MIN, 1, -1), -1);
}

#[test]
fn default_extent_from_dpi() {
    // Source-derived: 96 dpi (R17 path) gives 5080; a zero DPI gives the MulDiv error value.
    assert_eq!(default_extent(96, 96), Sizel { cx: 5080, cy: 5080 });
    assert_eq!(default_extent(0, 0), Sizel { cx: -1, cy: -1 });
}

#[test]
fn set_and_get_extent_aspects() {
    // Source-derived: DVASPECT_CONTENT (1) is the only accepted aspect.
    let mut extent = Sizel { cx: 0, cy: 0 };
    assert_eq!(
        object_set_extent(&mut extent, DVASPECT_CONTENT, Sizel { cx: 3, cy: 4 }),
        S_OK
    );
    assert_eq!(extent, Sizel { cx: 3, cy: 4 });
    assert_eq!(
        object_get_extent(&extent, DVASPECT_CONTENT),
        Ok(Sizel { cx: 3, cy: 4 })
    );
    // DVASPECT_THUMBNAIL (2) is rejected by both.
    assert_eq!(object_get_extent(&extent, 2), Err(E_FAIL));
    assert_eq!(
        object_set_extent(&mut extent, 2, Sizel { cx: 9, cy: 9 }),
        DV_E_DVASPECT
    );
    assert_eq!(extent, Sizel { cx: 3, cy: 4 });
}

#[test]
fn ole_verbs_and_connection_points() {
    // Source-derived: only INPLACEACTIVATE (-5) succeeds; other verbs are not implemented.
    assert_eq!(ole_object_do_verb(-4), E_NOTIMPL);
    assert_eq!(ole_object_do_verb(1), E_NOTIMPL);
    // NULL out pointer wins over the riid check.
    assert_eq!(
        connection_point_container_find_connection_point(true, Iid::DHTMLEditEvents),
        E_POINTER
    );
    assert_eq!(
        connection_point_container_find_connection_point(false, Iid::DHTMLEditEvents),
        S_OK
    );
    assert_eq!(
        connection_point_container_find_connection_point(false, Iid::IUnknown),
        CONNECT_E_NOCONNECTION
    );
    assert_eq!(connection_point_query_interface(Iid::IUnknown), S_OK);
    assert_eq!(
        connection_point_query_interface(Iid::IConnectionPoint),
        S_OK
    );
    assert_eq!(
        connection_point_query_interface(Iid::IDispatch),
        E_NOINTERFACE
    );
    assert_eq!(service_provider_query_service(), E_NOINTERFACE);
}

#[test]
fn stub_returns() {
    // Source-derived: the two stub return shapes.
    assert_eq!(stub_result(StubReturn::NotImplemented), E_NOTIMPL);
    assert_eq!(stub_result(StubReturn::Ok), S_OK);
}

#[test]
fn success_test() {
    // Source-derived: SUCCEEDED is hr >= 0.
    assert!(is_success(S_OK));
    assert!(is_success(1));
    assert!(!is_success(E_NOTIMPL));
}
