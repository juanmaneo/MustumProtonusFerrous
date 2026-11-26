//! Edge-case tests for the ported msdrm logic.
//!
//! These cover the constants, the pass-through of the stub record's arguments, and the
//! `DRMRegisterContent` argument range. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn register_content_ignores_values_next_to_the_extremes() {
    // Not oracle-derived: the oracle covers 0, 1, -1 and the two 32-bit extremes. These values
    // sit next to the extremes. The C body has no branch on value.
    for value in [i32::MIN + 1, -2, 2, i32::MAX - 1] {
        assert_eq!(drm_register_content(value), S_OK);
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the names of msdrm.spec. The code, flags and
    // parameter count are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn helpers_match_the_generic_stub_record() {
    // Not oracle-derived: a consistency check between the per-export helpers and the generic
    // record. The oracle already pins the full records for these two names.
    assert_eq!(
        drm_acquire_license(),
        wine_spec_unimplemented_stub("msdrm.dll", "DRMAcquireLicense")
    );
    assert_eq!(
        __add_machine_cert_to_license_store(),
        wine_spec_unimplemented_stub("msdrm.dll", "__AddMachineCertToLicenseStore")
    );
}
