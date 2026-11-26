/// `INFINITE` from `include/winbase.h`.
pub const INFINITE: u32 = 0xFFFF_FFFF;

/// `get_nt_timeout` from `dlls/ktmw32/ktmw32_main.c`. Returns the relative NT timeout in 100 ns
/// units, or `None` (a NULL `PLARGE_INTEGER`) for `INFINITE`. The C product is unsigned, so it
/// wraps modulo 2^64 before it is stored in the signed `QuadPart`.
pub fn get_nt_timeout(timeout: u32) -> Option<i64> {
    if timeout == INFINITE {
        return None;
    }
    Some(u64::from(timeout).wrapping_mul(-10_000_i64 as u64) as i64)
}
