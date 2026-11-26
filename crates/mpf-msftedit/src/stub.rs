/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msftedit.dll` in `dlls/msftedit/Makefile.in`.
const MSFTEDIT_DLL_NAME: &str = "msftedit.dll";

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

/// `SetCustomTextOutHandlerEx`, exported as `@ stub` in `dlls/msftedit/msftedit.spec`.
pub fn set_custom_text_out_handler_ex() -> StubException {
    wine_spec_unimplemented_stub(MSFTEDIT_DLL_NAME, "SetCustomTextOutHandlerEx")
}

/// `RichEditWndProc`, exported as `@ stub` in `dlls/msftedit/msftedit.spec`.
pub fn rich_edit_wnd_proc() -> StubException {
    wine_spec_unimplemented_stub(MSFTEDIT_DLL_NAME, "RichEditWndProc")
}

/// `RichListBoxWndProc`, exported as `@ stub` in `dlls/msftedit/msftedit.spec`.
pub fn rich_list_box_wnd_proc() -> StubException {
    wine_spec_unimplemented_stub(MSFTEDIT_DLL_NAME, "RichListBoxWndProc")
}

/// `RichComboBoxWndProc`, exported as `@ stub` in `dlls/msftedit/msftedit.spec`.
pub fn rich_combo_box_wnd_proc() -> StubException {
    wine_spec_unimplemented_stub(MSFTEDIT_DLL_NAME, "RichComboBoxWndProc")
}
