//! Edge-case tests for `d3dx10_check_version`.
//!
//! These cover each argument independently and the boundaries of `u32`. The expectations come
//! from the C condition `d3dsdkvers == D3D10_SDK_VERSION && d3dxsdkvers == 34`, not from the
//! harness.

use super::*;

#[test]
fn both_arguments_must_match() {
    assert!(d3dx10_check_version(29, 34));
    assert!(!d3dx10_check_version(29, 35));
    assert!(!d3dx10_check_version(30, 34));
    assert!(!d3dx10_check_version(30, 35));
}

#[test]
fn neighbouring_values_are_rejected() {
    assert!(!d3dx10_check_version(28, 34));
    assert!(!d3dx10_check_version(29, 33));
}

#[test]
fn extreme_values_are_rejected() {
    assert!(!d3dx10_check_version(0, 0));
    assert!(!d3dx10_check_version(u32::MAX, u32::MAX));
    assert!(!d3dx10_check_version(u32::MAX, 34));
    assert!(!d3dx10_check_version(29, u32::MAX));
}
