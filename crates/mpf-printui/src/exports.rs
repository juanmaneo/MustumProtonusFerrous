/// How `dlls/printui/printui.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::print_ui_entry_w`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall -private`: `printui.c` has no body for it, so it is listed but not modelled.
    Private,
}

/// The `printui.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 23] = [
    ("ConnectToPrinterDlg", ExportKind::Stub),
    ("ConnectToPrinterPropertyPage", ExportKind::Stub),
    ("ConstructPrinterFriendlyName", ExportKind::Stub),
    ("DllCanUnloadNow", ExportKind::Private),
    ("DllGetClassObject", ExportKind::Stub),
    ("DocumentPropertiesWrap", ExportKind::Stub),
    ("PnPInterface", ExportKind::Stub),
    ("PrintNotifyTray_Exit", ExportKind::Stub),
    ("PrintNotifyTray_Init", ExportKind::Stub),
    ("PrintUIEntryW", ExportKind::Stdcall),
    ("PrinterPropPageProvider", ExportKind::Stub),
    ("RegisterPrintNotify", ExportKind::Stub),
    ("ShowErrorMessageHR", ExportKind::Stub),
    ("ShowErrorMessageSC", ExportKind::Stub),
    ("UnregisterPrintNotify", ExportKind::Stub),
    ("bFolderEnumPrinters", ExportKind::Stub),
    ("bFolderGetPrinter", ExportKind::Stub),
    ("bFolderRefresh", ExportKind::Stub),
    ("bPrinterSetup", ExportKind::Stub),
    ("vDocumentDefaults", ExportKind::Stub),
    ("vPrinterPropPages", ExportKind::Stub),
    ("vQueueCreate", ExportKind::Stub),
    ("vServerPropPages", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
