/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = vcruntime140.dll` in `dlls/vcruntime140/Makefile.in`.
const VCRUNTIME140_DLL_NAME: &str = "vcruntime140.dll";

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

/// The record for the `@ stub` export `name`, or `None` when `name` is not a stub export.
pub fn stub_for_export(name: &str) -> Option<StubException> {
    Some(match name {
        "_NLG_Dispatch2" => _nlg_dispatch2(),
        "_NLG_Return" => _nlg_return(),
        "_NLG_Return2" => _nlg_return2(),
        "__BuildCatchObject" => __build_catch_object(),
        "__BuildCatchObjectHelper" => __build_catch_object_helper(),
        "__C_specific_handler_noexcept" => __c_specific_handler_noexcept(),
        "__FrameUnwindFilter" => __frame_unwind_filter(),
        "__GetPlatformExceptionInfo" => __get_platform_exception_info(),
        "__NLG_Dispatch2" => __nlg_dispatch2(),
        "__NLG_Return2" => __nlg_return2(),
        "__TypeMatch" => __type_match(),
        "__report_gsfailure" => __report_gsfailure(),
        "__vcrt_GetModuleFileNameW" => __vcrt_get_module_file_name_w(),
        "__vcrt_GetModuleHandleW" => __vcrt_get_module_handle_w(),
        "__vcrt_LoadLibraryExW" => __vcrt_load_library_ex_w(),
        "_is_exception_typeof" => _is_exception_typeof(),
        "unexpected" => unexpected(),
        _ => return None,
    })
}

/// `_NLG_Dispatch2`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn _nlg_dispatch2() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "_NLG_Dispatch2")
}

/// `_NLG_Return`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn _nlg_return() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "_NLG_Return")
}

/// `_NLG_Return2`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn _nlg_return2() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "_NLG_Return2")
}

/// `__BuildCatchObject`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __build_catch_object() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__BuildCatchObject")
}

/// `__BuildCatchObjectHelper`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __build_catch_object_helper() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__BuildCatchObjectHelper")
}

/// `__C_specific_handler_noexcept`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __c_specific_handler_noexcept() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__C_specific_handler_noexcept")
}

/// `__FrameUnwindFilter`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __frame_unwind_filter() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__FrameUnwindFilter")
}

/// `__GetPlatformExceptionInfo`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __get_platform_exception_info() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__GetPlatformExceptionInfo")
}

/// `__NLG_Dispatch2`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __nlg_dispatch2() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__NLG_Dispatch2")
}

/// `__NLG_Return2`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __nlg_return2() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__NLG_Return2")
}

/// `__TypeMatch`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __type_match() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__TypeMatch")
}

/// `__report_gsfailure`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __report_gsfailure() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__report_gsfailure")
}

/// `__vcrt_GetModuleFileNameW`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __vcrt_get_module_file_name_w() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__vcrt_GetModuleFileNameW")
}

/// `__vcrt_GetModuleHandleW`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __vcrt_get_module_handle_w() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__vcrt_GetModuleHandleW")
}

/// `__vcrt_LoadLibraryExW`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn __vcrt_load_library_ex_w() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "__vcrt_LoadLibraryExW")
}

/// `_is_exception_typeof`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn _is_exception_typeof() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "_is_exception_typeof")
}

/// `unexpected`, exported as `@ stub` in `dlls/vcruntime140/vcruntime140.spec`.
pub fn unexpected() -> StubException {
    wine_spec_unimplemented_stub(VCRUNTIME140_DLL_NAME, "unexpected")
}
