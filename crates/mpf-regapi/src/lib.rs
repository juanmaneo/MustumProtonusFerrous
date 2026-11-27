//! Port of the portable logic in Wine's `dlls/regapi/regapi.spec` and `dlls/regapi/Makefile.in`.
//!
//! `dlls/regapi` has no C sources, IDL, resource or type library. Its logic is the 69 `@ stub`
//! exports of `regapi.spec`, so this is a partial port: it models the export table, the
//! `__wine_spec_unimplemented_stub` record each export raises, and the `MODULE` name in that record.
//!
//! Ported:
//! - `regapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - One helper per `@ stub` export, named in snake_case, that raises the record for that export,
//!   from [`reg_build_number_query`] to [`wait_for_ts_connections_policy_changes`]
//!
//! Skipped:
//! - The behaviour of the 69 exports. Each one is a `@ stub`, so the C body never returns a value.
//!   It only raises the stub record, which the helpers model.
//! - Ordinals: every export has a name, so winebuild's ordinal fallback for the function name
//!   is not modelled.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `Makefile.in`, a build flag with no runtime effect.

mod exports;
mod stub;
mod stubs;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stubs::{
    reg_build_number_query, reg_cd_create_a, reg_cd_create_w, reg_cd_delete_a, reg_cd_delete_w,
    reg_cd_enumerate_a, reg_cd_enumerate_w, reg_cd_query_a, reg_cd_query_w, reg_close_server,
    reg_console_shadow_query_a, reg_console_shadow_query_w, reg_default_user_config_query_a,
    reg_default_user_config_query_w, reg_deny_ts_connections_policy, reg_free_utility_command_list,
    reg_get_machine_policy, reg_get_machine_policy_ex, reg_get_t_server_version,
    reg_get_user_config_from_user_parameters, reg_get_user_policy, reg_is_machine_in_help_mode,
    reg_is_machine_policy_allow_help, reg_is_t_server, reg_merge_user_config_with_user_parameters,
    reg_open_server_a, reg_open_server_w, reg_pd_create_a, reg_pd_create_w, reg_pd_delete_a,
    reg_pd_delete_w, reg_pd_enumerate_a, reg_pd_enumerate_w, reg_pd_query_a, reg_pd_query_w,
    reg_query_oem_id, reg_query_utility_command_list, reg_sam_user_config, reg_user_config_delete,
    reg_user_config_query, reg_user_config_rename, reg_user_config_set, reg_wd_create_a,
    reg_wd_create_w, reg_wd_delete_a, reg_wd_delete_w, reg_wd_enumerate_a, reg_wd_enumerate_w,
    reg_wd_query_a, reg_wd_query_w, reg_win_station_access_check, reg_win_station_create_a,
    reg_win_station_create_w, reg_win_station_delete_a, reg_win_station_delete_w,
    reg_win_station_enumerate_a, reg_win_station_enumerate_w, reg_win_station_query_a,
    reg_win_station_query_default_security, reg_win_station_query_ex,
    reg_win_station_query_num_value_w, reg_win_station_query_security_a,
    reg_win_station_query_security_w, reg_win_station_query_value_w, reg_win_station_query_w,
    reg_win_station_set_num_value_w, reg_win_station_set_security_a,
    reg_win_station_set_security_w, wait_for_ts_connections_policy_changes,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
