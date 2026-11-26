/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msimtf.dll` in `dlls/msimtf/Makefile.in`.
const MSIMTF_DLL_NAME: &str = "msimtf.dll";

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

/// `MsimtfIsGuidMapEnable`, exported as `@ stub` in `dlls/msimtf/msimtf.spec`.
pub fn msimtf_is_guid_map_enable() -> StubException {
    wine_spec_unimplemented_stub(MSIMTF_DLL_NAME, "MsimtfIsGuidMapEnable")
}

/// `MsimtfIsWindowFiltered`, exported as `@ stub` in `dlls/msimtf/msimtf.spec`.
pub fn msimtf_is_window_filtered() -> StubException {
    wine_spec_unimplemented_stub(MSIMTF_DLL_NAME, "MsimtfIsWindowFiltered")
}
