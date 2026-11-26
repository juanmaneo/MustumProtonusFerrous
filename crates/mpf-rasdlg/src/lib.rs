//! Port of the portable logic in Wine's `dlls/rasdlg/rasdlg_main.c` and `dlls/rasdlg/rasdlg.spec`.
//!
//! Ported:
//! - `RasEntryDlgW`, which the C body answers with `FALSE`: [`ras_entry_dlg_w`]
//! - `rasdlg.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 35 `@ stub` exports, each modelled as the `__wine_spec_unimplemented_stub` record it
//!   raises: [`wine_spec_unimplemented_stub`], [`StubException`] and one helper per export
//!
//! Skipped:
//! - The `FIXME` trace of the `phonebook`, `entry` and `lpInfo` parameters of `RasEntryDlgW`.
//!   The C body only passes them to the trace, so the port takes no arguments.
//! - The `WINAPI` and `__cdecl` calling conventions and the `RASENTRYDLGW` layout: the port
//!   has no Windows structs or pointers.

mod dialog;
mod exports;
mod stub;

pub use dialog::ras_entry_dlg_w;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    dw_terminal_dlg, get_ras_dial_out_protocols, ras_autodial_disable_dlg_a,
    ras_autodial_disable_dlg_w, ras_autodial_query_dlg_a, ras_autodial_query_dlg_w, ras_dial_dlg_a,
    ras_dial_dlg_w, ras_entry_dlg_a, ras_monitor_dlg_a, ras_monitor_dlg_w, ras_phonebook_dlg_a,
    ras_phonebook_dlg_w, ras_srv_add_prop_pages, ras_srv_add_wiz_pages,
    ras_srv_allow_connections_config, ras_srv_cleanup_service, ras_srv_enum_connections,
    ras_srv_hangup_connection, ras_srv_initialize_service, ras_srv_is_connection_connected,
    ras_srv_is_service_running, ras_srv_query_show_icon, ras_user_enable_manual_dial,
    ras_user_get_manual_dial, ras_user_prefs_dlg, ras_wiz_create_new_entry, ras_wiz_get_ncc_flags,
    ras_wiz_get_suggested_entry_name, ras_wiz_get_user_input_connection_name,
    ras_wiz_is_entry_renamable, ras_wiz_query_max_page_count, ras_wiz_set_entry_name,
    router_entry_dlg_a, router_entry_dlg_w, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
