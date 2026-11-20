//! Port of the mode handling in Wine's `dlls/ieframe/intshcut.c`, `PropertySetStorage_Open`.
//!
//! The Internet Shortcut forwards each open of its property sets to the storage that ole32 gives
//! it. Before the forward, Wine rewrites the access bits of `grfMode`: `STGM_WRITE` becomes
//! `STGM_READWRITE`, because ole32 rejects a write-only open, and `STGM_SHARE_EXCLUSIVE` is added to
//! work around a bug in ole32's share handling. [`open_mode`] computes that mode.
//!
//! Ported:
//! - The mode rewrite and the share flag of `PropertySetStorage_Open`: [`open_mode`].
//!
//! Skipped:
//! - `IPropertySetStorage_Open` and the other forwarding calls, the `QueryInterface`, `AddRef` and
//!   `Release` methods, and `PropertySetStorage_Create`, `Delete` and `Enum`: COM calls into ole32.
//! - The `TRACE` calls: logging.
//!
//! Divergences from C:
//! - `DWORD` is `u32`, as it is on Windows. Wine's headers define `DWORD` as `unsigned long`, which
//!   is 64 bits on LP64 hosts, but the Windows value that the caller passes is 32 bits.

/// `STGM_WRITE`: write-only access, which ole32 rejects.
pub const STGM_WRITE: u32 = 0x0000_0001;
/// `STGM_READWRITE`: the access that replaces `STGM_WRITE`.
pub const STGM_READWRITE: u32 = 0x0000_0002;
/// `STGM_SHARE_EXCLUSIVE`: the share mode that Wine adds to every open.
pub const STGM_SHARE_EXCLUSIVE: u32 = 0x0000_0010;

/// The access bits of `grfMode`, as `PropertySetStorage_Open` masks them.
const STGM_ACCESS_MASK: u32 = 0x0000_000f;

/// The `grfMode` that `PropertySetStorage_Open` passes on: the access bits with `STGM_WRITE`
/// replaced by `STGM_READWRITE`, the other bits unchanged, and `STGM_SHARE_EXCLUSIVE` added.
pub fn open_mode(grf_mode: u32) -> u32 {
    let mode = if (grf_mode & STGM_ACCESS_MASK) == STGM_WRITE {
        (grf_mode & !STGM_ACCESS_MASK) | STGM_READWRITE
    } else {
        grf_mode
    };
    mode | STGM_SHARE_EXCLUSIVE
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
