//! Portable deterministic logic from Wine's `dlls/spoolss`.
//!
//! This crate ports the copy semantics of `AllocSplStr` and the success check in
//! `SplInitializeWinSpoolDrv`. Calls that allocate through the Windows heap or load
//! `winspool.drv` are represented as pure inputs; provider routing and FIXME stubs are skipped.

mod winspool;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use winspool::{winspool_table_ready, TABLE_LEN};

/// Copies a NUL-terminated wide string up to and including its first NUL, like `AllocSplStr`.
///
/// `None` (a NULL pointer) returns `None`. `src` must contain a NUL unit; the copy stops at the
/// first one.
pub fn alloc_spl_str(src: Option<&[u16]>) -> Option<Vec<u16>> {
    let src = src?;
    let nul = src
        .iter()
        .position(|&unit| unit == 0)
        .expect("src must be NUL-terminated");
    Some(src[..=nul].to_vec())
}
