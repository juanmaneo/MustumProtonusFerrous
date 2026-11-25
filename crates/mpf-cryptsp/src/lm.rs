//! `SystemFunction006` and `SystemFunction008` from Wine's `dlls/cryptsp/lmhash.c`: the LM hash
//! of a password, and the LM challenge response built from that hash.

use crate::des::{take, Des};

/// `LMhash_Magic`: the plaintext that both LM hash halves encrypt.
const LM_MAGIC: [u8; 8] = *b"KGS!@#$%";

/// `SystemFunction006`: LM hash of `password`.
///
/// `password` is read as a C string. Bytes from the first NUL onward are ignored, and a slice
/// without a NUL is used whole. Only the first 14 bytes take part in the hash.
pub fn system_function_006(des: &impl Des, password: &[u8]) -> [u8; 16] {
    let len = password
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(password.len());
    let max = len.min(14);
    let mut tmp = [0u8; 14];
    tmp[..max].copy_from_slice(&password[..max]);

    let mut hash = [0u8; 16];
    hash[..8].copy_from_slice(&des.encrypt_block(&take(&tmp, 0), &LM_MAGIC));
    hash[8..].copy_from_slice(&des.encrypt_block(&take(&tmp, 7), &LM_MAGIC));
    hash
}

/// `SystemFunction008` (alias `SystemFunction009`): LM response to an 8-byte challenge.
///
/// The 16-byte hash is zero-padded to 21 bytes and split into three 7-byte DES keys. The
/// challenge is encrypted under each key in turn.
pub fn system_function_008(des: &impl Des, challenge: &[u8; 8], hash: &[u8; 16]) -> [u8; 24] {
    let mut key = [0u8; 21];
    key[..16].copy_from_slice(hash);

    let mut response = [0u8; 24];
    for i in 0..3 {
        let block = des.encrypt_block(&take(&key, 7 * i), challenge);
        response[8 * i..8 * i + 8].copy_from_slice(&block);
    }
    response
}
