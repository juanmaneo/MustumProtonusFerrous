//! Edge cases not present in the small oracle input set.

use super::*;

#[test]
fn version_check_rejects_boundary_and_combined_mismatches() {
    assert!(!d3dx10_check_version(0, 0));
    assert!(!d3dx10_check_version(u32::MAX, 39));
    assert!(!d3dx10_check_version(29, u32::MAX));
    assert!(!d3dx10_check_version(28, 38));
}

#[test]
fn generic_stub_record_preserves_supplied_names() {
    assert_eq!(
        wine_spec_unimplemented_stub("custom.dll", "Ordinal17"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "custom.dll",
            function: "Ordinal17",
        }
    );
}
