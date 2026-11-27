/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = olethk32.dll` in
/// `dlls/olethk32/Makefile.in`.
const OLETHK32_DLL_NAME: &str = "olethk32.dll";

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

/// `InvokeOn32`, `@ stub` ordinal 3 in `dlls/olethk32/olethk32.spec`.
pub fn invoke_on_32() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "InvokeOn32")
}

/// `IntOpInitialize`, `@ stub` ordinal 4 in `dlls/olethk32/olethk32.spec`.
pub fn int_op_initialize() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "IntOpInitialize")
}

/// `CallbackProcessing_3216`, `@ stub` ordinal 5 in `dlls/olethk32/olethk32.spec`.
pub fn callback_processing_3216() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "CallbackProcessing_3216")
}

/// `IUnknownObj32`, `@ stub` ordinal 6 in `dlls/olethk32/olethk32.spec`.
pub fn i_unknown_obj32() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "IUnknownObj32")
}

/// `CSm16ReleaseHandler_Release32`, `@ stub` ordinal 8 in `dlls/olethk32/olethk32.spec`.
pub fn c_sm16_release_handler_release32() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "CSm16ReleaseHandler_Release32")
}

/// `ThkMgrInitialize`, `@ stub` ordinal 9 in `dlls/olethk32/olethk32.spec`.
pub fn thk_mgr_initialize() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ThkMgrInitialize")
}

/// `ThkMgrUninitialize`, `@ stub` ordinal 10 in `dlls/olethk32/olethk32.spec`.
pub fn thk_mgr_uninitialize() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ThkMgrUninitialize")
}

/// `TransformHRESULT_1632`, `@ stub` ordinal 11 in `dlls/olethk32/olethk32.spec`.
pub fn transform_hresult_1632() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "TransformHRESULT_1632")
}

/// `TransformHRESULT_3216`, `@ stub` ordinal 12 in `dlls/olethk32/olethk32.spec`.
pub fn transform_hresult_3216() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "TransformHRESULT_3216")
}

/// `ConvertObjDescriptor`, `@ stub` ordinal 13 in `dlls/olethk32/olethk32.spec`.
pub fn convert_obj_descriptor() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ConvertObjDescriptor")
}

/// `ConvertHr1632Thunk`, `@ stub` ordinal 14 in `dlls/olethk32/olethk32.spec`.
pub fn convert_hr1632_thunk() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ConvertHr1632Thunk")
}

/// `ConvertHr3216Thunk`, `@ stub` ordinal 15 in `dlls/olethk32/olethk32.spec`.
pub fn convert_hr3216_thunk() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ConvertHr3216Thunk")
}

/// `IntOpUninitialize`, `@ stub` ordinal 16 in `dlls/olethk32/olethk32.spec`.
pub fn int_op_uninitialize() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "IntOpUninitialize")
}

/// `ThkAddAppCompatFlag`, `@ stub` ordinal 18 in `dlls/olethk32/olethk32.spec`.
pub fn thk_add_app_compat_flag() -> StubException {
    wine_spec_unimplemented_stub(OLETHK32_DLL_NAME, "ThkAddAppCompatFlag")
}
