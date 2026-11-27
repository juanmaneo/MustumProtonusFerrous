/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = comctl32.dll` in `dlls/comctl32/Makefile.in`.
const COMCTL32_DLL_NAME: &str = "comctl32.dll";

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

/// `CreatePage`, exported as `@ stub -noname` in `dlls/comctl32/comctl32.spec`.
pub fn create_page() -> StubException {
    wine_spec_unimplemented_stub(COMCTL32_DLL_NAME, "CreatePage")
}

/// `CreateProxyPage`, exported as `@ stub -noname` in `dlls/comctl32/comctl32.spec`.
pub fn create_proxy_page() -> StubException {
    wine_spec_unimplemented_stub(COMCTL32_DLL_NAME, "CreateProxyPage")
}

/// `DoReaderMode`, exported as `@ stub -noname` in `dlls/comctl32/comctl32.spec`.
pub fn do_reader_mode() -> StubException {
    wine_spec_unimplemented_stub(COMCTL32_DLL_NAME, "DoReaderMode")
}

/// `SHGetProcessDword`, exported as `@ stub -noname` in `dlls/comctl32/comctl32.spec`.
pub fn sh_get_process_dword() -> StubException {
    wine_spec_unimplemented_stub(COMCTL32_DLL_NAME, "SHGetProcessDword")
}
