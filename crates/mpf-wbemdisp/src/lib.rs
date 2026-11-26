//! Port of the portable logic in Wine's `dlls/wbemdisp/main.c` and `dlls/wbemdisp/wbemdisp.spec`.
//!
//! Ported:
//! - `wbemdisp.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The four `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`dll_can_unload_now`], [`dll_get_class_object`], [`dll_register_server`],
//!   [`dll_unregister_server`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The COM/OS-dependent bodies in `main.c` (the `WinMGMTS` `IParseDisplayName`
//!   implementation, the class factories and `DllGetClassObject`'s real body) and the
//!   `locator.c` WMI wrappers, which need the Windows runtime and WMI infrastructure.
//! - The `@ stub` exports are modelled with the Stub model, not as returning `E_NOTIMPL`
//!   or `0`.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    dll_can_unload_now, dll_get_class_object, dll_register_server, dll_unregister_server,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
