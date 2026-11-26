//! Port of the portable logic in Wine's `dlls/fltlib/fltlib.c`.
//!
//! Ported:
//! - `HRESULT_FROM_WIN32` (`winerror.h`): [`hresult_from_win32`]
//! - Return values of the stubs `FilterConnectCommunicationPort`, `FilterFindFirst`,
//!   `FilterFindClose`, `FilterLoad` and `FilterUnload` (including its NULL name check):
//!   [`filter_connect_communication_port`], [`filter_find_first`], [`filter_find_close`],
//!   [`filter_load`], [`filter_unload`]
//!
//! Skipped:
//! - The 22 `@ stub` exports in `fltlib.spec` (`FilterAttach`, `FilterCreate`,
//!   `FilterInstance*`, `FilterVolume*`, `FilterGetMessage`, ...). Wine has no C body for them,
//!   so there is no reference return value to model.
//! - The `FIXME` trace calls, which have no observable effect.
//! - Stub arguments: Wine does not read them, so the ported stubs take none except
//!   `filter_unload`, whose NULL check is the only argument-dependent behaviour.

mod filter;
mod hresult;

pub use filter::{
    filter_connect_communication_port, filter_find_close, filter_find_first, filter_load,
    filter_unload,
};
pub use hresult::{
    hresult_from_win32, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, ERROR_NO_MORE_ITEMS,
    E_NOTIMPL, FACILITY_WIN32, INVALID_HANDLE_VALUE, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
