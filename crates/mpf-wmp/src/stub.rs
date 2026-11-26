/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wmp.dll` in `dlls/wmp/Makefile.in`.
const WMP_DLL_NAME: &str = "wmp.dll";

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

/// `@ stub` export 3000 of `dlls/wmp/wmp.spec`, which has no name. The function is the decimal
/// ordinal.
pub fn ordinal_3000() -> StubException {
    wine_spec_unimplemented_stub(WMP_DLL_NAME, "3000")
}

/// `@ stub` export 3002 of `dlls/wmp/wmp.spec`, which has no name.
pub fn ordinal_3002() -> StubException {
    wine_spec_unimplemented_stub(WMP_DLL_NAME, "3002")
}

/// `@ stub` export 3003 of `dlls/wmp/wmp.spec`, which has no name.
pub fn ordinal_3003() -> StubException {
    wine_spec_unimplemented_stub(WMP_DLL_NAME, "3003")
}

/// `@ stub` export 3004 of `dlls/wmp/wmp.spec`, which has no name.
pub fn ordinal_3004() -> StubException {
    wine_spec_unimplemented_stub(WMP_DLL_NAME, "3004")
}
