//! Port of the portable logic in Wine's `dlls/oledb32/main.c`, `convert.c`, `datainit.c` and
//! `oledb32.spec`, plus the `DllCanUnloadNow` default from `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllGetClassObject` and its five CLSIDs: [`dll_get_class_object`], [`class_for_clsid`],
//!   [`FactoryKind`], [`ObjOut`], with the class-factory results [`cf_query_interface`],
//!   [`cf_add_ref`], [`cf_release`] and [`cf_lock_server`]
//! - `DllMain` and `DllCanUnloadNow`: [`dll_main`], [`dll_can_unload_now`]
//! - The DBTYPE helpers in `convert.c`: [`dbtype_length`], [`dbtype_class`], [`common_class`],
//!   [`array_type`]
//! - The init-string parser in `datainit.c`: [`parse_init_string`], [`DbProp`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`. Winecrt0 forwards them to
//!   `__wine_register_resources`, which writes the registry and the type library.
//! - The COM objects: vtables, the `create_*` constructors, `CF_CreateInstance`, object state
//!   and reference counts. `cf_add_ref` and `cf_release` return the fixed values the C code
//!   returns, not counts.
//! - `DllMain`'s instance store and `DisableThreadLibraryCalls`.
//! - `convert_DataConvert`, `convert_CanConvert` and `convert_GetConversionSize`.
//! - The data-source and initialization objects in `datainit.c` (`datainit_*`, `datainitialize_*`,
//!   `dbinit_*`, `dbprops_*`, `dcinfo_*`, `get_data_source`) and the data-link dialogs in
//!   `dslocator.c`.
//! - `errorinfo.c` and `rowpos.c`.
//! - Allocation failures (`E_OUTOFMEMORY`) and the `TRACE` and `FIXME` output.
//! - Non-ASCII case folding in `wcsicmp` and whitespace in `iswspace`. Only ASCII is modelled.

mod class_object;
mod dbtype;
mod dll;
mod init_string;

pub use class_object::{
    cf_add_ref, cf_lock_server, cf_query_interface, cf_release, class_for_clsid,
    dll_get_class_object, FactoryKind, Guid, Hresult, ObjOut, CLASS_E_CLASSNOTAVAILABLE,
    E_NOINTERFACE, S_OK,
};
pub use dbtype::{array_type, common_class, dbtype_class, dbtype_length};
pub use dll::{dll_can_unload_now, dll_main};
pub use init_string::{parse_init_string, DbProp};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
