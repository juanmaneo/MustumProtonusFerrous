//! Edge checks for the stub record model.
//!
//! These check the shared helper's argument pass-through and pin the exception constants. The
//! named export records themselves are checked against the oracle rows in `oracle_checks.rs`.

use super::*;

#[test]
fn exception_constants_are_pinned() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 1);
}

#[test]
fn shared_stub_helper_preserves_both_arguments() {
    assert_eq!(
        wine_spec_unimplemented_stub("custom-module.dll", "Ordinal42"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "custom-module.dll",
            function: "Ordinal42",
        }
    );
}
