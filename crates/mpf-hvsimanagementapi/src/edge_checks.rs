//! Edge-case tests for the ported hvsimanagementapi logic.
//!
//! These cover the name and IID boundaries, the refcount over several calls, and the stub table.
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

const HOST: &str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";

fn wide(name: &str) -> Vec<u16> {
    name.encode_utf16().collect()
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h (E_NOINTERFACE, E_NOTIMPL,
    // CLASS_E_CLASSNOTAVAILABLE) and S_OK = 0. The oracle rows return the same bit patterns.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as i32);
    assert_eq!(E_NOTIMPL, 0x8000_4001_u32 as i32);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111_u32 as i32);
}

#[test]
fn default_count_is_one() {
    // Not oracle-derived: the static initialiser `ref = 1` in host.c. Every oracle row starts
    // from ref=1 and the rows that succeed show ref 1->2.
    assert_eq!(IsolatedHostStatics::default().ref_count(), 1);
}

#[test]
fn leading_nul_makes_the_name_empty() {
    // Not oracle-derived: wcscmp stops at a NUL in the first position, so the name is empty
    // and cannot match. Row 06 covers a NUL after the host name.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(&format!("\0{HOST}"))),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(host.ref_count(), 1);
}

#[test]
fn extra_character_after_the_name_does_not_match() {
    // Not oracle-derived: row 08 covers a truncated name, and this covers one extra character.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(
        dll_get_activation_factory(&mut host, &wide(&format!("{HOST}2"))),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(host.ref_count(), 1);
}

#[test]
fn each_successful_activation_takes_one_reference() {
    // Not oracle-derived: the oracle activates once per fresh object. Each success calls
    // QueryInterface, so the count goes up by one per call.
    let mut host = IsolatedHostStatics::default();
    for expected in 2..=4 {
        dll_get_activation_factory(&mut host, &wide(HOST)).unwrap();
        assert_eq!(host.ref_count(), expected);
    }
}

#[test]
fn failed_activation_leaves_the_count_alone() {
    // Not oracle-derived: rows 04 and 07 cover one failure each. This checks that repeated
    // failures do not change the count either.
    let mut host = IsolatedHostStatics::default();
    for _ in 0..3 {
        assert_eq!(
            dll_get_activation_factory(
                &mut host,
                &wide("Windows.System.DispatcherQueueController")
            ),
            Err(CLASS_E_CLASSNOTAVAILABLE)
        );
    }
    assert_eq!(host.ref_count(), 1);
}

#[test]
fn every_iid_is_classified() {
    // Not oracle-derived for IInspectable, IActivationFactory and the statics IID taken
    // through the same QueryInterface. Rows 10 to 14 covered IUnknown, IAgileObject, the
    // statics IID and IDispatch. The C `if` chain in factory_QueryInterface gives the rest.
    let mut host = IsolatedHostStatics::default();
    for iid in [
        Iid::IUnknown,
        Iid::IInspectable,
        Iid::IAgileObject,
        Iid::IActivationFactory,
    ] {
        assert_eq!(host.query_interface(iid), Ok(Interface::ActivationFactory));
    }
    assert_eq!(
        host.query_interface(Iid::IIsolatedWindowsEnvironmentHostStatics),
        Ok(Interface::IsolatedWindowsEnvironmentHostStatics)
    );
    assert_eq!(host.ref_count(), 6);

    assert_eq!(host.query_interface(Iid::Other), Err(E_NOINTERFACE));
    assert_eq!(host.ref_count(), 6);
}

#[test]
fn release_below_minus_one_stays_a_ulong() {
    // Not oracle-derived: row 17 stops at -1. This continues to -2, where the ULONG result
    // is 0xFFFFFFFE.
    let mut host = IsolatedHostStatics::default();
    host.release();
    host.release();
    assert_eq!(host.release(), 0xFFFF_FFFE);
    assert_eq!(host.ref_count(), -2);
}

#[test]
fn add_ref_from_a_negative_count_counts_up() {
    // Not oracle-derived: AddRef from a negative count, which the C code also allows.
    let mut host = IsolatedHostStatics::default();
    host.release();
    host.release();
    assert_eq!(host.add_ref(), 0);
    assert_eq!(host.ref_count(), 0);
}

#[test]
fn every_stub_method_returns_e_notimpl() {
    // Not oracle-derived as a set: rows 18 to 22 and 24 each cover one variant. This checks
    // all five at once, so a change to any of them is caught.
    for method in [
        StubMethod::FactoryGetIids,
        StubMethod::FactoryGetRuntimeClassName,
        StubMethod::FactoryGetTrustLevel,
        StubMethod::FactoryActivateInstance,
        StubMethod::StaticsGetHostErrors,
    ] {
        assert_eq!(stub_method(method), E_NOTIMPL);
    }
}

#[test]
fn get_is_ready_does_not_depend_on_the_count() {
    // Not oracle-derived: row 23 covers one call from ref=1. This checks that a later count
    // gives the same result.
    let mut host = IsolatedHostStatics::default();
    assert_eq!(host.get_is_ready(), (S_OK, false));
    host.add_ref();
    assert_eq!(host.get_is_ready(), (S_OK, false));
}
