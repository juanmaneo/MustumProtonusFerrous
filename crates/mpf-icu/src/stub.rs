/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = icu.dll` in `dlls/icu/Makefile.in`.
const ICU_DLL_NAME: &str = "icu.dll";

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

/// `CloseDefaultICUGroupingLetters`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn close_default_icu_grouping_letters() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "CloseDefaultICUGroupingLetters")
}

/// `GetDefaultICUGroupingLetters`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn get_default_icu_grouping_letters() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "GetDefaultICUGroupingLetters")
}

/// `GetICUGroupingLetter`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn get_icu_grouping_letter() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "GetICUGroupingLetter")
}

/// `SortCloseHandle`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn sort_close_handle() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "SortCloseHandle")
}

/// `SortGetHandle`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn sort_get_handle() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "SortGetHandle")
}

/// `SortGetSearchKey`, exported as `@ stub` in `dlls/icu/icu.spec`.
pub fn sort_get_search_key() -> StubException {
    wine_spec_unimplemented_stub(ICU_DLL_NAME, "SortGetSearchKey")
}
