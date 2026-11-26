//! Port of the portable logic in Wine's `dlls/dciman32/dciman_main.c` and
//! `dlls/dciman32/dciman32.spec`.
//!
//! Ported:
//! - `DCIOpenProvider`, `DCICloseProvider`, `DCICreatePrimary`: the stub
//!   bodies (last error `ERROR_CALL_NOT_IMPLEMENTED`, `NULL` HDC, and
//!   `DCI_FAIL_UNSUPPORTED`): [`Dciman32`]
//! - `DCI_*` return codes from `include/dciddi.h`: [`DCI_OK`], [`DCI_FAIL_GENERIC`],
//!   [`DCI_FAIL_UNSUPPORTEDVERSION`], [`DCI_FAIL_INVALIDSURFACE`], [`DCI_FAIL_UNSUPPORTED`]
//! - The `stub` exports in `dciman32.spec` (generated `__wine_spec_unimplemented_stub`
//!   call raises `EXCEPTION_WINE_STUB`): [`STUB_EXPORTS`], [`stub_fault`]
//!
//! Skipped:
//! - `DllEntryPoint` (private, no body in `dciman_main.c`).
//! - The `stub` exports' real behaviour: Wine has none. Only the fault they raise is modelled.
//! - `FIXME` logging and the DCI surface/overlay implementation that the stubs never reach.

mod provider;
mod stubs;

pub use provider::{
    Dciman32, Hdc, DCI_FAIL_GENERIC, DCI_FAIL_INVALIDSURFACE, DCI_FAIL_UNSUPPORTED,
    DCI_FAIL_UNSUPPORTEDVERSION, DCI_OK, ERROR_CALL_NOT_IMPLEMENTED, NULL_HDC,
};
pub use stubs::{stub_fault, StubFault, EXCEPTION_WINE_STUB, MODULE_NAME, STUB_EXPORTS};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
