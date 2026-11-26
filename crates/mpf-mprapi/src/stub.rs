/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mprapi.dll` in `dlls/mprapi/Makefile.in`.
const MPRAPI_DLL_NAME: &str = "mprapi.dll";

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

/// `CompressPhoneNumber`, a `@ stub` export.
pub fn compress_phone_number() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "CompressPhoneNumber")
}

/// `MprAdminBufferFree`, a `@ stub` export.
pub fn mpr_admin_buffer_free() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminBufferFree")
}

/// `MprAdminConnectionClearStats`, a `@ stub` export.
pub fn mpr_admin_connection_clear_stats() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminConnectionClearStats")
}

/// `MprAdminConnectionEnum`, a `@ stub` export.
pub fn mpr_admin_connection_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminConnectionEnum")
}

/// `MprAdminConnectionGetInfo`, a `@ stub` export.
pub fn mpr_admin_connection_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminConnectionGetInfo")
}

/// `MprAdminDeregisterConnectionNotification`, a `@ stub` export.
pub fn mpr_admin_deregister_connection_notification() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminDeregisterConnectionNotification")
}

/// `MprAdminDeviceEnum`, a `@ stub` export.
pub fn mpr_admin_device_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminDeviceEnum")
}

/// `MprAdminEstablishDomainRasServer`, a `@ stub` export.
pub fn mpr_admin_establish_domain_ras_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminEstablishDomainRasServer")
}

/// `MprAdminGetPDCServer`, a `@ stub` export.
pub fn mpr_admin_get_pdc_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminGetPDCServer")
}

/// `MprAdminInterfaceConnect`, a `@ stub` export.
pub fn mpr_admin_interface_connect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceConnect")
}

/// `MprAdminInterfaceCreate`, a `@ stub` export.
pub fn mpr_admin_interface_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceCreate")
}

/// `MprAdminInterfaceDelete`, a `@ stub` export.
pub fn mpr_admin_interface_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceDelete")
}

/// `MprAdminInterfaceDeviceGetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_device_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceDeviceGetInfo")
}

/// `MprAdminInterfaceDeviceSetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_device_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceDeviceSetInfo")
}

/// `MprAdminInterfaceDisconnect`, a `@ stub` export.
pub fn mpr_admin_interface_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceDisconnect")
}

/// `MprAdminInterfaceEnum`, a `@ stub` export.
pub fn mpr_admin_interface_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceEnum")
}

/// `MprAdminInterfaceGetCredentials`, a `@ stub` export.
pub fn mpr_admin_interface_get_credentials() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceGetCredentials")
}

/// `MprAdminInterfaceGetCredentialsEx`, a `@ stub` export.
pub fn mpr_admin_interface_get_credentials_ex() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceGetCredentialsEx")
}

/// `MprAdminInterfaceGetHandle`, a `@ stub` export.
pub fn mpr_admin_interface_get_handle() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceGetHandle")
}

/// `MprAdminInterfaceGetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceGetInfo")
}

/// `MprAdminInterfaceQueryUpdateResult`, a `@ stub` export.
pub fn mpr_admin_interface_query_update_result() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceQueryUpdateResult")
}

/// `MprAdminInterfaceSetCredentials`, a `@ stub` export.
pub fn mpr_admin_interface_set_credentials() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceSetCredentials")
}

/// `MprAdminInterfaceSetCredentialsEx`, a `@ stub` export.
pub fn mpr_admin_interface_set_credentials_ex() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceSetCredentialsEx")
}

/// `MprAdminInterfaceSetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceSetInfo")
}

/// `MprAdminInterfaceTransportAdd`, a `@ stub` export.
pub fn mpr_admin_interface_transport_add() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceTransportAdd")
}

/// `MprAdminInterfaceTransportGetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_transport_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceTransportGetInfo")
}

/// `MprAdminInterfaceTransportRemove`, a `@ stub` export.
pub fn mpr_admin_interface_transport_remove() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceTransportRemove")
}

/// `MprAdminInterfaceTransportSetInfo`, a `@ stub` export.
pub fn mpr_admin_interface_transport_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceTransportSetInfo")
}

/// `MprAdminInterfaceUpdatePhonebookInfo`, a `@ stub` export.
pub fn mpr_admin_interface_update_phonebook_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceUpdatePhonebookInfo")
}

/// `MprAdminInterfaceUpdateRoutes`, a `@ stub` export.
pub fn mpr_admin_interface_update_routes() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminInterfaceUpdateRoutes")
}

/// `MprAdminIsDomainRasServer`, a `@ stub` export.
pub fn mpr_admin_is_domain_ras_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminIsDomainRasServer")
}

