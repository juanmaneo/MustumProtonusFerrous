//! Edge-case tests for the ported msvcm80 logic.
//!
//! These cover the constants, the stub record's fields, the uniqueness of the export names, and
//! the no-op helpers at pointer boundaries. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::oracle_checks::STUB_ROWS;
use super::stub::MSVCM80_DLL_NAME;
use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and the MODULE line in
    // dlls/msvcm80/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(MSVCM80_DLL_NAME, "msvcm80.dll");
}

#[test]
fn stub_record_copies_module_and_function() {
    // Not oracle-derived: the C body copies both pointers into ExceptionInformation unchanged, so
    // an empty function name and a module other than msvcm80.dll pass through as they are.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "",
        }
    );
}

#[test]
fn stub_export_names_are_distinct() {
    // Not oracle-derived: checks that the row table has no duplicate export names, so that no two
    // rows describe the same export.
    let names: HashSet<&str> = STUB_ROWS.iter().map(|&(_, function)| function).collect();
    assert_eq!(names.len(), STUB_ROWS.len());
}

#[test]
fn no_op_helpers_accept_pointer_boundaries() {
    // Not oracle-derived: NULL and the largest pointer value. The C bodies only trace these, so
    // the expectation is that each call returns without effect.
    for value in [0, usize::MAX] {
        crt_implementation_details_register_module_uninitializer(value);
        setusermatherr_m(value);
        crt_implementation_details_throw_module_load_exception(value);
        crt_implementation_details_throw_module_load_exception_inner(value, value);
        crt_implementation_details_throw_nested_module_load_exception(value, value);
    }
}
