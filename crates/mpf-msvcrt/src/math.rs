//! Integer and floating-point helpers from Wine's `dlls/msvcrt/math.c`.
//!
//! `abs` and `labs` wrap on the minimum value, as the GCC build of Wine does.
//! The C standard leaves that case undefined. `long` is 32 bits on Windows,
//! so `labs` and the `l` rotations share the 32-bit types.
//!
//! Predicates that return Wine's `int` 0/1 return `bool` here. Pointer
//! arguments such as `_dtest(double *)` become plain values; `_dtest` and
//! `_fdtest` are therefore the same functions as [`dclass`] and [`fdclass`].

use std::num::FpCategory;

/// `_FPCLASS_*` codes returned by [`fpclass`] and [`fpclassf`].
pub const FPCLASS_SNAN: i32 = 0x0001;
pub const FPCLASS_QNAN: i32 = 0x0002;
pub const FPCLASS_NINF: i32 = 0x0004;
pub const FPCLASS_NN: i32 = 0x0008;
pub const FPCLASS_ND: i32 = 0x0010;
pub const FPCLASS_NZ: i32 = 0x0020;
pub const FPCLASS_PZ: i32 = 0x0040;
pub const FPCLASS_PD: i32 = 0x0080;
pub const FPCLASS_PN: i32 = 0x0100;
pub const FPCLASS_PINF: i32 = 0x0200;

/// `FP_*` codes returned by [`dclass`] and [`fdclass`], as in `<math.h>`.
pub const FP_INFINITE: i16 = 1;
pub const FP_NAN: i16 = 2;
pub const FP_NORMAL: i16 = -1;
pub const FP_SUBNORMAL: i16 = -2;
pub const FP_ZERO: i16 = 0;

pub fn abs(n: i32) -> i32 {
    n.wrapping_abs()
}

pub fn labs(n: i32) -> i32 {
    n.wrapping_abs()
}

pub fn abs64(n: i64) -> i64 {
    n.wrapping_abs()
}

/// `_rotl`: rotates left by `shift & 31`, so negative shifts rotate right.
pub fn rotl(num: u32, shift: i32) -> u32 {
    num.rotate_left((shift & 31) as u32)
}

/// `_rotr`: rotates right by `shift & 31`.
pub fn rotr(num: u32, shift: i32) -> u32 {
    num.rotate_right((shift & 31) as u32)
}

pub fn lrotl(num: u32, shift: i32) -> u32 {
    rotl(num, shift)
}

pub fn lrotr(num: u32, shift: i32) -> u32 {
    rotr(num, shift)
}

/// `_rotl64`: rotates left by `shift & 63`.
pub fn rotl64(num: u64, shift: i32) -> u64 {
    num.rotate_left((shift & 63) as u32)
}

/// `_rotr64`: rotates right by `shift & 63`.
pub fn rotr64(num: u64, shift: i32) -> u64 {
    num.rotate_right((shift & 63) as u32)
}

/// `div`: returns `(quotient, remainder)`.
///
/// Returns `None` where Windows raises an arithmetic exception: a zero
/// divisor, or `i32::MIN / -1`, whose quotient does not fit in `int`.
pub fn div(numerator: i32, denominator: i32) -> Option<(i32, i32)> {
    numerator
        .checked_div(denominator)
        .zip(numerator.checked_rem(denominator))
}

/// `ldiv`: `long` is 32 bits on Windows, so this is identical to [`div`].
pub fn ldiv(numerator: i32, denominator: i32) -> Option<(i32, i32)> {
    div(numerator, denominator)
}

