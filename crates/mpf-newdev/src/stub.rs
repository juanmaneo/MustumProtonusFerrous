/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = newdev.dll` in `dlls/newdev/Makefile.in`.
const NEWDEV_DLL_NAME: &str = "newdev.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `DeviceInternetSettingUiW`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn device_internet_setting_ui_w() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "DeviceInternetSettingUiW")
}

/// `DiInstallDevice`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn di_install_device() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "DiInstallDevice")
}

/// `DiRollbackDriver`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn di_rollback_driver() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "DiRollbackDriver")
}

/// `DiShowUpdateDevice`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn di_show_update_device() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "DiShowUpdateDevice")
}

/// `DiUninstallDevice`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn di_uninstall_device() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "DiUninstallDevice")
}

/// `InstallWindowsUpdateDriver`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn install_windows_update_driver() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "InstallWindowsUpdateDriver")
}

/// `SetInternetPolicies`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn set_internet_policies() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "SetInternetPolicies")
}

/// `pDiDeviceInstallActionW`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn p_di_device_install_action_w() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "pDiDeviceInstallActionW")
}

/// `pDiDeviceInstallNotificationW`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn p_di_device_install_notification_w() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "pDiDeviceInstallNotificationW")
}

/// `pDiDoDeviceInstallAsAdmin`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn p_di_do_device_install_as_admin() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "pDiDoDeviceInstallAsAdmin")
}

/// `pDiDoFinishInstallAsAdmin`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn p_di_do_finish_install_as_admin() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "pDiDoFinishInstallAsAdmin")
}

/// `pDiDoNullDriverInstall`, exported as `@ stub` in `dlls/newdev/newdev.spec`.
pub fn p_di_do_null_driver_install() -> StubException {
    wine_spec_unimplemented_stub(NEWDEV_DLL_NAME, "pDiDoNullDriverInstall")
}
