//! Oracle-derived vectors for `D3DX10CheckVersion`.
//!
//! Values were captured from the unmodified `dlls/d3dx10_35/d3dx10_35_main.c`, compiled with
//! the shims in `target/canon/d3dx10_35/` using `gcc -fsanitize=address,undefined -g`. The
//! harness printed all six rows below; ASan and UBSan reported no findings.

use super::d3dx10_check_version;

#[test]
fn check_version_matches_c_oracle() {
    let rows = [
        (29, 35, true),
        (29, 34, false),
        (28, 35, false),
        (0, 0, false),
        (u32::MAX, 35, false),
        (29, 0, false),
    ];

    for (d3d, d3dx, expected) in rows {
        assert_eq!(d3dx10_check_version(d3d, d3dx), expected);
    }
}
