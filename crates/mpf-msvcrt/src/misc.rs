//! Byte swapping and `rand`/`srand` from Wine's `dlls/msvcrt/misc.c`.
//!
//! Wine keeps the `rand` seed per thread (`thread_data_t::random_seed`, initial
//! value 1). This port keeps it in an explicit `Rand` value instead of a global.

/// `RAND_MAX` on Windows.
pub const RAND_MAX: i32 = 0x7fff;

pub fn byteswap_ushort(value: u16) -> u16 {
    value.swap_bytes()
}

pub fn byteswap_ulong(value: u32) -> u32 {
    value.swap_bytes()
}

pub fn byteswap_uint64(value: u64) -> u64 {
    value.swap_bytes()
}

/// The linear congruential generator behind MSVC's `rand`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rand {
    seed: u32,
}

impl Default for Rand {
    fn default() -> Self {
        Self { seed: 1 }
    }
}

impl Rand {
    /// `srand`: replaces the seed.
    pub fn srand(&mut self, seed: u32) {
        self.seed = seed;
    }

    /// `rand`: advances the seed and returns a value in `0..=RAND_MAX`.
    pub fn rand(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.seed >> 16) & RAND_MAX as u32) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byteswap_reverses_each_width() {
        assert_eq!(byteswap_ushort(0x1234), 0x3412);
        assert_eq!(byteswap_ulong(0x1234_5678), 0x7856_3412);
        assert_eq!(
            byteswap_uint64(0x0102_0304_0506_0708),
            0x0807_0605_0403_0201
        );
    }

    #[test]
    fn default_seed_produces_msvc_sequence() {
        let mut rand = Rand::default();
        let values: Vec<i32> = (0..5).map(|_| rand.rand()).collect();
        assert_eq!(values, [41, 18467, 6334, 26500, 19169]);
    }

    #[test]
    fn srand_restarts_the_sequence() {
        let mut rand = Rand::default();
        rand.srand(0);
        let first = rand.rand();
        rand.srand(0);
        assert_eq!(rand.rand(), first);
        assert_eq!(first, 38);
    }

    #[test]
    fn values_stay_within_rand_max() {
        let mut rand = Rand::default();
        rand.srand(u32::MAX);
        for _ in 0..1000 {
            let value = rand.rand();
            assert!((0..=RAND_MAX).contains(&value));
        }
    }
}
