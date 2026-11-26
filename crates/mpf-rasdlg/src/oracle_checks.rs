//! Oracle-derived test vectors for the ported rasdlg logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `RasEntryDlgW`
//! (`dlls/rasdlg/rasdlg_main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/rasdlg/`. The harness prints
//! 38 rows: one per `@ stub` export (35 rows) and three `RasEntryDlgW` rows with different
//! arguments (rows 10 to 12). ASan and UBSan flagged no row, so none is excluded. The harness
//! replaces `RtlRaiseException` with a recorder that longjmps out, because the verbatim stub
//! loops on that call and never returns.

use super::*;

type StubHelper = fn() -> StubException;

/// The `@ stub` helpers in `rasdlg.spec` order, with the export name each one must raise. These
/// are harness rows 1 to 9 and 13 to 38.
const STUB_ROWS: [(StubHelper, &str); 35] = [
    (dw_terminal_dlg, "DwTerminalDlg"),
    (get_ras_dial_out_protocols, "GetRasDialOutProtocols"),
    (ras_autodial_disable_dlg_a, "RasAutodialDisableDlgA"),
    (ras_autodial_disable_dlg_w, "RasAutodialDisableDlgW"),
    (ras_autodial_query_dlg_a, "RasAutodialQueryDlgA"),
    (ras_autodial_query_dlg_w, "RasAutodialQueryDlgW"),
    (ras_dial_dlg_a, "RasDialDlgA"),
    (ras_dial_dlg_w, "RasDialDlgW"),
    (ras_entry_dlg_a, "RasEntryDlgA"),
    (ras_monitor_dlg_a, "RasMonitorDlgA"),
    (ras_monitor_dlg_w, "RasMonitorDlgW"),
    (ras_phonebook_dlg_a, "RasPhonebookDlgA"),
    (ras_phonebook_dlg_w, "RasPhonebookDlgW"),
    (ras_srv_add_prop_pages, "RasSrvAddPropPages"),
    (ras_srv_add_wiz_pages, "RasSrvAddWizPages"),
    (
        ras_srv_allow_connections_config,
        "RasSrvAllowConnectionsConfig",
    ),
    (ras_srv_cleanup_service, "RasSrvCleanupService"),
    (ras_srv_enum_connections, "RasSrvEnumConnections"),
    (ras_srv_hangup_connection, "RasSrvHangupConnection"),
    (ras_srv_initialize_service, "RasSrvInitializeService"),
    (
        ras_srv_is_connection_connected,
        "RasSrvIsConnectionConnected",
    ),
    (ras_srv_is_service_running, "RasSrvIsServiceRunning"),
    (ras_srv_query_show_icon, "RasSrvQueryShowIcon"),
    (ras_user_enable_manual_dial, "RasUserEnableManualDial"),
    (ras_user_get_manual_dial, "RasUserGetManualDial"),
    (ras_user_prefs_dlg, "RasUserPrefsDlg"),
    (ras_wiz_create_new_entry, "RasWizCreateNewEntry"),
    (ras_wiz_get_ncc_flags, "RasWizGetNCCFlags"),
    (
        ras_wiz_get_suggested_entry_name,
        "RasWizGetSuggestedEntryName",
    ),
    (
        ras_wiz_get_user_input_connection_name,
        "RasWizGetUserInputConnectionName",
    ),
    (ras_wiz_is_entry_renamable, "RasWizIsEntryRenamable"),
    (ras_wiz_query_max_page_count, "RasWizQueryMaxPageCount"),
    (ras_wiz_set_entry_name, "RasWizSetEntryName"),
    (router_entry_dlg_a, "RouterEntryDlgA"),
    (router_entry_dlg_w, "RouterEntryDlgW"),
];

#[test]
fn oracle_stub_rows() {
    // Each row: raise code=0x80000100 flags=0x1 nargs=2 p0=rasdlg.dll p1=<export>
    for (raise, function) in STUB_ROWS {
        assert_eq!(
            raise(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "rasdlg.dll",
                function,
            }
        );
    }
}

#[test]
fn oracle_ras_entry_dlg_w_rows() {
    // Rows 10 to 12: RasEntryDlgW(NULL, NULL, NULL), RasEntryDlgW("C:book", "MyConn", NULL) and
    // RasEntryDlgW(NULL, NULL, info) all return ret=0. The arguments only reach the FIXME trace,
    // so one call covers the three rows.
    assert!(!ras_entry_dlg_w());
}
