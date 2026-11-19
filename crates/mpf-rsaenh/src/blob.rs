//! The blob checks of `import_key`, `import_private_key`, `import_public_key` and `import_plaintext_key`
//! in `dlls/rsaenh/rsaenh.c`.
//!
//! An RSA blob is a `BLOBHEADER` (8 bytes), an `RSAPUBKEY` (12 bytes: magic, bitlen, pubexp) and the key
//! material. A plaintext blob is a `BLOBHEADER`, a 4-byte key length and the key.

use crate::{Nte, CRYPT_IPSEC_HMAC_KEY};

pub const SIMPLEBLOB: u8 = 0x01;
pub const PUBLICKEYBLOB: u8 = 0x06;
pub const PRIVATEKEYBLOB: u8 = 0x07;
pub const PLAINTEXTKEYBLOB: u8 = 0x08;
pub const CUR_BLOB_VERSION: u8 = 0x02;
pub const RSAENH_MAGIC_RSA1: u32 = 0x3141_5352;
pub const RSAENH_MAGIC_RSA2: u32 = 0x3241_5352;

/// `sizeof(BLOBHEADER)`.
const BLOB_HEADER_LEN: usize = 8;
/// `sizeof(BLOBHEADER) + sizeof(RSAPUBKEY)`: the offset of the key material in an RSA blob.
const RSA_KEY_OFFSET: u64 = 20;
/// `sizeof(BLOBHEADER) + sizeof(DWORD)`: the offset of the key bytes in a plaintext blob.
const PLAINTEXT_KEY_OFFSET: u64 = 12;

/// The kind of a blob, as `import_key` dispatches on `bType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlobKind {
    /// `PRIVATEKEYBLOB`.
    PrivateKey,
    /// `PUBLICKEYBLOB`.
    PublicKey,
    /// `SIMPLEBLOB`: a session key wrapped under a public key. Importing it needs a key handle, so
    /// this crate stops at the kind.
    SimpleKey,
    /// `PLAINTEXTKEYBLOB`.
    PlaintextKey,
}

/// The `RSAPUBKEY` of an RSA blob.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RsaPubKey {
    pub magic: u32,
    pub bitlen: u32,
    pub pubexp: u32,
}

/// The header checks of `import_key`, then its `bType` switch.
pub fn classify_blob(blob: &[u8]) -> Result<BlobKind, Nte> {
    if blob.len() < BLOB_HEADER_LEN || blob[1] != CUR_BLOB_VERSION {
        return Err(Nte::BadData);
    }
    match blob[0] {
        PRIVATEKEYBLOB => Ok(BlobKind::PrivateKey),
        PUBLICKEYBLOB => Ok(BlobKind::PublicKey),
        SIMPLEBLOB => Ok(BlobKind::SimpleKey),
        PLAINTEXTKEYBLOB => Ok(BlobKind::PlaintextKey),
        _ => Err(Nte::BadType),
    }
}

/// The checks of `import_private_key`: the flags, the `RSAPUBKEY` magic and the blob length.
pub fn check_private_key_blob(blob: &[u8], flags: u32) -> Result<RsaPubKey, Nte> {
    check_rsa_blob(blob, flags, RSAENH_MAGIC_RSA2, private_blob_len)
}

/// The checks of `import_public_key`: the flags, the `RSAPUBKEY` magic and the blob length.
pub fn check_public_key_blob(blob: &[u8], flags: u32) -> Result<RsaPubKey, Nte> {
    check_rsa_blob(blob, flags, RSAENH_MAGIC_RSA1, public_blob_len)
}

fn check_rsa_blob(
    blob: &[u8],
    flags: u32,
    magic: u32,
    blob_len: fn(u32) -> u64,
) -> Result<RsaPubKey, Nte> {
    if flags & CRYPT_IPSEC_HMAC_KEY != 0 {
        return Err(Nte::BadFlags);
    }
    let key = rsa_pub_key(blob)?;
    if key.magic != magic || (blob.len() as u64) < blob_len(key.bitlen) {
        return Err(Nte::BadData);
    }
    Ok(key)
}

/// The length a private key blob needs: the header, `bitlen / 8` bytes of modulus, then five CRT
/// components of about `bitlen / 16` bytes each, sized as `(bitlen + 8) >> 4`.
pub(crate) fn private_blob_len(bitlen: u32) -> u64 {
    let bitlen = u64::from(bitlen);
    RSA_KEY_OFFSET + (bitlen >> 3) + 5 * ((bitlen + 8) >> 4)
}

/// The length a public key blob needs: the header and `bitlen / 8` bytes of modulus.
pub(crate) fn public_blob_len(bitlen: u32) -> u64 {
    RSA_KEY_OFFSET + (u64::from(bitlen) >> 3)
}

/// The checks of `import_plaintext_key`. Returns the key bytes that follow the key length.
pub fn plaintext_key(blob: &[u8]) -> Result<&[u8], Nte> {
    if (blob.len() as u64) < PLAINTEXT_KEY_OFFSET {
        return Err(Nte::BadData);
    }
    let key_len = le_u32(blob, 8);
    if (blob.len() as u64) < PLAINTEXT_KEY_OFFSET + u64::from(key_len) {
        return Err(Nte::BadData);
    }
    Ok(&blob[12..12 + key_len as usize])
}

fn rsa_pub_key(blob: &[u8]) -> Result<RsaPubKey, Nte> {
    if (blob.len() as u64) < RSA_KEY_OFFSET {
        return Err(Nte::BadData);
    }
    Ok(RsaPubKey {
        magic: le_u32(blob, 8),
        bitlen: le_u32(blob, 12),
        pubexp: le_u32(blob, 16),
    })
}

fn le_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}
