//! Oracle vectors for the ported dataexchange logic.
//!
//! The vectors come from the verbatim Wine bodies in `dlls/dataexchange/main.c`, compiled with
//! the shims in `target/canon/dataexchange/private.h` under `gcc -fsanitize=address,undefined -g`.
//! The harness printed 25 rows (`rows.txt`). ASan and UBSan flagged none, so no row is excluded.
//! Expected values are the literal hex the harness printed, not crate constants.
//!
//! `refs` is the count after the call. Rows printed as `refs=-1` have no count to check.
//! `frees` is modelled by the manager being dropped on the error path, and by `release`
//! returning zero.

use super::*;

/// An `HRESULT` written as the hex the harness printed.
fn hr(bits: u32) -> Hresult {
    bits as Hresult
}

fn ordinary_manager() -> CoreDragDropManager {
    create_core_dragdrop_manager(Window::Ordinary, true, Iid::IUnknown).unwrap()
}

#[test]
fn oracle_l01_factory_query_interface_iunknown() {
    let mut f = DataExchange::static_object();
    assert_eq!(
        f.query_interface(Iid::IUnknown),
        Ok(FactoryInterface::ActivationFactory)
    );
    assert_eq!(f.ref_count(), 2);
}

#[test]
fn oracle_l02_factory_query_interface_statics_iid() {
    let mut f = DataExchange::static_object();
    assert_eq!(
        f.query_interface(Iid::ICoreDragDropManagerStatics),
        Ok(FactoryInterface::CoreDragDropManagerStatics)
    );
    assert_eq!(f.ref_count(), 2);
}

#[test]
fn oracle_l03_factory_query_interface_manager_iid() {
    let mut f = DataExchange::static_object();
    assert_eq!(
        f.query_interface(Iid::ICoreDragDropManager),
        Err(hr(0x8000_4002))
    );
    assert_eq!(f.ref_count(), 1);
}

#[test]
fn oracle_l04_factory_add_ref() {
    let mut f = DataExchange::static_object();
    assert_eq!(f.add_ref(), 2);
    assert_eq!(f.ref_count(), 2);
}

#[test]
fn oracle_l05_dll_get_activation_factory() {
    // The oracle's `out` is the static factory. The count is the observable effect here.
    let mut f = DataExchange::static_object();
    assert_eq!(dll_get_activation_factory(&mut f), hr(0x0000_0000));
    assert_eq!(f.ref_count(), 2);
}

#[test]
fn oracle_l06_get_for_window_invalid() {
    let f = DataExchange::static_object();
    assert_eq!(
        f.get_for_window(Window::Invalid, true, Iid::IUnknown)
            .unwrap_err(),
        hr(0x8007_0578)
    );
}

#[test]
fn oracle_l07_get_for_window_desktop() {
    let f = DataExchange::static_object();
    assert_eq!(
        f.get_for_window(Window::Desktop, true, Iid::IUnknown)
            .unwrap_err(),
        hr(0x8007_0005)
    );
}

#[test]
fn oracle_l08_get_for_window_alloc_fails() {
    let f = DataExchange::static_object();
    assert_eq!(
        f.get_for_window(Window::Ordinary, false, Iid::IUnknown)
            .unwrap_err(),
        hr(0x8007_000e)
    );
}

#[test]
fn oracle_l09_get_for_window_iunknown() {
    let f = DataExchange::static_object();
    let m = f
        .get_for_window(Window::Ordinary, true, Iid::IUnknown)
        .unwrap();
    assert_eq!(m.ref_count(), 1);
}

#[test]
fn oracle_l10_get_for_window_agile_object() {
    // The oracle's failed query frees the manager, which Rust models by dropping it.
    let f = DataExchange::static_object();
    assert_eq!(
        f.get_for_window(Window::Ordinary, true, Iid::IAgileObject)
            .unwrap_err(),
        hr(0x8000_4002)
    );
}

#[test]
fn oracle_l11_manager_query_interface_iunknown() {
    let mut m = ordinary_manager();
    assert_eq!(m.query_interface(Iid::IUnknown), Ok(()));
    assert_eq!(m.ref_count(), 2);
}

#[test]
fn oracle_l12_manager_release_last() {
    let mut m = ordinary_manager();
    assert_eq!(m.release(), 0);
    assert_eq!(m.ref_count(), 0);
}

#[test]
fn oracle_s01_factory_get_iids() {
    let f = DataExchange::static_object();
    assert_eq!(f.get_iids(), hr(0x8000_4001));
}

#[test]
fn oracle_s02_factory_get_runtime_class_name() {
    let f = DataExchange::static_object();
    assert_eq!(f.get_runtime_class_name(), hr(0x8000_4001));
}

#[test]
fn oracle_s03_factory_get_trust_level() {
    let f = DataExchange::static_object();
    assert_eq!(f.get_trust_level(), hr(0x8000_4001));
}

#[test]
fn oracle_s04_factory_activate_instance() {
    let f = DataExchange::static_object();
    assert_eq!(f.activate_instance(), hr(0x8000_4001));
}

#[test]
fn oracle_s05_statics_get_for_current_view() {
    let f = DataExchange::static_object();
    assert_eq!(f.get_for_current_view(), hr(0x8000_4001));
}

#[test]
fn oracle_s06_manager_get_iids() {
    let m = ordinary_manager();
    assert_eq!(m.get_iids(), hr(0x8000_4001));
}

#[test]
fn oracle_s07_manager_get_runtime_class_name() {
    let m = ordinary_manager();
    assert_eq!(m.get_runtime_class_name(), hr(0x8000_4001));
}

#[test]
fn oracle_s08_manager_get_trust_level() {
    let m = ordinary_manager();
    assert_eq!(m.get_trust_level(), hr(0x8000_4001));
}

#[test]
fn oracle_s09_manager_add_target_requested() {
    let m = ordinary_manager();
    assert_eq!(m.add_target_requested(), (hr(0x0000_0000), 0xdead_beef));
}

#[test]
fn oracle_s10_manager_remove_target_requested() {
    let m = ordinary_manager();
    assert_eq!(m.remove_target_requested(), hr(0x8000_4001));
}

#[test]
fn oracle_s11_manager_get_are_concurrent_operations_enabled() {
    let m = ordinary_manager();
    assert_eq!(m.get_are_concurrent_operations_enabled(), hr(0x8000_4001));
}

#[test]
fn oracle_s12_manager_put_are_concurrent_operations_enabled() {
    let m = ordinary_manager();
    assert_eq!(m.put_are_concurrent_operations_enabled(), hr(0x8000_4001));
}

#[test]
fn oracle_s13_dll_get_class_object() {
    assert_eq!(dll_get_class_object(), hr(0x8004_0111));
}
