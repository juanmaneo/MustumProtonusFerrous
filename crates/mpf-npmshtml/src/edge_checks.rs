//! Edge-case tests for the ported npmshtml logic.
//!
//! These cover the pointer boundary of `NP_Initialize`, the state independence of the two
//! exports, and the export table. Not every expectation is oracle-derived; the ones that aren't
//! say so in a comment.

use super::*;
use core::ffi::c_void;
use core::ptr::NonNull;

const FORWARD_TO_MSHTML: ExportKind = ExportKind::Forward {
    module: "mshtml",
    function: "NP_GetEntryPoints",
};

#[test]
fn np_initialize_ignores_the_pointer_value() {
    // Not oracle-derived: an aligned dangling pointer, which the oracle rows do not include.
    let dangling: *mut c_void = NonNull::<c_void>::dangling().as_ptr();
    assert_eq!(np_initialize(dangling), 0);
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(1)), 0);
    assert_eq!(np_initialize(core::ptr::null_mut()), 0);
}

#[test]
fn np_shutdown_is_stateless() {
    // Not oracle-derived: repeated calls with and without a preceding NP_Initialize. The C body
    // keeps no state, so every call returns the same zero.
    for _ in 0..3 {
        assert_eq!(np_shutdown(), 0);
    }
    assert_eq!(np_initialize(core::ptr::null_mut()), 0);
    assert_eq!(np_shutdown(), 0);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: names, kinds and order come from dlls/npmshtml/npmshtml.spec.
    assert_eq!(EXPORTS.len(), 3);
    assert_eq!(EXPORTS[0], ("NP_GetEntryPoints", FORWARD_TO_MSHTML));
    assert_eq!(EXPORTS[1], ("NP_Initialize", ExportKind::Stdcall));
    assert_eq!(EXPORTS[2], ("NP_Shutdown", ExportKind::Stdcall));
}

#[test]
fn export_kind_is_case_sensitive_and_exact() {
    // Not oracle-derived: lookup rules, including the names that are close to a real export.
    assert_eq!(export_kind("NP_GetEntryPoints"), Some(FORWARD_TO_MSHTML));
    assert_eq!(export_kind("NP_Initialize"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("NP_Shutdown"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("np_initialize"), None);
    assert_eq!(export_kind("NP_Init"), None);
    assert_eq!(export_kind("NP_Shutdown "), None);
    assert_eq!(export_kind(""), None);
}
