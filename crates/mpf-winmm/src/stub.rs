//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`, and the `@ stub` exports of
//! `dlls/winmm/winmm.spec`.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = winmm.dll` in `dlls/winmm/Makefile.in`.
const WINMM_DLL_NAME: &str = "winmm.dll";

/// The function argument of a stub: the export name, or the ordinal when the export has no name.
/// The `&'static str` of the fontsub port cannot hold an ordinal, so winmm needs this enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubFunction {
    Name(&'static str),
    Ordinal(u16),
}

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: StubFunction,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: StubFunction) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `mmioInstallIOProc16`, exported as `@ stub` in `dlls/winmm/winmm.spec`.
pub fn mmio_install_io_proc16() -> StubException {
    wine_spec_unimplemented_stub(WINMM_DLL_NAME, StubFunction::Name("mmioInstallIOProc16"))
}

/// The unnamed `@ stub` export at ordinal 3 in `dlls/winmm/winmm.spec`.
pub fn stub_ordinal_3() -> StubException {
    wine_spec_unimplemented_stub(WINMM_DLL_NAME, StubFunction::Ordinal(3))
}

/// The unnamed `@ stub` export at ordinal 4 in `dlls/winmm/winmm.spec`.
pub fn stub_ordinal_4() -> StubException {
    wine_spec_unimplemented_stub(WINMM_DLL_NAME, StubFunction::Ordinal(4))
}
