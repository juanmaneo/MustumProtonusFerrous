//! Port of the portable logic in Wine's `dlls/ksproxy.ax/ksproxy.ax.spec` and the
//! `__wine_spec_unimplemented_stub` body in `dlls/ntdll/exception.c`.
//!
//! `dlls/ksproxy.ax` has no C sources, only `ksproxy.ax.spec` and `Makefile.in`, so the port
//! covers the export table and the stub records. The DLL has no executable code of its own.
//!
//! Ported:
//! - `ksproxy.ax.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports (`DllGetClassObject`, `KsGetMediaType`, `KsGetMediaTypeCount`,
//!   `KsGetMultiplePinFactoryItems`, `KsOpenDefaultDevice`, `KsResolveRequiredAttributes`,
//!   `KsSynchronousDeviceControl`) and the exception record that `__wine_spec_unimplemented_stub`
//!   raises: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//!   such as [`dll_get_class_object`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: declared `-private` in
//!   `ksproxy.ax.spec` but with no body in `dlls/ksproxy.ax`, so there is no behaviour to port.
//!   [`export_kind`] classifies them only.
//! - The ordinal form of `function` (a number for an export with no name): every export in the
//!   spec has a name.
//! - A NULL `module` or `function`: `winebuild` never passes NULL, and the port takes
//!   `&'static str`.

mod spec;
mod stub;

pub use spec::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    dll_get_class_object, ks_get_media_type, ks_get_media_type_count,
    ks_get_multiple_pin_factory_items, ks_open_default_device, ks_resolve_required_attributes,
    ks_synchronous_device_control, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
