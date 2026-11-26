//! Oracle-derived vectors for the ported authz stubs.
//!
//! Vectors were captured by compiling the verbatim `dlls/authz/authz.c` bodies (lines 30-118)
//! with the Windows shims in `target/canon/authz/authz_shim.h` under
//! `gcc -fsanitize=address,undefined -g`. The harness printed 21 rows (7 exports x 3) with no
//! ASan or UBSan reports, so no row is excluded. The stubs ignore their arguments, so each row
//! is recorded as the prior last-error value, the expected return and the expected last error.
//! Rows 01-03 are the first three, in the order `null` prior 0, `populated` prior 5, and
//! `populated` prior 0xffffffff; the same three rows repeat for each export.

use super::*;

/// `(prior last error, expected return, expected last error)` for oracle rows 01-03.
const ROWS: [(u32, bool, u32); 3] = [(0, false, 120), (5, false, 120), (0xffff_ffff, false, 120)];

fn check_rows(export: fn(&mut u32) -> bool) {
    for (prior, ret, last_error) in ROWS {
        let mut slot = prior;
        assert_eq!(export(&mut slot), ret, "return for prior {prior}");
        assert_eq!(slot, last_error, "last error for prior {prior}");
    }
}

#[test]
fn oracle_authz_initialize_resource_manager_rows() {
    // oracle rows 01-03
    check_rows(authz_initialize_resource_manager);
}

#[test]
fn oracle_authz_free_resource_manager_rows() {
    // oracle rows 04-06
    check_rows(authz_free_resource_manager);
}

#[test]
fn oracle_authz_install_security_event_source_rows() {
    // oracle rows 07-09
    check_rows(authz_install_security_event_source);
}

#[test]
fn oracle_authz_access_check_rows() {
    // oracle rows 10-12
    check_rows(authz_access_check);
}

#[test]
fn oracle_authz_free_context_rows() {
    // oracle rows 13-15
    check_rows(authz_free_context);
}

#[test]
fn oracle_authz_initialize_context_from_sid_rows() {
    // oracle rows 16-18
    check_rows(authz_initialize_context_from_sid);
}

#[test]
fn oracle_authz_initialize_context_from_token_rows() {
    // oracle rows 19-21
    check_rows(authz_initialize_context_from_token);
}