/// `MprAdminMIBBufferFree`, a `@ stub` export.
pub fn mpr_admin_mib_buffer_free() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBBufferFree")
}

/// `MprAdminMIBEntryCreate`, a `@ stub` export.
pub fn mpr_admin_mib_entry_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntryCreate")
}

/// `MprAdminMIBEntryDelete`, a `@ stub` export.
pub fn mpr_admin_mib_entry_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntryDelete")
}

/// `MprAdminMIBEntryGet`, a `@ stub` export.
pub fn mpr_admin_mib_entry_get() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntryGet")
}

/// `MprAdminMIBEntryGetFirst`, a `@ stub` export.
pub fn mpr_admin_mib_entry_get_first() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntryGetFirst")
}

/// `MprAdminMIBEntryGetNext`, a `@ stub` export.
pub fn mpr_admin_mib_entry_get_next() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntryGetNext")
}

/// `MprAdminMIBEntrySet`, a `@ stub` export.
pub fn mpr_admin_mib_entry_set() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBEntrySet")
}

/// `MprAdminMIBServerConnect`, a `@ stub` export.
pub fn mpr_admin_mib_server_connect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBServerConnect")
}

/// `MprAdminMIBServerDisconnect`, a `@ stub` export.
pub fn mpr_admin_mib_server_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminMIBServerDisconnect")
}

/// `MprAdminPortClearStats`, a `@ stub` export.
pub fn mpr_admin_port_clear_stats() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminPortClearStats")
}

/// `MprAdminPortDisconnect`, a `@ stub` export.
pub fn mpr_admin_port_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminPortDisconnect")
}

/// `MprAdminPortEnum`, a `@ stub` export.
pub fn mpr_admin_port_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminPortEnum")
}

/// `MprAdminPortGetInfo`, a `@ stub` export.
pub fn mpr_admin_port_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminPortGetInfo")
}

/// `MprAdminPortReset`, a `@ stub` export.
pub fn mpr_admin_port_reset() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminPortReset")
}

/// `MprAdminRegisterConnectionNotification`, a `@ stub` export.
pub fn mpr_admin_register_connection_notification() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminRegisterConnectionNotification")
}

/// `MprAdminSendUserMessage`, a `@ stub` export.
pub fn mpr_admin_send_user_message() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminSendUserMessage")
}

/// `MprAdminServerConnect`, a `@ stub` export.
pub fn mpr_admin_server_connect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminServerConnect")
}

/// `MprAdminServerDisconnect`, a `@ stub` export.
pub fn mpr_admin_server_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminServerDisconnect")
}

/// `MprAdminServerGetCredentials`, a `@ stub` export.
pub fn mpr_admin_server_get_credentials() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminServerGetCredentials")
}

/// `MprAdminServerGetInfo`, a `@ stub` export.
pub fn mpr_admin_server_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminServerGetInfo")
}

/// `MprAdminServerSetCredentials`, a `@ stub` export.
pub fn mpr_admin_server_set_credentials() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminServerSetCredentials")
}

/// `MprAdminTransportCreate`, a `@ stub` export.
pub fn mpr_admin_transport_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminTransportCreate")
}

/// `MprAdminTransportGetInfo`, a `@ stub` export.
pub fn mpr_admin_transport_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminTransportGetInfo")
}

/// `MprAdminTransportSetInfo`, a `@ stub` export.
pub fn mpr_admin_transport_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminTransportSetInfo")
}

/// `MprAdminUpgradeUsers`, a `@ stub` export.
pub fn mpr_admin_upgrade_users() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUpgradeUsers")
}

/// `MprAdminUserClose`, a `@ stub` export.
pub fn mpr_admin_user_close() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserClose")
}

/// `MprAdminUserGetInfo`, a `@ stub` export.
pub fn mpr_admin_user_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserGetInfo")
}

/// `MprAdminUserOpen`, a `@ stub` export.
pub fn mpr_admin_user_open() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserOpen")
}

/// `MprAdminUserRead`, a `@ stub` export.
pub fn mpr_admin_user_read() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserRead")
}

/// `MprAdminUserReadProfFlags`, a `@ stub` export.
pub fn mpr_admin_user_read_prof_flags() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserReadProfFlags")
}

/// `MprAdminUserServerConnect`, a `@ stub` export.
pub fn mpr_admin_user_server_connect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserServerConnect")
}

/// `MprAdminUserServerDisconnect`, a `@ stub` export.
pub fn mpr_admin_user_server_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserServerDisconnect")
}

/// `MprAdminUserSetInfo`, a `@ stub` export.
pub fn mpr_admin_user_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserSetInfo")
}

