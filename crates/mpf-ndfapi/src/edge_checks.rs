//! Edge-case tests for the ported ndfapi logic.
//!
//! These cover the constants, the record that `__wine_spec_unimplemented_stub` raises for names
//! the harness never passed, and the repeat calls of the explicit exports. Not every expectation
//! is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    // The harness shims use the same values.
    assert_eq!(E_NOTIMPL, 0x8000_4001u32 as i32);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn explicit_exports_return_e_notimpl_on_every_call() {
    // Not oracle-derived: the C bodies keep no state, so a second call must match the first.
    for _ in 0..2 {
        assert_eq!(ndf_close_incident(), E_NOTIMPL);
        assert_eq!(ndf_create_connectivity_incident(), E_NOTIMPL);
        assert_eq!(ndf_create_web_incident(), E_NOTIMPL);
        assert_eq!(ndf_create_winsock_incident(), E_NOTIMPL);
        assert_eq!(ndf_diagnose_incident(), E_NOTIMPL);
        assert_eq!(ndf_execute_diagnosis(), E_NOTIMPL);
    }
}

#[test]
fn stub_record_passes_names_through_unchanged() {
    // Not oracle-derived: the harness only passed the names of ndfapi.spec. The code, flags and
    // parameter count are the same for any names, including empty ones.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Ordinal42"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Ordinal42",
        }
    );
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );
}

#[test]
fn stub_record_is_noncontinuable_and_copies() {
    // Not oracle-derived: the NONCONTINUABLE bit is what `__wine_spec_unimplemented_stub` sets.
    // The record is plain data, so a copy compares equal and a different function does not.
    let record = dll_get_class_object();
    let copy = record;
    assert_eq!(copy, record);
    assert_ne!(copy, ndf_repair_incident_ex());
    assert_ne!(record.flags & EXCEPTION_NONCONTINUABLE, 0);
}
