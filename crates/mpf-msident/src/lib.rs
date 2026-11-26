//! Port of the portable logic in Wine's `dlls/msident/msident.c` and `dlls/msident/msident.spec`.
//!
//! Ported:
//! - `DllGetClassObject` and the `CLSID_UserIdentityManager` match: [`dll_get_class_object`]
//! - `DllCanUnloadNow`, the default `S_FALSE` from `libs/winecrt0/dll_canunload.c` that `msident.spec` exports:
//!   [`dll_can_unload_now`]
//! - `UserIdentityManagerCF` (`IClassFactory`): `QueryInterface`, `AddRef`, `Release`, `LockServer` and
//!   `CreateInstance`: [`ClassFactory`]
//! - `UserIdentityManager`: `QueryInterface`, `AddRef`, `Release`, `EnumIdentities`, and the return values of
//!   `ManageIdentities`, `Logon`, `Logoff` and `GetIdentityByCookie`: [`UserIdentityManager`]
//! - `EnumUserIdentity`: `QueryInterface`, `AddRef` and `Release` with the `LONG` count returned as `ULONG`,
//!   the `E_NOTIMPL` methods `Next`, `Skip`, `Reset` and `Clone`, and `GetCount`: [`EnumUserIdentity`]
//! - `IsEqualGUID`, and the GUIDs that the C code compares: [`Guid`], [`is_equal_guid`], [`IID_IUNKNOWN`],
//!   [`IID_ICLASSFACTORY`], [`IID_IENUM_USER_IDENTITY`], [`IID_IUSER_IDENTITY_MANAGER`],
//!   [`CLSID_USER_IDENTITY_MANAGER`]
//! - `HRESULT` values from `include/winerror.h` and `include/msident.idl`, and `SUCCEEDED`: [`hresult`] items
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`, which `msident.spec` exports but `msident.c` does not define.
//!   They forward to `__wine_register_resources` and `__wine_unregister_resources`, which read the registration
//!   data that widl generates from `msident_classes.idl` (`#pragma makedep register`). They have no logic here.
//! - Out pointers (`ppv`, `ppEnumUser`, `ppIdentity`, `ppenum`, `rgelt`, `pceltFetched`): the Rust methods return
//!   `Result` or `HRESULT` and do not expose the pointer. `Ok` stands for the C `S_OK` path that writes it.
//! - Arguments that the C bodies never read: `this`, `outer`, `fLock`, `hwndParent`, `dwFlags`, `uidCookie`,
//!   `celt`.
//! - `malloc` and `free`: the allocation outcome is an input, and dropping the value replaces `free`.
//! - `TRACE`, `FIXME`, `WARN` and `debugstr_guid` output.
//! - COM vtable dispatch and the `IUnknown` casts: the Rust methods are called directly.

mod class_factory;
mod dll;
mod enum_user_identity;
mod guid;
mod hresult;
mod refcount;
mod user_identity_manager;

pub use class_factory::ClassFactory;
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use enum_user_identity::EnumUserIdentity;
pub use guid::{
    is_equal_guid, Guid, CLSID_USER_IDENTITY_MANAGER, IID_ICLASSFACTORY, IID_IENUM_USER_IDENTITY,
    IID_IUNKNOWN, IID_IUSER_IDENTITY_MANAGER,
};
pub use hresult::{
    succeeded, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    E_USER_CANCELLED, HRESULT, S_FALSE, S_OK,
};
pub use user_identity_manager::UserIdentityManager;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
