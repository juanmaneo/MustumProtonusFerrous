/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = updspapi.dll` in `dlls/updspapi/Makefile.in`.
const UPDSPAPI_DLL_NAME: &str = "updspapi.dll";

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

/// `UpdSpSetDynamicStringExA`, exported as `@ stub` in `dlls/updspapi/updspapi.spec`.
pub fn upd_sp_set_dynamic_string_ex_a() -> StubException {
    wine_spec_unimplemented_stub(UPDSPAPI_DLL_NAME, "UpdSpSetDynamicStringExA")
}

/// `UpdSpSetDynamicStringExW`, exported as `@ stub` in `dlls/updspapi/updspapi.spec`.
pub fn upd_sp_set_dynamic_string_ex_w() -> StubException {
    wine_spec_unimplemented_stub(UPDSPAPI_DLL_NAME, "UpdSpSetDynamicStringExW")
}

/// `UpdSpSetDynamicStringW`, exported as `@ stub` in `dlls/updspapi/updspapi.spec`.
pub fn upd_sp_set_dynamic_string_w() -> StubException {
    wine_spec_unimplemented_stub(UPDSPAPI_DLL_NAME, "UpdSpSetDynamicStringW")
}

/// `UpdSpStringTableEnum`, exported as `@ stub` in `dlls/updspapi/updspapi.spec`.
pub fn upd_sp_string_table_enum() -> StubException {
    wine_spec_unimplemented_stub(UPDSPAPI_DLL_NAME, "UpdSpStringTableEnum")
}
