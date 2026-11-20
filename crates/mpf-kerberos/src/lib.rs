//! Port of the portable logic in Wine's `dlls/kerberos`.
//!
//! The functions below compute from values alone and are ported in behavior:
//!
//! - `unixlib.c`: `krb5_error_to_status` (`krb5_error_code` to NTSTATUS),
//!   `principal_to_name_and_realm` (splitting `name@realm` at the first `@`),
//!   `status_gss_to_sspi` (GSS-API major status to SECURITY_STATUS) and the context flag maps
//!   `flags_gss_to_asc_ret`, `flags_isc_req_to_gss` and `flags_gss_to_isc_ret`.
//! - `krb5_ap.c`: `get_buffer_index` (position of a SecBuffer type) and `expiry_to_timestamp`
//!   (the TimeStamp arithmetic; the caller supplies the local time that Wine gets from
//!   `NtQuerySystemTime` and `RtlSystemTimeToLocalTime`).
//!
//! Skipped: the krb5 and GSS-API calls, the ticket cache, the credential and context handling,
//! the unix code page and UTF-16 conversions, the LSA and SSPI entry points, the WoW64 thunks,
//! the GSS status tracing, and the handle conversions, which are plain casts.
//!
//! Principal names are byte slices holding the C string without its terminating NUL.

/// `STATUS_SUCCESS`.
pub const STATUS_SUCCESS: u32 = 0x0000_0000;
/// `STATUS_UNSUCCESSFUL`, returned by `krb5_error_to_status` for any nonzero error.
pub const STATUS_UNSUCCESSFUL: u32 = 0xC000_0001;

/// `SEC_E_OK`.
pub const SEC_E_OK: u32 = 0x0000_0000;
/// `SEC_E_INVALID_HANDLE`.
pub const SEC_E_INVALID_HANDLE: u32 = 0x8009_0301;
/// `SEC_E_INTERNAL_ERROR`, the fallback for GSS-API statuses without a mapping.
pub const SEC_E_INTERNAL_ERROR: u32 = 0x8009_0304;
/// `SEC_E_SECPKG_NOT_FOUND`.
pub const SEC_E_SECPKG_NOT_FOUND: u32 = 0x8009_0305;
/// `SEC_E_INVALID_TOKEN`.
pub const SEC_E_INVALID_TOKEN: u32 = 0x8009_0308;
/// `SEC_E_QOP_NOT_SUPPORTED`.
pub const SEC_E_QOP_NOT_SUPPORTED: u32 = 0x8009_030A;
/// `SEC_E_NO_CREDENTIALS`.
pub const SEC_E_NO_CREDENTIALS: u32 = 0x8009_030E;
/// `SEC_E_MESSAGE_ALTERED`.
pub const SEC_E_MESSAGE_ALTERED: u32 = 0x8009_030F;
/// `SEC_E_OUT_OF_SEQUENCE`.
pub const SEC_E_OUT_OF_SEQUENCE: u32 = 0x8009_0310;
/// `SEC_I_CONTINUE_NEEDED`.
pub const SEC_I_CONTINUE_NEEDED: u32 = 0x0009_0312;
/// `SEC_E_CONTEXT_EXPIRED`.
pub const SEC_E_CONTEXT_EXPIRED: u32 = 0x8009_0317;

