/// The bit pattern of a `double` without its sign, `~0ull >> 1` in `dlls/msvcrt/math.c`.
const MAGNITUDE_MASK: u64 = u64::MAX >> 1;

/// The bit pattern of `+INFINITY` in a `double`, `0x7ffull << 52` in `dlls/msvcrt/math.c`.
const INFINITY_BITS: u64 = 0x7ff << 52;

/// The sign bit of a `double`, `1ull << 63` in `dlls/msvcrt/math.c`.
const SIGN_BIT: u64 = 1 << 63;

/// `abs` from `dlls/msvcrt/math.c`. The C body negates a negative `int`. For `i32::MIN` the
/// negation wraps to `i32::MIN`, as the x86 `neg` instruction does, and `wrapping_abs` gives the
/// same result without a debug-build panic.
pub fn abs(n: i32) -> i32 {
    n.wrapping_abs()
}

/// `labs` from `dlls/msvcrt/math.c`. `__msvcrt_long` is 32 bits on the target, so this is [`abs`].
pub fn labs(n: i32) -> i32 {
    n.wrapping_abs()
}

/// The `div_t` and `ldiv_t` results: a quotient and a remainder, both `int` (`long`, 32 bits, for
/// `ldiv_t`) on the target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DivResult {
    pub quot: i32,
    pub rem: i32,
}

/// `div` from `dlls/msvcrt/math.c`, the `__WINE_PE_BUILD` branch that returns `div_t`.
/// A zero `denom` and `i32::MIN / -1` trap in C (SIGFPE) and panic here.
pub fn div(num: i32, denom: i32) -> DivResult {
    DivResult {
        quot: num / denom,
        rem: num % denom,
    }
}

/// `ldiv` from `dlls/msvcrt/math.c`, the `__WINE_PE_BUILD` branch that returns `ldiv_t`. The
/// traps are the same as for [`div`].
pub fn ldiv(num: i32, denom: i32) -> DivResult {
    DivResult {
        quot: num / denom,
        rem: num % denom,
    }
}

/// `_rotl` from `dlls/msvcrt/math.c`: rotates left by `shift & 31`. The C body reads `num >> 32`
/// when the masked shift is 0, which is undefined, and gives `num` on x86. `rotate_left` gives `num`
/// too.
pub fn rotl(num: u32, shift: i32) -> u32 {
    num.rotate_left((shift & 31) as u32)
}

/// `_lrotl` from `dlls/msvcrt/math.c`. `unsigned long` is 32 bits on the target, so this is [`rotl`].
pub fn lrotl(num: u32, shift: i32) -> u32 {
    rotl(num, shift)
}

/// `_rotr` from `dlls/msvcrt/math.c`: rotates right by `shift & 31`. The masked shift of 0 has the
/// same undefined `num << 32` as [`rotl`], and the result is `num` here too.
pub fn rotr(num: u32, shift: i32) -> u32 {
    num.rotate_right((shift & 31) as u32)
}

/// `_lrotr` from `dlls/msvcrt/math.c`. `unsigned long` is 32 bits on the target, so this is [`rotr`].
pub fn lrotr(num: u32, shift: i32) -> u32 {
    rotr(num, shift)
}

/// `_chgsign` from `dlls/msvcrt/math.c`: flips the sign bit, which also flips the sign of a NaN.
pub fn chgsign(num: f64) -> f64 {
    f64::from_bits(num.to_bits() ^ SIGN_BIT)
}

/// `_finite` from `dlls/msvcrt/math.c`: 1 when the exponent bits are not all ones, so the value is
/// neither an infinity nor a NaN.
pub fn finite(num: f64) -> i32 {
    i32::from((num.to_bits() & MAGNITUDE_MASK) < INFINITY_BITS)
}

/// `_isnan` from `dlls/msvcrt/math.c`: 1 when the value is a NaN, whatever its sign.
pub fn isnan(num: f64) -> i32 {
    i32::from((num.to_bits() & MAGNITUDE_MASK) > INFINITY_BITS)
}
