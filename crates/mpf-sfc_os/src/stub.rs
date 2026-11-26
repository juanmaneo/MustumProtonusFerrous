/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = sfc_os.dll` in `dlls/sfc_os/Makefile.in`.
const SFC_OS_DLL_NAME: &str = "sfc_os.dll";

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

/// `BeginFileMapEnumeration`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn begin_file_map_enumeration() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "BeginFileMapEnumeration")
}

/// `CloseFileMapEnumeration`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn close_file_map_enumeration() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "CloseFileMapEnumeration")
}

/// `GetNextFileMapContent`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn get_next_file_map_content() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "GetNextFileMapContent")
}

/// `SfcClose`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_close() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcClose")
}

/// `SfcFileException`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_file_exception() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcFileException")
}

/// `SfcInitProt`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_init_prot() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcInitProt")
}

/// `SfcInitiateScan`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_initiate_scan() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcInitiateScan")
}

/// `SfcInstallProtectedFiles`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_install_protected_files() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcInstallProtectedFiles")
}

/// `SfcTerminateWatcherThread`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfc_terminate_watcher_thread() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfcTerminateWatcherThread")
}

/// `SfpDeleteCatalog`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfp_delete_catalog() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfpDeleteCatalog")
}

/// `SfpInstallCatalog`, exported as `@ stub` in `dlls/sfc_os/sfc_os.spec`.
pub fn sfp_install_catalog() -> StubException {
    wine_spec_unimplemented_stub(SFC_OS_DLL_NAME, "SfpInstallCatalog")
}
