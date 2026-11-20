//! Headless port of the deterministic NTLM code in Wine's `dlls/msv1_0`: the NTLM1 and NTLM2
//! session keys, the RC4, MD4, MD5, HMAC-MD5 and CRC-32 helpers they use, base64, and the
//! NTLMSSP signature, seal and unseal functions of `main.c`.
//!
//! Ported:
//! - `create_ntlm1_session_key` and `create_ntlm2_subkeys`: [keys::ntlm1_session_key] and
//!   [keys::ntlm2_subkeys].
//! - `encode_base64` and `decode_base64`: [base64::encode] and [base64::decode].
//! - `arc4_init`, `arc4_process`, `MD4Init`/`MD4Update`/`MD4Final`, `MD5Init`/`MD5Update`/
//!   `MD5Final`, `hmac_md5_*` and `RtlComputeCrc32`, as used by the signature functions.
//! - `create_signature` (the extended, NTLM1 and dummy signatures), `verify_signature`,
//!   `ntlm_SpMakeSignature`, `ntlm_SpVerifySignature`, `ntlm_SpSealMessage` and
//!   `ntlm_SpUnsealMessage`: [session::Session] and [message::Buffer].
//!
//! Skipped:
//! - The entry points that manage credentials, contexts and handles, such as
//!   `ntlm_SpAcquireCredentialsHandle`, `ntlm_SpInitLsaModeContext`,
//!   `ntlm_SpAcceptLsaModeContext`, `ntlm_SpQueryContextAttributes` and
//!   `ntlm_SpInitUserModeContext`, together with the handle lists in `main.c`.
//! - The local authentication and helper process code (`local_auth_*`, `ntlm_fork`,
//!   `ntlm_chat`, `ntlm_pool_*`), the cached credential lookup, and `unixlib.c`. They need
//!   processes, the registry or the Unix side.
//! - The package information tables (`ntlm_SpGetInfo`, `build_package_info`).
//!
//! Deviations from Wine, where Wine has undefined behaviour or depends on a C pointer:
//! - Base64 decoding reads past the end of the input in Wine when the length is zero or not a
//!   multiple of four. The port returns no bytes for empty input and `None` for the other lengths.
//! - The C code writes the token length back as 16. A slice cannot shrink, so the caller keeps
//!   its length and the first 16 bytes hold the signature.
//! - `unseal` with a STREAM buffer keeps only the 16-byte signature in the stream. The rest
//!   becomes the first DATA buffer, as in Wine.
//! - Modes other than `Client` use the server layout, as the C code does.
//! - An RC4 key must not be empty. Wine reads past the end of the key in that case.
//! - There is no handle table, so `SEC_E_INVALID_HANDLE` is never returned.
//! - No sanitizer-flagged oracle vectors were excluded.

pub mod base64;
mod crc32;
pub mod keys;
mod md4;
mod md5;
pub mod message;
mod rc4;
pub mod session;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The `NTLMSSP_NEGOTIATE_*` flags of `main.c` that the functions here test.
pub const NEGOTIATE_SIGN: u32 = 0x0000_0010;
pub const NEGOTIATE_SEAL: u32 = 0x0000_0020;
pub const NEGOTIATE_ALWAYS_SIGN: u32 = 0x0000_8000;
pub const NEGOTIATE_EXTENDED_SESSIONSECURITY: u32 = 0x0008_0000;
pub const NEGOTIATE_KEY_EXCH: u32 = 0x4000_0000;

/// The `SECBUFFER_*` types of `sspi.h` that the message functions use.
pub const SECBUFFER_DATA: u32 = 1;
pub const SECBUFFER_TOKEN: u32 = 2;
pub const SECBUFFER_PADDING: u32 = 9;
pub const SECBUFFER_STREAM: u32 = 10;

/// The failures of the message functions, with the `SECURITY_STATUS` code Wine returns for each.
pub enum Error {
    InvalidToken,
    BufferTooSmall,
    UnsupportedFunction,
    MessageAltered,
}

impl Error {
    /// The `SECURITY_STATUS` value of the error, such as `SEC_E_INVALID_TOKEN`.
    pub fn code(self) -> u32 {
        match self {
            Error::InvalidToken => 0x8009_0308,
            Error::BufferTooSmall => 0x8009_0321,
            Error::UnsupportedFunction => 0x8009_0302,
            Error::MessageAltered => 0x8009_030f,
        }
    }
}
