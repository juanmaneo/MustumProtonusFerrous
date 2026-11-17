//! Integer helpers from Wine's `dlls/msvcrt/math.c`.
//!
//! `abs` and `labs` wrap on the minimum value, as the GCC build of Wine does.
//! The C standard leaves that case undefined. `long` is 32 bits on Windows,
//! so `labs` and the `l` rotations share the 32-bit types.

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
}
