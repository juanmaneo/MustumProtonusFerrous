//! Port of the portable logic in Wine's `dlls/mprapi/mprapi.c` and `dlls/mprapi/mprapi.spec`.
//!
//! Ported:
//! - `MprAdminGetErrorString`, `MprAdminIsServiceRunning` and `MprConfigServerConnect`
//!   (`@ stdcall`): [`crate::mpr_admin_get_error_string`], [`crate::mpr_admin_is_service_running`]
//!   and [`crate::mpr_config_server_connect`], with the `ERROR_*` values from `include/winerror.h`
//! - The `mprapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 129 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`] and one helper per export, such as
//!   [`compress_phone_number`]
//!
//! Skipped:
//! - The FIXME traces. The arguments that only feed them are kept as underscore-prefixed
//!   parameters, so the oracle rows can still vary them.
//! - The stdcall calling convention (APIENTRY). BOOL is modelled as bool, and HANDLE as usize
//!   with NULL as None.
//! - A NULL out-pointer. The C bodies write through localstr and hmprconfig unconditionally, so
//!   the port takes &mut and cannot be called with NULL.

mod exports;
mod functions;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use functions::{
    mpr_admin_get_error_string, mpr_admin_is_service_running, mpr_config_server_connect,
    ERROR_MR_MID_NOT_FOUND, ERROR_NOT_SUPPORTED,
};

pub use stub::{
    compress_phone_number, mpr_admin_buffer_free, mpr_admin_connection_clear_stats,
    mpr_admin_connection_enum, mpr_admin_connection_get_info,
    mpr_admin_deregister_connection_notification, mpr_admin_device_enum,
    mpr_admin_establish_domain_ras_server, mpr_admin_get_pdc_server, mpr_admin_interface_connect,
    mpr_admin_interface_create, mpr_admin_interface_delete, mpr_admin_interface_device_get_info,
    mpr_admin_interface_device_set_info, mpr_admin_interface_disconnect, mpr_admin_interface_enum,
    mpr_admin_interface_get_credentials, mpr_admin_interface_get_credentials_ex,
    mpr_admin_interface_get_handle, mpr_admin_interface_get_info,
    mpr_admin_interface_query_update_result, mpr_admin_interface_set_credentials,
    mpr_admin_interface_set_credentials_ex, mpr_admin_interface_set_info,
    mpr_admin_interface_transport_add, mpr_admin_interface_transport_get_info,
    mpr_admin_interface_transport_remove, mpr_admin_interface_transport_set_info,
    mpr_admin_interface_update_phonebook_info, mpr_admin_interface_update_routes,
    mpr_admin_is_domain_ras_server, mpr_admin_mib_buffer_free, mpr_admin_mib_entry_create,
    mpr_admin_mib_entry_delete, mpr_admin_mib_entry_get, mpr_admin_mib_entry_get_first,
    mpr_admin_mib_entry_get_next, mpr_admin_mib_entry_set, mpr_admin_mib_server_connect,
    mpr_admin_mib_server_disconnect, mpr_admin_port_clear_stats, mpr_admin_port_disconnect,
    mpr_admin_port_enum, mpr_admin_port_get_info, mpr_admin_port_reset,
    mpr_admin_register_connection_notification, mpr_admin_send_user_message,
    mpr_admin_server_connect, mpr_admin_server_disconnect, mpr_admin_server_get_credentials,
    mpr_admin_server_get_info, mpr_admin_server_set_credentials, mpr_admin_transport_create,
    mpr_admin_transport_get_info, mpr_admin_transport_set_info, mpr_admin_upgrade_users,
    mpr_admin_user_close, mpr_admin_user_get_info, mpr_admin_user_open, mpr_admin_user_read,
    mpr_admin_user_read_prof_flags, mpr_admin_user_server_connect,
    mpr_admin_user_server_disconnect, mpr_admin_user_set_info, mpr_admin_user_write,
    mpr_admin_user_write_prof_flags, mpr_config_buffer_free, mpr_config_get_friendly_name,
    mpr_config_get_guid_name, mpr_config_interface_create, mpr_config_interface_delete,
    mpr_config_interface_enum, mpr_config_interface_get_handle, mpr_config_interface_get_info,
    mpr_config_interface_set_info, mpr_config_interface_transport_add,
    mpr_config_interface_transport_enum, mpr_config_interface_transport_get_handle,
    mpr_config_interface_transport_get_info, mpr_config_interface_transport_remove,
    mpr_config_interface_transport_set_info, mpr_config_server_backup,
    mpr_config_server_disconnect, mpr_config_server_get_info, mpr_config_server_install,
    mpr_config_server_refresh, mpr_config_server_restore, mpr_config_transport_create,
    mpr_config_transport_delete, mpr_config_transport_enum, mpr_config_transport_get_handle,
    mpr_config_transport_get_info, mpr_config_transport_set_info, mpr_domain_query_access,
    mpr_domain_query_ras_server, mpr_domain_register_ras_server, mpr_domain_set_access,
    mpr_get_usr_params, mpr_info_block_add, mpr_info_block_find, mpr_info_block_query_size,
    mpr_info_block_remove, mpr_info_block_set, mpr_info_create, mpr_info_delete,
    mpr_info_duplicate, mpr_info_remove_all, mpr_port_set_usage,
    mpr_setup_ip_in_ip_interface_friendly_name_create,
    mpr_setup_ip_in_ip_interface_friendly_name_delete,
    mpr_setup_ip_in_ip_interface_friendly_name_enum,
    mpr_setup_ip_in_ip_interface_friendly_name_free, ras_admin_buffer_free,
    ras_admin_connection_clear_stats, ras_admin_connection_enum, ras_admin_connection_get_info,
    ras_admin_get_error_string, ras_admin_get_pdc_server, ras_admin_is_service_running,
    ras_admin_port_clear_stats, ras_admin_port_disconnect, ras_admin_port_enum,
    ras_admin_port_get_info, ras_admin_port_reset, ras_admin_server_connect,
    ras_admin_server_disconnect, ras_admin_user_get_info, ras_admin_user_set_info,
    ras_privilege_and_call_back_number, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
