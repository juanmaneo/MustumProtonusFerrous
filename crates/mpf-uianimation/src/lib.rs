//! Port of the portable logic in Wine's `dlls/uianimation/main.c` and `dlls/uianimation/uianimation.spec`,
//! plus the `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer` defaults from `libs/winecrt0`.
//!
//! Ported:
//! - `DllGetClassObject` class dispatch: [`dll_get_class_object`], [`class_for_clsid`], [`ClassKind`]
//!   and the `CLSID_*` and `IID_*` values in `include/uianimation.idl`
//! - class factory methods: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`], [`class_factory_lock_server`], [`class_factory_create_instance`]
//! - the `QueryInterface` that the factories and created objects share: [`query_interface`],
//!   [`Out`] for the `void **obj` out-parameter
//! - `DllCanUnloadNow`, [`dll_register_server`], [`dll_unregister_server`] and the [`EXPORTS`] table
//!
//! Skipped:
//! - The COM method bodies of the storyboard, variable, manager, timer, transition factory and
//!   transition library vtables. Most return `E_NOTIMPL` or `S_OK` after a `FIXME`.
//! - Object allocation, reference counting and freeing. A creator frees only the object of a
//!   failed query, and the caller never sees that object.
//! - The `outer` argument of `CreateInstance`, which the C code ignores.
//! - The `__wine_register_resources` and `__wine_unregister_resources` work, which is passed in as a closure.
//! - The `TRACE` and `FIXME` output.

mod class;
mod com;
mod dll;
mod factory;

pub use class::{
    class_for_clsid, class_iid, ClassKind, CLSID_UIANIMATION_MANAGER, CLSID_UIANIMATION_TIMER,
    CLSID_UIANIMATION_TRANSITION_FACTORY, CLSID_UIANIMATION_TRANSITION_LIBRARY,
    IID_IUIANIMATION_MANAGER, IID_IUIANIMATION_TIMER, IID_IUIANIMATION_TRANSITION_FACTORY,
    IID_IUIANIMATION_TRANSITION_LIBRARY,
};
pub use com::{
    query_interface, Guid, Out, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, HRESULT,
    IID_ICLASSFACTORY, IID_IUNKNOWN, S_FALSE, S_OK,
};
pub use dll::{
    dll_can_unload_now, dll_get_class_object, dll_register_server, dll_unregister_server,
    is_export, EXPORTS,
};
pub use factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
