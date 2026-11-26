//! Oracle-derived test vectors for the ported d3dx11_42 logic.
//!
//! Vectors were captured by compiling the verbatim `D3DX11CheckVersion` body
//! from `dlls/d3dx11_42/d3dx11_42_main.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/d3dx11_42/` and
//! running the harness. The 25 printed rows (5 `d3d_sdk_ver` values x 5
//! `d3dx_sdk_ver` values) were all clean under ASan and UBSan, so none are excluded.

use super::*;

#[test]
fn oracle_check_version_rows() {
    let d3d = [0u32, 6, 7, 8, u32::MAX];
    let dx = [0u32, 41, 42, 43, u32::MAX];
    let expected = [
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ];
    for (i, &d) in d3d.iter().enumerate() {
        for (j, &x) in dx.iter().enumerate() {
            assert_eq!(check_version(d, x), expected[i][j], "d3d={d} dx={x}");
        }
    }
}
