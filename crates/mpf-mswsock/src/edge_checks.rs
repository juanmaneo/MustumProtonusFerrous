//! Boundary and branch tests for the mswsock port. The oracle rows in `oracle_checks.rs` cover the
//! C paths. The tests here cover the state that the rows do not reach directly. Each expectation
//! that is not oracle-derived is marked with a comment.

use super::*;

#[test]
fn default_state_is_uninitialised() {
    // Not oracle-derived: the C statics start NULL and FALSE. F09 shows `init=0` after one call.
    let ext: Extensions<&'static str> = Extensions::default();
    assert!(!ext.initialised());
}

#[test]
fn get_accept_ex_sockaddrs_calls_only_a_filled_slot_and_never_initialises() {
    // Not oracle-derived: F09 shows an empty call leaves `init=0`, and F12 shows the call after
    // AcceptEx. This test runs both states on one instance, and checks that the name in the slot
    // reaches `call`.
    let mut ext: Extensions<&'static str> = Extensions::default();
    let mut seen = Vec::new();
    let mut record = |f: &'static str| seen.push(f);
    ext.get_accept_ex_sockaddrs(&mut record);
    assert!(!ext.initialised());
    let mut lookups = 0;
    let mut ioctl = |_: u32, _: &Guid| {
        lookups += 1;
        Some("cached")
    };
    assert!(ext.accept_ex(1u32, &mut ioctl, |_| true));
    assert!(ext.initialised());
    ext.get_accept_ex_sockaddrs(&mut record);
    assert_eq!(seen, ["cached"]);
    assert_eq!(lookups, 3);
}

#[test]
fn lookups_run_once_across_all_three_pointers() {
    // Not oracle-derived: counts the lookups. F05 and F07 show that a later call does not look up
    // again, and this test counts the three lookups that the first call makes.
    let mut lookups = 0;
    let mut ioctl = |_: u32, _: &Guid| {
        lookups += 1;
        Some("pointer")
    };
    let mut ext = Extensions::default();
    assert!(ext.accept_ex(1u32, &mut ioctl, |_| true));
    assert!(ext.transmit_file(2u32, &mut ioctl, |_| true));
    assert_eq!(lookups, 3);
}

#[test]
fn empty_slots_are_not_called_and_stay_empty_after_initialisation() {
    // Not oracle-derived: only the AcceptEx lookup succeeds here, so TransmitFile's slot stays
    // empty. The `call` closure runs once, for AcceptEx, and the later TransmitFile calls neither
    // call it nor look up again. F13 and F14 cover the same rule with the oracle.
    let mut lookups = 0;
    let mut ioctl = |_: u32, guid: &Guid| {
        lookups += 1;
        (*guid == WSAID_ACCEPTEX).then_some("AcceptEx")
    };
    let mut calls = 0;
    let mut call = |_: &'static str| {
        calls += 1;
        true
    };
    let mut ext: Extensions<&'static str> = Extensions::default();
    assert!(ext.accept_ex(1u32, &mut ioctl, &mut call));
    assert!(!ext.transmit_file(2u32, &mut ioctl, &mut call));
    assert!(!ext.transmit_file(2u32, &mut ioctl, &mut call));
    assert_eq!(lookups, 3);
    assert_eq!(calls, 1);
}

#[test]
fn wsa_recv_ex_returns_socket_error() {
    // Not oracle-derived: `SOCKET_ERROR` is `(-1)` in `include/winsock2.h`. W01 shows the value.
    assert_eq!(SOCKET_ERROR, -1);
    assert_eq!(wsa_recv_ex(), SOCKET_ERROR);
}

#[test]
fn stub_record_passes_its_names_through() {
    // Not oracle-derived: the oracle only used mswsock.dll names. The C body copies its two
    // arguments into `ExceptionInformation` unchanged, so other names pass through as well.
    let record = wine_spec_unimplemented_stub("other.dll", "Function");
    assert_eq!(
        record,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Function",
        }
    );
}
