//! Edge-case tests for the ported dciman32 logic.
//!
//! Each expectation below is either taken from the oracle or comes from the
//! reference source, as the comment on each test says.

use super::*;

#[test]
fn last_error_is_overwritten_by_every_call() {
    // Derived from the reference source: each stub unconditionally calls SetLastError.
    let mut d = Dciman32 {
        last_error: u32::MAX,
    };
    d.close_provider(usize::MAX);
    assert_eq!(d.last_error(), ERROR_CALL_NOT_IMPLEMENTED);
    let _ = d.open_provider();
    assert_eq!(d.last_error(), ERROR_CALL_NOT_IMPLEMENTED);
}

#[test]
fn create_primary_ignores_surface_info_contents() {
    // Derived from the reference source: the pointer is only passed to FIXME.
    let mut d = Dciman32::default();
    let mut info = 0xdead_beefu64;
    assert_eq!(
        d.create_primary(NULL_HDC, Some(&mut info)),
        DCI_FAIL_UNSUPPORTED
    );
    assert_eq!(info, 0xdead_beef);
}

#[test]
fn default_state_is_no_error() {
    // Derived from the reference source: a fresh slot carries no error.
    assert_eq!(Dciman32::default().last_error(), 0);
}

#[test]
fn stub_fault_rejects_non_stub_exports() {
    // Derived from the reference source: the stdcall exports are not in the stub list.
    assert_eq!(stub_fault("DCIOpenProvider"), None);
    assert_eq!(stub_fault("DCICloseProvider"), None);
    assert_eq!(stub_fault("DCICreatePrimary"), None);
    assert_eq!(stub_fault("DllEntryPoint"), None);
}

#[test]
fn stub_fault_boundary_names() {
    // Derived from the reference source: lookup is exact and case-sensitive, like GetProcAddress.
    assert_eq!(stub_fault(""), None);
    assert_eq!(stub_fault("dcidestroy"), None);
    assert_eq!(stub_fault("DCIDestroy "), None);
    assert!(stub_fault("DCIDestroy").is_some());
}

#[test]
fn stub_list_has_no_duplicates() {
    // Derived from the reference source: each `stub` line in dciman32.spec names one export.
    for (i, a) in STUB_EXPORTS.iter().enumerate() {
        for b in &STUB_EXPORTS[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn dci_codes_are_distinct() {
    // Derived from the reference source: the five DCI codes in dciddi.h have distinct values.
    let codes = [
        DCI_OK,
        DCI_FAIL_GENERIC,
        DCI_FAIL_UNSUPPORTEDVERSION,
        DCI_FAIL_INVALIDSURFACE,
        DCI_FAIL_UNSUPPORTED,
    ];
    for (i, a) in codes.iter().enumerate() {
        for b in &codes[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
