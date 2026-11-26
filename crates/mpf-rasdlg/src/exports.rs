/// How `dlls/rasdlg/rasdlg.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::ras_entry_dlg_w`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `rasdlg.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 36] = [
    ("DwTerminalDlg", ExportKind::Stub),
    ("GetRasDialOutProtocols", ExportKind::Stub),
    ("RasAutodialDisableDlgA", ExportKind::Stub),
    ("RasAutodialDisableDlgW", ExportKind::Stub),
    ("RasAutodialQueryDlgA", ExportKind::Stub),
    ("RasAutodialQueryDlgW", ExportKind::Stub),
    ("RasDialDlgA", ExportKind::Stub),
    ("RasDialDlgW", ExportKind::Stub),
    ("RasEntryDlgA", ExportKind::Stub),
    ("RasEntryDlgW", ExportKind::Stdcall),
    ("RasMonitorDlgA", ExportKind::Stub),
    ("RasMonitorDlgW", ExportKind::Stub),
    ("RasPhonebookDlgA", ExportKind::Stub),
    ("RasPhonebookDlgW", ExportKind::Stub),
    ("RasSrvAddPropPages", ExportKind::Stub),
    ("RasSrvAddWizPages", ExportKind::Stub),
    ("RasSrvAllowConnectionsConfig", ExportKind::Stub),
    ("RasSrvCleanupService", ExportKind::Stub),
    ("RasSrvEnumConnections", ExportKind::Stub),
    ("RasSrvHangupConnection", ExportKind::Stub),
    ("RasSrvInitializeService", ExportKind::Stub),
    ("RasSrvIsConnectionConnected", ExportKind::Stub),
    ("RasSrvIsServiceRunning", ExportKind::Stub),
    ("RasSrvQueryShowIcon", ExportKind::Stub),
    ("RasUserEnableManualDial", ExportKind::Stub),
    ("RasUserGetManualDial", ExportKind::Stub),
    ("RasUserPrefsDlg", ExportKind::Stub),
    ("RasWizCreateNewEntry", ExportKind::Stub),
    ("RasWizGetNCCFlags", ExportKind::Stub),
    ("RasWizGetSuggestedEntryName", ExportKind::Stub),
    ("RasWizGetUserInputConnectionName", ExportKind::Stub),
    ("RasWizIsEntryRenamable", ExportKind::Stub),
    ("RasWizQueryMaxPageCount", ExportKind::Stub),
    ("RasWizSetEntryName", ExportKind::Stub),
    ("RouterEntryDlgA", ExportKind::Stub),
    ("RouterEntryDlgW", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
