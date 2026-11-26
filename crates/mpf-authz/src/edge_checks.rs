//! Edge-case tests for the ported authz stubs.
//!
//! The stubs have no branches, so the edges are the last-error values the slot can hold before
//! the call and the repeated-call behaviour. Expectations that are not oracle-derived say so.

use super::*;

#[test]
fn error_code_matches_winerror_header() {
    // Not oracle-derived: checked against include/winerror.h:311 (`ERROR_CALL_NOT_IMPLEMENTED 120`).
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
}

#[test]
fn stub_overwrites_every_prior_last_error() {
    // Not oracle-derived for these values: authz.c calls SetLastError unconditionally,
    // so the outcome is the same for every prior, including values just around 120.
    for prior in [0, 1, 119, 120, 121, u32::MAX - 1, u32::MAX] {
        let mut slot = prior;
        assert!(!authz_free_context(&mut slot));
        assert_eq!(slot, ERROR_CALL_NOT_IMPLEMENTED);
    }
}

#[test]
fn repeated_calls_keep_the_same_outcome() {
    // Not oracle-derived: authz.c keeps no static state, so each call sees only its own inputs.
    let mut slot = 0;
    for _ in 0..3 {
        assert!(!authz_access_check(&mut slot));
        assert_eq!(slot, ERROR_CALL_NOT_IMPLEMENTED);
    }
}
