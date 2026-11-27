//! Oracle-derived test vectors for `D3DX10CheckVersion`.
//!
//! Vectors were captured from `target/canon/d3dx10_42/harness.c`. That harness compiles the
//! unmodified `dlls/d3dx10_42/d3dx10_42_main.c` with the scalar Win32 shims in
//! `target/canon/d3dx10_42/shim/`, using `gcc -fsanitize=address,undefined -g`. The run printed
//! 20 rows, one per input pair, and ASan and UBSan reported nothing, so no vector is excluded.

use super::*;

/// `(d3d_sdk_version, d3dx_sdk_version, raw result of D3DX10CheckVersion)`, in harness order.
const ROWS: [(u32, u32, i32); 20] = [
    // row 1
    (29, 42, 1),
    // row 2
    (29, 43, 0),
    // row 3
    (29, 41, 0),
    // row 4
    (29, 0, 0),
    // row 5
    (29, 1, 0),
    // row 6
    (29, 0xffff_ffff, 0),
    // row 7
    (28, 42, 0),
    // row 8
    (30, 42, 0),
    // row 9
    (0, 42, 0),
    // row 10
    (0xffff_ffff, 42, 0),
    // row 11
    (42, 29, 0),
    // row 12
    (0, 0, 0),
    // row 13
    (0xffff_ffff, 0xffff_ffff, 0),
    // row 14
    (29, 0x2a, 1),
    // row 15
    (29, 0x102a, 0),
    // row 16
    (29 | 0x100, 42, 0),
    // row 17
    (42, 42, 0),
    // row 18
    (1, 42, 0),
    // row 19
    (29, 0x8000_0000 | 42, 0),
    // row 20
    (0x8000_0000 | 29, 42, 0),
];

#[test]
fn check_version_matches_oracle_rows() {
    for (index, &(d3d_sdk_version, d3dx_sdk_version, expected)) in ROWS.iter().enumerate() {
        assert_eq!(
            d3dx10_check_version(d3d_sdk_version, d3dx_sdk_version),
            expected != 0,
            "oracle row {}: ({d3d_sdk_version}, {d3dx_sdk_version})",
            index + 1
        );
    }
}
