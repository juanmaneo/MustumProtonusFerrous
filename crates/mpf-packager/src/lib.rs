//! Port of the portable logic in Wine's `dlls/packager/packager_main.c` and `dlls/packager/packager.spec`.
//!
//! Ported:
//! - `OleObject_*` methods: [`query_interface`], [`add_ref`], [`release`], [`set_client_site`],
//!   [`close`], [`do_verb`], [`get_misc_status`], and the `E_NOTIMPL` methods
//!   ([`get_client_site`], [`set_host_names`], [`set_moniker`], [`get_moniker`],
//!   [`init_from_data`], [`get_clipboard_data`], [`enum_verbs`], [`update`], [`is_up_to_date`],
//!   [`get_user_class_id`], [`get_user_type`], [`set_extent`], [`get_extent`], [`advise`],
//!   [`unadvise`], [`enum_advise`], [`set_color_scheme`]), with [`Package`] and [`OleInterface`]
//! - `PersistStorage_*` methods, except `Load`: [`persist_storage_query_interface`],
//!   [`persist_storage_add_ref`], [`persist_storage_release`], [`get_class_id`], [`is_dirty`],
//!   [`init_new`], [`save`], [`save_completed`], [`hands_off_storage`]
//! - `discard_string`, the NUL-terminated string skipper of `PersistStorage_Load`: [`discard_string`]
//! - `PackageCF_*` class factory methods: [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`create_instance`], [`lock_server`]
//! - `DllGetClassObject`: [`dll_get_class_object`]
//! - `HRESULT` values and the `IID_*` and `CLSID_Package` GUIDs: [`Guid`], [`HRESULT`], [`failed`]
//!
//! Skipped:
//! - `PersistStorage_Load`: it opens the `\1Ole10Native` stream, creates a file on disk and
//!   copies the payload into it. Only the string skipper it uses, `discard_string`, is ported.
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: these are `-private`
//!   exports in `packager.spec`, and `dlls/packager` has no C body for them.
//! - `FIXME`, `TRACE` and `WARN` logging, the vtable tables, and the Win32 calls
//!   (`ShellExecuteW`, `DeleteFileW`, `HeapAlloc`, `HeapFree`, `IOleClientSite_*`). The port models
//!   their effects as flags in [`ReleaseOutcome`] and [`ClientSiteChange`], or as the `allocated`
//!   argument of [`create_instance`].

mod class_factory;
mod com;
mod dll;
mod ole_object;
mod persist_storage;

pub use class_factory::{
    add_ref as class_factory_add_ref, create_instance, lock_server,
    query_interface as class_factory_query_interface, release as class_factory_release,
};
pub use com::{
    failed, Guid, CLASS_E_CLASSNOTAVAILABLE, CLSID_PACKAGE, E_FAIL, E_INVALIDARG, E_NOINTERFACE,
    E_NOTIMPL, E_OUTOFMEMORY, HRESULT, IID_ICLASS_FACTORY, IID_IOLE_OBJECT, IID_IPERSIST_STORAGE,
    IID_IUNKNOWN, S_FALSE, S_OK,
};
pub use dll::dll_get_class_object;
pub use ole_object::{
    add_ref, advise, close, do_verb, enum_advise, enum_verbs, get_client_site, get_clipboard_data,
    get_extent, get_misc_status, get_moniker, get_user_class_id, get_user_type, init_from_data,
    is_up_to_date, query_interface, release, set_client_site, set_color_scheme, set_extent,
    set_host_names, set_moniker, unadvise, update, ClientSiteChange, OleInterface, Package,
    ReleaseOutcome, OLEMISC_ONLYICONIC,
};
pub use persist_storage::{
    add_ref as persist_storage_add_ref, discard_string, get_class_id, hands_off_storage, init_new,
    is_dirty, query_interface as persist_storage_query_interface,
    release as persist_storage_release, save, save_completed,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
