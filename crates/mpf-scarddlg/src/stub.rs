/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = scarddlg.dll` in `dlls/scarddlg/Makefile.in`.
const SCARDDLG_DLL_NAME: &str = "scarddlg.dll";

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

/// `GetOpenCardNameA`, exported as `@ stub` in `dlls/scarddlg/scarddlg.spec`.
pub fn get_open_card_name_a() -> StubException {
    wine_spec_unimplemented_stub(SCARDDLG_DLL_NAME, "GetOpenCardNameA")
}

/// `GetOpenCardNameW`, exported as `@ stub` in `dlls/scarddlg/scarddlg.spec`.
pub fn get_open_card_name_w() -> StubException {
    wine_spec_unimplemented_stub(SCARDDLG_DLL_NAME, "GetOpenCardNameW")
}

/// `SCardDlgExtendedError`, exported as `@ stub` in `dlls/scarddlg/scarddlg.spec`.
pub fn scard_dlg_extended_error() -> StubException {
    wine_spec_unimplemented_stub(SCARDDLG_DLL_NAME, "SCardDlgExtendedError")
}

/// `SCardUIDlgSelectCardA`, exported as `@ stub` in `dlls/scarddlg/scarddlg.spec`.
pub fn scard_ui_dlg_select_card_a() -> StubException {
    wine_spec_unimplemented_stub(SCARDDLG_DLL_NAME, "SCardUIDlgSelectCardA")
}

/// `SCardUIDlgSelectCardW`, exported as `@ stub` in `dlls/scarddlg/scarddlg.spec`.
pub fn scard_ui_dlg_select_card_w() -> StubException {
    wine_spec_unimplemented_stub(SCARDDLG_DLL_NAME, "SCardUIDlgSelectCardW")
}
