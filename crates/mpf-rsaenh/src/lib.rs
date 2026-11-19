//! rsaenh subset for MPF: the PKCS #1 padding, hash signature encoding, key blob checks and
//! `copy_param` of `dlls/rsaenh/rsaenh.c`.
//!
//! Reference Wine's `dlls/rsaenh` implements the CryptoAPI RSA provider. Most of `rsaenh.c` touches
//! handles, key containers, the random source or the crypto back ends. This crate ports the pure byte
//! and value logic that those paths apply to their buffers. It has no handles, no locks and no random
//! source: the caller passes the random bytes in. Blob fields are little-endian, as on the x86 and x64
//! builds that the C runs on.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `copy_param`: the size-checked copy of a parameter into a caller buffer.
//! - `build_hash_signature`: the PKCS #1 v1.5 and X9.31 signature encodings, with the DigestInfo
//!   prefixes for MD2, MD4, MD5, SHA-1, SHA-256, SHA-384, SHA-512 and the SSL 3.0 MD5 plus SHA-1 pair.
//! - `pad_data_pkcs1` and `unpad_data_pkcs1`: PKCS #1 v1.5 block type 2, with the SSL 2 fallback fill.
//! - `classify_blob`: the header checks and the `bType` switch of `import_key`.
//! - `check_private_key_blob` and `check_public_key_blob`: the flag, magic and length checks of
//!   `import_private_key` and `import_public_key`.
//! - `plaintext_key`: the length check of `import_plaintext_key`.
//!
//! Deviations from the C. Each one is an input where the C either writes past a buffer, which ASan flags
//! and the oracle excludes, or accepts input that its unsigned arithmetic should reject, which ASan does
//! not flag. The crate rejects both kinds, and the edge checks pin each one.
//! - `build_hash_signature` with an output shorter than the encoding returns `Nte::BadLen`. The C writes
//!   past the end (ASan `heap-buffer-overflow`).
//! - `pad_data_pkcs1` with an output shorter than `data.len() + 11` returns `Nte::BadLen`. Below 11 bytes,
//!   the C's `dwBufferLen - 11` wraps, so its check never fires. Data that fills or overflows the output makes
//!   the C write past the end (ASan). Shorter data gives a block with fewer than 8 padding bytes, and data
//!   within two bytes of the output length has its first bytes overwritten by the separator and the header
//!   (`00 02 00 42 43` for `ABC` in 5 bytes, not flagged). A 9-byte output with `hello` gives
//!   `00 02 20 00 'hello'`, also not flagged.
//! - `plaintext_key` with fewer than 12 bytes returns `Nte::BadData`. The C reads the key length past the
//!   end (ASan `heap-buffer-overflow`).
//! - `check_private_key_blob` computes the required length in 64 bits. The C adds 8 to `bitlen` in 32 bits,
//!   so `bitlen >= 0xfffffff8` wraps to a small requirement, and the C accepts a blob without the five
//!   CRT components. This crate rejects that blob. The oracle cannot reach this case at a practical size.
//! - `copy_param` returns the required size as `Err`, where the C writes it to `*pdwBufferSize`.
//!
//! Skipped (reason per item):
//! - `get_key_container`, `new_key`, `setup_key`, `release_and_install_key`, `copy_data_blob`, and
//!   `import_symmetric_key` (`SIMPLEBLOB`): handle and key-container lookups.
//! - `import_private_key_impl`, `import_public_key_impl`, `encrypt_block_impl`, `decrypt_block_impl` and
//!   `setup_key_impl`: the crypto back ends.
//! - `tls1_p` and `tls1_prf`: they need an HMAC hash handle from the provider.
//! - `pkcs1_mgf1` and the OAEP paths: they need a hash object from the provider.
//! - The `CRYPT_IPSEC_HMAC_KEY` plaintext branch: it needs a key handle and HMAC state.
//! - `CRYPT_EXPORTABLE` and the `dwPermissions` update: they belong to the key object.
//! - `RtlGenRandom`: the caller supplies the random bytes.
//! - The `TRACE`, `WARN`, `FIXME` and `ERR` logging.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

mod blob;
mod pkcs1;
mod signature;

pub use blob::{
    check_private_key_blob, check_public_key_blob, classify_blob, plaintext_key, BlobKind,
    RsaPubKey,
};
pub use pkcs1::{pad_data_pkcs1, unpad_data_pkcs1};
pub use signature::{
    build_hash_signature, CALG_MD2, CALG_MD4, CALG_MD5, CALG_SHA, CALG_SHA_256, CALG_SHA_384,
    CALG_SHA_512, CALG_SSL3_SHAMD5,
};

/// `CRYPT_NOHASHOID`: the signature omits the DigestInfo prefix.
pub const CRYPT_NOHASHOID: u32 = 0x0000_0001;
/// `CRYPT_SSL2_FALLBACK`: the padding ends with eight `03` bytes.
pub const CRYPT_SSL2_FALLBACK: u32 = 0x0000_0002;
/// `CRYPT_X931_FORMAT`: the signature uses the X9.31 layout.
pub const CRYPT_X931_FORMAT: u32 = 0x0000_0004;
/// `CRYPT_IPSEC_HMAC_KEY`: the key is an HMAC key, which this crate does not import.
pub const CRYPT_IPSEC_HMAC_KEY: u32 = 0x0000_0100;

/// The `NTE_*` error the C reports through `SetLastError`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nte {
    /// `NTE_BAD_LEN`: the buffer cannot hold the output.
    BadLen,
    /// `NTE_BAD_DATA`: the data is malformed.
    BadData,
    /// `NTE_BAD_FLAGS`: the flags are not supported.
    BadFlags,
    /// `NTE_BAD_TYPE`: the blob type is unknown.
    BadType,
    /// `NTE_BAD_ALGID`: the algorithm has no DigestInfo.
    BadAlgId,
}

impl Nte {
    /// The 32-bit `NTE_*` value.
    pub const fn code(self) -> u32 {
        match self {
            Nte::BadLen => 0x8009_0004,
            Nte::BadData => 0x8009_0005,
            Nte::BadAlgId => 0x8009_0008,
            Nte::BadFlags => 0x8009_0009,
            Nte::BadType => 0x8009_000a,
        }
    }
}

/// `copy_param`: copies `param` into `buffer`, when there is one, and returns the size of `param`.
///
/// `Err` carries the size the caller must allocate. The C sets `ERROR_MORE_DATA` in that case and
/// leaves `buffer` unchanged.
pub fn copy_param(buffer: Option<&mut [u8]>, param: &[u8]) -> Result<usize, usize> {
    if let Some(buffer) = buffer {
        if param.len() > buffer.len() {
            return Err(param.len());
        }
        buffer[..param.len()].copy_from_slice(param);
    }
    Ok(param.len())
}
