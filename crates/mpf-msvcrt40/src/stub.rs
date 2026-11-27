/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msvcrt40.dll` in `dlls/msvcrt40/Makefile.in`.
pub const MSVCRT40_DLL_NAME: &str = "msvcrt40.dll";

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

/// `??_EIostream_init@@QAEPAXI@Z`: the vector deleting destructor of `Iostream_init`, a `@ stub`
/// export in `dlls/msvcrt40/msvcrt40.spec`.
pub fn iostream_init_vector_deleting_dtor() -> StubException {
    wine_spec_unimplemented_stub(MSVCRT40_DLL_NAME, "??_EIostream_init@@QAEPAXI@Z")
}

/// `??_GIostream_init@@QAEPAXI@Z`: the scalar deleting destructor of `Iostream_init`, a `@ stub`
/// export in `dlls/msvcrt40/msvcrt40.spec`.
pub fn iostream_init_scalar_deleting_dtor() -> StubException {
    wine_spec_unimplemented_stub(MSVCRT40_DLL_NAME, "??_GIostream_init@@QAEPAXI@Z")
}
