use crate::status::{clear_reboot, ERROR_SUCCESS, ERROR_UNSUPPORTED_TYPE};

/// Stands in for `PCINSTALLERINFO_*`, the log callbacks and `VOID *ctx`.
/// The stubs never dereference these, so only whether one is passed matters.
pub struct Opaque;

/// `DriverPackagePreinstallA`: stub, returns `ERROR_SUCCESS`.
pub fn driver_package_preinstall_a(_inf: Option<&[u8]>, _flags: u32) -> u32 {
    ERROR_SUCCESS
}

/// `DriverPackagePreinstallW`: stub, returns `ERROR_SUCCESS`.
pub fn driver_package_preinstall_w(_inf: Option<&[u16]>, _flags: u32) -> u32 {
    ERROR_SUCCESS
}

/// `DriverPackageInstallA`: stub, clears `*reboot` if given, returns `ERROR_SUCCESS`.
pub fn driver_package_install_a(
    _inf: Option<&[u8]>,
    _flags: u32,
    _info: Option<&Opaque>,
    reboot: Option<&mut i32>,
) -> u32 {
    clear_reboot(reboot);
    ERROR_SUCCESS
}

/// `DriverPackageInstallW`: stub, clears `*reboot` if given, returns `ERROR_SUCCESS`.
pub fn driver_package_install_w(
    _inf: Option<&[u16]>,
    _flags: u32,
    _info: Option<&Opaque>,
    reboot: Option<&mut i32>,
) -> u32 {
    clear_reboot(reboot);
    ERROR_SUCCESS
}

/// `DriverPackageUninstallA`: stub, clears `*reboot` if given, returns `ERROR_SUCCESS`.
pub fn driver_package_uninstall_a(
    _inf: Option<&[u8]>,
    _flags: u32,
    _info: Option<&Opaque>,
    reboot: Option<&mut i32>,
) -> u32 {
    clear_reboot(reboot);
    ERROR_SUCCESS
}

/// `DriverPackageUninstallW`: stub, clears `*reboot` if given, returns `ERROR_SUCCESS`.
pub fn driver_package_uninstall_w(
    _inf: Option<&[u16]>,
    _flags: u32,
    _info: Option<&Opaque>,
    reboot: Option<&mut i32>,
) -> u32 {
    clear_reboot(reboot);
    ERROR_SUCCESS
}

/// `DriverPackageGetPathA`: stub, returns `ERROR_UNSUPPORTED_TYPE`, leaves `dest` and `*count` untouched.
pub fn driver_package_get_path_a(
    _inf: Option<&[u8]>,
    _dest: Option<&mut [u8]>,
    _count: Option<&mut u32>,
) -> u32 {
    ERROR_UNSUPPORTED_TYPE
}

/// `DriverPackageGetPathW`: stub, returns `ERROR_UNSUPPORTED_TYPE`, leaves `dest` and `*count` untouched.
pub fn driver_package_get_path_w(
    _inf: Option<&[u16]>,
    _dest: Option<&mut [u16]>,
    _count: Option<&mut u32>,
) -> u32 {
    ERROR_UNSUPPORTED_TYPE
}

/// `DIFXAPISetLogCallbackA`: stub, neither stores nor invokes `cb`.
pub fn difxapi_set_log_callback_a(_cb: Option<&Opaque>, _ctx: Option<&Opaque>) {}

/// `DIFXAPISetLogCallbackW`: stub, neither stores nor invokes `cb`.
pub fn difxapi_set_log_callback_w(_cb: Option<&Opaque>, _ctx: Option<&Opaque>) {}

/// `SetDifxLogCallbackA`: stub, neither stores nor invokes `cb`.
pub fn set_difx_log_callback_a(_cb: Option<&Opaque>, _ctx: Option<&Opaque>) {}

/// `SetDifxLogCallbackW`: stub, neither stores nor invokes `cb`.
pub fn set_difx_log_callback_w(_cb: Option<&Opaque>, _ctx: Option<&Opaque>) {}
