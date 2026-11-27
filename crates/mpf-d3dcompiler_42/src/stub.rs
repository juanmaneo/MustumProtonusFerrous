/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3dcompiler_42.dll` in `dlls/d3dcompiler_42/Makefile.in`.
const D3DCOMPILER_42_DLL_NAME: &str = "d3dcompiler_42.dll";

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

/// `DebugSetMute`, exported as `@ stub` in `dlls/d3dcompiler_42/d3dcompiler_42.spec`.
pub fn debug_set_mute() -> StubException {
    wine_spec_unimplemented_stub(D3DCOMPILER_42_DLL_NAME, "DebugSetMute")
}

/// `D3DDisassemble10Effect`, exported as `@ stub` in `dlls/d3dcompiler_42/d3dcompiler_42.spec`.
pub fn d3d_disassemble10_effect() -> StubException {
    wine_spec_unimplemented_stub(D3DCOMPILER_42_DLL_NAME, "D3DDisassemble10Effect")
}

/// `D3DReturnFailure1`, exported as `@ stub` in `dlls/d3dcompiler_42/d3dcompiler_42.spec`.
pub fn d3d_return_failure1() -> StubException {
    wine_spec_unimplemented_stub(D3DCOMPILER_42_DLL_NAME, "D3DReturnFailure1")
}
