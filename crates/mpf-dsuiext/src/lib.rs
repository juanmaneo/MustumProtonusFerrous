//! Port of the portable logic in Wine's `dlls/dsuiext/dsuiext.c`.
//!
//! Ported:
//! - `DllGetClassObject` and its `class_info` table: [`dll_get_class_object`], [`ClassInfo`]
//! - `IDsDisplaySpecifier` (`dispspec_QueryInterface`, `dispspec_AddRef`, `dispspec_Release`,
//!   `DsDisplaySpecifier_create`): [`DisplaySpec`], [`ADSTYPE_INVALID`]
//! - `IClassFactory` (`factory_*`, `factory_constructor`): [`ClassFactory`], [`factory_constructor`]
//! - Vtable entries that return a fixed value (`E_NOTIMPL`, NULL, `FALSE`, `ADSTYPE_INVALID`):
//!   the associated functions of [`DisplaySpec`]
//! - `DllCanUnloadNow` (the `winecrt0` default, `S_FALSE`): [`dll_can_unload_now`]
//! - `@ stub` exports, modelled as the `EXCEPTION_WINE_STUB` each raises: [`unimplemented_stub`],
//!   [`dll_install`], [`ds_browse_for_container_a`], [`ds_browse_for_container_w`],
//!   [`ds_get_friendly_class_name`], [`ds_get_icon`]
//!
//! The module name `dsuiext` in the stubs is inferred: winebuild derives the module argument
//! from the `.spec` file name. No real build was run to confirm it.
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`: winecrt0 wrappers around
//!   `__wine_register_resources`, which writes the registry and reads embedded resources.
//! - `malloc` failure paths (`E_OUTOFMEMORY`): allocation is not modelled.
//! - COM vtable layout and `CONTAINING_RECORD` pointer arithmetic: replaced by safe Rust types.
//! - `TRACE` and `FIXME` logging.
//! - `InterlockedIncrement` and `InterlockedDecrement`: plain wrapping arithmetic on [`RefCount`].

mod class_factory;
mod com;
mod display_spec;
mod dll;
mod guid;
mod stub;

pub use class_factory::{factory_constructor, ClassFactory};
pub use com::{
    Hresult, Interface, RefCount, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_INVALIDARG,
    E_NOINTERFACE, E_NOTIMPL, S_FALSE, S_OK,
};
pub use display_spec::{DisplaySpec, ADSTYPE_INVALID};
pub use dll::{dll_can_unload_now, dll_get_class_object, ClassInfo};
pub use guid::{
    Guid, CLSID_DS_DISPLAY_SPECIFIER, IID_ICLASSFACTORY, IID_IDS_DISPLAY_SPECIFIER, IID_IUNKNOWN,
};
pub use stub::{
    dll_install, ds_browse_for_container_a, ds_browse_for_container_w, ds_get_friendly_class_name,
    ds_get_icon, unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
