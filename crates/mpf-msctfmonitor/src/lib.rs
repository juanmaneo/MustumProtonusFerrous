//! Port of the portable logic in Wine's `dlls/msctfmonitor/main.c` and
//! `dlls/msctfmonitor/msctfmonitor.spec`, plus the default `DllCanUnloadNow`, `DllRegisterServer`
//! and `DllUnregisterServer` bodies from `libs/winecrt0` that the spec exports as `-private`.
//!
//! Ported:
//! - `InitLocalMsCtfMonitor`: returns `E_FAIL` for every flags value: [`init_local_ms_ctf_monitor`],
//!   [`E_FAIL`]
//! - `DllCanUnloadNow`: returns `S_FALSE`: [`dll_can_unload_now`], [`S_FALSE`]
//! - `DllRegisterServer`, `DllUnregisterServer`: return `S_OK`, the value they start with when the
//!   module has no `WINE_REGISTRY` resources: [`dll_register_server`], [`dll_unregister_server`]
//! - `msctfmonitor.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports (`DllGetClassObject`, `DoMsCtfMonitor`, `UnInitLocalMsCtfMonitor`) and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`dll_get_class_object`],
//!   [`do_ms_ctf_monitor`], [`un_init_local_ms_ctf_monitor`], [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of `flags`, which the C body only passes to the trace.
//! - The `__wine_register_resources` enumeration of `WINE_REGISTRY` resources and the ATL registrar.
//!   `dlls/msctfmonitor/Makefile.in` lists no `RC_SRCS`, so the enumeration never calls back.
//! - The `__cdecl` and `WINAPI` calling conventions, and the `RtlRaiseException` loop: the port
//!   returns the record instead of raising it.

mod dll;
mod exports;
mod init;
mod stub;

pub use dll::{
    dll_can_unload_now, dll_register_server, dll_unregister_server, Hresult, S_FALSE, S_OK,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use init::{init_local_ms_ctf_monitor, E_FAIL};
pub use stub::{
    dll_get_class_object, do_ms_ctf_monitor, un_init_local_ms_ctf_monitor,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
