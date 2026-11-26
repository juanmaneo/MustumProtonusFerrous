//! Port of the portable logic in Wine's `dlls/sti/sti.c`, `dlls/sti/sti_main.c` and
//! `dlls/sti/sti.spec`, plus `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `sti.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `StiCreateInstanceW` object model: [`sti_create_instance_w`], [`StillImage`], [`Face`],
//!   [`OuterUnknown`]. This covers `Internal_QueryInterface` (IUnknown and IStillImageW
//!   identification, E_NOINTERFACE), reference counting, and the `IStillImageW` forwarding
//!   to the outer object.
//! - `StiCreateInstanceA` (`STG_E_UNIMPLEMENTEDFUNCTION`): [`sti_create_instance_a`]
//! - The class factory for `CLSID_Sti`: [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_lock_server`],
//!   [`class_factory_create_instance`] (the `sti_create` dispatch, including
//!   `CLASS_E_NOAGGREGATION`)
//! - `DllGetClassObject`, `DllRegisterServer`, `DllUnregisterServer` and `DllCanUnloadNow`:
//!   [`dll_get_class_object`], [`dll_register_server`], [`dll_unregister_server`],
//!   [`dll_can_unload_now`]
//! - The `IStillImageW` methods that return `E_NOTIMPL`: the functions in [`unimplemented`]
//! - `RegisterLaunchApplication` and `UnregisterLaunchApplication` control flow, the
//!   `REG_SZ` data layout and `HRESULT_FROM_WIN32`: [`register_launch_application`],
//!   [`unregister_launch_application`], [`Registry`]
//!
//! Skipped:
//! - `DllMain`, `IStillImage_Initialize`'s `hinst` and `dwVersion` (ignored by the C body), and
//!   the `TRACE` and `FIXME` output.
//! - The `HeapAlloc` and `GetProcessHeap` calls, which are replaced by an `alloc_ok` flag.
//! - The midl-generated proxy-stub (`STI_DllGetClassObject`, `STI_DllRegisterServer`,
//!   `STI_DllUnregisterServer`). The port takes the forwarded result as a closure.
//! - COM pointers. A query returns a [`Face`] or an `Option<Face>` in place of `*out`. A
//!   forwarded query to an external outer object returns only its HRESULT.
//! - The `ERROR_*` and `HKEY` values that the registry calls return, and the registry
//!   handle, which the [`Registry`] implementation owns.
//! - The `@ stub` model: `sti.spec` has no `@ stub` entries, so `__wine_spec_unimplemented_stub`
//!   is not used.

mod class_factory;
mod com;
mod dll;
mod exports;
mod registry;
mod still_image;
mod unimplemented;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release,
};
pub use com::{
    hresult_from_win32, Guid, Hresult, CLASS_E_NOAGGREGATION, CLSID_STI, E_NOINTERFACE, E_NOTIMPL,
    E_OUTOFMEMORY, IID_ICLASSFACTORY, IID_ISTILLIMAGEA, IID_ISTILLIMAGEW, IID_IUNKNOWN,
    STG_E_UNIMPLEMENTEDFUNCTION, S_FALSE, S_OK,
};
pub use dll::{
    dll_can_unload_now, dll_get_class_object, dll_register_server, dll_unregister_server,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use registry::{
    launch_command_data, register_launch_application, unregister_launch_application, Registry,
    REGISTERED_APPS_LAUNCH_PATH,
};
pub use still_image::{
    sti_create_instance_a, sti_create_instance_w, Face, OuterUnknown, StillImage,
};
pub use unimplemented::{
    create_device, enable_hw_notifications, get_device_info, get_device_list, get_device_value,
    get_hw_notification_state, get_sti_launch_information, launch_application_for_device,
    refresh_device_bus, set_device_value, setup_device_parameters, write_to_error_log,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
