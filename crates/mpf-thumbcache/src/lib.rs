//! Port of the portable logic in Wine's `dlls/thumbcache/thumbcache.c` and `dlls/thumbcache/thumbcache.spec`.
//!
//! Ported:
//! - `DllGetClassObject` dispatch on `CLSID_LocalThumbnailCache`: [`dll_get_class_object`]
//! - the class factory (`class_factory_*`): [`ClassFactory`]
//! - the `IThumbnailCache` object (`thumbcache_*`, reference counts, `thumbcache_create`):
//!   [`ThumbCache`], [`thumb_cache_create`]
//! - the `E_NOTIMPL` results of `GetThumbnail` and `GetThumbnailByID`
//! - the `QueryInterface` check, the `HRESULT` values and the GUIDs: [`query_interface_hr`],
//!   [`Hresult`], [`Guid`]
//! - the `thumbcache.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The `TRACE` and `FIXME` output and the `debugstr_guid` text.
//! - The `outer` argument of `thumbcache_create` and `class_factory_CreateInstance`, which the
//!   C code never reads.
//! - The `obj` out-pointer. `Some` stands for the object or factory that C stores in `*obj`,
//!   `None` for NULL. C leaves `*obj` untouched on the `thumbcache_create` out-of-memory path
//!   and for an unknown CLSID in `DllGetClassObject`; the port cannot tell that apart from NULL.
//! - `malloc` and `free`: the allocation is a closure argument and dropping the object stands
//!   in for `free`.
//! - The arguments of `GetThumbnail` and `GetThumbnailByID`, which the C code never reads.
//! - The `lock` argument of `LockServer`, which only feeds the `FIXME`.
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: `thumbcache.spec` lists
//!   them, but `thumbcache.c` does not define them, so there is no body to port
//!   (see [`ExportKind::NotInSource`]).

mod class_factory;
mod dll;
mod exports;
mod guid;
mod hresult;
mod thumb_cache;

pub use class_factory::ClassFactory;
pub use dll::dll_get_class_object;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use guid::{
    Guid, CLSID_LOCAL_THUMBNAIL_CACHE, IID_ICLASSFACTORY, IID_ITHUMBNAILCACHE, IID_IUNKNOWN,
};
pub use hresult::{
    query_interface_hr, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL,
    E_OUTOFMEMORY, S_OK,
};
pub use thumb_cache::{thumb_cache_create, ThumbCache};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
