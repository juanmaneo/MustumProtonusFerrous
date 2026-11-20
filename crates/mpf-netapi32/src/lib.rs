//! Port of the deterministic logic in Wine's `dlls/netapi32`: the NetBIOS status mapping from
//! `netapi32.c`, the name encoding and query packet from `nbt.c`, the DAV path conversion and
//! server name check from `netapi32.c`, the adapter address formatting from `netapi32.c`, and the
//! Samba ACL mapping from `unixlib.c`.
//!
//! Ported:
//! - `NetpNetBiosStatusToApiStatus`: [`netbios_status_to_api_status`].
//! - `NetBTNameEncode` and `NetBTNameReq`: [`nbt::encode_name`] and [`nbt::name_request`].
//! - `DavGetHTTPFromUNCPath` and `DavGetUNCFromHTTPPath`: [`dav::http_from_unc`] and
//!   [`dav::unc_from_http`].
//! - `NETAPI_ValidateServername`: [`server::validate_servername`].
//! - `wprint_mac`: [`mac::format_mac`].
//! - `sd_control_to_samba`, `ace_type_to_samba`, `ace_flags_to_samba` and `access_mask_to_samba`:
//!   [`samba`].
//!
//! Skipped:
//! - `wprint_name`. When the name is truncated, the C code writes its NUL one unit past the end
//!   of the buffer, so there is no bounded behavior to port.
//! - The NBSTAT and FindName reply parsers in `nbt.c`. They are follow-up work.
//! - Sockets, the NetBIOS command queue and name cache, the registry, Samba calls, and the heap.
//! - `sid_to_samba` and `ace_to_samba`. They need the Samba SID and ACE layouts, which are not
//!   ported here.
//!
//! Reference quirks that are kept:
//! - An ACE `CONTAINER_INHERIT` flag maps to `NO_PROPAGATE_INHERIT`, not to the container bit.
//! - An ACE flag or access-mask bit outside the known set makes the whole result 0.
//! - `DavGetHTTPFromUNCPath` accepts an empty server, so `\\\` gives `http://`. It does not reject
//!   `//` in the path.
//! - `DavGetUNCFromHTTPPath` writes an `@` for an empty port, so `http://server:` gives
//!   `\\server@\DavWWWRoot`. Text after the port digits becomes the path with no separator, so
//!   `http://server:80x` gives `\\server\DavWWWRootx`.
//!
//! Divergences from C:
//! - The C code writes past the caller's buffer in some cases that the oracle's sanitizers flag.
//!   This port returns `None` for a name request that does not fit, and `format_mac` stops at
//!   `capacity - 1` digits, so the result is always within the buffer.
//! - Case folding and white space are ASCII only, as in the oracle's shim. msvcrt also folds
//!   and classifies code points above U+00FF, which is not reproduced.

pub mod dav;
pub mod mac;
pub mod nbt;
pub mod samba;
pub mod server;
mod wide;

pub const NO_ERROR: u32 = 0;
pub const NERR_SUCCESS: u32 = 0;
pub const NERR_NO_NETWORK_RESOURCE: u32 = 2105;
pub const NERR_NETWORK_ERROR: u32 = 2136;
pub const NERR_NAME_NOT_FOUND: u32 = 2273;
pub const NERR_ALREADY_EXISTS: u32 = 2276;
pub const NERR_TOO_MANY_NAMES: u32 = 2277;
pub const NERR_DUPLICATE_NAME: u32 = 2297;
pub const NERR_DELETE_LATER: u32 = 2298;
pub const ERROR_REM_NOT_LIST: u32 = 51;
pub const ERROR_BAD_NETPATH: u32 = 53;
pub const ERROR_BAD_NET_NAME: u32 = 67;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
pub const ERROR_INVALID_NAME: u32 = 123;

const NRC_GOODRET: u32 = 0x00;
const NRC_NORES: u32 = 0x09;
const NRC_DUPNAME: u32 = 0x0d;
const NRC_NAMTFUL: u32 = 0x0e;
const NRC_ACTSES: u32 = 0x0f;
const NRC_REMTFUL: u32 = 0x12;
const NRC_NOCALL: u32 = 0x14;
const NRC_NOWILD: u32 = 0x15;
const NRC_INUSE: u32 = 0x16;
const NRC_NAMERR: u32 = 0x17;
const NRC_NAMCONF: u32 = 0x19;

/// `NetpNetBiosStatusToApiStatus`: maps a NetBIOS return code to a Win32 or NERR status. Codes
/// with no mapping give `NERR_NETWORK_ERROR`.
pub fn netbios_status_to_api_status(nrc: u32) -> u32 {
    match nrc {
        NRC_GOODRET => NO_ERROR,
        NRC_NORES => NERR_NO_NETWORK_RESOURCE,
        NRC_DUPNAME => NERR_ALREADY_EXISTS,
        NRC_NAMTFUL => NERR_TOO_MANY_NAMES,
        NRC_ACTSES => NERR_DELETE_LATER,
        NRC_REMTFUL => ERROR_REM_NOT_LIST,
        NRC_NOCALL => NERR_NAME_NOT_FOUND,
        NRC_NOWILD | NRC_NAMERR => ERROR_INVALID_PARAMETER,
        NRC_INUSE | NRC_NAMCONF => NERR_DUPLICATE_NAME,
        _ => NERR_NETWORK_ERROR,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