/// `GSS_S_COMPLETE`.
const GSS_S_COMPLETE: u32 = 0;
/// `GSS_S_CONTINUE_NEEDED` (supplementary bit 0).
const GSS_S_CONTINUE_NEEDED: u32 = 1;
/// `GSS_S_DUPLICATE_TOKEN` (supplementary bit 1).
const GSS_S_DUPLICATE_TOKEN: u32 = 1 << 1;
/// `GSS_S_OLD_TOKEN` (supplementary bit 2).
const GSS_S_OLD_TOKEN: u32 = 1 << 2;
/// `GSS_S_UNSEQ_TOKEN` (supplementary bit 3).
const GSS_S_UNSEQ_TOKEN: u32 = 1 << 3;
/// `GSS_S_GAP_TOKEN` (supplementary bit 4).
const GSS_S_GAP_TOKEN: u32 = 1 << 4;
/// `GSS_S_BAD_MECH` (routine error 1).
const GSS_S_BAD_MECH: u32 = 1 << 16;
/// `GSS_S_BAD_SIG` (routine error 6).
const GSS_S_BAD_SIG: u32 = 6 << 16;
/// `GSS_S_NO_CRED` (routine error 7).
const GSS_S_NO_CRED: u32 = 7 << 16;
/// `GSS_S_NO_CONTEXT` (routine error 8).
const GSS_S_NO_CONTEXT: u32 = 8 << 16;
/// `GSS_S_DEFECTIVE_TOKEN` (routine error 9).
const GSS_S_DEFECTIVE_TOKEN: u32 = 9 << 16;
/// `GSS_S_DEFECTIVE_CREDENTIAL` (routine error 10).
const GSS_S_DEFECTIVE_CREDENTIAL: u32 = 10 << 16;
/// `GSS_S_CREDENTIALS_EXPIRED` (routine error 11).
const GSS_S_CREDENTIALS_EXPIRED: u32 = 11 << 16;
/// `GSS_S_CONTEXT_EXPIRED` (routine error 12).
const GSS_S_CONTEXT_EXPIRED: u32 = 12 << 16;
/// `GSS_S_FAILURE` (routine error 13).
const GSS_S_FAILURE: u32 = 13 << 16;
/// `GSS_S_BAD_QOP` (routine error 14).
const GSS_S_BAD_QOP: u32 = 14 << 16;

const GSS_C_DELEG_FLAG: u32 = 1;
const GSS_C_MUTUAL_FLAG: u32 = 2;
const GSS_C_REPLAY_FLAG: u32 = 4;
const GSS_C_SEQUENCE_FLAG: u32 = 8;
const GSS_C_CONF_FLAG: u32 = 16;
const GSS_C_INTEG_FLAG: u32 = 32;
const GSS_C_ANON_FLAG: u32 = 64;
const GSS_C_DCE_STYLE: u32 = 0x1000;
const GSS_C_IDENTIFY_FLAG: u32 = 0x2000;
const GSS_C_EXTENDED_ERROR_FLAG: u32 = 0x4000;

const ASC_RET_DELEGATE: u32 = 0x0000_0001;
const ASC_RET_MUTUAL_AUTH: u32 = 0x0000_0002;
const ASC_RET_REPLAY_DETECT: u32 = 0x0000_0004;
const ASC_RET_SEQUENCE_DETECT: u32 = 0x0000_0008;
const ASC_RET_CONFIDENTIALITY: u32 = 0x0000_0010;
const ASC_RET_USED_DCE_STYLE: u32 = 0x0000_0200;
const ASC_RET_EXTENDED_ERROR: u32 = 0x0000_8000;
const ASC_RET_INTEGRITY: u32 = 0x0002_0000;
const ASC_RET_IDENTIFY: u32 = 0x0008_0000;
const ASC_RET_NULL_SESSION: u32 = 0x0010_0000;

const ISC_REQ_DELEGATE: u32 = 0x0000_0001;
const ISC_REQ_MUTUAL_AUTH: u32 = 0x0000_0002;
const ISC_REQ_REPLAY_DETECT: u32 = 0x0000_0004;
const ISC_REQ_SEQUENCE_DETECT: u32 = 0x0000_0008;
const ISC_REQ_CONFIDENTIALITY: u32 = 0x0000_0010;
const ISC_REQ_USE_DCE_STYLE: u32 = 0x0000_0200;
const ISC_REQ_EXTENDED_ERROR: u32 = 0x0000_4000;
const ISC_REQ_INTEGRITY: u32 = 0x0001_0000;
const ISC_REQ_IDENTIFY: u32 = 0x0002_0000;
const ISC_REQ_NULL_SESSION: u32 = 0x0004_0000;

