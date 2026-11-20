//! Port of the deterministic logic in Wine's `dlls/bcrypt/bcrypt_main.c`,
//! the Cryptography Next Generation front end that `bcrypt.dll` implements.
//!
//! Ported:
//! - [`algorithm`]: the built-in algorithm table, `BCryptEnumAlgorithms`
//!   filtering, `BCryptOpenAlgorithmProvider` argument checks and
//!   `is_symmetric_alg` ([`algorithm::AlgId::is_symmetric`]).
//! - [`property`]: `BCryptGetProperty` and `BCryptSetProperty` for those
//!   algorithms (`get_alg_property`, `set_alg_property`).
//! - [`key`]: the AES key length check and the derived key copy
//!   (`validate_len_aes`, `derive_key_raw`).
//! - [`blob`]: the ECC magic numbers, the curve mapping of an ECC key blob and
//!   the blob type resolution (`ecc_magic`, `ecc_blob_curve`,
//!   `resolve_blob_type`).
//!
//! Skipped: hashing, cipher and RNG operations; RSA, DSA, DH and ECC key
//! generation, import and export; signing and verification; the SymCrypt-backed
//! key derivation; pseudo-handles and the secret object; RSA header parsing;
//! and the handle out-parameter of the open call, which here returns an
//! [`algorithm::Algorithm`] value.
//!
//! Deviations:
//! - A NULL buffer for `ChainingMode` is answered with the size and nothing is
//!   written. Wine's `memcpy` into the NULL buffer crashes.
//! - `CHAIN_MODE_CCM` is not modelled. No call accepts it, because AES
//!   `ChainingMode` sets only recognise ECB, CBC, CFB and GCM.
//! - Wide strings end at their first NUL or at the end of the slice, so a
//!   string never reads past what it was given.
//! - [`key::derive_key_raw`] takes the output length from the output slice,
//!   where Wine takes a separate `out_len` argument.
//! - [`property::set_alg_property`] drops Wine's size and flags arguments,
//!   which Wine never reads for these properties.
//!
//! Quirks that are kept on purpose, so that the port matches Wine's output:
//! - `ChainingMode` reports twice the byte length of its NUL-terminated name.
//! - DWORD properties check the buffer size even when the buffer is NULL, but
//!   the key length structure only checks the size when a buffer is given.
//! - The AES key length increment check uses unsigned arithmetic, so a minimum
//!   that is not a multiple of 8 wraps.

pub mod algorithm;
pub mod blob;
pub mod key;
pub mod property;
mod wide;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The result of a bcrypt call, with the NTSTATUS code that Wine reports for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// `STATUS_SUCCESS`.
    Success,
    /// `STATUS_NOT_IMPLEMENTED`.
    NotImplemented,
    /// `STATUS_INVALID_PARAMETER`.
    InvalidParameter,
    /// `STATUS_BUFFER_TOO_SMALL`.
    BufferTooSmall,
    /// `STATUS_NOT_SUPPORTED`.
    NotSupported,
}

impl Status {
    /// The NTSTATUS value for this status.
    pub fn code(self) -> u32 {
        match self {
            Status::Success => 0x0000_0000,
            Status::NotImplemented => 0xC000_0002,
            Status::InvalidParameter => 0xC000_000D,
            Status::BufferTooSmall => 0xC000_0023,
            Status::NotSupported => 0xC000_00BB,
        }
    }
}

/// A status and the byte count that Wine writes to `*ret_size`.
pub type Reply = (Status, u32);
