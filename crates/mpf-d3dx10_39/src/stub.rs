/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// Module name from `MODULE = d3dx10_39.dll` in `dlls/d3dx10_39/Makefile.in`.
const MODULE_NAME: &str = "d3dx10_39.dll";

/// The exception record raised by `__wine_spec_unimplemented_stub`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Models Wine's `__wine_spec_unimplemented_stub`; the C function raises this record forever.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

/// `D3DX10CreateReduction`, exported as `@ stub` in `dlls/d3dx10_39/d3dx10_39.spec`.
pub fn d3dx10_create_reduction() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DX10CreateReduction")
}

/// `D3DX10DisassembleEffect`, exported as `@ stub` in `dlls/d3dx10_39/d3dx10_39.spec`.
pub fn d3dx10_disassemble_effect() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DX10DisassembleEffect")
}

/// `D3DX10DisassembleShader`, exported as `@ stub` in `dlls/d3dx10_39/d3dx10_39.spec`.
pub fn d3dx10_disassemble_shader() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DX10DisassembleShader")
}

/// `D3DX10ReflectShader`, exported as `@ stub` in `dlls/d3dx10_39/d3dx10_39.spec`.
pub fn d3dx10_reflect_shader() -> StubException {
    wine_spec_unimplemented_stub(MODULE_NAME, "D3DX10ReflectShader")
}
