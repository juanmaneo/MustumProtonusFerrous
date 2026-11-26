//! Edge-case tests for the ported `D3D10CoreCreateDevice` control flow.
//!
//! Most expectations are oracle-derived. The ones that are not say so in a comment.

use super::*;
use std::cell::RefCell;

#[test]
fn failed_sign_boundaries() {
    // Not oracle-derived for -1 and i32::MIN: `FAILED` is `(HRESULT)(hr) < 0` (winerror.h).
    // 1, i32::MAX and 0x80000000 are also exercised end to end by the oracle rows.
    assert!(!failed(0));
    assert!(!failed(1));
    assert!(!failed(i32::MAX));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
}

#[test]
fn constants_match_winerror_h() {
    // Not oracle-derived: literal values from include/winerror.h.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
}

#[test]
fn success_calls_create_then_query() {
    // Not oracle-derived: the order comes from the C statement sequence. The oracle
    // only counts calls.
    let log = RefCell::new(Vec::new());
    let hr = d3d10_core_create_device(
        0,
        0xb000,
        |_, _| {
            log.borrow_mut().push("create");
            S_OK
        },
        || {
            log.borrow_mut().push("query");
            S_OK
        },
    );
    assert_eq!(hr, S_OK);
    assert_eq!(*log.borrow(), ["create", "query"]);
}

#[test]
fn query_failure_maps_to_e_fail_for_any_negative_code() {
    // Not oracle-derived for i32::MIN: the oracle covers E_NOINTERFACE, E_FAIL, E_NOTIMPL and
    // E_POINTER. The C code maps every failed QueryInterface to E_FAIL.
    let hr = d3d10_core_create_device(0, 0xa000, |_, _| S_OK, || i32::MIN);
    assert_eq!(hr, E_FAIL);
}

#[test]
fn query_success_code_maps_to_s_ok() {
    // Not oracle-derived for i32::MAX: the oracle covers S_FALSE (1). The C code only tests
    // FAILED on the query result and returns S_OK otherwise.
    let hr = d3d10_core_create_device(0, 0xa000, |_, _| S_OK, || i32::MAX);
    assert_eq!(hr, S_OK);
}
