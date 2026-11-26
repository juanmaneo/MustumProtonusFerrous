//! Edge-case tests for the ported connect logic.
//!
//! Each expectation below is either taken from the oracle or comes from the
//! reference source, as the comment on each test says.

use super::*;

#[test]
fn non_stub_exports_raise_no_fault() {
    // Derived from connect.spec: IsInternetConnected and DllCanUnloadNow are not `stub` entries.
    assert_eq!(stub_fault("IsInternetConnected"), None);
    assert_eq!(stub_fault("DllCanUnloadNow"), None);
}

#[test]
fn stub_names_are_matched_exactly() {
    // Not oracle-derived: GetProcAddress-style lookups compare names exactly, so case,
    // whitespace and prefixes must not match.
    assert_eq!(stub_fault("isinternetconnectedguid"), None);
    assert_eq!(stub_fault("UnregisterPage "), None);
    assert_eq!(stub_fault("UnregisterPag"), None);
    assert_eq!(stub_fault(""), None);
}

#[test]
fn first_and_last_stub_entries_match() {
    // Not oracle-derived: boundary entries of STUB_EXPORTS, the first and last `stub` lines in connect.spec.
    let first = stub_fault("AddConnectionOptionListEntries").expect("first stub");
    assert_eq!(first.function, "AddConnectionOptionListEntries");
    let last = stub_fault("UnregisterPagesLink").expect("last stub");
    assert_eq!(last.function, "UnregisterPagesLink");
}

#[test]
fn stub_exports_are_sorted_and_unique() {
    // Derived from the reference source: connect.spec lists its exports in ascending order,
    // so the stub subset must be strictly increasing.
    assert!(STUB_EXPORTS.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn hresult_success_and_false_values() {
    // Not oracle-derived: S_OK and S_FALSE encodings from winerror.h.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert!(is_internet_connected() >= 0, "SUCCEEDED(S_OK)");
    assert_ne!(dll_can_unload_now(), S_OK);
}
