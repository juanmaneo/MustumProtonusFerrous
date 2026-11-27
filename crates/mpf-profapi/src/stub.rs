/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = profapi.dll` in `dlls/profapi/Makefile.in`.
const PROFAPI_DLL_NAME: &str = "profapi.dll";

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

/// `@ stub` export 101 of `dlls/profapi/profapi.spec`, which has no name. The function is the decimal ordinal.
pub fn ordinal_101() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "101")
}

/// `@ stub` export 102 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_102() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "102")
}

/// `@ stub` export 103 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_103() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "103")
}

/// `@ stub` export 104 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_104() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "104")
}

/// `@ stub` export 105 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_105() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "105")
}

/// `@ stub` export 106 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_106() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "106")
}

/// `@ stub` export 107 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_107() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "107")
}

/// `@ stub` export 108 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_108() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "108")
}

/// `@ stub` export 109 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_109() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "109")
}

/// `@ stub` export 110 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_110() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "110")
}

/// `@ stub` export 111 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_111() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "111")
}

/// `@ stub` export 112 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_112() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "112")
}

/// `@ stub` export 113 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_113() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "113")
}

/// `@ stub` export 114 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_114() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "114")
}

/// `@ stub` export 115 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_115() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "115")
}

/// `@ stub` export 116 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_116() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "116")
}

/// `@ stub` export 117 of `dlls/profapi/profapi.spec`, which has no name.
pub fn ordinal_117() -> StubException {
    wine_spec_unimplemented_stub(PROFAPI_DLL_NAME, "117")
}
