//! The `@ stub` exports of `dlls/mmcndmgr/mmcndmgr.spec` and the `__wine_spec_unimplemented_stub`
//! record they raise.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mmcndmgr.dll` in
/// `dlls/mmcndmgr/Makefile.in`.
const MMCNDMGR_DLL_NAME: &str = "mmcndmgr.dll";

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

/// `CreateExecutivePlatform`, exported as `@ stub` in `dlls/mmcndmgr/mmcndmgr.spec`.
pub fn create_executive_platform() -> StubException {
    wine_spec_unimplemented_stub(MMCNDMGR_DLL_NAME, "CreateExecutivePlatform")
}

/// `_GetAllocCounters@0`, exported as `@ stub -arch=win32`. `winebuild` passes the spec name
/// unchanged, so the `@0` suffix is part of the function name.
pub fn get_alloc_counters_win32() -> StubException {
    wine_spec_unimplemented_stub(MMCNDMGR_DLL_NAME, "_GetAllocCounters@0")
}

/// `GetAllocCounters`, exported as `@ stub -arch=win64`.
pub fn get_alloc_counters_win64() -> StubException {
    wine_spec_unimplemented_stub(MMCNDMGR_DLL_NAME, "GetAllocCounters")
}
