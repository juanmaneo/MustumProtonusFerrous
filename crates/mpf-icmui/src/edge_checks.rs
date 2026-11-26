//! Edge-case tests for the ported icmui logic.
//!
//! These cover the constants, the fixed `SetupColorMatchingW` result, the stub record's fields,
//! and the derived traits. Not every expectation is oracle-derived; the ones that aren't say so
//! in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn setup_color_matching_w_is_false_not_success() {
    // Oracle-derived: every row returned ret=0 with last_error=87.
    let result = setup_color_matching_w();
    assert!(!result.ret);
    assert_eq!(result.last_error, ERROR_INVALID_PARAMETER);
}

#[test]
fn setup_color_matching_w_keeps_no_state() {
    // Partly oracle-derived: rows 1, 8 and 5 started from last errors 0, 87 and 0xffffffff and
    // all returned the same result. The repeated call is not in the harness, so this is not
    // oracle-derived in full.
    assert_eq!(setup_color_matching_w(), setup_color_matching_w());
}

#[test]
fn setup_color_matching_w_result_is_copy_and_debuggable() {
    // Not oracle-derived: checks the derived traits the callers rely on.
    let result = setup_color_matching_w();
    let copy = result;
    assert_eq!(result, copy);
    assert_eq!(
        format!("{result:?}"),
        "SetupColorMatchingWResult { ret: false, last_error: 87 }"
    );
}

#[test]
fn wine_spec_unimplemented_stub_keeps_the_names_it_is_given() {
    // Not oracle-derived: the C body stores the two pointers unchanged. The harness only passed
    // "icmui.dll" and "SetupColorMatchingA", so the empty strings and the second name are not
    // oracle rows.
    let record = wine_spec_unimplemented_stub("", "");
    assert_eq!(record.module, "");
    assert_eq!(record.function, "");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);

    let record = wine_spec_unimplemented_stub("other.dll", "Other");
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Other");
}

#[test]
fn setup_color_matching_a_names_its_module_and_export() {
    // Not oracle-derived for the names: the module comes from MODULE = icmui.dll in
    // dlls/icmui/Makefile.in and the export name from icmui.spec. The oracle rows print the same
    // values, so they are only confirmed by the rows in oracle_checks.rs.
    let record = setup_color_matching_a();
    assert_eq!(record.module, "icmui.dll");
    assert_eq!(record.function, "SetupColorMatchingA");
    assert_eq!(
        record,
        wine_spec_unimplemented_stub("icmui.dll", "SetupColorMatchingA")
    );
}

#[test]
fn stub_record_is_copy_and_debuggable() {
    // Not oracle-derived: checks the derived traits the callers rely on.
    let record = setup_color_matching_a();
    let copy = record;
    assert_eq!(record, copy);
    assert!(format!("{record:?}").contains("SetupColorMatchingA"));
}
