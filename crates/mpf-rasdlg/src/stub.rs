/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = rasdlg.dll` in `dlls/rasdlg/Makefile.in`.
const RASDLG_DLL_NAME: &str = "rasdlg.dll";

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

/// `DwTerminalDlg`, a `@ stub` export of `rasdlg.spec`.
pub fn dw_terminal_dlg() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "DwTerminalDlg")
}

/// `GetRasDialOutProtocols`, a `@ stub` export of `rasdlg.spec`.
pub fn get_ras_dial_out_protocols() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "GetRasDialOutProtocols")
}

/// `RasAutodialDisableDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_autodial_disable_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasAutodialDisableDlgA")
}

/// `RasAutodialDisableDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_autodial_disable_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasAutodialDisableDlgW")
}

/// `RasAutodialQueryDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_autodial_query_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasAutodialQueryDlgA")
}

/// `RasAutodialQueryDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_autodial_query_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasAutodialQueryDlgW")
}

/// `RasDialDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_dial_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasDialDlgA")
}

/// `RasDialDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_dial_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasDialDlgW")
}

/// `RasEntryDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_entry_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasEntryDlgA")
}

/// `RasMonitorDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_monitor_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasMonitorDlgA")
}

/// `RasMonitorDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_monitor_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasMonitorDlgW")
}

/// `RasPhonebookDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_phonebook_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasPhonebookDlgA")
}

/// `RasPhonebookDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_phonebook_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasPhonebookDlgW")
}

/// `RasSrvAddPropPages`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_add_prop_pages() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvAddPropPages")
}

/// `RasSrvAddWizPages`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_add_wiz_pages() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvAddWizPages")
}

/// `RasSrvAllowConnectionsConfig`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_allow_connections_config() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvAllowConnectionsConfig")
}

/// `RasSrvCleanupService`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_cleanup_service() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvCleanupService")
}

/// `RasSrvEnumConnections`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_enum_connections() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvEnumConnections")
}

/// `RasSrvHangupConnection`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_hangup_connection() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvHangupConnection")
}

/// `RasSrvInitializeService`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_initialize_service() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvInitializeService")
}

/// `RasSrvIsConnectionConnected`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_is_connection_connected() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvIsConnectionConnected")
}

/// `RasSrvIsServiceRunning`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_is_service_running() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvIsServiceRunning")
}

/// `RasSrvQueryShowIcon`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_srv_query_show_icon() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasSrvQueryShowIcon")
}

/// `RasUserEnableManualDial`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_user_enable_manual_dial() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasUserEnableManualDial")
}

/// `RasUserGetManualDial`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_user_get_manual_dial() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasUserGetManualDial")
}

/// `RasUserPrefsDlg`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_user_prefs_dlg() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasUserPrefsDlg")
}

/// `RasWizCreateNewEntry`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_create_new_entry() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizCreateNewEntry")
}

/// `RasWizGetNCCFlags`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_get_ncc_flags() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizGetNCCFlags")
}

/// `RasWizGetSuggestedEntryName`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_get_suggested_entry_name() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizGetSuggestedEntryName")
}

/// `RasWizGetUserInputConnectionName`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_get_user_input_connection_name() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizGetUserInputConnectionName")
}

/// `RasWizIsEntryRenamable`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_is_entry_renamable() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizIsEntryRenamable")
}

/// `RasWizQueryMaxPageCount`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_query_max_page_count() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizQueryMaxPageCount")
}

/// `RasWizSetEntryName`, a `@ stub` export of `rasdlg.spec`.
pub fn ras_wiz_set_entry_name() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RasWizSetEntryName")
}

/// `RouterEntryDlgA`, a `@ stub` export of `rasdlg.spec`.
pub fn router_entry_dlg_a() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RouterEntryDlgA")
}

/// `RouterEntryDlgW`, a `@ stub` export of `rasdlg.spec`.
pub fn router_entry_dlg_w() -> StubException {
    wine_spec_unimplemented_stub(RASDLG_DLL_NAME, "RouterEntryDlgW")
}
