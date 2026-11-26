//! Port of the portable logic in Wine's `dlls/inseng/inseng_main.c` and `dlls/inseng/inseng.spec`,
//! with the `DllCanUnloadNow` default from `libs/winecrt0/dll_canunload.c`.
//!
//! Ported:
//! - `DllGetClassObject` for `CLSID_InstallEngine`, and the static class factory's `QueryInterface`,
//!   `AddRef`, `Release`, `LockServer` and `CreateInstance`: [`dll_get_class_object`],
//!   [`class_factory_query_interface`], [`class_factory_add_ref`], [`class_factory_release`],
//!   [`class_factory_lock_server`], [`class_factory_create_instance`]
//! - The `InstallEngine` object's `QueryInterface`, `AddRef` and `Release`: [`InstallEngine`]
//! - The 24 `InstallEngine` and `InstallEngine2` methods that return `E_NOTIMPL`: the
//!   `install_engine_*` and `install_engine2_*` functions
//! - `CheckTrustEx`, `DllInstall` and `DllCanUnloadNow`: [`check_trust_ex`], [`dll_install`],
//!   [`dll_can_unload_now`]
//! - The `inseng.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The six `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`: `libs/winecrt0/dll_register.c` forwards them to
//!   `__wine_register_resources`, which enumerates the module's registration resources and hands
//!   them to an `IRegistrar`, so the result depends on the host's registry.
//! - The `FIXME` trace arguments, and the parameters the C bodies never read: `outer` in
//!   `CreateInstance`, the five pointers in `CheckTrustEx`, and `bInstall` and `cmdline` in `DllInstall`.
//! - The `ppv` out-pointer: the ported functions return a `Result` for the success or the `HRESULT`.
//! - The OS heap and the interlocked counters: the count is a plain field, and the allocation
//!   result is passed as a bool.

mod class_factory;
mod com;
mod exports;
mod install_engine;
mod not_implemented;
mod stub;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, dll_get_class_object,
};
pub use com::{
    is_equal_guid, Guid, HResult, CLASS_E_CLASSNOTAVAILABLE, CLSID_INSTALL_ENGINE, E_NOINTERFACE,
    E_NOTIMPL, E_OUTOFMEMORY, IID_CLASS_FACTORY, IID_INSTALL_ENGINE, IID_INSTALL_ENGINE2,
    IID_UNKNOWN, S_FALSE, S_OK,
};
pub use exports::{
    check_trust_ex, dll_can_unload_now, dll_install, export_kind, ExportKind, EXPORTS, TRUE,
};
pub use install_engine::InstallEngine;
pub use not_implemented::{
    install_engine2_get_icif_file, install_engine2_set_local_cif, install_engine_abort,
    install_engine_download_components, install_engine_enum_download_ids,
    install_engine_enum_install_ids, install_engine_get_display_name,
    install_engine_get_engine_status, install_engine_get_sizes, install_engine_install_components,
    install_engine_is_component_installed, install_engine_launch_extra_command,
    install_engine_register_install_engine_callback, install_engine_resume,
    install_engine_set_action, install_engine_set_base_url, install_engine_set_cif_file,
    install_engine_set_download_dir, install_engine_set_hwnd, install_engine_set_install_drive,
    install_engine_set_install_options, install_engine_set_istream, install_engine_suspend,
    install_engine_unregister_install_engine_callback,
};
pub use stub::{
    check_for_version_conflict, check_trust, download_file, get_icif_file_from_file,
    get_icif_rw_file_from_file, purge_download_directory, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
