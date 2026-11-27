//! Edge-case tests for the ported d3dx10_41 logic.
//!
//! The expectations here are boundary and branch checks. They are not
//! oracle-derived unless a comment says otherwise.

use super::*;

#[test]
fn sdk_constants_match_wine_build() {
    // Not oracle-derived: pins the values the oracle harness relies on.
    assert_eq!(D3D10_SDK_VERSION, 29);
    assert_eq!(D3DX10_SDK_VERSION, 41);
}

#[test]
fn each_operand_is_checked_independently() {
    // Not oracle-derived: the d3d half alone matching must fail, and vice versa.
    assert!(check_version(29, 41));
    assert!(!check_version(29, 40));
    assert!(!check_version(28, 41));
}

#[test]
fn off_by_one_and_wrap_values_are_rejected() {
    // Not oracle-derived: neighbours of the exact versions, and u32 extremes.
    for d in [28u32, 30, u32::MAX] {
        assert!(!check_version(d, 41));
    }
    for x in [40u32, 42, u32::MAX] {
        assert!(!check_version(29, x));
    }
}
