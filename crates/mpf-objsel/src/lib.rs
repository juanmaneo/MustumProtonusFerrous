//! Port of the portable logic in Wine's `dlls/objsel/objsel.c` and `dlls/objsel/objsel.spec`.
//!
//! Ported:
//! - `DllGetClassObject` for `CLSID_DsObjectPicker`, which serves the static class factory: [`dll_get_class_object`]
//! - The static class factory (`class_factory`): [`ClassFactory`], with `QueryInterface` (`IUnknown`,
//!   `IClassFactory`), `AddRef`, `Release`, `CreateInstance` (`IDsObjectPicker` only), and `LockServer`
//! - The `IDsObjectPicker` object: [`ObjectPicker`], with `QueryInterface` (`IUnknown`, `IDsObjectPicker`),
//!   `AddRef`, `Release`, [`object_picker_create`], and the two `FIXME ... stub!` methods,
//!   [`ObjectPicker::initialize`] (`S_OK`) and [`ObjectPicker::invoke_dialog`] (`S_FALSE`)
//! - `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`): [`dll_can_unload_now`]
//! - `DllRegisterServer` and `DllUnregisterServer` (`libs/winecrt0/dll_register.c`): [`dll_register_server`],
//!   [`dll_unregister_server`]
//! - The `HRESULT` values from `include/winerror.h`, and `FAILED`: [`Hresult`], [`failed`]
//!
//! Skipped:
//! - `TRACE`, `WARN` and `FIXME` output, and the `pInitInfo`, `hwndParent` and `ppdoSelections` arguments,
//!   which the C bodies never read.
//! - `HeapAlloc` and `HeapFree`: allocation is the `heap_alloc` closure argument, and the free is the
//!   `destroyed` flag.
//! - `__wine_register_resources` and `__wine_unregister_resources` (`libs/winecrt0/register.c`). They load
//!   ATL100 and write the module's registry resources through the OS, so the port takes their result as an argument.
//! - `InterlockedIncrement` and `InterlockedDecrement`, and thread safety. The counts use 32-bit wrapping arithmetic.
//! - The COM vtable layout and `CONTAINING_RECORD`. Interface pointers are [`Interface`] values.
//! - `@ stub` exports: `objsel.spec` declares none, so there is no `__wine_spec_unimplemented_stub` model.

mod class_factory;
mod dll;
mod guid;
mod hresult;
mod object_picker;

pub use class_factory::ClassFactory;
pub use dll::{
    dll_can_unload_now, dll_get_class_object, dll_register_server, dll_unregister_server,
};
pub use guid::{
    Guid, CLSID_DS_OBJECT_PICKER, IID_CLASS_FACTORY, IID_DS_OBJECT_PICKER, IID_UNKNOWN,
};
pub use hresult::{
    failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_FAIL, E_NOINTERFACE,
    E_OUTOFMEMORY, E_POINTER, S_FALSE, S_OK,
};
pub use object_picker::{object_picker_create, Interface, ObjectPicker};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
