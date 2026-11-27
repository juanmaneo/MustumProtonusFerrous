use crate::{wine_spec_unimplemented_stub, StubException};

/// The module name `winebuild` passes to stubs, from `MODULE = regapi.dll` in `dlls/regapi/Makefile.in`.
const REGAPI_DLL_NAME: &str = "regapi.dll";

/// `RegBuildNumberQuery`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_build_number_query() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegBuildNumberQuery")
}

/// `RegCdCreateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_create_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdCreateA")
}

/// `RegCdCreateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_create_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdCreateW")
}

/// `RegCdDeleteA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_delete_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdDeleteA")
}

/// `RegCdDeleteW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_delete_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdDeleteW")
}

/// `RegCdEnumerateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_enumerate_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdEnumerateA")
}

/// `RegCdEnumerateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_enumerate_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdEnumerateW")
}

/// `RegCdQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdQueryA")
}

/// `RegCdQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_cd_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCdQueryW")
}

/// `RegCloseServer`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_close_server() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegCloseServer")
}

/// `RegConsoleShadowQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_console_shadow_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegConsoleShadowQueryA")
}

/// `RegConsoleShadowQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_console_shadow_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegConsoleShadowQueryW")
}

/// `RegDefaultUserConfigQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_default_user_config_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegDefaultUserConfigQueryA")
}

/// `RegDefaultUserConfigQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_default_user_config_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegDefaultUserConfigQueryW")
}

/// `RegDenyTSConnectionsPolicy`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_deny_ts_connections_policy() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegDenyTSConnectionsPolicy")
}

/// `RegFreeUtilityCommandList`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_free_utility_command_list() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegFreeUtilityCommandList")
}

/// `RegGetMachinePolicy`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_get_machine_policy() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegGetMachinePolicy")
}

/// `RegGetMachinePolicyEx`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_get_machine_policy_ex() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegGetMachinePolicyEx")
}

/// `RegGetTServerVersion`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_get_t_server_version() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegGetTServerVersion")
}

/// `RegGetUserConfigFromUserParameters`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_get_user_config_from_user_parameters() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegGetUserConfigFromUserParameters")
}

/// `RegGetUserPolicy`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_get_user_policy() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegGetUserPolicy")
}

/// `RegIsMachineInHelpMode`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_is_machine_in_help_mode() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegIsMachineInHelpMode")
}

/// `RegIsMachinePolicyAllowHelp`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_is_machine_policy_allow_help() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegIsMachinePolicyAllowHelp")
}

/// `RegIsTServer`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_is_t_server() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegIsTServer")
}

/// `RegMergeUserConfigWithUserParameters`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_merge_user_config_with_user_parameters() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegMergeUserConfigWithUserParameters")
}

/// `RegOpenServerA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_open_server_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegOpenServerA")
}

/// `RegOpenServerW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_open_server_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegOpenServerW")
}

/// `RegPdCreateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_create_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdCreateA")
}

/// `RegPdCreateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_create_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdCreateW")
}

/// `RegPdDeleteA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_delete_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdDeleteA")
}

/// `RegPdDeleteW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_delete_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdDeleteW")
}

/// `RegPdEnumerateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_enumerate_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdEnumerateA")
}

/// `RegPdEnumerateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_enumerate_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdEnumerateW")
}

/// `RegPdQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdQueryA")
}

/// `RegPdQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_pd_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegPdQueryW")
}

/// `RegQueryOEMId`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_query_oem_id() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegQueryOEMId")
}

/// `RegQueryUtilityCommandList`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_query_utility_command_list() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegQueryUtilityCommandList")
}

/// `RegSAMUserConfig`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_sam_user_config() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegSAMUserConfig")
}

/// `RegUserConfigDelete`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_user_config_delete() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegUserConfigDelete")
}

/// `RegUserConfigQuery`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_user_config_query() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegUserConfigQuery")
}

/// `RegUserConfigRename`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_user_config_rename() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegUserConfigRename")
}

/// `RegUserConfigSet`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_user_config_set() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegUserConfigSet")
}

/// `RegWdCreateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_create_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdCreateA")
}

/// `RegWdCreateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_create_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdCreateW")
}

/// `RegWdDeleteA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_delete_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdDeleteA")
}

/// `RegWdDeleteW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_delete_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdDeleteW")
}

/// `RegWdEnumerateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_enumerate_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdEnumerateA")
}

/// `RegWdEnumerateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_enumerate_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdEnumerateW")
}

/// `RegWdQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdQueryA")
}

/// `RegWdQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_wd_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWdQueryW")
}

/// `RegWinStationAccessCheck`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_access_check() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationAccessCheck")
}

/// `RegWinStationCreateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_create_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationCreateA")
}

/// `RegWinStationCreateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_create_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationCreateW")
}

/// `RegWinStationDeleteA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_delete_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationDeleteA")
}

/// `RegWinStationDeleteW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_delete_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationDeleteW")
}

/// `RegWinStationEnumerateA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_enumerate_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationEnumerateA")
}

/// `RegWinStationEnumerateW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_enumerate_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationEnumerateW")
}

/// `RegWinStationQueryA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryA")
}

/// `RegWinStationQueryDefaultSecurity`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_default_security() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryDefaultSecurity")
}

/// `RegWinStationQueryEx`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_ex() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryEx")
}

/// `RegWinStationQueryNumValueW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_num_value_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryNumValueW")
}

/// `RegWinStationQuerySecurityA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_security_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQuerySecurityA")
}

/// `RegWinStationQuerySecurityW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_security_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQuerySecurityW")
}

/// `RegWinStationQueryValueW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_value_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryValueW")
}

/// `RegWinStationQueryW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_query_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationQueryW")
}

/// `RegWinStationSetNumValueW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_set_num_value_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationSetNumValueW")
}

/// `RegWinStationSetSecurityA`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_set_security_a() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationSetSecurityA")
}

/// `RegWinStationSetSecurityW`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn reg_win_station_set_security_w() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "RegWinStationSetSecurityW")
}

/// `WaitForTSConnectionsPolicyChanges`, exported as `@ stub` in `dlls/regapi/regapi.spec`.
pub fn wait_for_ts_connections_policy_changes() -> StubException {
    wine_spec_unimplemented_stub(REGAPI_DLL_NAME, "WaitForTSConnectionsPolicyChanges")
}
