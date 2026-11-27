//! Boundary checks inferred from the source and `D3D10_SDK_VERSION` in `include/d3d10.idl`.

use super::d3dx10_check_version;

#[test]
fn only_the_exact_sdk_pair_is_accepted() {
    assert!(d3dx10_check_version(29, 35));
    assert!(!d3dx10_check_version(29, 36));
    assert!(!d3dx10_check_version(30, 35));
    assert!(!d3dx10_check_version(u32::MAX, u32::MAX));
}
