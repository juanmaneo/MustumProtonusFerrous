//! Argument checks of the NCrypt key entry points. A key handle is modelled by its algorithm, and the BCrypt calls run in the caller.

use crate::algid::{from_persisted_name, AlgId};
use crate::key::Key;
use crate::wide::{fold_eq, utf16};
use crate::{
    map_ntstatus, NtStatus, Status, NCRYPT_NO_PADDING_FLAG, NCRYPT_PAD_OAEP_FLAG,
    NCRYPT_PAD_PKCS1_FLAG, NCRYPT_SILENT_FLAG, NTE_BAD_FLAGS, NTE_INVALID_HANDLE,
    NTE_INVALID_PARAMETER, NTE_NOT_SUPPORTED, NTE_NO_MEMORY, NULL_REF_POINTER, STATUS_SUCCESS,
};

const CIPHER_OPERATION: u32 = 0x0000_0001;
const ASYMMETRIC_ENCRYPTION_OPERATION: u32 = 0x0000_0004;
const SECRET_AGREEMENT_OPERATION: u32 = 0x0000_0008;
const SIGNATURE_OPERATION: u32 = 0x0000_0010;

/// The BCrypt operation mask that NCryptIsAlgSupported enumerates.
pub const SUPPORTED_OPERATIONS: u32 = CIPHER_OPERATION
    | ASYMMETRIC_ENCRYPTION_OPERATION
    | SECRET_AGREEMENT_OPERATION
    | SIGNATURE_OPERATION;

const ENCRYPT_FLAGS: u32 =
    NCRYPT_NO_PADDING_FLAG | NCRYPT_PAD_PKCS1_FLAG | NCRYPT_PAD_OAEP_FLAG | NCRYPT_SILENT_FLAG;

const RSA_SIGN_ALGORITHM: &str = "RSA_SIGN";

/// The handle that an NCrypt call receives: a provider, or a key of the given algorithm.
pub enum Handle {
    Provider,
    Key(AlgId),
}

/// A caller buffer as the argument checks see it: whether its pointer is NULL, and its size.
pub struct Buffer {
    pub is_null: bool,
    pub size: u32,
}

pub(crate) fn require_provider(provider: Option<Handle>) -> Result<(), Status> {
    match provider {
        Some(Handle::Provider) => Ok(()),
        _ => Err(NTE_INVALID_HANDLE),
    }
}

fn key_of(handle: Option<Handle>) -> Result<AlgId, Status> {
    match handle {
        Some(Handle::Key(algid)) => Ok(algid),
        _ => Err(NTE_INVALID_HANDLE),
    }
}

pub(crate) fn accept_flags(flags: u32) -> Result<(), Status> {
    if (flags & !NCRYPT_SILENT_FLAG) == 0 {
        Ok(())
    } else {
        Err(NTE_BAD_FLAGS)
    }
}

/// NCryptCreatePersistedKey: the provider, name and algorithm checks, then the key template. The generation runs in the caller.
pub fn create_persisted_key(
    provider: Option<Handle>,
    algid: Option<&[u16]>,
    generate: &mut dyn FnMut(AlgId, u32) -> NtStatus,
) -> Result<Key, Status> {
    require_provider(provider)?;
    let name = algid.ok_or(NULL_REF_POINTER)?;
    let (alg, bits) = from_persisted_name(name).ok_or(NTE_NOT_SUPPORTED)?;
    Key::new(alg, bits)
        .ok_or(NTE_NO_MEMORY)
        .and_then(|key| match generate(alg, bits) {
            STATUS_SUCCESS => Ok(key),
            status => Err(map_ntstatus(status)),
        })
}

/// NCryptEncrypt: the flag mask, then the key handle. The encryption runs in the caller.
pub fn encrypt(handle: Option<Handle>, flags: u32) -> Result<(), Status> {
    if (flags & !ENCRYPT_FLAGS) != 0 {
        return Err(NTE_BAD_FLAGS);
    }
    key_of(handle)?;
    Ok(())
}

/// NCryptDecrypt: the key handle, of any algorithm. The decryption runs in the caller.
pub fn decrypt(handle: Option<Handle>) -> Result<(), Status> {
    key_of(handle)?;
    Ok(())
}

/// NCryptSignHash: the key handle, then the flags that reach BCrypt, which lack the silent flag.
pub fn sign_hash(handle: Option<Handle>, flags: u32) -> Result<u32, Status> {
    key_of(handle)?;
    Ok(flags & !NCRYPT_SILENT_FLAG)
}

/// NCryptVerifySignature: the buffer sizes, then the NULL pointers, then the key handle and its symmetry.
pub fn verify_signature(
    handle: Option<Handle>,
    hash: Buffer,
    signature: Buffer,
) -> Result<(), Status> {
    if hash.size == 0 || signature.size == 0 {
        return Err(NTE_INVALID_PARAMETER);
    }
    if hash.is_null || signature.is_null {
        return Err(NULL_REF_POINTER);
    }
    if key_of(handle)?.is_symmetric() {
        return Err(NTE_NOT_SUPPORTED);
    }
    Ok(())
}

/// NCryptExportKey: the export options are refused, then the key handle is checked.
pub fn export_key(
    handle: Option<Handle>,
    encrypt_key_set: bool,
    params_set: bool,
) -> Result<(), Status> {
    if encrypt_key_set || params_set {
        return Err(NTE_NOT_SUPPORTED);
    }
    key_of(handle)?;
    Ok(())
}

/// NCryptIsAlgSupported: the provider, name and flag checks, then a case-insensitive match against the names that BCrypt enumerates.
/// The enumeration runs in the caller and returns the names or an NTSTATUS.
pub fn is_alg_supported(
    provider: Option<Handle>,
    algid: Option<&[u16]>,
    flags: u32,
    enumerate: &mut dyn FnMut(u32) -> Result<Vec<Vec<u16>>, NtStatus>,
) -> Result<(), Status> {
    require_provider(provider)?;
    let name = algid.ok_or(NULL_REF_POINTER)?;
    accept_flags(flags)?;
    if fold_eq(name, &utf16(RSA_SIGN_ALGORITHM)) {
        return Err(NTE_NOT_SUPPORTED);
    }
    let names = enumerate(SUPPORTED_OPERATIONS).map_err(map_ntstatus)?;
    if names.iter().any(|candidate| fold_eq(candidate, name)) {
        Ok(())
    } else {
        Err(NTE_NOT_SUPPORTED)
    }
}
