//! `@ stub` exports of `dhcpcsvc.spec`. Each one resolves to `__wine_spec_unimplemented_stub`,
//! which raises `EXCEPTION_WINE_STUB` with the module and export names and never returns.

/// `EXCEPTION_WINE_STUB`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 1;
/// The module name that the spec file passes to its stubs.
pub const MODULE_NAME: &str = "dhcpcsvc.dll";

/// Exports declared `@ stub` in `dhcpcsvc.spec`.
pub const STUB_EXPORTS: &[&str] = &[
    "DhcpAcquireParameters",
    "DhcpAcquireParametersByBroadcast",
    "DhcpDeRegisterConnectionStateNotification",
    "DhcpDeRegisterOptions",
    "DhcpDeRegisterParamChange",
    "DhcpDelPersistentRequestParams",
    "DhcpEnumClasses",
    "DhcpFallbackRefreshParams",
    "DhcpFreeMem",
    "DhcpGetDhcpServicedConnctions",
    "DhcpGetNotificationStatus",
    "DhcpHandlePnPEvent",
    "DhcpLeaseIpAddress",
    "DhcpLeaseIpAddressEx",
    "DhcpNotifyConfigChange",
    "DhcpNotifyConfigChangeEx",
    "DhcpNotifyMediaReconnected",
    "DhcpOpenGlobalEvent",
    "DhcpPersistentRequestParams",
    "DhcpQueryHWInfo",
    "DhcpRegisterConnectionStateNotification",
    "DhcpRegisterOptions",
    "DhcpRegisterParamChange",
    "DhcpReleaseIpAddressLease",
    "DhcpReleaseIpAddressLeaseEx",
    "DhcpReleaseParameters",
    "DhcpRemoveDNSRegistration",
    "DhcpRenewIpAddressLease",
    "DhcpRenewIpAddressLeaseEx",
    "DhcpRequestCachedParams",
    "DhcpRequestOptions",
    "DhcpSetMSFTVendorSpecificOptions",
    "DhcpStaticRefreshParams",
    "DhcpUndoRequestParams",
    "McastApiCleanup",
    "McastApiStartup",
    "McastEnumerateScopes",
    "McastGenUID",
    "McastReleaseAddress",
    "McastRenewAddress",
    "McastRequestAddress",
    "ServiceMain",
];

/// The exception that `__wine_spec_unimplemented_stub` raises.
#[derive(Debug, PartialEq, Eq)]
pub struct StubFault {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub(module, function)`.
pub fn unimplemented_stub(module: &'static str, function: &'static str) -> StubFault {
    StubFault {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// The fault for a call to `name`, or `None` when `name` is not a stub export. The implemented
/// exports (`DhcpCApiInitialize`, `DhcpCApiCleanup`, `DhcpRequestParams`) are not stubs.
pub fn call_stub_export(name: &str) -> Option<StubFault> {
    STUB_EXPORTS
        .iter()
        .find(|&&export| export == name)
        .map(|&export| unimplemented_stub(MODULE_NAME, export))
}
