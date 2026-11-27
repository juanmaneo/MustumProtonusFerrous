/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

const D3DX10_37_DLL_NAME: &str = "d3dx10_37.dll";

/// The record raised by Wine's `__wine_spec_unimplemented_stub`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Models the exception record constructed by `__wine_spec_unimplemented_stub` in
/// `dlls/ntdll/exception.c`; the C function raises it forever, so this port returns the record.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// `D3DX10CreateReduction`, exported as `@ stub` in `dlls/d3dx10_37/d3dx10_37.spec`.
pub fn d3dx10_create_reduction() -> StubException {
    wine_spec_unimplemented_stub(D3DX10_37_DLL_NAME, "D3DX10CreateReduction")
}

/// `D3DX10GetDriverLevel`, exported as `@ stub` in `dlls/d3dx10_37/d3dx10_37.spec`.
pub fn d3dx10_get_driver_level() -> StubException {
    wine_spec_unimplemented_stub(D3DX10_37_DLL_NAME, "D3DX10GetDriverLevel")
}
