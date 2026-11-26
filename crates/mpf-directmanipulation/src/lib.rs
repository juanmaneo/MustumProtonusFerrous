//! Port of the portable logic in Wine's `dlls/directmanipulation/directmanipulation.c`.
//!
//! Ported:
//! - `QueryInterface` routing and `AddRef`/`Release` counting for the manager, update manager,
//!   primary content (whose Content interface shares its count), viewport, and compositor
//!   (whose FrameInfoProvider interface shares its count): [`query_interface`], [`Manager`],
//!   [`UpdateManager`], [`PrimaryContext`], [`Viewport`], [`Compositor`]
//! - `GetUpdateManager`, `CreateViewport`, `GetPrimaryContent` and compositor `SetUpdateManager`
//!   (including its `E_INVALIDARG` check and its overwrite that never releases the old manager):
//!   [`Manager::get_update_manager`], [`Manager::create_viewport`],
//!   [`Viewport::get_primary_content`], [`Compositor::set_update_manager`]
//! - `DirectManipulation_CreateInstance` and `DirectCompositor_CreateInstance`, where the object
//!   survives only if `QueryInterface` succeeds: [`create_manager_instance`],
//!   [`create_compositor_instance`]
//! - `DllGetClassObject` CLSID routing and the class factory's `QueryInterface`, constant
//!   `AddRef`/`Release` and `LockServer`: [`dll_get_class_object`], [`class_factory_query_interface`]
//! - GUID and CLSID classification from `directmanipulation.idl`: [`classify_iid`], [`classify_clsid`]
//! - Return codes of the 61 exports that only log a FIXME (57 `E_NOTIMPL`, 4 `S_OK`):
//!   [`stub_return`]
//!
//! Skipped:
//! - Vtable dispatch, `ppv` out-parameters and `IUnknown_*` pointer plumbing: Rust methods replace them.
//! - `malloc`/`free` and the `E_OUTOFMEMORY` path of every `create_*` helper.
//! - Behaviour of the stubs (wait handles, frame info, input, content and viewport state): only their
//!   return codes are modelled.

mod class;
mod compositor;
mod guid;
mod hresult;
mod instance;
mod manager;
mod primary;
mod query;
mod refcount;
mod stubs;
mod update_manager;
mod viewport;

pub use class::{
    class_factory_add_ref, class_factory_lock_server, class_factory_query_interface,
    class_factory_release, dll_get_class_object, Factory,
};
pub use compositor::Compositor;
pub use guid::{
    classify_clsid, classify_iid, Clsid, Guid, Iid, CLSID_DCOMP_COMPOSITOR, CLSID_MANAGER,
    CLSID_SHARED_MANAGER, IID_CLASS_FACTORY, IID_COMPOSITOR, IID_COMPOSITOR2, IID_CONTENT,
    IID_FRAME_INFO_PROVIDER, IID_MANAGER, IID_MANAGER2, IID_PRIMARY_CONTENT, IID_UNKNOWN,
    IID_UPDATE_MANAGER, IID_VIEWPORT, IID_VIEWPORT2,
};
pub use hresult::{
    succeeded, HResult, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL,
    S_OK,
};
pub use instance::{create_compositor_instance, create_manager_instance};
pub use manager::Manager;
pub use primary::PrimaryContext;
pub use query::{query_counted, query_interface, Iface, Object};
pub use refcount::RefCount;
pub use stubs::stub_return;
pub use update_manager::UpdateManager;
pub use viewport::Viewport;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