/// `MprAdminUserWrite`, a `@ stub` export.
pub fn mpr_admin_user_write() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserWrite")
}

/// `MprAdminUserWriteProfFlags`, a `@ stub` export.
pub fn mpr_admin_user_write_prof_flags() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprAdminUserWriteProfFlags")
}

/// `MprConfigBufferFree`, a `@ stub` export.
pub fn mpr_config_buffer_free() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigBufferFree")
}

/// `MprConfigGetFriendlyName`, a `@ stub` export.
pub fn mpr_config_get_friendly_name() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigGetFriendlyName")
}

/// `MprConfigGetGuidName`, a `@ stub` export.
pub fn mpr_config_get_guid_name() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigGetGuidName")
}

/// `MprConfigInterfaceCreate`, a `@ stub` export.
pub fn mpr_config_interface_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceCreate")
}

/// `MprConfigInterfaceDelete`, a `@ stub` export.
pub fn mpr_config_interface_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceDelete")
}

/// `MprConfigInterfaceEnum`, a `@ stub` export.
pub fn mpr_config_interface_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceEnum")
}

/// `MprConfigInterfaceGetHandle`, a `@ stub` export.
pub fn mpr_config_interface_get_handle() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceGetHandle")
}

/// `MprConfigInterfaceGetInfo`, a `@ stub` export.
pub fn mpr_config_interface_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceGetInfo")
}

/// `MprConfigInterfaceSetInfo`, a `@ stub` export.
pub fn mpr_config_interface_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceSetInfo")
}

/// `MprConfigInterfaceTransportAdd`, a `@ stub` export.
pub fn mpr_config_interface_transport_add() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportAdd")
}

/// `MprConfigInterfaceTransportEnum`, a `@ stub` export.
pub fn mpr_config_interface_transport_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportEnum")
}

/// `MprConfigInterfaceTransportGetHandle`, a `@ stub` export.
pub fn mpr_config_interface_transport_get_handle() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportGetHandle")
}

/// `MprConfigInterfaceTransportGetInfo`, a `@ stub` export.
pub fn mpr_config_interface_transport_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportGetInfo")
}

/// `MprConfigInterfaceTransportRemove`, a `@ stub` export.
pub fn mpr_config_interface_transport_remove() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportRemove")
}

/// `MprConfigInterfaceTransportSetInfo`, a `@ stub` export.
pub fn mpr_config_interface_transport_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigInterfaceTransportSetInfo")
}

/// `MprConfigServerBackup`, a `@ stub` export.
pub fn mpr_config_server_backup() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerBackup")
}

/// `MprConfigServerDisconnect`, a `@ stub` export.
pub fn mpr_config_server_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerDisconnect")
}

/// `MprConfigServerGetInfo`, a `@ stub` export.
pub fn mpr_config_server_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerGetInfo")
}

/// `MprConfigServerInstall`, a `@ stub` export.
pub fn mpr_config_server_install() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerInstall")
}

/// `MprConfigServerRefresh`, a `@ stub` export.
pub fn mpr_config_server_refresh() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerRefresh")
}

/// `MprConfigServerRestore`, a `@ stub` export.
pub fn mpr_config_server_restore() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigServerRestore")
}

/// `MprConfigTransportCreate`, a `@ stub` export.
pub fn mpr_config_transport_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportCreate")
}

/// `MprConfigTransportDelete`, a `@ stub` export.
pub fn mpr_config_transport_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportDelete")
}

/// `MprConfigTransportEnum`, a `@ stub` export.
pub fn mpr_config_transport_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportEnum")
}

/// `MprConfigTransportGetHandle`, a `@ stub` export.
pub fn mpr_config_transport_get_handle() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportGetHandle")
}

/// `MprConfigTransportGetInfo`, a `@ stub` export.
pub fn mpr_config_transport_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportGetInfo")
}

/// `MprConfigTransportSetInfo`, a `@ stub` export.
pub fn mpr_config_transport_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprConfigTransportSetInfo")
}

/// `MprDomainQueryAccess`, a `@ stub` export.
pub fn mpr_domain_query_access() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprDomainQueryAccess")
}

/// `MprDomainQueryRasServer`, a `@ stub` export.
pub fn mpr_domain_query_ras_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprDomainQueryRasServer")
}

/// `MprDomainRegisterRasServer`, a `@ stub` export.
pub fn mpr_domain_register_ras_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprDomainRegisterRasServer")
}

/// `MprDomainSetAccess`, a `@ stub` export.
pub fn mpr_domain_set_access() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprDomainSetAccess")
}

/// `MprGetUsrParams`, a `@ stub` export.
pub fn mpr_get_usr_params() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprGetUsrParams")
}