/// `_fpclass`: classifies `num` into one of the `_FPCLASS_*` codes.
///
/// Unlike [`f64::classify`], this tells quiet and signaling NaNs apart, using the
/// top mantissa bit.
pub fn fpclass(num: f64) -> i32 {
    let bits = num.to_bits();
    let negative = bits >> 63 == 1;
    match (bits >> 52) & 0x7ff {
        0 if bits << 1 != 0 => sign_code(negative, FPCLASS_ND, FPCLASS_PD),
        0 => sign_code(negative, FPCLASS_NZ, FPCLASS_PZ),
        0x7ff if bits << 12 != 0 => {
            if (bits >> 51) & 1 == 1 {
                FPCLASS_QNAN
            } else {
                FPCLASS_SNAN
            }
        }
        0x7ff => sign_code(negative, FPCLASS_NINF, FPCLASS_PINF),
        _ => sign_code(negative, FPCLASS_NN, FPCLASS_PN),
    }
}

/// `_fpclassf`: the single-precision form of [`fpclass`].
pub fn fpclassf(num: f32) -> i32 {
    let bits = num.to_bits();
    let negative = bits >> 31 == 1;
    match (bits >> 23) & 0xff {
        0 if bits << 1 != 0 => sign_code(negative, FPCLASS_ND, FPCLASS_PD),
        0 => sign_code(negative, FPCLASS_NZ, FPCLASS_PZ),
        0xff if bits << 9 != 0 => {
            if (bits >> 22) & 1 == 1 {
                FPCLASS_QNAN
            } else {
                FPCLASS_SNAN
            }
        }
        0xff => sign_code(negative, FPCLASS_NINF, FPCLASS_PINF),
        _ => sign_code(negative, FPCLASS_NN, FPCLASS_PN),
    }
}

fn sign_code(negative: bool, negative_code: i32, positive_code: i32) -> i32 {
    if negative {
        negative_code
    } else {
        positive_code
    }
}

/// `_finite`: true unless `num` is an infinity or a NaN.
pub fn finite(num: f64) -> bool {
    num.is_finite()
}

/// `_finitef`: the single-precision form of [`finite`].
pub fn finitef(num: f32) -> bool {
    num.is_finite()
}

/// `_isnan`: true for any NaN, quiet or signaling.
pub fn isnan(num: f64) -> bool {
    num.is_nan()
}

/// `_isnanf`: the single-precision form of [`isnan`].
pub fn isnanf(num: f32) -> bool {
    num.is_nan()
}

/// `_chgsign`: flips the sign bit, which also changes the sign of NaNs and zeros.
pub fn chgsign(num: f64) -> f64 {
    f64::from_bits(num.to_bits() ^ (1 << 63))
}

/// `_chgsignf`: the single-precision form of [`chgsign`].
pub fn chgsignf(num: f32) -> f32 {
    f32::from_bits(num.to_bits() ^ (1 << 31))
}

/// `_dclass`: classifies `x` into one of the `FP_*` codes.
pub fn dclass(x: f64) -> i16 {
    match x.classify() {
        FpCategory::Zero => FP_ZERO,
        FpCategory::Subnormal => FP_SUBNORMAL,
        FpCategory::Normal => FP_NORMAL,
        FpCategory::Infinite => FP_INFINITE,
        FpCategory::Nan => FP_NAN,
    }
}

/// `_fdclass`: the single-precision form of [`dclass`].
pub fn fdclass(x: f32) -> i16 {
    match x.classify() {
        FpCategory::Zero => FP_ZERO,
        FpCategory::Subnormal => FP_SUBNORMAL,
        FpCategory::Normal => FP_NORMAL,
        FpCategory::Infinite => FP_INFINITE,
        FpCategory::Nan => FP_NAN,
    }
}

/// `_dsign`: `0x8000` when the sign bit is set, otherwise `0`.
pub fn dsign(x: f64) -> i32 {
    ((x.to_bits() >> 48) & 0x8000) as i32
}

/// `_fdsign`: the single-precision form of [`dsign`].
pub fn fdsign(x: f32) -> i32 {
    ((x.to_bits() >> 16) & 0x8000) as i32
}

/// `_dpcomp`: compares `x` with `y`.
///
/// Returns `0` if either operand is NaN, `2` if they are equal, `1` if `x < y`
/// and `4` if `x > y`.
pub fn dpcomp(x: f64, y: f64) -> i32 {
    if x.is_nan() || y.is_nan() {
        0
    } else if x == y {
        2
    } else if x < y {
        1
    } else {
        4
    }
}

