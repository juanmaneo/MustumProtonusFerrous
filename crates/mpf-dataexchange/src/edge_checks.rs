//! Edge-case tests for the ported dataexchange logic.
//!
//! None of these expectations come from the oracle run; the oracle vectors are in
//! `oracle_checks.rs`. Each test says whether it follows Wine's `tests/dataexchange.c` or
//! reads `main.c` and `winerror.h` directly.

use super::*;

/// A fresh manager as `GetForWindow(ordinary, IUnknown)` returns it, with one reference.
fn manager() -> CoreDragDropManager {
    create_core_dragdrop_manager(Window::Ordinary, true, Iid::IUnknown).unwrap()
}

#[test]
fn hresult_from_win32_passes_zero_and_negative_through() {
    // Not oracle-derived: HRESULT_FROM_WIN32 passes values <= 0 through unchanged.
    assert_eq!(hresult_from_win32(0), 0);
    assert_eq!(hresult_from_win32(0x8000_0000), i32::MIN);
    assert_eq!(hresult_from_win32(u32::MAX), -1);
}

#[test]
fn hresult_from_win32_maps_positive_codes() {
    // Not oracle-derived: keeps the low 16 bits, then adds FACILITY_WIN32 and the severity bit.
    assert_eq!(hresult_from_win32(1), 0x8007_0001u32 as i32);
    assert_eq!(hresult_from_win32(0x1_0000), 0x8007_0000u32 as i32);
    assert_eq!(hresult_from_win32(0x7FFF_FFFF), 0x8007_FFFFu32 as i32);
}

#[test]
fn factory_query_interface_accepts_inspectable_agile_and_activation_factory() {
    // Not oracle-derived: Wine tests/dataexchange.c lines 74-83 expect the statics to return
    // the factory object for IInspectable and IAgileObject.
    for iid in [
        Iid::IInspectable,
        Iid::IAgileObject,
        Iid::IActivationFactory,
    ] {
        let mut f = DataExchange::static_object();
        assert_eq!(
            f.query_interface(iid),
            Ok(FactoryInterface::ActivationFactory)
        );
        assert_eq!(f.ref_count(), 2);
    }
}

#[test]
fn factory_query_interface_returns_interop_forwarder() {
    // Not oracle-derived: Wine tests/dataexchange.c line 102 queries the interop from the statics.
    let mut f = DataExchange::static_object();
    assert_eq!(
        f.query_interface(Iid::IDragDropManagerInterop),
        Ok(FactoryInterface::DragDropManagerInterop)
    );
    assert_eq!(f.ref_count(), 2);
}

#[test]
fn factory_query_interface_rejects_unknown_iid_without_add_ref() {
    // Not oracle-derived: main.c returns E_NOINTERFACE and takes no reference.
    let mut f = DataExchange::static_object();
    assert_eq!(f.query_interface(Iid::Other), Err(E_NOINTERFACE));
    assert_eq!(f.ref_count(), 1);
}

#[test]
fn factory_release_wraps_below_zero() {
    // Not oracle-derived: InterlockedDecrement on a LONG wraps, and the ULONG return reads back
    // as a large unsigned value.
    let mut f = DataExchange::static_object();
    assert_eq!(f.release(), 0);
    assert_eq!(f.release(), u32::MAX);
    assert_eq!(f.ref_count(), u32::MAX);
}

#[test]
fn dll_get_activation_factory_adds_one_reference_per_call() {
    // Not oracle-derived: the oracle calls DllGetActivationFactory once (L05).
    let mut f = DataExchange::static_object();
    assert_eq!(dll_get_activation_factory(&mut f), S_OK);
    assert_eq!(dll_get_activation_factory(&mut f), S_OK);
    assert_eq!(f.ref_count(), 3);
}

#[test]
fn manager_query_interface_accepts_inspectable_and_self() {
    // Not oracle-derived: main.c accepts IInspectable and ICoreDragDropManager on the manager.
    for iid in [Iid::IInspectable, Iid::ICoreDragDropManager] {
        let mut m = manager();
        assert_eq!(m.query_interface(iid), Ok(()));
        assert_eq!(m.ref_count(), 2);
    }
}

#[test]
fn manager_query_interface_rejects_agile_factory_and_other_without_add_ref() {
    // Not oracle-derived: the manager does not answer IAgileObject, unlike the factory.
    for iid in [Iid::IAgileObject, Iid::IActivationFactory, Iid::Other] {
        let mut m = manager();
        assert_eq!(m.query_interface(iid), Err(E_NOINTERFACE));
        assert_eq!(m.ref_count(), 1);
    }
}

#[test]
fn manager_release_from_two_reports_one() {
    // Not oracle-derived: a plain decrement. The oracle covers only the 1 -> 0 case (L12).
    let mut m = manager();
    m.add_ref();
    assert_eq!(m.release(), 1);
    assert_eq!(m.ref_count(), 1);
}

#[test]
fn create_invalid_window_reports_handle_error_before_allocation() {
    // Not oracle-derived: main.c checks IsWindow before calloc, so a failed allocation does not
    // change the result.
    assert_eq!(
        create_core_dragdrop_manager(Window::Invalid, false, Iid::IUnknown).unwrap_err(),
        0x8007_0578u32 as i32
    );
}

#[test]
fn create_desktop_reports_access_denied_before_allocation() {
    // Not oracle-derived: main.c checks GetDesktopWindow before calloc, so a failed allocation
    // does not change the result.
    assert_eq!(
        create_core_dragdrop_manager(Window::Desktop, false, Iid::IUnknown).unwrap_err(),
        E_ACCESSDENIED
    );
}

#[test]
fn create_ordinary_with_inspectable_returns_manager_with_one_reference() {
    // Not oracle-derived: the manager QueryInterface accepts IInspectable.
    let m = create_core_dragdrop_manager(Window::Ordinary, true, Iid::IInspectable).unwrap();
    assert_eq!(m.ref_count(), 1);
}

#[test]
fn create_ordinary_with_manager_iid_returns_one_reference() {
    // Not oracle-derived: Wine tests/dataexchange.c lines 118-119 query the manager IID.
    let m =
        create_core_dragdrop_manager(Window::Ordinary, true, Iid::ICoreDragDropManager).unwrap();
    assert_eq!(m.ref_count(), 1);
}

#[test]
fn create_ordinary_with_other_iid_fails() {
    // Not oracle-derived: the query fails, the reference is dropped, and no manager is returned.
    assert_eq!(
        create_core_dragdrop_manager(Window::Ordinary, true, Iid::Other).unwrap_err(),
        E_NOINTERFACE
    );
}
