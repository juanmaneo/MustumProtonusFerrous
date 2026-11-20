//! The AES key length check and the derived key copy, from Wine's
//! `dlls/bcrypt/bcrypt_main.c`.

use crate::{Reply, Status};

/// A key length range in bits, as `BCRYPT_KEY_LENGTHS_STRUCT` holds it.
#[derive(Debug, Clone, Copy)]
pub struct KeyLengths {
    pub min: u32,
    pub max: u32,
    pub increment: u32,
}

/// Checks a secret of `secret_len` bytes against `lengths`, as `validate_len_aes`
/// does. A secret longer than the maximum is clamped to it. Returns the key
/// length in bytes.
pub fn validate_len_aes(lengths: KeyLengths, secret_len: u32) -> Result<u32, Status> {
    let max = lengths.max / 8;
    if secret_len > max {
        return Ok(max);
    }
    if secret_len < lengths.min / 8 {
        return Err(Status::InvalidParameter);
    }
    // Wine does this in unsigned arithmetic, so a minimum that is not a multiple
    // of 8 wraps around. The product cannot overflow, because `secret_len` is at
    // most `max / 8`.
    if lengths.increment != 0
        && !(secret_len * 8)
            .wrapping_sub(lengths.min)
            .is_multiple_of(lengths.increment)
    {
        return Err(Status::InvalidParameter);
    }
    Ok(secret_len)
}

/// Copies a derived key into `output` with its bytes reversed, as `derive_key_raw`
/// does. Without an output buffer it reports the key length. Otherwise it copies
/// `min(derived.len(), output.len())` bytes and reports that count.
pub fn derive_key_raw(derived: &[u8], output: Option<&mut [u8]>) -> Reply {
    let Some(output) = output else {
        return (Status::Success, derived.len() as u32);
    };
    let len = derived.len().min(output.len());
    for (dst, src) in output[..len].iter_mut().zip(derived[..len].iter().rev()) {
        *dst = *src;
    }
    (Status::Success, len as u32)
}
