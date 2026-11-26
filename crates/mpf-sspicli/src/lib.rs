//! Port of the portable logic in Wine's `dlls/sspicli/main.c`.
//!
//! Ported:
//! - `SspiEncodeStringsAsAuthIdentity`: [`encode_strings_as_auth_identity`]
//! - `SspiMarshalAuthIdentity`: [`marshal_auth_identity`]
//! - `SspiUnmarshalAuthIdentity`: [`unmarshal_auth_identity`]
//! - Name and blob composition in `SspiPrepareForCredWrite`: [`prepare_for_cred_write`]
//!
//! Skipped:
//! - `SspiEncodeAuthIdentityAsStrings` and `dup_auth_str`: LocalAlloc copies and the
//!   `CP_ACP` conversion of ANSI input.
//! - `SspiZeroAuthIdentity`, `SspiFreeAuthIdentity`, `SspiLocalFree`: they only manage caller memory.
//! - Allocation and `SecureZeroMemory` cleanup in `SspiPrepareForCredWrite`.
//! - Version dispatch on the raw first ULONG: replaced by the [`AuthIdentity`] variants.
//! - Allocation-failure returns (`ERROR_OUTOFMEMORY`, `SEC_E_INSUFFICIENT_MEMORY`).
//! - `secur32` forwarders in `sspicli.spec`.
//!
//! Byte layouts are little-endian, as on the x86 hosts Wine targets.

mod cred;
mod identity;
mod marshal;
mod unmarshal;

pub use cred::{prepare_for_cred_write, CredWrite, CRED_TYPE_DOMAIN_PASSWORD};
pub use identity::{
    encode_strings_as_auth_identity, AuthIdentity, Credentials, AUTH_IDENTITY_ANSI,
    AUTH_IDENTITY_MARSHALLED, AUTH_IDENTITY_UNICODE, AUTH_IDENTITY_VERSION,
    AUTH_IDENTITY_VERSION_2, SEC_E_INTERNAL_ERROR, SEC_E_INVALID_TOKEN,
};
pub use marshal::marshal_auth_identity;
pub use unmarshal::unmarshal_auth_identity;

/// Cuts a wide string at its first NUL, as `wcslen`-based C code does.
pub(crate) fn nul_terminated(s: &[u16]) -> &[u16] {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..len]
}

/// UTF-16LE bytes of `s`, as stored in marshalled buffers.
pub(crate) fn wide_bytes(s: &[u16]) -> Vec<u8> {
    s.iter().flat_map(|c| c.to_le_bytes()).collect()
}

/// Reads a little-endian ULONG at `off`. Callers check the length first.
pub(crate) fn read_u32(bytes: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]])
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
