//! Edge checks for the SDK version comparison and the portable stub record.

use super::*;

#[test]
fn each_version_argument_must_match_independently() {
    assert!(d3dx10_check_version(29, 38));
    assert!(!d3dx10_check_version(28, 38));
    assert!(!d3dx10_check_version(29, 37));
    assert!(!d3dx10_check_version(u32::MAX, u32::MAX));
}

#[test]
fn generic_stub_preserves_its_arguments_and_constants() {
    let record = wine_spec_unimplemented_stub("other.dll", "Ordinal");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Ordinal");
}
