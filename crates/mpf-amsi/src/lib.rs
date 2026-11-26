//! Port of the portable logic in Wine's `dlls/amsi/main.c`, `dlls/amsi/amsi.spec`,
//! and the `__wine_spec_unimplemented_stub` handler in `libs/winecrt0/stub.c`.
//!
//! Ported:
//! - `AmsiInitialize`, `AmsiOpenSession`: the fixed handle `0xdeadbeef` is stored
//!   through the out pointer and `S_OK` is returned: [`amsi_initialize`], [`amsi_open_session`]
//! - `AmsiScanBuffer`, `AmsiScanString`: `AMSI_RESULT_NOT_DETECTED` is stored through the
//!   result pointer and `S_OK` is returned: [`amsi_scan_buffer`], [`amsi_scan_string`]
//! - `AmsiCloseSession`, `AmsiUninitialize`: no-ops: [`amsi_close_session`], [`amsi_uninitialize`]
//! - Export declarations in `amsi.spec`: [`export_kind`], [`ExportKind`], [`AMSI_DLL_NAME`]
//! - `@ stub` exports (`AmsiUacInitialize`, `AmsiUacScan`, `AmsiUacUninitialize`,
//!   `DllGetClassObject`): the exception record raised by [`wine_spec_unimplemented_stub`]
//!
//! Skipped:
//! - Input arguments that the C bodies only pass to `FIXME` tracing and never read
//!   (`appname`, `context`, `session`, `buffer`, `length`, `name`, `string`).
//! - A NULL out pointer: the C bodies store through it and crash. The port takes `&mut`,
//!   so this input is not representable (see the excluded oracle vector).
//! - `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`: declared `-private` in
//!   `amsi.spec` but with no body in `dlls/amsi/main.c`, so there is no behaviour to port.
//!   [`export_kind`] classifies them only.

mod api;
mod spec;
mod stub;

pub use api::{
    amsi_close_session, amsi_initialize, amsi_open_session, amsi_scan_buffer, amsi_scan_string,
    amsi_uninitialize, AmsiResult, Handle, Hresult, AMSI_RESULT_NOT_DETECTED, FAKE_HANDLE, S_OK,
};
pub use spec::{export_kind, ExportKind, AMSI_DLL_NAME};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
