//! ProcessPrng: the chunked fill loop over RtlGenRandom.

/// ULONG_MAX on the PE target (msvcrt/limits.h, 0xffffffffUL). ProcessPrng
/// caps each RtlGenRandom call at this length.
pub const ULONG_MAX: u64 = 0xffff_ffff;

/// min(size, (SIZE_T)ULONG_MAX): length of the next RtlGenRandom call while
/// `remaining` bytes are left. SIZE_T is modelled as u64 (Win64).
pub fn next_chunk_len(remaining: u64) -> u64 {
    remaining.min(ULONG_MAX)
}

/// The loop in ProcessPrng over `size` bytes. `rtl_gen_random(offset, len)`
/// stands in for RtlGenRandom(data + offset, len) and returns its BOOLEAN
/// result. The first failed call ends the loop with false, and later calls
/// are not made.
pub fn process_prng_chunks(size: u64, rtl_gen_random: &mut dyn FnMut(u64, u64) -> bool) -> bool {
    let mut offset = 0;
    while offset < size {
        let len = next_chunk_len(size - offset);
        if !rtl_gen_random(offset, len) {
            return false;
        }
        offset += len;
    }
    true
}

/// ProcessPrng(data, size) with size = data.len(). `rtl_gen_random` fills each
/// chunk it receives and returns its BOOLEAN result.
pub fn process_prng(data: &mut [u8], rtl_gen_random: &mut dyn FnMut(&mut [u8]) -> bool) -> bool {
    process_prng_chunks(data.len() as u64, &mut |offset, len| {
        let start = offset as usize;
        rtl_gen_random(&mut data[start..start + len as usize])
    })
}
