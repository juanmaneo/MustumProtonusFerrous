/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = printui.dll` in `dlls/printui/Makefile.in`.
const PRINTUI_DLL_NAME: &str = "printui.dll";

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

/// `ConnectToPrinterDlg`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn connect_to_printer_dlg() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "ConnectToPrinterDlg")
}

/// `ConnectToPrinterPropertyPage`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn connect_to_printer_property_page() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "ConnectToPrinterPropertyPage")
}

/// `ConstructPrinterFriendlyName`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn construct_printer_friendly_name() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "ConstructPrinterFriendlyName")
}

/// `DllGetClassObject`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn dll_get_class_object() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "DllGetClassObject")
}

/// `DocumentPropertiesWrap`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn document_properties_wrap() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "DocumentPropertiesWrap")
}

/// `PnPInterface`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn pnp_interface() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "PnPInterface")
}

/// `PrintNotifyTray_Exit`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn print_notify_tray_exit() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "PrintNotifyTray_Exit")
}

/// `PrintNotifyTray_Init`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn print_notify_tray_init() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "PrintNotifyTray_Init")
}

/// `PrinterPropPageProvider`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn printer_prop_page_provider() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "PrinterPropPageProvider")
}

/// `RegisterPrintNotify`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn register_print_notify() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "RegisterPrintNotify")
}

/// `ShowErrorMessageHR`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn show_error_message_hr() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "ShowErrorMessageHR")
}

/// `ShowErrorMessageSC`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn show_error_message_sc() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "ShowErrorMessageSC")
}

/// `UnregisterPrintNotify`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn unregister_print_notify() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "UnregisterPrintNotify")
}

/// `bFolderEnumPrinters`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn b_folder_enum_printers() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "bFolderEnumPrinters")
}

/// `bFolderGetPrinter`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn b_folder_get_printer() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "bFolderGetPrinter")
}

/// `bFolderRefresh`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn b_folder_refresh() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "bFolderRefresh")
}

/// `bPrinterSetup`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn b_printer_setup() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "bPrinterSetup")
}

/// `vDocumentDefaults`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn v_document_defaults() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "vDocumentDefaults")
}

/// `vPrinterPropPages`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn v_printer_prop_pages() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "vPrinterPropPages")
}

/// `vQueueCreate`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn v_queue_create() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "vQueueCreate")
}

/// `vServerPropPages`, exported as `@ stub` in `dlls/printui/printui.spec`.
pub fn v_server_prop_pages() -> StubException {
    wine_spec_unimplemented_stub(PRINTUI_DLL_NAME, "vServerPropPages")
}
