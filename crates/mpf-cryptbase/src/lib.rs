//! Deterministic DES helpers from Wine's `dlls/cryptbase/des.c`.
//!
//! This crate ports the DES block routines and the deterministic portions of
//! SystemFunction001 through SystemFunction005. It does not implement OS-backed
//! cryptbase exports.

mod des;

pub type NtStatus = i32;

pub const STATUS_SUCCESS: NtStatus = 0;
pub const STATUS_UNSUCCESSFUL: NtStatus = 0xC000_0001u32 as i32;
pub const STATUS_INVALID_PARAMETER: NtStatus = 0xC000_000Du32 as i32;
pub const STATUS_INVALID_PARAMETER_2: NtStatus = 0xC000_00F0u32 as i32;
pub const STATUS_UNKNOWN_REVISION: NtStatus = 0xC000_0058u32 as i32;
pub const STATUS_BUFFER_TOO_SMALL: NtStatus = 0xC000_0023u32 as i32;

const LMHASH_MAGIC: [u8; 8] = *b"KGS!@#$%";

/// Encrypts one 8-byte block with the 7-byte key used by Wine's DES routines.
pub fn encrypt_block(key: &[u8; 7], data: &[u8; 8]) -> [u8; 8] {
    des::hash(key, data)
}

/// Decrypts one 8-byte block with the 7-byte key used by Wine's DES routines.
pub fn decrypt_block(key: &[u8; 7], data: &[u8; 8]) -> [u8; 8] {
    des::unhash(key, data)
}

/// Implements SystemFunction003's DES hash of the fixed LM hash magic block.
pub fn hash_lm_key(key: &[u8; 7]) -> [u8; 8] {
    encrypt_block(key, &LMHASH_MAGIC)
}

/// Implements SystemFunction004's length-prefixed DES-ECB encoding.
///
/// The key is zero-padded or truncated to seven bytes. The returned vector
/// includes the encrypted 8-byte length/version header and zero-padded payload.
pub fn encrypt_data(data: &[u8], key: &[u8], output_capacity: usize) -> Result<Vec<u8>, NtStatus> {
    encrypt_data_with_length(data, key, output_capacity, data.len())
}

fn encrypt_data_with_length(
    data: &[u8],
    key: &[u8],
    output_capacity: usize,
    input_length: usize,
) -> Result<Vec<u8>, NtStatus> {
    if key.is_empty() {
        return Err(STATUS_INVALID_PARAMETER_2);
    }

    let crypt_len = rounded_len(input_length)?;
    let output_len = with_header(crypt_len)?;
    let input_len = wire_len(input_length)?;
    if output_capacity < output_len {
        return Err(STATUS_BUFFER_TOO_SMALL);
    }

    let des_key = normalize_key(key);
    let mut output = vec![0; output_len];
    let header = input_len.to_le_bytes();
    let mut header_block = [0; 8];
    header_block[..4].copy_from_slice(&header);
    header_block[4..].copy_from_slice(&1u32.to_le_bytes());
    output[..8].copy_from_slice(&encrypt_block(&des_key, &header_block));

    for (index, chunk) in data.chunks(8).enumerate() {
        let mut block = [0; 8];
        block[..chunk.len()].copy_from_slice(chunk);
        let start = 8 + index * 8;
        output[start..start + 8].copy_from_slice(&encrypt_block(&des_key, &block));
    }
    Ok(output)
}

/// Implements SystemFunction005's length-prefixed DES-ECB decoding.
///
/// The input must contain the encrypted header and every ciphertext block
/// implied by its plaintext length. This safely rejects malformed buffers that
/// the original C routine would read past.
pub fn decrypt_data(data: &[u8], key: &[u8], output_capacity: usize) -> Result<Vec<u8>, NtStatus> {
    if key.is_empty() {
        return Err(STATUS_INVALID_PARAMETER_2);
    }
    if data.len() < 8 {
        return Err(STATUS_INVALID_PARAMETER);
    }

    let des_key = normalize_key(key);
    let mut encrypted_header = [0; 8];
    encrypted_header.copy_from_slice(&data[..8]);
    let header = decrypt_block(&des_key, &encrypted_header);
    let plain_len = u32::from_le_bytes([header[0], header[1], header[2], header[3]]) as usize;
    let version = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    if version != 1 {
        return Err(STATUS_UNKNOWN_REVISION);
    }
    decrypt_payload(data, &des_key, output_capacity, plain_len)
}

fn decrypt_payload(
    data: &[u8],
    des_key: &[u8; 7],
    output_capacity: usize,
    plain_len: usize,
) -> Result<Vec<u8>, NtStatus> {
    if plain_len > output_capacity {
        return Err(STATUS_BUFFER_TOO_SMALL);
    }

    let cipher_len = rounded_len(plain_len)?;
    let expected_len = with_header(cipher_len)?;
    if data.len() < expected_len {
        return Err(STATUS_INVALID_PARAMETER);
    }

    let mut output = Vec::with_capacity(plain_len);
    for block in data[8..expected_len].as_chunks::<8>().0 {
        output.extend_from_slice(&decrypt_block(des_key, block));
    }
    output.truncate(plain_len);
    Ok(output)
}

fn rounded_len(len: usize) -> Result<usize, NtStatus> {
    len.checked_add(7)
        .map(|rounded| rounded & !7)
        .ok_or(STATUS_INVALID_PARAMETER)
}

fn with_header(len: usize) -> Result<usize, NtStatus> {
    len.checked_add(8).ok_or(STATUS_INVALID_PARAMETER)
}

fn wire_len(len: usize) -> Result<u32, NtStatus> {
    u32::try_from(len).map_err(|_| STATUS_INVALID_PARAMETER)
}

fn normalize_key(key: &[u8]) -> [u8; 7] {
    let mut result = [0; 7];
    let count = key.len().min(result.len());
    result[..count].copy_from_slice(&key[..count]);
    result
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
