//! Port of the portable logic in Wine's `dlls/difxapi/main.c`.
//!
//! Every export in this DLL is a stub that logs `FIXME` and returns a fixed value.
//!
//! Ported:
//! - Return codes and `FALSE` from `winerror.h` and `windef.h`: [`ERROR_SUCCESS`],
//!   [`ERROR_UNSUPPORTED_TYPE`], [`FALSE`]
//! - `if (reboot) *reboot = FALSE;`, shared by the install and uninstall exports: [`clear_reboot`]
//! - Stub exports modelled by return value and out-parameter effect:
//!   `DriverPackagePreinstall{A,W}`, `DriverPackageInstall{A,W}`,
//!   `DriverPackageUninstall{A,W}`, `DriverPackageGetPath{A,W}`,
//!   `DIFXAPISetLogCallback{A,W}`, `SetDifxLogCallback{A,W}`
//!
//! Skipped:
//! - `wine_dbgstr_a` / `wine_dbgstr_w` and `FIXME` tracing: debug output only.
//! - Reading the `inf` strings, the `INSTALLERINFO` fields and the `DRIVER_PACKAGE_*`
//!   flag bits: the stubs never read them.
//! - Storing or invoking log callbacks: the stubs never do either, so `cb` and `ctx`
//!   are modelled as [`Opaque`] pointers.

mod exports;
mod status;

pub use exports::{
    difxapi_set_log_callback_a, difxapi_set_log_callback_w, driver_package_get_path_a,
    driver_package_get_path_w, driver_package_install_a, driver_package_install_w,
    driver_package_preinstall_a, driver_package_preinstall_w, driver_package_uninstall_a,
    driver_package_uninstall_w, set_difx_log_callback_a, set_difx_log_callback_w, Opaque,
};
pub use status::{clear_reboot, ERROR_SUCCESS, ERROR_UNSUPPORTED_TYPE, FALSE};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