const ISC_RET_DELEGATE: u32 = 0x0000_0001;
const ISC_RET_MUTUAL_AUTH: u32 = 0x0000_0002;
const ISC_RET_REPLAY_DETECT: u32 = 0x0000_0004;
const ISC_RET_SEQUENCE_DETECT: u32 = 0x0000_0008;
const ISC_RET_CONFIDENTIALITY: u32 = 0x0000_0010;
const ISC_RET_USED_DCE_STYLE: u32 = 0x0000_0200;
const ISC_RET_EXTENDED_ERROR: u32 = 0x0000_4000;
const ISC_RET_INTEGRITY: u32 = 0x0001_0000;
const ISC_RET_IDENTIFY: u32 = 0x0002_0000;
const ISC_RET_NULL_SESSION: u32 = 0x0004_0000;

/// `GSS_C_*` to `ASC_RET_*`, in the order of `flags_gss_to_asc_ret`.
const GSS_TO_ASC_RET: &[(u32, u32)] = &[
    (GSS_C_DELEG_FLAG, ASC_RET_DELEGATE),
    (GSS_C_MUTUAL_FLAG, ASC_RET_MUTUAL_AUTH),
    (GSS_C_REPLAY_FLAG, ASC_RET_REPLAY_DETECT),
    (GSS_C_SEQUENCE_FLAG, ASC_RET_SEQUENCE_DETECT),
    (GSS_C_CONF_FLAG, ASC_RET_CONFIDENTIALITY),
    (GSS_C_INTEG_FLAG, ASC_RET_INTEGRITY),
    (GSS_C_ANON_FLAG, ASC_RET_NULL_SESSION),
    (GSS_C_DCE_STYLE, ASC_RET_USED_DCE_STYLE),
    (GSS_C_IDENTIFY_FLAG, ASC_RET_IDENTIFY),
    (GSS_C_EXTENDED_ERROR_FLAG, ASC_RET_EXTENDED_ERROR),
];

/// `ISC_REQ_*` to `GSS_C_*`, in the order of `flags_isc_req_to_gss`.
const ISC_REQ_TO_GSS: &[(u32, u32)] = &[
    (ISC_REQ_DELEGATE, GSS_C_DELEG_FLAG),
    (ISC_REQ_MUTUAL_AUTH, GSS_C_MUTUAL_FLAG),
    (ISC_REQ_REPLAY_DETECT, GSS_C_REPLAY_FLAG),
    (ISC_REQ_SEQUENCE_DETECT, GSS_C_SEQUENCE_FLAG),
    (ISC_REQ_CONFIDENTIALITY, GSS_C_CONF_FLAG),
    (ISC_REQ_INTEGRITY, GSS_C_INTEG_FLAG),
    (ISC_REQ_NULL_SESSION, GSS_C_ANON_FLAG),
    (ISC_REQ_USE_DCE_STYLE, GSS_C_DCE_STYLE),
    (ISC_REQ_IDENTIFY, GSS_C_IDENTIFY_FLAG),
    (ISC_REQ_EXTENDED_ERROR, GSS_C_EXTENDED_ERROR_FLAG),
];

/// `GSS_C_*` to `ISC_RET_*`, in the order of `flags_gss_to_isc_ret`.
const GSS_TO_ISC_RET: &[(u32, u32)] = &[
    (GSS_C_DELEG_FLAG, ISC_RET_DELEGATE),
    (GSS_C_MUTUAL_FLAG, ISC_RET_MUTUAL_AUTH),
    (GSS_C_REPLAY_FLAG, ISC_RET_REPLAY_DETECT),
    (GSS_C_SEQUENCE_FLAG, ISC_RET_SEQUENCE_DETECT),
    (GSS_C_CONF_FLAG, ISC_RET_CONFIDENTIALITY),
    (GSS_C_INTEG_FLAG, ISC_RET_INTEGRITY),
    (GSS_C_ANON_FLAG, ISC_RET_NULL_SESSION),
    (GSS_C_DCE_STYLE, ISC_RET_USED_DCE_STYLE),
    (GSS_C_IDENTIFY_FLAG, ISC_RET_IDENTIFY),
    (GSS_C_EXTENDED_ERROR_FLAG, ISC_RET_EXTENDED_ERROR),
];

/// The NTSTATUS of a krb5 error code: success for 0, `STATUS_UNSUCCESSFUL` otherwise.
pub fn krb5_error_to_status(err: i32) -> u32 {
    match err {
        0 => STATUS_SUCCESS,
        _ => STATUS_UNSUCCESSFUL,
    }
}

