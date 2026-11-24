//! `token_to_str` from `dlls/fusion/fusionpriv.h`: a public key token as lowercase hex.

/// Bytes in a public key token.
pub const BYTES_PER_TOKEN: usize = 8;

/// Wide characters in the token string, including the terminating NUL (`TOKEN_LENGTH`).
pub const TOKEN_LENGTH: usize = BYTES_PER_TOKEN * 2 + 1;

/// Writes each byte as two lowercase hex digits and a NUL, as the C does into its buffer.
pub fn token_to_str(bytes: &[u8; BYTES_PER_TOKEN]) -> [u16; TOKEN_LENGTH] {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut out = [0; TOKEN_LENGTH];
    for (i, &byte) in bytes.iter().enumerate() {
        out[i * 2] = u16::from(HEX[usize::from(byte >> 4)]);
        out[i * 2 + 1] = u16::from(HEX[usize::from(byte & 0x0F)]);
    }
    out
}
