/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = atl80.dll` in `dlls/atl80/Makefile.in`.
const ATL80_DLL_NAME: &str = "atl80.dll";

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

/// `AtlSetErrorInfo`, exported as `@ stub` at ordinal 25 in `dlls/atl80/atl80.spec`.
pub fn atl_set_error_info() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlSetErrorInfo")
}

/// `AtlDevModeW2A`, exported as `@ stub` at ordinal 29 in `dlls/atl80/atl80.spec`.
pub fn atl_dev_mode_w2a() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlDevModeW2A")
}

/// `AtlWinModuleRegisterWndClassInfoW`, exported as `@ stub` at ordinal 45 in `dlls/atl80/atl80.spec`.
pub fn atl_win_module_register_wnd_class_info_w() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlWinModuleRegisterWndClassInfoW")
}

/// `AtlWinModuleRegisterWndClassInfoA`, exported as `@ stub` at ordinal 46 in `dlls/atl80/atl80.spec`.
pub fn atl_win_module_register_wnd_class_info_a() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlWinModuleRegisterWndClassInfoA")
}

/// `AtlUnRegisterTypeLib`, exported as `@ stub` at ordinal 55 in `dlls/atl80/atl80.spec`.
pub fn atl_un_register_type_lib() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlUnRegisterTypeLib")
}

/// `AtlWinModuleRegisterClassExW`, exported as `@ stub` at ordinal 62 in `dlls/atl80/atl80.spec`.
pub fn atl_win_module_register_class_ex_w() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlWinModuleRegisterClassExW")
}

/// `AtlWinModuleRegisterClassExA`, exported as `@ stub` at ordinal 63 in `dlls/atl80/atl80.spec`.
pub fn atl_win_module_register_class_ex_a() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlWinModuleRegisterClassExA")
}

/// `AtlWinModuleTerm`, exported as `@ stub` at ordinal 66 in `dlls/atl80/atl80.spec`.
pub fn atl_win_module_term() -> StubException {
    wine_spec_unimplemented_stub(ATL80_DLL_NAME, "AtlWinModuleTerm")
}
