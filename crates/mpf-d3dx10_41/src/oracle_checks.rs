//! Oracle-derived test vectors for the ported d3dx10_41 logic.
//!
//! Vectors were captured by compiling the unmodified `dlls/d3dx10_41/d3dx10_41_main.c` with
//! Windows header shims (`target/canon/d3dx10_41/shim/`) and `gcc -fsanitize=address,undefined -g`,
//! then running `target/canon/d3dx10_41/harness.c` (output in `vectors.txt`). The run printed 25
//! rows (5 `d3dsdkvers` values x 5 `d3dxsdkvers` values). ASan and UBSan reported nothing, so no
//! vector is excluded.

use super::*;

#[test]
fn oracle_check_version_rows() {
    let d3d = [0u32, 28, 29, 30, u32::MAX];
    let dx = [0u32, 40, 41, 42, u32::MAX];
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
