//! Port of the portable logic in Wine's `dlls/pstorec/pstorec.c` and `dlls/pstorec/pstorec.spec`,
//! plus `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`, which the DLL links.
//!
//! Ported:
//! - `IPStore` reference counting and `QueryInterface` (`PStore_fnAddRef`, `PStore_fnRelease`,
//!   `PStore_fnQueryInterface`): [`PStoreProvider`], [`Guid`], [`IID_IPSTORE`], [`IID_IUNKNOWN`]
//! - `PStoreCreateInstance`: [`pstore_create_instance`]
//! - `DllCanUnloadNow` and `DllGetClassObject`: [`dll_can_unload_now`], [`dll_get_class_object`]
//! - The 19 `IPStore` methods that only trace and return `E_NOTIMPL`, such as [`get_info`]
//! - `pstorec.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports `GetPStoreProvider` and `PStoreEnumProviders`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`get_pstore_provider`],
//!   [`pstore_enum_providers`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer` (`libs/winecrt0/dll_register.c`). They
//!   register the module's resources through `__wine_register_resources`, which needs the image
//!   resources and the ATL registrar. They appear in [`EXPORTS`] only.
//! - The `FIXME` and `TRACE` output, and the arguments that the C bodies only trace.
//! - The `IPStore` vtable layout and the `This` pointer. Rust methods take no vtable.
//! - The `*ppvObj` out pointer of `QueryInterface`, which is implied by the return value.
//! - `HeapAlloc`, `HeapFree` and `GetProcessHeap`. `allocate` is a closure argument, and the
//!   owner drops the provider when `release` returns 0.
//! - A NULL `ppProvider` in `PStoreCreateInstance`. The Rust argument is a reference, so the
//!   C write through NULL has no equivalent.
//! - `pstorec_tlb.idl`, which only describes the type library.

mod dll;
mod exports;
mod not_implemented;
mod provider;
mod stub;

pub use dll::{dll_can_unload_now, dll_get_class_object, CLASS_E_CLASSNOTAVAILABLE, S_FALSE};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use not_implemented::{
    close_item, create_subtype, create_type, delete_item, delete_subtype, delete_type, enum_items,
    enum_subtypes, enum_types, get_info, get_prov_param, get_subtype_info, get_type_info,
    open_item, read_access_ruleset, read_item, set_prov_param, write_access_ruleset, write_item,
    E_NOTIMPL,
};
pub use provider::{
    pstore_create_instance, Guid, PStoreProvider, E_NOINTERFACE, E_OUTOFMEMORY, IID_IPSTORE,
    IID_IUNKNOWN, S_OK,
};
pub use stub::{
    get_pstore_provider, pstore_enum_providers, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
