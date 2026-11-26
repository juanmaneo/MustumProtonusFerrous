//! Edge-case tests for the ported clusapi logic.
//!
//! Expectations marked "not oracle-derived" come from reading `clusapi.c`,
//! `winerror.h`, `winnt.h`, `winternl.h` or `clusapi.spec`. They are not
//! harness output.

use super::*;

#[test]
fn get_cluster_information_stores_zero_whatever_the_old_length() {
    // Not oracle-derived: the C body stores 0 unconditionally.
    for old in [0, 1, 260, u32::MAX] {
        let mut cch = old;
        assert_eq!(get_cluster_information(&mut cch), ERROR_SUCCESS);
        assert_eq!(cch, 0);
    }
}

#[test]
fn get_node_cluster_state_always_reports_not_installed() {
    // Not oracle-derived: the C body stores 0 regardless of the old state.
    for old in [
        CLUSTER_STATE_NOT_INSTALLED,
        CLUSTER_STATE_NOT_CONFIGURED,
        CLUSTER_STATE_NOT_RUNNING,
        CLUSTER_STATE_RUNNING,
        u32::MAX,
    ] {
        let mut state = old;
        assert_eq!(get_node_cluster_state(&mut state), ERROR_SUCCESS);
        assert_eq!(state, CLUSTER_STATE_NOT_INSTALLED);
    }
}

#[test]
fn cluster_state_values_match_the_clusapi_c_comment() {
    // Not oracle-derived: values from the `PARAMS` comment of `GetNodeClusterState`.
    assert_eq!(CLUSTER_STATE_NOT_INSTALLED, 0x00);
    assert_eq!(CLUSTER_STATE_NOT_CONFIGURED, 0x01);
    assert_eq!(CLUSTER_STATE_NOT_RUNNING, 0x03);
    assert_eq!(CLUSTER_STATE_RUNNING, 0x13);
}

#[test]
fn winerror_constants_match_winerror_h() {
    // Not oracle-derived: `ERROR_SUCCESS` (line 222) and `ERROR_NO_MORE_ITEMS` (line 411).
    assert_eq!(ERROR_SUCCESS, 0);
    assert_eq!(ERROR_NO_MORE_ITEMS, 259);
}

#[test]
fn stub_handle_is_the_clusapi_c_literal() {
    // Not oracle-derived: `(HCLUSTER)0xdeadbeef` in `clusapi.c`.
    assert_eq!(STUB_HANDLE, 0xdead_beef);
}

#[test]
fn cluster_enum_reports_no_more_items_for_any_length() {
    // Not oracle-derived: the C body ignores the length it reads for its trace.
    for cch in [0, 260, u32::MAX] {
        assert_eq!(cluster_enum(&cch), ERROR_NO_MORE_ITEMS);
    }
}

#[test]
fn close_cluster_and_handle_stubs_take_no_state() {
    // Not oracle-derived: every stub returns the same value on every call.
    assert!(close_cluster());
    assert!(close_cluster());
    assert_eq!(open_cluster(), cluster_open_enum());
    assert_eq!(cluster_close_enum(), ERROR_SUCCESS);
}

#[test]
fn spec_stub_names_are_unique_and_sorted() {
    // Not oracle-derived: the names are copied from `clusapi.spec` `stub` lines,
    // which are in ascending byte order with no repeats.
    assert_eq!(SPEC_STUBS.len(), 108);
    for pair in SPEC_STUBS.windows(2) {
        assert!(pair[0] < pair[1], "{} then {}", pair[0], pair[1]);
    }
}

#[test]
fn spec_stub_names_are_recognised() {
    // Not oracle-derived: every name in the table answers `is_spec_stub`.
    for name in SPEC_STUBS {
        assert!(is_spec_stub(name), "{name}");
    }
}

#[test]
fn implemented_exports_are_not_spec_stubs() {
    // Not oracle-derived: the seven `stdcall` lines of `clusapi.spec` are not `stub` lines.
    for name in [
        "CloseCluster",
        "ClusterCloseEnum",
        "ClusterEnum",
        "ClusterOpenEnum",
        "GetClusterInformation",
        "GetNodeClusterState",
        "OpenCluster",
    ] {
        assert!(!is_spec_stub(name), "{name}");
    }
}

#[test]
fn spec_stub_lookup_is_exact() {
    // Not oracle-derived: lookup is by exact, case-sensitive name.
    assert!(is_spec_stub("CloseClusterGroup"));
    assert!(!is_spec_stub(""));
    assert!(!is_spec_stub("closecluster"));
    assert!(!is_spec_stub("CloseClusterGroup "));
    assert!(!is_spec_stub("CloseClusterGroupX"));
}

#[test]
fn exception_constants_match_winnt_and_winternl() {
    // Not oracle-derived: `EXCEPTION_NONCONTINUABLE` in `winnt.h`, `EXCEPTION_WINE_STUB` in `winternl.h`.
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
}
