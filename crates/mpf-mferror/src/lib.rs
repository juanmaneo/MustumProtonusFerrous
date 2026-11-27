//! Port of the portable logic in Wine's `dlls/mferror/mferror.mc` and `dlls/mferror/Makefile.in`.
//!
//! `dlls/mferror` has no `.spec`, `.c`, `.idl`, `.rc` or `.tlb` file. Its Makefile links with
//! `-Wl,-noentry` and has `mferror.mc` as its only source, so the DLL runs no code and exports
//! nothing. Its deterministic content is the message table that `wmc` compiles from `mferror.mc`.
//!
//! Ported:
//! - the `MessageId` table in `mferror.mc` order: [`MESSAGES`] and [`Message`]
//! - the `Severity` and `Facility` values: [`STATUS_SEVERITY_ERROR`], [`STATUS_SEVERITY_SUCCESS`]
//!   and [`FACILITY_MF`]
//! - the 32-bit value that `wmc` writes for each entry, from `complete_msg` in `tools/wmc/mcy.y`:
//!   [`make_message_id`] and [`Message::hresult`]
//! - lookups by `SymbolicName` and by HRESULT: [`find_by_symbol`] and [`find_by_hresult`]
//!
//! Skipped:
//! - The `Customer` bit (`1 << 29`) that `complete_msg` sets when `custombit` is on. `mferror.mc`
//!   never sets it.
//! - The `wmc` output formats (`.h`, `.rc` and the `RT_MESSAGETABLE` resource) and the
//!   `FormatMessage` path that reads them. These are build tooling and OS code.
//! - Stub exports: the DLL has no `@ stub` export, so no `StubException` model applies.

mod message_id;
mod messages;

pub use message_id::{
    make_message_id, FACILITY_MF, STATUS_SEVERITY_ERROR, STATUS_SEVERITY_SUCCESS,
};
pub use messages::{find_by_hresult, find_by_symbol, Message, MESSAGES};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
