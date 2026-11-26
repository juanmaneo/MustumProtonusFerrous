/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = winsta.dll` in `dlls/winsta/Makefile.in`.
pub const WINSTA_DLL_NAME: &str = "winsta.dll";

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
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/winsta/winsta.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WINSTA_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `winsta.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    logon_id_from_win_station_name_a => "LogonIdFromWinStationNameA",
    logon_id_from_win_station_name_w => "LogonIdFromWinStationNameW",
    remote_assistance_prepare_system_restore => "RemoteAssistancePrepareSystemRestore",
    server_get_internet_connector_status => "ServerGetInternetConnectorStatus",
    server_licensing_close => "ServerLicensingClose",
    server_licensing_deactivate_current_policy => "ServerLicensingDeactivateCurrentPolicy",
    server_licensing_free_policy_information => "ServerLicensingFreePolicyInformation",
    server_licensing_get_available_policy_ids => "ServerLicensingGetAvailablePolicyIds",
    server_licensing_get_policy => "ServerLicensingGetPolicy",
    server_licensing_get_policy_information_a => "ServerLicensingGetPolicyInformationA",
    server_licensing_get_policy_information_w => "ServerLicensingGetPolicyInformationW",
    server_licensing_load_policy => "ServerLicensingLoadPolicy",
    server_licensing_open_a => "ServerLicensingOpenA",
    server_licensing_open_w => "ServerLicensingOpenW",
    server_licensing_set_policy => "ServerLicensingSetPolicy",
    server_licensing_unload_policy => "ServerLicensingUnloadPolicy",
    server_query_inet_connector_information_a => "ServerQueryInetConnectorInformationA",
    server_query_inet_connector_information_w => "ServerQueryInetConnectorInformationW",
    server_set_internet_connector_status => "ServerSetInternetConnectorStatus",
    win_station_activate_license => "WinStationActivateLicense",
    win_station_auto_reconnect => "WinStationAutoReconnect",
    win_station_broadcast_system_message => "WinStationBroadcastSystemMessage",
    win_station_check_loop_back => "WinStationCheckLoopBack",
    win_station_close_server => "WinStationCloseServer",
    win_station_connect_a => "WinStationConnectA",
    win_station_connect_callback => "WinStationConnectCallback",
    win_station_connect_w => "WinStationConnectW",
    win_station_disconnect => "WinStationDisconnect",
    win_station_enumerate_a => "WinStationEnumerateA",
    win_station_enumerate_licenses => "WinStationEnumerateLicenses",
    win_station_enumerate_processes => "WinStationEnumerateProcesses",
    win_station_enumerate_indexed_a => "WinStationEnumerate_IndexedA",
    win_station_enumerate_indexed_w => "WinStationEnumerate_IndexedW",
    win_station_free_gap_memory => "WinStationFreeGAPMemory",
    win_station_free_memory => "WinStationFreeMemory",
    win_station_generate_license => "WinStationGenerateLicense",
    win_station_get_lan_adapter_name_a => "WinStationGetLanAdapterNameA",
    win_station_get_lan_adapter_name_w => "WinStationGetLanAdapterNameW",
    win_station_get_machine_policy => "WinStationGetMachinePolicy",
    win_station_get_term_srv_counters_value => "WinStationGetTermSrvCountersValue",
    win_station_install_license => "WinStationInstallLicense",
    win_station_is_help_assistant_session => "WinStationIsHelpAssistantSession",
    win_station_name_from_logon_id_a => "WinStationNameFromLogonIdA",
    win_station_name_from_logon_id_w => "WinStationNameFromLogonIdW",
    win_station_ntsd_debug => "WinStationNtsdDebug",
    win_station_open_server_a => "WinStationOpenServerA",
    win_station_open_server_w => "WinStationOpenServerW",
    win_station_query_license => "WinStationQueryLicense",
    win_station_query_logon_credentials_w => "WinStationQueryLogonCredentialsW",
    win_station_query_update_required => "WinStationQueryUpdateRequired",
    win_station_remove_license => "WinStationRemoveLicense",
    win_station_rename_a => "WinStationRenameA",
    win_station_rename_w => "WinStationRenameW",
    win_station_reset => "WinStationReset",
    win_station_send_message_a => "WinStationSendMessageA",
    win_station_send_message_w => "WinStationSendMessageW",
    win_station_send_window_message => "WinStationSendWindowMessage",
    win_station_server_ping => "WinStationServerPing",
    win_station_set_information_a => "WinStationSetInformationA",
    win_station_set_information_w => "WinStationSetInformationW",
    win_station_set_pool_count => "WinStationSetPoolCount",
    win_station_shadow => "WinStationShadow",
    win_station_shadow_stop => "WinStationShadowStop",
    win_station_shutdown_system => "WinStationShutdownSystem",
    win_station_terminate_process => "WinStationTerminateProcess",
    win_station_wait_system_event => "WinStationWaitSystemEvent",
    _nw_logon_query_admin => "_NWLogonQueryAdmin",
    _nw_logon_set_admin => "_NWLogonSetAdmin",
    _win_station_annoyance_popup => "_WinStationAnnoyancePopup",
    _win_station_beep_open => "_WinStationBeepOpen",
    _win_station_break_point => "_WinStationBreakPoint",
    _win_station_callback => "_WinStationCallback",
    _win_station_check_for_application_name => "_WinStationCheckForApplicationName",
    _win_station_fus_can_remote_user_disconnect => "_WinStationFUSCanRemoteUserDisconnect",
    _win_station_get_application_info => "_WinStationGetApplicationInfo",
    _win_station_notify_disconnect_pipe => "_WinStationNotifyDisconnectPipe",
    _win_station_notify_logoff => "_WinStationNotifyLogoff",
    _win_station_notify_logon => "_WinStationNotifyLogon",
    _win_station_notify_new_session => "_WinStationNotifyNewSession",
    _win_station_re_initialize_security => "_WinStationReInitializeSecurity",
    _win_station_read_registry => "_WinStationReadRegistry",
    _win_station_shadow_target => "_WinStationShadowTarget",
    _win_station_shadow_target_setup => "_WinStationShadowTargetSetup",
    _win_station_update_client_cached_credentials => "_WinStationUpdateClientCachedCredentials",
    _win_station_update_settings => "_WinStationUpdateSettings",
    _win_station_update_user_config => "_WinStationUpdateUserConfig",
    _win_station_wait_for_connect => "_WinStationWaitForConnect",
}
