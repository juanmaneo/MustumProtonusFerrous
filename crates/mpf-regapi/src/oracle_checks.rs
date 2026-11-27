//! Oracle-derived test vectors for the ported regapi stub model.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c:504-516`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/regapi/`, and running the harness once per `@ stub` export of
//! `dlls/regapi/regapi.spec`. The harness replaces `RtlRaiseException` with a recorder that copies
//! the record and jumps out with `longjmp`, because the C body loops on that call. All 69 rows
//! printed and ASan and UBSan reported nothing, so 0 vectors are excluded.

use super::*;

/// `(helper, code, flags, number_parameters, module, function)` for one `@ stub` export.
type Row = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// One row per `@ stub` export, in `dlls/regapi/regapi.spec` order.
#[rustfmt::skip]
const ROWS: [Row; 69] = [
    (reg_build_number_query, 0x8000_0100, 0x01, 2, "regapi.dll", "RegBuildNumberQuery"),
    (reg_cd_create_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdCreateA"),
    (reg_cd_create_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdCreateW"),
    (reg_cd_delete_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdDeleteA"),
    (reg_cd_delete_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdDeleteW"),
    (reg_cd_enumerate_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdEnumerateA"),
    (reg_cd_enumerate_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdEnumerateW"),
    (reg_cd_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdQueryA"),
    (reg_cd_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCdQueryW"),
    (reg_close_server, 0x8000_0100, 0x01, 2, "regapi.dll", "RegCloseServer"),
    (reg_console_shadow_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegConsoleShadowQueryA"),
    (reg_console_shadow_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegConsoleShadowQueryW"),
    (reg_default_user_config_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegDefaultUserConfigQueryA"),
    (reg_default_user_config_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegDefaultUserConfigQueryW"),
    (reg_deny_ts_connections_policy, 0x8000_0100, 0x01, 2, "regapi.dll", "RegDenyTSConnectionsPolicy"),
    (reg_free_utility_command_list, 0x8000_0100, 0x01, 2, "regapi.dll", "RegFreeUtilityCommandList"),
    (reg_get_machine_policy, 0x8000_0100, 0x01, 2, "regapi.dll", "RegGetMachinePolicy"),
    (reg_get_machine_policy_ex, 0x8000_0100, 0x01, 2, "regapi.dll", "RegGetMachinePolicyEx"),
    (reg_get_t_server_version, 0x8000_0100, 0x01, 2, "regapi.dll", "RegGetTServerVersion"),
    (reg_get_user_config_from_user_parameters, 0x8000_0100, 0x01, 2, "regapi.dll", "RegGetUserConfigFromUserParameters"),
    (reg_get_user_policy, 0x8000_0100, 0x01, 2, "regapi.dll", "RegGetUserPolicy"),
    (reg_is_machine_in_help_mode, 0x8000_0100, 0x01, 2, "regapi.dll", "RegIsMachineInHelpMode"),
    (reg_is_machine_policy_allow_help, 0x8000_0100, 0x01, 2, "regapi.dll", "RegIsMachinePolicyAllowHelp"),
    (reg_is_t_server, 0x8000_0100, 0x01, 2, "regapi.dll", "RegIsTServer"),
    (reg_merge_user_config_with_user_parameters, 0x8000_0100, 0x01, 2, "regapi.dll", "RegMergeUserConfigWithUserParameters"),
    (reg_open_server_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegOpenServerA"),
    (reg_open_server_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegOpenServerW"),
    (reg_pd_create_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdCreateA"),
    (reg_pd_create_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdCreateW"),
    (reg_pd_delete_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdDeleteA"),
    (reg_pd_delete_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdDeleteW"),
    (reg_pd_enumerate_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdEnumerateA"),
    (reg_pd_enumerate_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdEnumerateW"),
    (reg_pd_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdQueryA"),
    (reg_pd_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegPdQueryW"),
    (reg_query_oem_id, 0x8000_0100, 0x01, 2, "regapi.dll", "RegQueryOEMId"),
    (reg_query_utility_command_list, 0x8000_0100, 0x01, 2, "regapi.dll", "RegQueryUtilityCommandList"),
    (reg_sam_user_config, 0x8000_0100, 0x01, 2, "regapi.dll", "RegSAMUserConfig"),
    (reg_user_config_delete, 0x8000_0100, 0x01, 2, "regapi.dll", "RegUserConfigDelete"),
    (reg_user_config_query, 0x8000_0100, 0x01, 2, "regapi.dll", "RegUserConfigQuery"),
    (reg_user_config_rename, 0x8000_0100, 0x01, 2, "regapi.dll", "RegUserConfigRename"),
    (reg_user_config_set, 0x8000_0100, 0x01, 2, "regapi.dll", "RegUserConfigSet"),
    (reg_wd_create_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdCreateA"),
    (reg_wd_create_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdCreateW"),
    (reg_wd_delete_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdDeleteA"),
    (reg_wd_delete_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdDeleteW"),
    (reg_wd_enumerate_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdEnumerateA"),
    (reg_wd_enumerate_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdEnumerateW"),
    (reg_wd_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdQueryA"),
    (reg_wd_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWdQueryW"),
    (reg_win_station_access_check, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationAccessCheck"),
    (reg_win_station_create_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationCreateA"),
    (reg_win_station_create_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationCreateW"),
    (reg_win_station_delete_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationDeleteA"),
    (reg_win_station_delete_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationDeleteW"),
    (reg_win_station_enumerate_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationEnumerateA"),
    (reg_win_station_enumerate_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationEnumerateW"),
    (reg_win_station_query_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryA"),
    (reg_win_station_query_default_security, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryDefaultSecurity"),
    (reg_win_station_query_ex, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryEx"),
    (reg_win_station_query_num_value_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryNumValueW"),
    (reg_win_station_query_security_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQuerySecurityA"),
    (reg_win_station_query_security_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQuerySecurityW"),
    (reg_win_station_query_value_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryValueW"),
    (reg_win_station_query_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationQueryW"),
    (reg_win_station_set_num_value_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationSetNumValueW"),
    (reg_win_station_set_security_a, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationSetSecurityA"),
    (reg_win_station_set_security_w, 0x8000_0100, 0x01, 2, "regapi.dll", "RegWinStationSetSecurityW"),
    (wait_for_ts_connections_policy_changes, 0x8000_0100, 0x01, 2, "regapi.dll", "WaitForTSConnectionsPolicyChanges"),
];

#[test]
fn oracle_stub_rows() {
    for (index, &(call, code, flags, number_parameters, module, function)) in
        ROWS.iter().enumerate()
    {
        assert_eq!(
            EXPORTS[index],
            (function, ExportKind::Stub),
            "row {:02}",
            index + 1
        );
        assert_eq!(
            call(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            },
            "row {:02}",
            index + 1
        );
    }
}
