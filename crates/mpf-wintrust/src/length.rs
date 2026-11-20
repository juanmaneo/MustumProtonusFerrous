//! DER length octets: `CRYPT_EncodeLen` and `CRYPT_GetLen` from `dlls/wintrust/asn.c`.

use crate::Error;

/// The most length octets (the first one included) that `CRYPT_GetLen` accepts: `sizeof(DWORD) + 1`.
const MAX_FIELD_LEN: usize = std::mem::size_of::<u32>() + 1;

/// `CRYPT_EncodeLen`: one octet up to `0x7f`, otherwise `0x80 | n` followed by the `n`
/// significant big-endian octets of `len`.
pub fn encode(len: usize) -> Vec<u8> {
    if len <= 0x7f {
        return vec![len as u8];
    }
    let bytes = len.to_be_bytes();
    let significant = &bytes[(len.leading_zeros() / 8) as usize..];
    let mut out = vec![0x80 | significant.len() as u8];
    out.extend_from_slice(significant);
    out
}

/// `CRYPT_GetLen`: the content length of the element at the start of `input`. Indefinite
/// lengths (`0x80`) are corrupt, as in Wine.
pub fn decode(input: &[u8]) -> Result<u32, Error> {
    if input.len() <= 1 {
        return Err(Error::Corrupt);
    }
    let first = input[1];
    if first <= 0x7f {
        if usize::from(first) + 1 > input.len() {
            return Err(Error::EndOfData);
        }
        return Ok(u32::from(first));
    }
    if first == 0x80 {
        return Err(Error::Corrupt);
    }
    let width = field_len(first);
    if width > MAX_FIELD_LEN {
        return Err(Error::TooLarge);
    }
    if width + 2 > input.len() {
        return Err(Error::Corrupt);
    }
    let value = input[2..=width]
        .iter()
        .fold(0u32, |value, &byte| (value << 8) | u32::from(byte));
    if (value.wrapping_add(1) as usize) > input.len() {
        return Err(Error::EndOfData);
    }
    Ok(value)
}

/// The `len` content octets of the element at the start of `input`. `input` must have passed [`decode`].
pub(crate) fn content(input: &[u8], len: u32) -> Result<&[u8], Error> {
    let start = 1 + field_len(input[1]);
    input[start..].get(..len as usize).ok_or(Error::EndOfData)
}

/// The number of octets in the length field that begins with `first`.
pub(crate) fn field_len(first: u8) -> usize {
    if first <= 0x7f {
        1
    } else {
        1 + usize::from(first & 0x7f)
    }
}
