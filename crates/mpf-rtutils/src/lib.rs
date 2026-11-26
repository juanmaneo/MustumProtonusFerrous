//! Port of the portable logic in Wine's `dlls/rtutils/tracing.c` and `dlls/rtutils/rtutils.spec`.
//!
//! Ported:
//! - `TraceRegisterExW` and `TraceRegisterExA`: [`trace_register_ex_w`], [`trace_register_ex_a`],
//!   [`TraceRegistration`], with `INVALID_TRACEID` and `ERROR_OUTOFMEMORY` from `include/rtutils.h`
//!   and `include/winerror.h`
//! - `rtutils.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 54 `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper
//!   per export (for example [`create_wait_event`]) that raises the record for its name
//!
//! Skipped:
//! - The `FIXME` traces of the arguments, which the C bodies only print. The `name` and `flags`
//!   of `TraceRegisterExW` are therefore unused.
//! - The OS heap and `SetLastError`: `alloc` is a closure argument, and the last error is returned
//!   as data.
//! - The code page conversion beyond one unit per byte. Under a double-byte `CP_ACP` the converted
//!   length depends on OS state, which this port does not model.
//! - The `__cdecl`, `WINAPI` and `__stdcall` calling conventions.
//! - The `__wine_spec_unimplemented_stub` loop. The port returns the record it would raise.
//!   The spec has no ordinal-only exports, so the ordinal branch of the stub record is not modelled.

mod exports;
mod stub;
mod tracing;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::*;
pub use tracing::{
    trace_register_ex_a, trace_register_ex_w, TraceRegistration, ERROR_OUTOFMEMORY, INVALID_TRACEID,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
