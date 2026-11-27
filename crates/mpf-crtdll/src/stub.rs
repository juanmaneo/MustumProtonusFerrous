/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = crtdll.dll` in `dlls/crtdll/Makefile.in`.
const CRTDLL_DLL_NAME: &str = "crtdll.dll";

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

/// `_getsystime`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn getsystime() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_getsystime")
}

/// `_setsystime`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn setsystime() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_setsystime")
}

/// `_ismbbalnum`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbalnum() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbalnum")
}

/// `_ismbbalpha`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbalpha() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbalpha")
}

/// `_ismbbgraph`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbgraph() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbgraph")
}

/// `_ismbbkalnum`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbkalnum() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbkalnum")
}

/// `_ismbbkpunct`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbkpunct() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbkpunct")
}

/// `_ismbbprint`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbprint() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbprint")
}

/// `_ismbbpunct`, exported as `@ stub` in `dlls/crtdll/crtdll.spec`.
pub fn ismbbpunct() -> StubException {
    wine_spec_unimplemented_stub(CRTDLL_DLL_NAME, "_ismbbpunct")
}
