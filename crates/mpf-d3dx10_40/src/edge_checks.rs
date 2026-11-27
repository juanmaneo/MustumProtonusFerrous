//! Boundary checks beyond the captured oracle rows.

use super::d3dx10_check_version;

#[test]
fn version_requires_both_sdk_versions_to_match() {
    let rows = [
        (29, 40, true),
        (0, 40, false),
        (u32::MAX, 40, false),
        (29, 0, false),
        (29, u32::MAX, false),
        (28, 39, false),
    ];

    for (d3d, d3dx, expected) in rows {
        assert_eq!(d3dx10_check_version(d3d, d3dx), expected, "{d3d} {d3dx}");
    }
}
