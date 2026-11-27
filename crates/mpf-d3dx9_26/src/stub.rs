use crate::exports::{export_kind, ExportKind};

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = d3dx9_26.dll` in `dlls/d3dx9_26/Makefile.in`.
const D3DX9_26_DLL_NAME: &str = "d3dx9_26.dll";

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

/// Returns the record that the export `function` raises when `d3dx9_26.spec` declares it as
/// `@ stub`. Returns `None` for `@ stdcall` exports and for names the spec does not declare.
pub fn stub_exception(function: &'static str) -> Option<StubException> {
    match export_kind(function)? {
        ExportKind::Stub => Some(wine_spec_unimplemented_stub(D3DX9_26_DLL_NAME, function)),
        ExportKind::Stdcall => None,
    }
}
