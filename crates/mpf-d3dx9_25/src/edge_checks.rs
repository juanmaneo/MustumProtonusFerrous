//! Non-oracle edge checks for the common stub record fields.

use super::*;

#[test]
fn stub_constants_and_custom_record_fields_are_preserved() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(
        wine_spec_unimplemented_stub("custom.dll", "CustomExport"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "custom.dll",
            function: "CustomExport",
        },
    );
}
