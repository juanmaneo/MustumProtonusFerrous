//! The `@ stub` exports of `dlls/inseng/inseng.spec` and the `__wine_spec_unimplemented_stub`
//! record they raise. All six stub exports are named, so the ordinal form never applies.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = inseng.dll` in `dlls/inseng/Makefile.in`.
const INSENG_DLL_NAME: &str = "inseng.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises the
/// record in a loop and never returns, so the port returns the record instead.
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

/// `CheckForVersionConflict`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn check_for_version_conflict() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "CheckForVersionConflict")
}

/// `CheckTrust`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn check_trust() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "CheckTrust")
}

/// `DownloadFile`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn download_file() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "DownloadFile")
}

/// `GetICifFileFromFile`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn get_icif_file_from_file() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "GetICifFileFromFile")
}

/// `GetICifRWFileFromFile`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn get_icif_rw_file_from_file() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "GetICifRWFileFromFile")
}

/// `PurgeDownloadDirectory`, exported as `@ stub` in `dlls/inseng/inseng.spec`.
pub fn purge_download_directory() -> StubException {
    wine_spec_unimplemented_stub(INSENG_DLL_NAME, "PurgeDownloadDirectory")
}
