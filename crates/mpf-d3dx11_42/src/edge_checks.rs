//! Edge-case tests for the ported d3dx11_42 logic.
//!
//! The expectations here are boundary and branch checks. They are not
//! oracle-derived unless a comment says otherwise.

use super::*;

#[test]
fn sdk_constants_match_wine_build() {
    // Not oracle-derived: pins the values the oracle harness hard-codes.
    assert_eq!(D3D11_SDK_VERSION, 7);
    assert_eq!(D3DX11_SDK_VERSION, 42);
}

#[test]
fn each_operand_is_checked_independently() {
    // Not oracle-derived: the d3d half alone matching must fail, and vice versa.
    assert!(check_version(7, 42));
    assert!(!check_version(7, 41));
    assert!(!check_version(6, 42));
}

#[test]
fn off_by_one_and_wrap_values_are_rejected() {
    // Not oracle-derived: neighbours of the exact versions, and u32 extremes.
    for d in [6u32, 8, u32::MAX] {
        assert!(!check_version(d, 42));
    }
    for x in [41u32, 43, u32::MAX] {
        assert!(!check_version(7, x));
    }
}
