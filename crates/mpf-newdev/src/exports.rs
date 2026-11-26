/// How `dlls/newdev/newdev.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real C function. The ones with OS-free logic are ported in this crate.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `newdev.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 18] = [
    ("DeviceInternetSettingUiW", ExportKind::Stub),
    ("DiInstallDevice", ExportKind::Stub),
    ("DiInstallDriverA", ExportKind::Stdcall),
    ("DiInstallDriverW", ExportKind::Stdcall),
    ("DiRollbackDriver", ExportKind::Stub),
    ("DiShowUpdateDevice", ExportKind::Stub),
    ("DiUninstallDevice", ExportKind::Stub),
    ("InstallNewDevice", ExportKind::Stdcall),
    ("InstallSelectedDriver", ExportKind::Stdcall),
    ("InstallWindowsUpdateDriver", ExportKind::Stub),
    ("SetInternetPolicies", ExportKind::Stub),
    ("UpdateDriverForPlugAndPlayDevicesA", ExportKind::Stdcall),
    ("UpdateDriverForPlugAndPlayDevicesW", ExportKind::Stdcall),
    ("pDiDeviceInstallActionW", ExportKind::Stub),
    ("pDiDeviceInstallNotificationW", ExportKind::Stub),
    ("pDiDoDeviceInstallAsAdmin", ExportKind::Stub),
    ("pDiDoFinishInstallAsAdmin", ExportKind::Stub),
    ("pDiDoNullDriverInstall", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
