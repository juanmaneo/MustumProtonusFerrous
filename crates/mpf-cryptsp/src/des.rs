//! `SystemFunction012`, `SystemFunction013`, `SystemFunction024` and `SystemFunction025` from
//! Wine's `dlls/cryptsp/lmhash.c`: each call runs two DES blocks, one per 7-byte key.

/// One DES block with a 7-byte key: encryption as `SystemFunction001` and decryption as
/// `SystemFunction002` in `dlls/cryptbase`.
pub trait Des {
    fn encrypt_block(&self, key: &[u8; 7], data: &[u8; 8]) -> [u8; 8];
    fn decrypt_block(&self, key: &[u8; 7], data: &[u8; 8]) -> [u8; 8];
}

/// Copies `N` bytes starting at `at`, as the C code's `key + at` pointer does.
pub(crate) fn take<const N: usize>(bytes: &[u8], at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes[at..at + N]);
    out
}

/// Block 0 uses `key[0..7]` and `data[0..8]`. Block 1 uses `key[7..14]` and `data[8..16]`.
fn two_blocks(
    block: impl Fn(&[u8; 7], &[u8; 8]) -> [u8; 8],
    data: &[u8; 16],
    key: &[u8],
) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&block(&take(key, 0), &take(data, 0)));
    out[8..].copy_from_slice(&block(&take(key, 7), &take(data, 8)));
    out
}

/// `memcpy` of the 4-byte key four times into a 16-byte buffer.
fn expand_key4(key: &[u8; 4]) -> [u8; 16] {
    std::array::from_fn(|i| key[i % 4])
}

/// `SystemFunction012` (aliases 014, 016, 018, 020 and 022): encrypts two DES blocks.
pub fn system_function_012(des: &impl Des, data: &[u8; 16], key: &[u8; 14]) -> [u8; 16] {
    two_blocks(|k, d| des.encrypt_block(k, d), data, key)
}

/// `SystemFunction013` (aliases 015, 021 and 023): decrypts two DES blocks.
pub fn system_function_013(des: &impl Des, data: &[u8; 16], key: &[u8; 14]) -> [u8; 16] {
    two_blocks(|k, d| des.decrypt_block(k, d), data, key)
}

/// `SystemFunction024` (alias 026): encrypts two DES blocks under a 4-byte key.
pub fn system_function_024(des: &impl Des, data: &[u8; 16], key: &[u8; 4]) -> [u8; 16] {
    two_blocks(|k, d| des.encrypt_block(k, d), data, &expand_key4(key))
}

/// `SystemFunction025` (alias 027): decrypts two DES blocks under a 4-byte key.
pub fn system_function_025(des: &impl Des, data: &[u8; 16], key: &[u8; 4]) -> [u8; 16] {
    two_blocks(|k, d| des.decrypt_block(k, d), data, &expand_key4(key))
}
