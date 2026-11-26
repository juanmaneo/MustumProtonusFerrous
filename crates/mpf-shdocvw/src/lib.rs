//! Port of the portable logic in Wine's `dlls/shdocvw/shdocvw_main.c`, `dlls/shdocvw/shlinstobj.c`
//! and `dlls/shdocvw/shdocvw.spec`.
//!
//! Ported:
//! - Fixed-value and no-op exports from `shdocvw_main.c`: [`win_list_init`],
//!   [`run_install_uninstall_stubs`], [`run_install_uninstall_stubs2`],
//!   [`set_query_net_session_count`], [`ie_parse_display_name_with_bcw`], [`sh_restricted2_w`],
//!   [`sh_restricted2_a`], [`reset_profile_sharing`], [`install_reg_run_dll`],
//!   [`do_file_download`], [`do_organize_fav_dlg_w`], [`do_organize_fav_dlg`], with the
//!   `HRESULT` and error values from `include/winerror.h`
//! - `ImportPrivacySettings`: [`import_privacy_settings`]
//! - `DllCanUnloadNow` and `DllInstall`: [`dll_can_unload_now`], [`dll_install`]
//! - `DllGetClassObject` routing: [`dll_get_class_object_route`], [`ClassRoute`], [`Guid`] and the
//!   CLSIDs it compares
//! - `IPropertyBag` `QueryInterface` and `Write` from `shlinstobj.c`:
//!   [`property_bag_query_interface`], [`property_bag_write`]
//! - Ordinal forwards to shlwapi and shell32: [`stop_watch_mode_forward`],
//!   [`stop_watch_flush_forward`], [`stop_watch_a_forward`], [`stop_watch_w_forward`],
//!   [`shell_dde_init`]
//! - `shdocvw.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`stub_for_export`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - `DllMain`, `IEWinMain` and the `ieframe.dll` and `shell32.dll` loads: process state and
//!   `ExitProcess`, which need the OS.
//! - The `static` caches of the resolved pointers and of the module handles. [`stop_watch_a_forward`]
//!   and the other forwards take the resolver as an argument instead.
//! - `SHDOCVW_refCount` and the `SHDOCVW_GetShellInstanceObjectClassObject` lookup, which reads the
//!   registry and calls `CoCreateInstance`. [`dll_can_unload_now`] takes the count as an argument.
//! - The registry-backed `RegistryPropertyBag::Read` and the `InstanceObjectFactory`.
//! - The ANSI-to-wide conversion in `SHRestricted2A` and `DoOrganizeFavDlg`. Its only result is
//!   passed to a function whose return value does not depend on it.
//! - The arguments the C bodies never read, such as the `FIXME` trace inputs, the `arg` of
//!   `RunInstallUninstallStubs2`, and the file name of `ImportPrivacySettings`.
//! - The `*ppv` write of `QueryInterface`. [`property_bag_query_interface`] reports only whether
//!   the out-pointer was given.
//! - `DllGetVersion`, `DllRegisterServer` and `DllUnregisterServer`: the spec exports them, but
//!   `dlls/shdocvw` has no definition, so they are `NotPorted`.

mod com;
mod exports;
mod fixed;
mod forward;
mod stub;

pub use com::{
    dll_can_unload_now, dll_get_class_object_route, dll_install, property_bag_query_interface,
    property_bag_write, ClassRoute, Guid, CLSID_CURL_HISTORY, CLSID_INTERNET_SHORTCUT,
    CLSID_TASKBAR_LIST, CLSID_WEB_BROWSER, CLSID_WEB_BROWSER_V1, IID_IPROPERTY_BAG, IID_IUNKNOWN,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use fixed::{
    do_file_download, do_organize_fav_dlg, do_organize_fav_dlg_w, ie_parse_display_name_with_bcw,
    import_privacy_settings, install_reg_run_dll, reset_profile_sharing,
    run_install_uninstall_stubs, run_install_uninstall_stubs2, set_query_net_session_count,
    sh_restricted2_a, sh_restricted2_w, win_list_init, ERROR_CALL_NOT_IMPLEMENTED, E_FAIL,
    E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, S_FALSE, S_OK,
};
pub use forward::{
    shell_dde_init, stop_watch_a_forward, stop_watch_flush_forward, stop_watch_mode_forward,
    stop_watch_w_forward, SHELL32_SHELL_DDE_INIT, SHLWAPI_STOP_WATCH_A, SHLWAPI_STOP_WATCH_FLUSH,
    SHLWAPI_STOP_WATCH_MODE, SHLWAPI_STOP_WATCH_W,
};
pub use stub::{
    stub_for_export, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
