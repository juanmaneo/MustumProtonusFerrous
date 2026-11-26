//! Port of the portable logic in Wine's `dlls/newdev/main.c` and `dlls/newdev/newdev.spec`.
//!
//! Ported:
//! - `InstallNewDevice`, `InstallSelectedDriver`, `DiInstallDriverA` and `DiInstallDriverW`, which
//!   log a `FIXME` and return `TRUE`: [`install_new_device`], [`install_selected_driver`],
//!   [`di_install_driver_a`] and [`di_install_driver_w`]
//! - the ANSI-to-wide thunk of `UpdateDriverForPlugAndPlayDevicesA`:
//!   [`update_driver_for_plug_and_play_devices_a`], with the buffer sizes [`MAX_DEVICE_ID_LEN`]
//!   and [`MAX_PATH`]
//! - the multi-string hardware ID match used by `UpdateDriverForPlugAndPlayDevicesW`:
//!   [`hardware_id_matches`]
//! - the `newdev.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the twelve `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   one helper per stub export, such as [`di_install_device`], plus [`StubException`] and
//!   [`wine_spec_unimplemented_stub`]
//!
//! Skipped:
//! - `UpdateDriverForPlugAndPlayDevicesW`'s device loop, which runs against SetupDi. The thunk
//!   takes that function as a callback, and the caller supplies it.
//! - The `HWND parent` argument of the thunk. The C body forwards it unchanged to the W function,
//!   so the caller's callback captures it instead.
//! - The `FIXME` and `TRACE` output, and the parameters that the `FIXME` stubs never read.
//! - Locale-dependent `CP_ACP` conversion. Bytes widen 1:1 to the code unit of the same value.
//!   Only ASCII is oracle-checked.
//! - `NULL` `hardware_id` and `inf_path` pointers. The C body ignores the failed conversion and
//!   reads an uninitialised buffer, so the port takes byte slices instead.
//! - Bug-compatibility where the C reads past its buffer: an oversized ANSI input leaves the W
//!   buffer without a NUL, and an unterminated multi-string is read up to the slice end. The port
//!   keeps both NUL-terminated and bounded.

mod exports;
mod install;
mod stub;
mod update;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use install::{
    di_install_driver_a, di_install_driver_w, install_new_device, install_selected_driver,
};
pub use stub::{
    device_internet_setting_ui_w, di_install_device, di_rollback_driver, di_show_update_device,
    di_uninstall_device, install_windows_update_driver, p_di_device_install_action_w,
    p_di_device_install_notification_w, p_di_do_device_install_as_admin,
    p_di_do_finish_install_as_admin, p_di_do_null_driver_install, set_internet_policies,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use update::{
    hardware_id_matches, update_driver_for_plug_and_play_devices_a, MAX_DEVICE_ID_LEN, MAX_PATH,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