/// `_fdpcomp`: the single-precision form of [`dpcomp`].
pub fn fdpcomp(x: f32, y: f32) -> i32 {
    dpcomp(x.into(), y.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_family_returns_magnitude() {
        assert_eq!(abs(-5), 5);
        assert_eq!(abs(7), 7);
        assert_eq!(labs(-9), 9);
        assert_eq!(abs64(-1 << 40), 1 << 40);
        assert_eq!(abs64(3), 3);
    }

    #[test]
    fn abs_of_minimum_wraps_like_gcc_build() {
        assert_eq!(abs(i32::MIN), i32::MIN);
        assert_eq!(labs(i32::MIN), i32::MIN);
        assert_eq!(abs64(i64::MIN), i64::MIN);
    }

    #[test]
    fn rotations_mask_the_shift_count() {
        assert_eq!(rotl(0x8000_0001, 1), 0x3);
        assert_eq!(rotl(0x1234_5678, 0), 0x1234_5678);
        assert_eq!(rotl(0x1234_5678, 32), 0x1234_5678);
        assert_eq!(rotr(1, 1), 0x8000_0000);
        assert_eq!(rotr(1, -1), 2);
        assert_eq!(rotl(0x8000_0000, -31), 1);
    }

    #[test]
    fn long_rotations_match_32_bit_rotations() {
        assert_eq!(lrotl(0x8000_0001, 1), rotl(0x8000_0001, 1));
        assert_eq!(lrotr(1, 1), rotr(1, 1));
    }

    #[test]
    fn rotations_64_bit() {
        assert_eq!(rotl64(1, 63), 1 << 63);
        assert_eq!(rotl64(1, 64), 1);
        assert_eq!(rotr64(1, 1), 1 << 63);
        assert_eq!(rotr64(1, -1), 2);
    }

    #[test]
    fn div_returns_quotient_and_remainder_truncated_toward_zero() {
        assert_eq!(div(7, 2), Some((3, 1)));
        assert_eq!(div(-7, 2), Some((-3, -1)));
        assert_eq!(div(7, -2), Some((-3, 1)));
        assert_eq!(div(0, 5), Some((0, 0)));
        assert_eq!(div(i32::MIN, 2), Some((i32::MIN / 2, 0)));
        assert_eq!(ldiv(-9, 4), Some((-2, -1)));
    }

    #[test]
    fn div_rejects_zero_divisor_and_minimum_over_minus_one() {
        assert_eq!(div(1, 0), None);
        assert_eq!(ldiv(i32::MIN, -1), None);
        assert_eq!(div(i32::MIN, -1), None);
    }

    #[test]
    fn classification_matches_wine_table() {
        // Wine's `_fpclass`, `_finite`, `_isnan`, `_dclass`, `_dsign` and `_dpcomp`
        // (and their float forms) on the C build of `math.c`, one row per (x, y).
        // Columns: fpclass(x), fpclassf(y), finite(x), finitef(y), isnan(x), isnanf(y),
        // dclass(x), fdclass(y), dsign(x), fdsign(y), dpcomp(x, y), fdpcomp(y, x).
        let rows: &[(u64, u32, [i64; 12])] = &[
            (
                0x0000000000000000,
                0x00000000,
                [64, 64, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2],
            ),
            (
                0x8000000000000000,
                0x80000000,
                [32, 32, 1, 1, 0, 0, 0, 0, 32768, 32768, 2, 2],
            ),
            (
                0x0000000000000001,
                0x00000000,
                [128, 64, 1, 1, 0, 0, -2, 0, 0, 0, 4, 2],
            ),
            (
                0x8000000000000001,
                0x80000000,
                [16, 32, 1, 1, 0, 0, -2, 0, 32768, 32768, 1, 2],
            ),
            (
                0x0010000000000000,
                0x00000000,
                [256, 64, 1, 1, 0, 0, -1, 0, 0, 0, 4, 2],
            ),
            (
                0x8010000000000000,
                0x00000000,
                [8, 64, 1, 1, 0, 0, -1, 0, 32768, 0, 1, 2],
            ),
            (
                0x7ff0000000000000,
                0x00000000,
                [512, 64, 0, 1, 0, 0, 1, 0, 0, 0, 4, 1],
            ),
            (
                0xfff0000000000000,
                0x00000000,
                [4, 64, 0, 1, 0, 0, 1, 0, 32768, 0, 1, 4],
            ),
            (
                0x7ff0000000000001,
                0x00000000,
                [1, 64, 0, 1, 1, 0, 2, 0, 0, 0, 0, 0],
            ),
            (
                0x7ff8000000000000,
                0x80000000,
                [2, 32, 0, 1, 1, 0, 2, 0, 0, 32768, 0, 0],
            ),
            (
                0x0000000000000000,
                0x00000001,
                [64, 128, 1, 1, 0, 0, 0, -2, 0, 0, 1, 4],
            ),
            (
                0x0000000000000000,
                0x80000001,
                [64, 16, 1, 1, 0, 0, 0, -2, 0, 32768, 4, 1],
            ),
            (
                0x0000000000000000,
                0x7f800000,
                [64, 512, 1, 0, 0, 0, 0, 1, 0, 0, 1, 4],
            ),
            (
                0x0000000000000000,
                0xff800000,
                [64, 4, 1, 0, 0, 0, 0, 1, 0, 32768, 4, 1],
            ),
            (
                0x0000000000000000,
                0x7fc00000,
                [64, 2, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            ),
            (
                0x0000000000000000,
                0x7f800001,
                [64, 1, 1, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            ),
            (
                0x0000000000000001,
                0x00000001,
                [128, 128, 1, 1, 0, 0, -2, -2, 0, 0, 1, 4],
            ),
            (
                0x8000000000000000,
                0x80000001,
                [32, 16, 1, 1, 0, 0, 0, -2, 32768, 32768, 4, 1],
            ),
            (
                0x7ff8000000000000,
                0x00000001,
                [2, 128, 0, 1, 1, 0, 2, -2, 0, 0, 0, 0],
            ),
            (
                0x0010000000000000,
                0x80000000,
                [256, 32, 1, 1, 0, 0, -1, 0, 0, 32768, 4, 2],
            ),
            (
                0x7ff0000000000000,
                0x80000000,
                [512, 32, 0, 1, 0, 0, 1, 0, 0, 32768, 4, 1],
            ),
            (
                0x8000000000000000,
                0x7f800000,
                [32, 512, 1, 0, 0, 0, 0, 1, 32768, 0, 1, 4],
            ),
            (
                0x3ff0000000000000,
                0x3f800000,
                [256, 256, 1, 1, 0, 0, -1, -1, 0, 0, 2, 2],
            ),
            (
                0xbff0000000000000,
                0xbf800000,
                [8, 8, 1, 1, 0, 0, -1, -1, 32768, 32768, 2, 2],
            ),
        ];
        for &(x_bits, y_bits, expected) in rows {
            let (x, y) = (f64::from_bits(x_bits), f32::from_bits(y_bits));
            let got = [
                fpclass(x) as i64,
                fpclassf(y) as i64,
                finite(x) as i64,
                finitef(y) as i64,
                isnan(x) as i64,
                isnanf(y) as i64,
                dclass(x) as i64,
                fdclass(y) as i64,
                dsign(x) as i64,
                fdsign(y) as i64,
                dpcomp(x, y as f64) as i64,
                fdpcomp(y, x as f32) as i64,
            ];
            assert_eq!(got, expected, "x {x_bits:#018x}, y {y_bits:#010x}");
        }
    }

    #[test]
    fn chgsign_flips_only_the_sign_bit() {
        assert_eq!(chgsign(0.0).to_bits(), 1 << 63);
        assert_eq!(chgsign(-0.0).to_bits(), 0);
        assert_eq!(chgsignf(1.0).to_bits(), 0x8000_0000 | 1.0_f32.to_bits());
    }
}
