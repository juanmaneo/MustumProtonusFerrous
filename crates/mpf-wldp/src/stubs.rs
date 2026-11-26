/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wldp.dll` in `dlls/wldp/Makefile.in`.
pub const WLDP_DLL_NAME: &str = "wldp.dll";

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

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/wldp/wldp.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WLDP_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `wldp.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    wldp_is_app_approved_by_policy => "WldpIsAppApprovedByPolicy",
    wldp_is_production_configuration => "WldpIsProductionConfiguration",
    wldp_is_wcos_production_configuration => "WldpIsWcosProductionConfiguration",
    wldp_query_policy_setting_enabled2 => "WldpQueryPolicySettingEnabled2",
    wldp_query_policy_setting_enabled => "WldpQueryPolicySettingEnabled",
    wldp_query_policy_setting_enabled_internal => "WldpQueryPolicySettingEnabledInternal",
    wldp_query_windows_lockdown_restriction => "WldpQueryWindowsLockdownRestriction",
    wldp_reset_production_configuration => "WldpResetProductionConfiguration",
    wldp_reset_wcos_production_configuration => "WldpResetWcosProductionConfiguration",
    wldp_set_windows_lockdown_restriction => "WldpSetWindowsLockdownRestriction",
    wldp_trace_logging_wdac_block_dialog_shown => "WldpTraceLoggingWDACBlockDialogShown",
    wldp_add_developer_certificate_for_dynamic_code_trust => "WldpAddDeveloperCertificateForDynamicCodeTrust",
    wldp_check_device_encryption_not_started => "WldpCheckDeviceEncryptionNotStarted",
    wldp_check_retail_configuration => "WldpCheckRetailConfiguration",
    wldp_check_wcos_device_encryption_secure => "WldpCheckWcosDeviceEncryptionSecure",
    wldp_disable_developer_mode => "WldpDisableDeveloperMode",
    wldp_enable_developer_mode => "WldpEnableDeveloperMode",
    wldp_is_allowed_entry_point => "WldpIsAllowedEntryPoint",
    wldp_is_class_in_approved_list => "WldpIsClassInApprovedList",
    wldp_is_debug_allowed => "WldpIsDebugAllowed",
    wldp_is_production_configuration_in_proc => "WldpIsProductionConfigurationInProc",
    wldp_query_device_security_information => "WldpQueryDeviceSecurityInformation",
    wldp_query_dynamic_code_trust => "WldpQueryDynamicCodeTrust",
    wldp_query_security_policy => "WldpQuerySecurityPolicy",
    wldp_set_dynamic_code_trust => "WldpSetDynamicCodeTrust",
    wldp_set_dynamic_code_trust2 => "WldpSetDynamicCodeTrust2",
}
