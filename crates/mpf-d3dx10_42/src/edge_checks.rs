//! Edge-case tests for `D3DX10CheckVersion`.
//!
//! These cross the values adjacent to the two accepted versions (28, 29, 30, 41, 42, 43) with
//! the unsigned limits. The expectations are the C condition `d3dsdkvers == 29 && d3dxsdkvers ==
//! 42` written out, not oracle output.

use super::*;

const BOUNDARY: [u32; 10] = [0, 1, 28, 29, 30, 41, 42, 43, u32::MAX - 1, u32::MAX];

#[test]
fn only_the_exact_sdk_pair_is_accepted() {
    for d3d_sdk_version in BOUNDARY {
        for d3dx_sdk_version in BOUNDARY {
            assert_eq!(
                d3dx10_check_version(d3d_sdk_version, d3dx_sdk_version),
                d3d_sdk_version == 29 && d3dx_sdk_version == 42,
                "({d3d_sdk_version}, {d3dx_sdk_version})"
            );
        }
    }
}
