//! NCryptImportKey: the decrypt, flag and blob checks, then the import. The BCrypt import runs in the caller.

use crate::algid::AlgId;
use crate::calls::{accept_flags, require_provider, Handle};
use crate::key::Key;
use crate::{
    map_ntstatus, NtStatus, Status, NTE_BAD_DATA, NTE_INVALID_PARAMETER, NTE_NOT_SUPPORTED,
    NTE_NO_MEMORY, STATUS_SUCCESS,
};

const RSA_PUBLIC_MAGIC: u32 = 0x3141_5352;
const RSA_PRIVATE_MAGIC: u32 = 0x3241_5352;
const RSA_FULL_PRIVATE_MAGIC: u32 = 0x3341_5352;
const ECDSA_P256_PUBLIC_MAGIC: u32 = 0x3153_4345;
const ECDSA_P256_PRIVATE_MAGIC: u32 = 0x3253_4345;
const ECDSA_P384_PUBLIC_MAGIC: u32 = 0x3353_4345;
const ECDSA_P384_PRIVATE_MAGIC: u32 = 0x3453_4345;
const ECDSA_P521_PUBLIC_MAGIC: u32 = 0x3553_4345;
const ECDSA_P521_PRIVATE_MAGIC: u32 = 0x3653_4345;

/// NCryptImportKey: the decrypt, flag, blob and provider checks, then the key template. The import runs in the caller.
pub fn import_key(
    provider: Option<Handle>,
    decrypt_key_set: bool,
    flags: u32,
    blob: Option<&[u8]>,
    import: &mut dyn FnMut(AlgId) -> NtStatus,
) -> Result<Key, Status> {
    if decrypt_key_set {
        return Err(NTE_NOT_SUPPORTED);
    }
    accept_flags(flags)?;
    let blob = blob.ok_or(NTE_INVALID_PARAMETER)?;
    let (alg, bits) = blob_kind(blob)?;
    require_provider(provider)?;
    Key::new(alg, bits)
        .ok_or(NTE_NO_MEMORY)
        .and_then(|key| match import(alg) {
            STATUS_SUCCESS => Ok(key),
            status => Err(map_ntstatus(status)),
        })
}

/// The algorithm and bit length that the blob magic selects. RSA blobs carry their bit length after the magic.
fn blob_kind(blob: &[u8]) -> Result<(AlgId, u32), Status> {
    let magic = read_u32(blob).ok_or(NTE_BAD_DATA)?;
    match magic {
        RSA_PUBLIC_MAGIC | RSA_PRIVATE_MAGIC | RSA_FULL_PRIVATE_MAGIC => {
            let bits = blob.get(4..).and_then(read_u32).ok_or(NTE_BAD_DATA)?;
            Ok((AlgId::Rsa, bits))
        }
        ECDSA_P256_PUBLIC_MAGIC | ECDSA_P256_PRIVATE_MAGIC => Ok((AlgId::EcdsaP256, 256)),
        ECDSA_P384_PUBLIC_MAGIC | ECDSA_P384_PRIVATE_MAGIC => Ok((AlgId::EcdsaP384, 384)),
        ECDSA_P521_PUBLIC_MAGIC | ECDSA_P521_PRIVATE_MAGIC => Ok((AlgId::EcdsaP521, 521)),
        _ => Err(NTE_INVALID_PARAMETER),
    }
}

fn read_u32(bytes: &[u8]) -> Option<u32> {
    match bytes {
        [a, b, c, d, ..] => Some(u32::from_le_bytes([*a, *b, *c, *d])),
        _ => None,
    }
}
