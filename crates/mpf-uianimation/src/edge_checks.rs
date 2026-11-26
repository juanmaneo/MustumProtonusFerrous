//! Edge-case tests for the ported uianimation logic.
//!
//! These cover the CLSID and IID boundaries, the `QueryInterface` acceptance rules, the export
//! table and the pass-through of the resource registration closures. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the values come from the uuid text in include/uianimation.idl and
    // include/unknwn.idl, and the HRESULT values from include/winerror.h.
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(IID_IUNKNOWN, 0x0000_0000_0000_0000_c000_0000_0000_0046);
    assert_eq!(IID_ICLASSFACTORY, 0x0000_0001_0000_0000_c000_0000_0000_0046);
}

#[test]
fn class_for_clsid_maps_each_class_and_rejects_others() {
    // Not oracle-derived: the boundary inputs. The four CLSIDs are covered by the oracle rows.
    assert_eq!(
        class_for_clsid(CLSID_UIANIMATION_MANAGER),
        Some(ClassKind::Manager)
    );
    assert_eq!(
        class_for_clsid(CLSID_UIANIMATION_TIMER),
        Some(ClassKind::Timer)
    );
    assert_eq!(
        class_for_clsid(CLSID_UIANIMATION_TRANSITION_FACTORY),
        Some(ClassKind::TransitionFactory)
    );
    assert_eq!(
        class_for_clsid(CLSID_UIANIMATION_TRANSITION_LIBRARY),
        Some(ClassKind::TransitionLibrary)
    );

    assert_eq!(class_for_clsid(0), None);
    assert_eq!(class_for_clsid(IID_IUNKNOWN), None);
    assert_eq!(class_for_clsid(IID_IUIANIMATION_MANAGER), None);
    // One bit away from the manager CLSID: the match must be exact.
    assert_eq!(class_for_clsid(CLSID_UIANIMATION_MANAGER ^ 1), None);
}

#[test]
fn class_iid_returns_the_interface_of_each_class() {
    // Not oracle-derived: the mapping is the one the QueryInterface rows exercise.
    assert_eq!(class_iid(ClassKind::Manager), IID_IUIANIMATION_MANAGER);
    assert_eq!(class_iid(ClassKind::Timer), IID_IUIANIMATION_TIMER);
    assert_eq!(
        class_iid(ClassKind::TransitionFactory),
        IID_IUIANIMATION_TRANSITION_FACTORY
    );
    assert_eq!(
        class_iid(ClassKind::TransitionLibrary),
        IID_IUIANIMATION_TRANSITION_LIBRARY
    );
}

#[test]
fn query_interface_accepts_iunknown_and_the_own_iid_only() {
    // Not oracle-derived: the acceptance rule for each branch of the shared QueryInterface.
    assert_eq!(
        query_interface(7u8, IID_IUNKNOWN, IID_ICLASSFACTORY),
        (S_OK, Out::Set(7))
    );
    assert_eq!(
        query_interface(7u8, IID_ICLASSFACTORY, IID_ICLASSFACTORY),
        (S_OK, Out::Set(7))
    );
    assert_eq!(
        query_interface(7u8, IID_ICLASSFACTORY, IID_IUNKNOWN),
        (E_NOINTERFACE, Out::Null)
    );
    // One bit away from IUnknown: the match must be exact.
    assert_eq!(
        query_interface(7u8, IID_IUNKNOWN ^ 1, 0),
        (E_NOINTERFACE, Out::Null)
    );
}

#[test]
fn class_factory_query_interface_accepts_iclassfactory() {
    // Not oracle-derived: IClassFactory is the factory's own interface, so it is accepted in
    // addition to IUnknown. The oracle rows cover the IUnknown and IUIAnimationManager cases.
    assert_eq!(
        class_factory_query_interface(ClassKind::Timer, IID_ICLASSFACTORY),
        (S_OK, Out::Set(ClassKind::Timer))
    );
    assert_eq!(
        class_factory_query_interface(ClassKind::Timer, IID_IUIANIMATION_TIMER),
        (E_NOINTERFACE, Out::Null)
    );
}

#[test]
fn create_instance_never_leaves_the_out_parameter_untouched() {
    // Not oracle-derived: the C code always clears *obj before calling the creator, so the
    // result is Set or Null for every class. The oracle covers one IID per class.
    for kind in [
        ClassKind::Manager,
        ClassKind::Timer,
        ClassKind::TransitionFactory,
        ClassKind::TransitionLibrary,
    ] {
        assert_eq!(
            class_factory_create_instance(kind, class_iid(kind)),
            (S_OK, Out::Set(kind))
        );
        assert_eq!(
            class_factory_create_instance(kind, IID_ICLASSFACTORY),
            (E_NOINTERFACE, Out::Null)
        );
        assert_eq!(
            class_factory_create_instance(kind, IID_IUNKNOWN),
            (S_OK, Out::Set(kind))
        );
    }
}

#[test]
fn dll_get_class_object_leaves_out_untouched_only_for_unknown_clsid() {
    // Not oracle-derived: the None branch with a valid IID. The oracle covers it with the storyboard IID.
    assert_eq!(
        dll_get_class_object(0, IID_IUNKNOWN),
        (CLASS_E_CLASSNOTAVAILABLE, Out::Untouched)
    );
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_TIMER, IID_ICLASSFACTORY),
        (S_OK, Out::Set(ClassKind::Timer))
    );
}

#[test]
fn class_factory_counts_and_lock_are_fixed() {
    // Not oracle-derived: the oracle rows call each method once. These calls repeat them.
    assert_eq!(class_factory_add_ref(), 2);
    assert_eq!(class_factory_add_ref(), 2);
    assert_eq!(class_factory_release(), 1);
    assert_eq!(class_factory_lock_server(false), S_OK);
    assert_eq!(class_factory_lock_server(true), S_OK);
}

#[test]
fn dll_can_unload_now_is_always_s_false() {
    // Not oracle-derived: repeated calls. The oracle covers one call.
    assert_eq!(dll_can_unload_now(), S_FALSE);
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn registration_passes_the_closure_result_through_once() {
    // Not oracle-derived: a non-zero success HRESULT and a repeated call. The oracle covers
    // the S_OK and E_OUTOFMEMORY results.
    let calls = Cell::new(0);
    let hr = dll_register_server(|| {
        calls.set(calls.get() + 1);
        S_FALSE
    });
    assert_eq!((hr, calls.get()), (S_FALSE, 1));

    let calls = Cell::new(0);
    let hr = dll_unregister_server(|| {
        calls.set(calls.get() + 1);
        E_NOINTERFACE
    });
    assert_eq!((hr, calls.get()), (E_NOINTERFACE, 1));
}

#[test]
fn export_table_matches_the_spec_order() {
    // Not oracle-derived: the names and order come from dlls/uianimation/uianimation.spec.
    assert_eq!(
        EXPORTS,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer"
        ]
    );
    for name in EXPORTS {
        assert!(is_export(name));
    }
    assert!(!is_export("DllMain"));
    assert!(!is_export("dllcanunloadnow"));
    assert!(!is_export(""));
}
