//! Oracle-derived test vectors for `d3dx10_check_version`.
//!
//! Vectors were captured from `target/canon/d3dx10_33/harness.c`. That harness compiles the
//! unmodified `dlls/d3dx10_33/d3dx10_33_main.c` from the reference tree, with Windows header
//! shims in `target/canon/d3dx10_33/shim/`, using
//! `gcc -fsanitize=address,undefined -g`. The run printed 14 rows. ASan and UBSan reported
//! nothing, so no vector is excluded. `D3D10_SDK_VERSION` is 29 in `include/d3d10.idl`.

use super::*;

#[test]
fn oracle_vectors() {
    // (d3dsdkvers, d3dxsdkvers, expected TRUE/FALSE from D3DX10CheckVersion)
    let rows: [(u32, u32, bool); 14] = [
        (29, 33, true),
        (29, 32, false),
        (29, 34, false),
        (29, 33, true),
        (29, 0, false),
        (29, 0xffff_ffff, false),
        (28, 33, false),
        (30, 33, false),
        (0, 33, false),
        (0xffff_ffff, 33, false),
        (29, 33, true),
        (33, 33, false),
        (33, 29, false),
        (29, 0, false),
    ];
    for (d3d, d3dx, expected) in rows {
        assert_eq!(
            d3dx10_check_version(d3d, d3dx),
            expected,
            "D3DX10CheckVersion({d3d}, {d3dx})"
        );
    }
}
