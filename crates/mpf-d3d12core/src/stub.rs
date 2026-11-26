/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3d12core.dll` in `dlls/d3d12core/Makefile.in`.
const D3D12CORE_DLL_NAME: &str = "d3d12core.dll";

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

/// `D3D12GetInterface`, exported as `@ stub` in `dlls/d3d12core/d3d12core.spec`.
pub fn d3d12_get_interface() -> StubException {
    wine_spec_unimplemented_stub(D3D12CORE_DLL_NAME, "D3D12GetInterface")
}

/// `D3D12SDKVersion`, exported as `@ stub` in `dlls/d3d12core/d3d12core.spec`.
pub fn d3d12_sdk_version() -> StubException {
    wine_spec_unimplemented_stub(D3D12CORE_DLL_NAME, "D3D12SDKVersion")
}
