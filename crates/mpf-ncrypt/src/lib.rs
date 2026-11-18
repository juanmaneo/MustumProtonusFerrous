//! Headless subset of Wine's dlls/ncrypt/main.c (NCrypt key and provider glue).
//!
//! Ported, checked against the verbatim C source by oracle_checks.rs and edge_checks.rs:
//! - [create_persisted_key]: the provider and name checks of NCryptCreatePersistedKey, the persisted name table and the key template from create_key_object.
//! - [import_key]: the decrypt, flag, blob magic and bit length checks of NCryptImportKey.
//! - [encrypt], [decrypt], [sign_hash], [verify_signature] and [export_key]: the argument checks of NCryptEncrypt, NCryptDecrypt, NCryptSignHash, NCryptVerifySignature and NCryptExportKey.
//! - [is_alg_supported]: the NCryptIsAlgSupported checks and the case-insensitive name match.
//! - [get_property] and [set_property]: the NCryptGetProperty and NCryptSetProperty rules over a [PropertyStore].
//! - [map_ntstatus]: the NTSTATUS to winerror mapping.
//!
//! Skipped: NCryptFinalizeKey, NCryptDeleteKey, NCryptEnumKeys, NCryptOpenKey, NCryptOpenStorageProvider and NCryptIsKeyHandle (key-pair state, handle tables, registry or files); NCryptFreeBuffer and NCryptFreeObject (allocation and reference counts); NCryptEnumAlgorithms and the other FIXME stubs.
//!
//! The BCrypt calls are not ported. The caller supplies their NTSTATUS result through a closure.
//!
//! Deviations from Wine:
//! - Blobs shorter than the magic or the RSA bit length are rejected with [NTE_BAD_DATA]. Wine reads past the end.
//! - A NULL handle, a provider handle where a key is needed, or a key handle where a provider is needed is rejected with [NTE_INVALID_HANDLE]. Wine crashes or accepts these. A NULL provider in NCryptImportKey is rejected the same way.
//! - Key objects do not carry the Provider Handle property. Setting it is refused with [NTE_NOT_SUPPORTED], and reading it reports [NTE_INVALID_PARAMETER].
//! - NULL property names and allocation failures are not modelled.
//! - Names are folded for ASCII letters only. Wine uses the Unicode-aware lstrcmpiW.
//! - On a BCrypt failure Wine frees the key without releasing its property and provider references. Here the key is simply dropped.
//! - The Length and PublicKeyLength properties are written by [Key::new], where Wine writes them after BCrypt succeeds. The caller sees the same result.

mod algid;
mod calls;
mod import;
mod key;
mod property;
mod wide;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use algid::{from_persisted_name, AlgId};
pub use calls::{
    create_persisted_key, decrypt, encrypt, export_key, is_alg_supported, sign_hash,
    verify_signature, Buffer, Handle, SUPPORTED_OPERATIONS,
};
pub use import::import_key;
pub use key::Key;
pub use property::{
    get_property, set_property, PropertyRead, PropertyStore, ALGORITHM_GROUP_PROPERTY,
    ALGORITHM_PROPERTY, BLOCK_LENGTH_PROPERTY, EXPORT_POLICY_PROPERTY, KEY_TYPE_PROPERTY,
    KEY_USAGE_PROPERTY, LENGTHS_PROPERTY, LENGTH_PROPERTY, PROVIDER_HANDLE_PROPERTY,
    PUBLIC_KEY_LENGTH_PROPERTY, SIGNATURE_LENGTH_PROPERTY,
};
pub use wide::{fold_eq, utf16, wide_bytes, wide_eq, wide_str};

/// A winerror value: the return type of the NCrypt entry points (SECURITY_STATUS).
pub type Status = u32;
/// An NTSTATUS value: the return type of the BCrypt calls that the caller supplies.
pub type NtStatus = u32;

pub const ERROR_SUCCESS: Status = 0;
pub const NTE_BAD_DATA: Status = 0x8009_0005;
pub const NTE_BAD_SIGNATURE: Status = 0x8009_0006;
pub const NTE_BAD_FLAGS: Status = 0x8009_0009;
pub const NTE_NO_MEMORY: Status = 0x8009_000E;
pub const NTE_INVALID_HANDLE: Status = 0x8009_0026;
pub const NTE_INVALID_PARAMETER: Status = 0x8009_0027;
pub const NTE_BUFFER_TOO_SMALL: Status = 0x8009_0028;
pub const NTE_NOT_SUPPORTED: Status = 0x8009_0029;
pub const NTE_INTERNAL_ERROR: Status = 0x8009_002D;
/// HRESULT_FROM_WIN32(RPC_X_NULL_REF_POINTER): a required pointer is NULL.
pub const NULL_REF_POINTER: Status = 0x8007_06F4;

pub const STATUS_SUCCESS: NtStatus = 0x0000_0000;
pub const STATUS_UNSUCCESSFUL: NtStatus = 0xC000_0001;
pub const STATUS_INVALID_HANDLE: NtStatus = 0xC000_0008;
pub const STATUS_INVALID_PARAMETER: NtStatus = 0xC000_000D;
pub const STATUS_NO_MEMORY: NtStatus = 0xC000_0017;
pub const STATUS_BUFFER_TOO_SMALL: NtStatus = 0xC000_0023;
pub const STATUS_NOT_SUPPORTED: NtStatus = 0xC000_00BB;
pub const STATUS_INVALID_SIGNATURE: NtStatus = 0xC000_A000;

pub const NCRYPT_NO_PADDING_FLAG: u32 = 0x0000_0001;
pub const NCRYPT_PAD_PKCS1_FLAG: u32 = 0x0000_0002;
pub const NCRYPT_PAD_OAEP_FLAG: u32 = 0x0000_0004;
pub const NCRYPT_SILENT_FLAG: u32 = 0x0000_0040;
pub const NCRYPT_ALLOW_ALL_USAGES: u32 = 0x00ff_ffff;

/// The winerror that NCrypt reports for an NTSTATUS returned by BCrypt.
pub fn map_ntstatus(status: NtStatus) -> Status {
    match status {
        STATUS_INVALID_HANDLE => NTE_INVALID_HANDLE,
        STATUS_INVALID_SIGNATURE => NTE_BAD_SIGNATURE,
        STATUS_SUCCESS => ERROR_SUCCESS,
        STATUS_INVALID_PARAMETER => NTE_INVALID_PARAMETER,
        STATUS_NO_MEMORY => NTE_NO_MEMORY,
        STATUS_NOT_SUPPORTED => NTE_NOT_SUPPORTED,
        NTE_BAD_DATA => NTE_BAD_DATA,
        STATUS_BUFFER_TOO_SMALL => NTE_BUFFER_TOO_SMALL,
        _ => NTE_INTERNAL_ERROR,
    }
}
