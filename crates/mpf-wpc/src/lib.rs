//! Port of the portable logic in Wine's `dlls/wpc/wpc.c` and `dlls/wpc/wpc.spec`.
//!
//! Ported:
//! - `DllGetClassObject` and its CLSID check: [`dll_get_class_object`]
//! - Class factory `QueryInterface`, `AddRef`, `Release`, `LockServer` and `CreateInstance`:
//!   [`class_factory`]
//! - `WindowsParentalControls` `QueryInterface`, `AddRef`, `Release` and the five `E_NOTIMPL`
//!   methods: [`object`]
//! - The CLSID and IIDs from `include/wpcapi.idl` and `include/unknwn.idl`: [`Guid`] constants
//! - The `HRESULT` values from `include/winerror.h`: [`Hresult`]
//! - `ApplyParentalControlsW` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it
//!   raises: [`apply_parental_controls_w`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - `wpc.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. `dlls/wpc` has no C body
//!   for them; the midl proxy code provides them (`include/rpcproxy.h`). They are listed in
//!   [`EXPORTS`] as [`ExportKind::Generated`].
//! - The `FIXME` and `TRACE` output, the arguments that the C code only traces (the
//!   `visibility`, `sid`, `settings`, `id`, `name` and `fLock` parameters), the `outer` argument
//!   of `CreateInstance`, and the `IUnknown_AddRef` call on the QueryInterface success path.
//! - The COM vtable layout and the `WINAPI` calling convention.

pub mod class_factory;
mod exports;
mod guid;
mod hresult;
mod interface;
pub mod object;
mod stub;

pub use class_factory::dll_get_class_object;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use guid::{
    is_equal_guid, Guid, CLSID_WINDOWS_PARENTAL_CONTROLS, IID_CLASS_FACTORY, IID_UNKNOWN,
    IID_WINDOWS_PARENTAL_CONTROLS, IID_WINDOWS_PARENTAL_CONTROLS_CORE,
};
pub use hresult::{Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, S_OK};
pub use object::Object;
pub use stub::{
    apply_parental_controls_w, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
