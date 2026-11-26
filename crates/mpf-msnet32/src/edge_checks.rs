//! Edge-case tests for the ported msnet32 logic.
//!
//! These cover the pinned constants, the stub record's pass-through, and the argument
//! independence of `MSNET32_57`. Not every expectation is oracle-derived; the ones that aren't
//! say so in a comment.

use super::*;
use core::ffi::c_void;
use std::ptr::null_mut;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The harness shim
    // uses the same values, but the harness only prints the records that the stubs raise.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_record_passes_module_and_function_through() {
    // Not oracle-derived: the harness only ran the ordinal exports. These check that the port
    // copies both strings unchanged, including empty ones and names.
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
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Name"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Name",
        }
    );
}

#[test]
fn msnet32_57_ignores_its_arguments() {
    // Not oracle-derived: the C body ignores all five arguments and returns -1, so these
    // values follow from reading dlls/msnet32/msnet_main.c. They extend the four harness vectors
    // to the LONG extremes and to both pointer states.
    let mut slot = 0u8;
    let ptr = (&mut slot as *mut u8).cast::<c_void>();
    assert_eq!(
        msnet32_57(i32::MAX, i32::MIN, null_mut(), i32::MAX, null_mut()),
        -1
    );
    assert_eq!(msnet32_57(i32::MIN, i32::MIN, ptr, i32::MIN, ptr), -1);
    assert_eq!(msnet32_57(0, -1, ptr, 0, null_mut()), -1);
}

#[test]
fn stub_helpers_name_their_ordinal_in_decimal() {
    // Not oracle-derived: the first and last stub ordinals, and the ordinals on either side of
    // the named export 57, which must stay stubs. The oracle rows cover all 95 helpers.
    assert_eq!(ordinal_1().function, "1");
    assert_eq!(ordinal_56().function, "56");
    assert_eq!(ordinal_58().function, "58");
    assert_eq!(ordinal_96().function, "96");
    assert_eq!(ordinal_58().module, "msnet32.dll");
}
