//! Deterministic logic from Wine's `dlls/schannel`.
//!
//! The ported functions are the package-info and initialization entry points of
//! `usermode.c` and `lsamode.c`. They are pure functions over values and structs:
//! argument checks, constant assignments and bitwise flag composition.
//!
//! Ported:
//! - [`sp_user_mode_initialize`]: the `LsaVersion` argument check and the
//!   package-version / table-count replies ([`Status`]).
//! - [`sp_lsa_mode_initialize`]: the fixed package version and table count.
//! - [`sp_get_info_unified`] and [`sp_get_info_schannel`]: the `SecPkgInfoW`
//!   struct fill ([`PackageInfo`]).
//!
//! Skipped:
//! - `SslEmptyCacheA` and `SslEmptyCacheW` from `schannel_main.c`: FIXME stubs
//!   that only return `TRUE`.
//! - The function-pointer tables themselves: they are all `NULL` in Wine, so
//!   there is no logic to port.
//! - All sealing, signature, credential and context operations: they need
//!   handles, OS state, registry, files, network or COM.

/// NTSTATUS codes returned by Schannel's initialization and info functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// `STATUS_SUCCESS`.
    Success,
    /// `STATUS_INVALID_PARAMETER`.
    InvalidParameter,
}

impl Status {
    /// The NTSTATUS value for this status.
    pub fn code(self) -> u32 {
        match self {
            Status::Success => 0x0000_0000,
            Status::InvalidParameter => 0xC000_000D,
        }
    }
}

/// The package info filled by `SpGetInfoUnified` and `SpGetInfoSChannel`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    /// Capability flags bitmask.
    pub f_capabilities: u32,
    /// Package version.
    pub w_version: u16,
    /// RPC identifier.
    pub w_rpc_id: u16,
    /// Maximum token size in bytes.
    pub cb_max_token: u32,
    /// Package name.
    pub name: &'static str,
    /// Package comment.
    pub comment: &'static str,
}

/// The SECPKG interface version checked by [`sp_user_mode_initialize`].
pub const SECPKG_INTERFACE_VERSION: u32 = 0x10000;
/// The SECPKG interface version reported by [`sp_lsa_mode_initialize`].
pub const SECPKG_INTERFACE_VERSION_8: u32 = 0x800000;

/// Capability flags reported by both info functions.
pub const SECPKG_FLAG_MUTUAL_AUTH: u32 = 0x0001_0000;
pub const SECPKG_FLAG_INTEGRITY: u32 = 0x0000_0001;
pub const SECPKG_FLAG_PRIVACY: u32 = 0x0000_0002;
pub const SECPKG_FLAG_CONNECTION: u32 = 0x0000_0010;
pub const SECPKG_FLAG_MULTI_REQUIRED: u32 = 0x0000_0020;
pub const SECPKG_FLAG_EXTENDED_ERROR: u32 = 0x0000_0080;
pub const SECPKG_FLAG_IMPERSONATION: u32 = 0x0000_0100;
pub const SECPKG_FLAG_ACCEPT_WIN32_NAME: u32 = 0x0000_0200;
pub const SECPKG_FLAG_STREAM: u32 = 0x0000_0400;

/// The combined capability mask reported by [`sp_get_info_unified`] and
/// [`sp_get_info_schannel`].
pub const SCHANNEL_CAPABILITIES: u32 = SECPKG_FLAG_MUTUAL_AUTH
    | SECPKG_FLAG_INTEGRITY
    | SECPKG_FLAG_PRIVACY
    | SECPKG_FLAG_CONNECTION
    | SECPKG_FLAG_MULTI_REQUIRED
    | SECPKG_FLAG_EXTENDED_ERROR
    | SECPKG_FLAG_IMPERSONATION
    | SECPKG_FLAG_ACCEPT_WIN32_NAME
    | SECPKG_FLAG_STREAM;

/// The RPC identifier reported by both info functions.
pub const UNISP_RPC_ID: u16 = 14;
/// The maximum token size reported by both info functions.
pub const SCHANNEL_CB_MAX_TOKEN: u32 = 0x4000;
/// The package version reported by both info functions.
pub const SCHANNEL_W_VERSION: u16 = 1;

/// The number of function tables reported by both initialization functions.
pub const SCHANNEL_TABLE_COUNT: u32 = 2;

/// `SpUserModeInitialize` from Wine's `usermode.c`.
///
/// Checks `lsa_version` against [`SECPKG_INTERFACE_VERSION`]. On mismatch
/// returns [`Status::InvalidParameter`] with zero outputs. On match reports
/// [`SECPKG_INTERFACE_VERSION`] as the package version and
/// [`SCHANNEL_TABLE_COUNT`] as the table count.
pub fn sp_user_mode_initialize(lsa_version: u32) -> (Status, u32, u32) {
    if lsa_version != SECPKG_INTERFACE_VERSION {
        return (Status::InvalidParameter, 0, 0);
    }
    (
        Status::Success,
        SECPKG_INTERFACE_VERSION,
        SCHANNEL_TABLE_COUNT,
    )
}

/// `SpLsaModeInitialize` from Wine's `lsamode.c`.
///
/// Always succeeds, reporting [`SECPKG_INTERFACE_VERSION_8`] as the package
/// version and [`SCHANNEL_TABLE_COUNT`] as the table count.
pub fn sp_lsa_mode_initialize() -> (Status, u32, u32) {
    (
        Status::Success,
        SECPKG_INTERFACE_VERSION_8,
        SCHANNEL_TABLE_COUNT,
    )
}

/// `SpGetInfoUnified` from Wine's `lsamode.c`.
///
/// Fills a [`PackageInfo`] for the Microsoft Unified Security Protocol Provider.
pub fn sp_get_info_unified() -> (Status, PackageInfo) {
    (
        Status::Success,
        PackageInfo {
            f_capabilities: SCHANNEL_CAPABILITIES,
            w_version: SCHANNEL_W_VERSION,
            w_rpc_id: UNISP_RPC_ID,
            cb_max_token: SCHANNEL_CB_MAX_TOKEN,
            name: "Microsoft Unified Security Protocol Provider",
            comment: "Microsoft Unified Security Protocol Provider",
        },
    )
}

/// `SpGetInfoSChannel` from Wine's `lsamode.c`.
///
/// Fills a [`PackageInfo`] for Schannel.
pub fn sp_get_info_schannel() -> (Status, PackageInfo) {
    (
        Status::Success,
        PackageInfo {
            f_capabilities: SCHANNEL_CAPABILITIES,
            w_version: SCHANNEL_W_VERSION,
            w_rpc_id: UNISP_RPC_ID,
            cb_max_token: SCHANNEL_CB_MAX_TOKEN,
            name: "Schannel",
            comment: "Schannel Security Package",
        },
    )
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
