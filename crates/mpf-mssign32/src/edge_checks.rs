//! Edge-case tests for the ported mssign32 logic.
//!
//! The crate has no branches: the explicit exports return fixed values and each stub helper builds
//! one record. These tests pin the constants and the record shape. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the values are from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(S_OK, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn hresult_bit_patterns_read_as_negative_i32() {
    // Not oracle-derived: the signed view of the bit pattern, since a Windows HRESULT is a 32-bit LONG.
    assert_eq!(E_FAIL as i32, -2_147_467_259);
    assert_eq!(E_NOTIMPL as i32, -2_147_467_263);
    assert_eq!(S_OK as i32, 0);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed export names. The code, flags and
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
