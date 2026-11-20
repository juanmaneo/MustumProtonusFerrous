//! The CRC-32 of `RtlComputeCrc32`, which the NTLM1 signature uses. `crc` is the value of the
//! previous call, so the calls chain over several buffers.

pub(crate) fn crc32(crc: u32, data: &[u8]) -> u32 {
    let mut c = !crc;
    for &byte in data {
        c ^= u32::from(byte);
        for _ in 0..8 {
            c = if c & 1 != 0 {
                (c >> 1) ^ 0xedb8_8320
            } else {
                c >> 1
            };
        }
    }
    !c
}
