//! Oracle vectors from Wine's verbatim `math.c`, built with
//! `gcc -fsanitize=address,undefined -g` (harness in `target/canon/msvcp140_2/`).
//!
//! Each row holds the IEEE bit patterns of the inputs and of the result, so the
//! comparison is exact, including signed zeros and NaN payloads. The 24 printed
//! rows (16 double, 8 float) were all clean under ASan and UBSan, so none were
//! excluded.

use crate::{std_smf_hypot3, std_smf_hypot3f};

/// `(label, [x, y, z], result)` as `f64` bit patterns.
const HYPOT3_ROWS: &[(&str, [u64; 3], u64)] = &[
    // 0, 0, 0
    (
        "zeros",
        [0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
        0x0000000000000000,
    ),
    // 3, 4, 12 -> 13
    (
        "3_4_12",
        [0x4008000000000000, 0x4010000000000000, 0x4028000000000000],
        0x402a000000000000,
    ),
    // 1, 2, 2 -> 3
    (
        "1_2_2",
        [0x3ff0000000000000, 0x4000000000000000, 0x4000000000000000],
        0x4008000000000000,
    ),
    // -3, -4, 12 -> 13
    (
        "neg_3_neg4_12",
        [0xc008000000000000, 0xc010000000000000, 0x4028000000000000],
        0x402a000000000000,
    ),
    // 7, 24, 60 -> 65
    (
        "7_24_60",
        [0x401c000000000000, 0x4038000000000000, 0x404e000000000000],
        0x4050400000000000,
    ),
    // 1e308, 1e308, 1e308 -> 1.7320508075688774e308
    (
        "dbl_max_each",
        [0x7fe1ccf385ebc8a0, 0x7fe1ccf385ebc8a0, 0x7fe1ccf385ebc8a0],
        0x7feed4df0150215b,
    ),
    // DBL_MAX, 0, 0
    (
        "dbl_max_x",
        [0x7fefffffffffffff, 0x0000000000000000, 0x0000000000000000],
        0x7fefffffffffffff,
    ),
    // min subnormal, 0, 0
    (
        "min_subnormal_x",
        [0x0000000000000001, 0x0000000000000000, 0x0000000000000000],
        0x0000000000000001,
    ),
    // 1e-320 each (subnormal) -> 1.7317000886735691e-320
    (
        "subnormal_each",
        [0x00000000000007e8, 0x00000000000007e8, 0x00000000000007e8],
        0x0000000000000db1,
    ),
    // DBL_MIN each -> 3.8539409735277962e-308
    (
        "dbl_min_each",
        [0x0010000000000000, 0x0010000000000000, 0x0010000000000000],
        0x001bb67ae8584cab,
    ),
    // -0, -0, -0 -> +0
    (
        "neg_zeros",
        [0x8000000000000000, 0x8000000000000000, 0x8000000000000000],
        0x0000000000000000,
    ),
    // inf, nan, 0 -> inf
    (
        "inf_nan_0",
        [0x7ff0000000000000, 0x7ff8000000000000, 0x0000000000000000],
        0x7ff0000000000000,
    ),
    // nan, 1, 2 -> nan
    (
        "nan_1_2",
        [0x7ff8000000000000, 0x3ff0000000000000, 0x4000000000000000],
        0x7ff8000000000000,
    ),
    // inf, -inf, nan -> inf
    (
        "inf_neg_inf_nan",
        [0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000000],
        0x7ff0000000000000,
    ),
    // 1, 1, 1 -> 1.7320508075688774
    (
        "ones",
        [0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000],
        0x3ffbb67ae8584cab,
    ),
    // 0.1, 0.2, 0.3 -> 0.37416573867739417
    (
        "tenth_fifth_third",
        [0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333],
        0x3fd7f254dab9cc3b,
    ),
];

/// `(label, [x, y, z], result)` as `f32` bit patterns.
const HYPOT3F_ROWS: &[(&str, [u32; 3], u32)] = &[
    // 0, 0, 0
    ("zeros", [0x00000000, 0x00000000, 0x00000000], 0x00000000),
    // 3, 4, 12 -> 13
    ("3_4_12", [0x40400000, 0x40800000, 0x41400000], 0x41500000),
    // FLT_MAX each: the float intermediate overflows to inf
    (
        "flt_max_each",
        [0x7f7fffff, 0x7f7fffff, 0x7f7fffff],
        0x7f800000,
    ),
    // min subnormal, 0, 0
    (
        "min_subnormal_x",
        [0x00000001, 0x00000000, 0x00000000],
        0x00000001,
    ),
    // -0, -0, -0 -> +0
    (
        "neg_zeros",
        [0x80000000, 0x80000000, 0x80000000],
        0x00000000,
    ),
    // inf, nan, 0 -> inf
    (
        "inf_nan_0",
        [0x7f800000, 0x7fc00000, 0x00000000],
        0x7f800000,
    ),
    // 1, 1, 1 -> 1.73205078
    ("ones", [0x3f800000, 0x3f800000, 0x3f800000], 0x3fddb3d7),
    // 1e20 each -> 1.73205082e20
    (
        "1e20_each",
        [0x60ad78ec, 0x60ad78ec, 0x60ad78ec],
        0x61163b41,
    ),
];

#[test]
fn hypot3_matches_wine_oracle_rows() {
    for &(label, [x, y, z], expected) in HYPOT3_ROWS {
        let got = std_smf_hypot3(f64::from_bits(x), f64::from_bits(y), f64::from_bits(z));
        assert_eq!(got.to_bits(), expected, "hypot3 row {label}");
    }
}

#[test]
fn hypot3f_matches_wine_oracle_rows() {
    for &(label, [x, y, z], expected) in HYPOT3F_ROWS {
        let got = std_smf_hypot3f(f32::from_bits(x), f32::from_bits(y), f32::from_bits(z));
        assert_eq!(got.to_bits(), expected, "hypot3f row {label}");
    }
}