/// `MprInfoBlockAdd`, a `@ stub` export.
pub fn mpr_info_block_add() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoBlockAdd")
}

/// `MprInfoBlockFind`, a `@ stub` export.
pub fn mpr_info_block_find() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoBlockFind")
}

/// `MprInfoBlockQuerySize`, a `@ stub` export.
pub fn mpr_info_block_query_size() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoBlockQuerySize")
}

/// `MprInfoBlockRemove`, a `@ stub` export.
pub fn mpr_info_block_remove() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoBlockRemove")
}

/// `MprInfoBlockSet`, a `@ stub` export.
pub fn mpr_info_block_set() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoBlockSet")
}

/// `MprInfoCreate`, a `@ stub` export.
pub fn mpr_info_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoCreate")
}

/// `MprInfoDelete`, a `@ stub` export.
pub fn mpr_info_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoDelete")
}

/// `MprInfoDuplicate`, a `@ stub` export.
pub fn mpr_info_duplicate() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoDuplicate")
}

/// `MprInfoRemoveAll`, a `@ stub` export.
pub fn mpr_info_remove_all() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprInfoRemoveAll")
}

/// `MprPortSetUsage`, a `@ stub` export.
pub fn mpr_port_set_usage() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprPortSetUsage")
}

/// `MprSetupIpInIpInterfaceFriendlyNameCreate`, a `@ stub` export.
pub fn mpr_setup_ip_in_ip_interface_friendly_name_create() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprSetupIpInIpInterfaceFriendlyNameCreate")
}

/// `MprSetupIpInIpInterfaceFriendlyNameDelete`, a `@ stub` export.
pub fn mpr_setup_ip_in_ip_interface_friendly_name_delete() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprSetupIpInIpInterfaceFriendlyNameDelete")
}

/// `MprSetupIpInIpInterfaceFriendlyNameEnum`, a `@ stub` export.
pub fn mpr_setup_ip_in_ip_interface_friendly_name_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprSetupIpInIpInterfaceFriendlyNameEnum")
}

/// `MprSetupIpInIpInterfaceFriendlyNameFree`, a `@ stub` export.
pub fn mpr_setup_ip_in_ip_interface_friendly_name_free() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "MprSetupIpInIpInterfaceFriendlyNameFree")
}

/// `RasAdminBufferFree`, a `@ stub` export.
pub fn ras_admin_buffer_free() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminBufferFree")
}

/// `RasAdminConnectionClearStats`, a `@ stub` export.
pub fn ras_admin_connection_clear_stats() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminConnectionClearStats")
}

/// `RasAdminConnectionEnum`, a `@ stub` export.
pub fn ras_admin_connection_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminConnectionEnum")
}

/// `RasAdminConnectionGetInfo`, a `@ stub` export.
pub fn ras_admin_connection_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminConnectionGetInfo")
}

/// `RasAdminGetErrorString`, a `@ stub` export.
pub fn ras_admin_get_error_string() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminGetErrorString")
}

/// `RasAdminGetPDCServer`, a `@ stub` export.
pub fn ras_admin_get_pdc_server() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminGetPDCServer")
}

/// `RasAdminIsServiceRunning`, a `@ stub` export.
pub fn ras_admin_is_service_running() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminIsServiceRunning")
}

/// `RasAdminPortClearStats`, a `@ stub` export.
pub fn ras_admin_port_clear_stats() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminPortClearStats")
}

/// `RasAdminPortDisconnect`, a `@ stub` export.
pub fn ras_admin_port_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminPortDisconnect")
}

/// `RasAdminPortEnum`, a `@ stub` export.
pub fn ras_admin_port_enum() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminPortEnum")
}

/// `RasAdminPortGetInfo`, a `@ stub` export.
pub fn ras_admin_port_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminPortGetInfo")
}

/// `RasAdminPortReset`, a `@ stub` export.
pub fn ras_admin_port_reset() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminPortReset")
}

/// `RasAdminServerConnect`, a `@ stub` export.
pub fn ras_admin_server_connect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminServerConnect")
}

/// `RasAdminServerDisconnect`, a `@ stub` export.
pub fn ras_admin_server_disconnect() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminServerDisconnect")
}

/// `RasAdminUserGetInfo`, a `@ stub` export.
pub fn ras_admin_user_get_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminUserGetInfo")
}

/// `RasAdminUserSetInfo`, a `@ stub` export.
pub fn ras_admin_user_set_info() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasAdminUserSetInfo")
}

/// `RasPrivilegeAndCallBackNumber`, a `@ stub` export.
pub fn ras_privilege_and_call_back_number() -> StubException {
    wine_spec_unimplemented_stub(MPRAPI_DLL_NAME, "RasPrivilegeAndCallBackNumber")
}
