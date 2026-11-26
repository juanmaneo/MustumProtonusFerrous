//! Port of the portable logic in Wine's `dlls/wmp/wmp_main.c` and `dlls/wmp/wmp.spec`.
//!
//! Ported:
//! - `DllGetClassObject` (explicit export): [`dll_get_class_object`], [`ClassObject`],
//!   [`CLASS_E_CLASSNOTAVAILABLE`]
//! - `ClassFactory_QueryInterface`, `ClassFactory_AddRef`, `ClassFactory_Release` and
//!   `ClassFactory_LockServer`: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`], [`class_factory_lock_server`], with [`S_OK`] and [`E_NOINTERFACE`]
//! - `DllMain` state logic: [`dll_main`], [`DllMainOutcome`], [`DLL_PROCESS_ATTACH`],
//!   [`DLL_PROCESS_DETACH`]
//! - `wmp.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The four unnamed `@ stub` ordinals and the `__wine_spec_unimplemented_stub` record they
//!   raise: [`ordinal_3000`], [`ordinal_3002`], [`ordinal_3003`], [`ordinal_3004`],
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: `wmp.spec` exports them
//!   as `-private` but `dlls/wmp` has no body for them. They are listed in [`EXPORTS`] as
//!   [`ExportKind::Generated`] and not modelled.
//! - `WMPFactory_CreateInstance`, `load_typelib`, `get_typeinfo`, `release_typelib` and the
//!   `unregister_*` calls: they create objects, load the typelib or call into other `dlls/wmp`
//!   units, so they need the OS or COM.
//! - The `TRACE` and `FIXME` output, the stored `HINSTANCE` value, and the `__cdecl`/`WINAPI`
//!   calling conventions.

mod class_factory;
mod dll_get_class_object;
mod dll_main;
mod exports;
mod guid;
mod stub;

pub use class_factory::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release, E_NOINTERFACE, S_OK,
};
pub use dll_get_class_object::{dll_get_class_object, ClassObject, CLASS_E_CLASSNOTAVAILABLE};
pub use dll_main::{dll_main, DllMainOutcome, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use guid::{is_equal_guid, Guid, CLSID_WINDOWS_MEDIA_PLAYER, IID_ICLASSFACTORY, IID_IUNKNOWN};
pub use stub::{
    ordinal_3000, ordinal_3002, ordinal_3003, ordinal_3004, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
