//! The `checksum` of `fdi.c`, which covers a CFDATA block.

/// XORs the little-endian 32-bit words of `data` into `seed`, then XORs in the
/// trailing bytes that do not fill a word, packed big-endian.
pub fn checksum(data: &[u8], seed: u32) -> u32 {
    let (words, tail) = data.as_chunks::<4>();
    let csum = words
        .iter()
        .fold(seed, |acc, w| acc ^ u32::from_le_bytes(*w));
    let packed = tail.iter().fold(0, |acc, &b| (acc << 8) | u32::from(b));
    csum ^ packed
}
