/// How `dlls/wldp/wldp.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function in `wldp.c`, ported as a function in [`crate::lockdown`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Stdcall, Stub};

/// The `wldp.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 29] = [
    ("WldpIsAppApprovedByPolicy", Stub),
    ("WldpIsProductionConfiguration", Stub),
    ("WldpIsWcosProductionConfiguration", Stub),
    ("WldpQueryPolicySettingEnabled2", Stub),
    ("WldpQueryPolicySettingEnabled", Stub),
    ("WldpQueryPolicySettingEnabledInternal", Stub),
    ("WldpQueryWindowsLockdownMode", Stdcall),
    ("WldpQueryWindowsLockdownRestriction", Stub),
    ("WldpResetProductionConfiguration", Stub),
    ("WldpResetWcosProductionConfiguration", Stub),
    ("WldpSetWindowsLockdownRestriction", Stub),
    ("WldpTraceLoggingWDACBlockDialogShown", Stub),
    ("WldpAddDeveloperCertificateForDynamicCodeTrust", Stub),
    ("WldpCheckDeviceEncryptionNotStarted", Stub),
    ("WldpCheckRetailConfiguration", Stub),
    ("WldpCheckWcosDeviceEncryptionSecure", Stub),
    ("WldpDisableDeveloperMode", Stub),
    ("WldpEnableDeveloperMode", Stub),
    ("WldpGetLockdownPolicy", Stdcall),
    ("WldpIsAllowedEntryPoint", Stub),
    ("WldpIsClassInApprovedList", Stub),
    ("WldpIsDebugAllowed", Stub),
    ("WldpIsDynamicCodePolicyEnabled", Stdcall),
    ("WldpIsProductionConfigurationInProc", Stub),
    ("WldpQueryDeviceSecurityInformation", Stub),
    ("WldpQueryDynamicCodeTrust", Stub),
    ("WldpQuerySecurityPolicy", Stub),
    ("WldpSetDynamicCodeTrust", Stub),
    ("WldpSetDynamicCodeTrust2", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