/// Splits `name@realm` at the first `@`. Without an `@`, the whole input is the name and the
/// realm is empty.
pub fn principal_to_name_and_realm(name_with_realm: &[u8]) -> (&[u8], &[u8]) {
    match name_with_realm.iter().position(|&c| c == b'@') {
        Some(at) => (&name_with_realm[..at], &name_with_realm[at + 1..]),
        None => (name_with_realm, &name_with_realm[name_with_realm.len()..]),
    }
}

/// The SECURITY_STATUS of a GSS-API major status. Statuses without a mapping give
/// `SEC_E_INTERNAL_ERROR`.
pub fn status_gss_to_sspi(status: u32) -> u32 {
    match status {
        GSS_S_COMPLETE => SEC_E_OK,
        GSS_S_BAD_MECH => SEC_E_SECPKG_NOT_FOUND,
        GSS_S_BAD_SIG => SEC_E_MESSAGE_ALTERED,
        GSS_S_NO_CRED => SEC_E_NO_CREDENTIALS,
        GSS_S_NO_CONTEXT => SEC_E_INVALID_HANDLE,
        GSS_S_DEFECTIVE_TOKEN => SEC_E_INVALID_TOKEN,
        GSS_S_DEFECTIVE_CREDENTIAL => SEC_E_NO_CREDENTIALS,
        GSS_S_CREDENTIALS_EXPIRED => SEC_E_CONTEXT_EXPIRED,
        GSS_S_CONTEXT_EXPIRED => SEC_E_CONTEXT_EXPIRED,
        GSS_S_BAD_QOP => SEC_E_QOP_NOT_SUPPORTED,
        GSS_S_CONTINUE_NEEDED => SEC_I_CONTINUE_NEEDED,
        GSS_S_DUPLICATE_TOKEN => SEC_E_INVALID_TOKEN,
        GSS_S_OLD_TOKEN => SEC_E_INVALID_TOKEN,
        GSS_S_UNSEQ_TOKEN => SEC_E_OUT_OF_SEQUENCE,
        GSS_S_GAP_TOKEN => SEC_E_OUT_OF_SEQUENCE,
        GSS_S_FAILURE => SEC_E_INTERNAL_ERROR,
        _ => SEC_E_INTERNAL_ERROR,
    }
}

fn map_flags(table: &[(u32, u32)], flags: u32) -> u32 {
    table
        .iter()
        .filter(|(from, _)| flags & from != 0)
        .fold(0, |ret, (_, to)| ret | to)
}

/// `GSS_C_*` context flags to `ASC_RET_*` accept flags. Bits without a mapping are dropped.
pub fn flags_gss_to_asc_ret(flags: u32) -> u32 {
    map_flags(GSS_TO_ASC_RET, flags)
}

/// `ISC_REQ_*` request flags to `GSS_C_*` context flags. Bits without a mapping are dropped.
pub fn flags_isc_req_to_gss(flags: u32) -> u32 {
    map_flags(ISC_REQ_TO_GSS, flags)
}

/// `GSS_C_*` context flags to `ISC_RET_*` return flags. Bits without a mapping are dropped.
pub fn flags_gss_to_isc_ret(flags: u32) -> u32 {
    map_flags(GSS_TO_ISC_RET, flags)
}

/// The position of the first buffer with `buffer_type`, or `None` when there is no descriptor
/// (`buffer_types` is `None`) or no such buffer. `buffer_types` lists the `BufferType` of each
/// buffer in order.
pub fn get_buffer_index(buffer_types: Option<&[u32]>, buffer_type: u32) -> Option<usize> {
    buffer_types?.iter().position(|&t| t == buffer_type)
}

/// The `TimeStamp` (low and high 32-bit halves) of `local_time` plus `expiry` seconds.
/// `local_time` counts 100 ns ticks and is the already converted local time.
pub fn expiry_to_timestamp(local_time: i64, expiry: u32) -> (u32, i32) {
    let time = (local_time as u64).wrapping_add(u64::from(expiry) * 10_000_000) as i64;
    (time as u32, (time >> 32) as i32)
}

#[cfg(test)]
pub mod edge_checks;
#[cfg(test)]
pub mod oracle_checks;
