//! Port of the portable logic in Wine's `dlls/appxdeploymentclient/main.c` and `package.c`.
//!
//! Ported:
//! - `DllGetClassObject` and the class-id match in `DllGetActivationFactory`
//!   (`wcscmp` against `Windows.Management.Deployment.PackageManager`): `dll_get_class_object`,
//!   `dll_get_activation_factory`
//! - `QueryInterface` IID tables of `IPackageManager`/`IPackageManager2` and `IActivationFactory`:
//!   `Iid`, `View`, `PackageManager::query_interface`, `ActivationFactory::query_interface`
//! - `AddRef` / `Release` with the `LONG` count returned as `ULONG` and zero meaning `free`:
//!   `PackageManager::add_ref`, `PackageManager::release`, `ActivationFactory::add_ref`,
//!   `ActivationFactory::release`
//! - `factory_ActivateInstance` success and `E_OUTOFMEMORY` paths: `ActivationFactory::activate_instance`
//! - The NULL argument check in `package_manager_FindPackagesByNamePublisher`:
//!   `find_packages_by_name_publisher`
//! - `HRESULT` values from `winerror.h`, and `SUCCEEDED`: `hresult`
//!
//! Skipped:
//! - `WindowsGetStringRawBuffer` HSTRING storage: the class id is a UTF-16 slice, and `None` is a NULL HSTRING.
//! - COM vtable dispatch (`IActivationFactory_QueryInterface`, `IInspectable_*`) and the
//!   `DEFINE_IINSPECTABLE` delegation macro: the Rust methods are called directly.
//! - `calloc` and `free`: the OOM outcome is an input, and dropping the object replaces `free`.
//! - `FIXME`/`TRACE` output and `debugstr_*` helpers.
//! - `DllGetClassObject` arguments (`clsid`, `riid`, `out`): the C body ignores them.
//! - The `E_NOTIMPL` stubs of `IPackageManager`, `IPackageManager2`, and `IActivationFactory`
//!   are modelled as `StubExport` values rather than ported logic.

mod dll;
mod factory;
mod hresult;
mod package_manager;
mod query;
mod refcount;
mod stub;

pub use dll::{dll_get_activation_factory, dll_get_class_object, PACKAGE_MANAGER_CLASS};
pub use factory::ActivationFactory;
pub use hresult::{
    succeeded, CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    HRESULT, S_OK,
};
pub use package_manager::{find_packages_by_name_publisher, PackageManager};
pub use query::{Iid, View};
pub use stub::StubExport;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
