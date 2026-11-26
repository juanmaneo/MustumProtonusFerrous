//! Port of the portable logic in Wine's `dlls/msdaps/row_server.c`, `dlls/msdaps/main.c` and
//! `dlls/msdaps/msdaps.spec`.
//!
//! Ported:
//! - `db_type_size` from `row_server.c`: [`db_type_size`], with the `DBTYPE_*` values from
//!   `include/dbs.idl`
//! - The CLSID dispatch at the top of `DllGetClassObject` in `main.c`: [`class_object_for_clsid`],
//!   [`ClassObject`]
//! - `CF_QueryInterface`, `CF_AddRef`, `CF_Release` and `CF_LockServer` in `main.c`:
//!   [`class_factory_query_interface`], [`class_factory_add_ref`], [`class_factory_release`],
//!   [`class_factory_lock_server`]
//! - The GUIDs and `HRESULT` values those functions compare or return: [`Guid`],
//!   [`is_equal_guid`], [`S_OK`], [`E_NOINTERFACE`], [`E_NOTIMPL`]
//!
//! Modelled as a constant:
//! - `E_NOTIMPL` is the value of the unimplemented paths in `row_server.c` and `usrmarshal.c`
//!   (18 and 84 `return E_NOTIMPL` sites, and `hr = E_NOTIMPL` at `row_server.c:239`). None of
//!   them is an export, so none has an oracle row. The value is pinned in `edge_checks.rs`.
//!
//! Skipped:
//! - `msdaps.spec` has no `@ stub` exports. Its four exports are `-private` and forward to
//!   `msdaps_*` functions that `dlldata.c` generates at build time, which is not in the reference
//!   tree. That covers `DllMain`, `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`
//!   and the fall-through of `DllGetClassObject`.
//! - `CF_CreateInstance`, and the `IClassFactory_AddRef` call and the `*obj` store in
//!   `CF_QueryInterface`. They need interface pointers, so the port returns the `HRESULT` that
//!   decides them.
//! - The `create_*` functions, the `create_proxy` class dispatch in `row_server.c`, the COM
//!   methods and the `usrmarshal.c` proxy and stub functions. They need `HeapAlloc`, the RPC
//!   runtime and MIDL-generated `Remote*_Proxy` code, none of which is in the tree.
//! - `FIXME` and `TRACE` output.

mod class_factory;
mod class_object;
mod com;
mod db_type;

pub use class_factory::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release,
};
pub use class_object::{class_object_for_clsid, ClassObject};
pub use com::{
    is_equal_guid, Guid, CLSID_WINE_ROWSET_PROXY, CLSID_WINE_ROWSET_SERVER, CLSID_WINE_ROW_PROXY,
    CLSID_WINE_ROW_SERVER, E_NOINTERFACE, E_NOTIMPL, HRESULT, IID_ICLASSFACTORY, IID_IUNKNOWN,
    S_OK,
};
pub use db_type::{
    db_type_size, DBTYPE_BSTR, DBTYPE_CY, DBTYPE_FILETIME, DBTYPE_GUID, DBTYPE_I1, DBTYPE_I2,
    DBTYPE_I4, DBTYPE_I8, DBTYPE_R4, DBTYPE_R8, DBTYPE_UI1, DBTYPE_UI2, DBTYPE_UI4, DBTYPE_UI8,
    DBTYPE_WSTR,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
