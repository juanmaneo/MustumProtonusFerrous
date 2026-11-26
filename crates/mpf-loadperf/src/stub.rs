/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = loadperf.dll` in
/// `dlls/loadperf/Makefile.in`.
const LOADPERF_DLL_NAME: &str = "loadperf.dll";

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

/// `BackupPerfRegistryToFileW`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn backup_perf_registry_to_file_w() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "BackupPerfRegistryToFileW")
}

/// `LoadMofFromInstalledServiceA`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn load_mof_from_installed_service_a() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "LoadMofFromInstalledServiceA")
}

/// `LoadMofFromInstalledServiceW`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn load_mof_from_installed_service_w() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "LoadMofFromInstalledServiceW")
}

/// `RestorePerfRegistryFromFileW`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn restore_perf_registry_from_file_w() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "RestorePerfRegistryFromFileW")
}

/// `SetServiceAsTrustedA`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn set_service_as_trusted_a() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "SetServiceAsTrustedA")
}

/// `SetServiceAsTrustedW`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn set_service_as_trusted_w() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "SetServiceAsTrustedW")
}

/// `UpdatePerfNameFilesA`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn update_perf_name_files_a() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "UpdatePerfNameFilesA")
}

/// `UpdatePerfNameFilesW`, exported as `@ stub` in `dlls/loadperf/loadperf.spec`.
pub fn update_perf_name_files_w() -> StubException {
    wine_spec_unimplemented_stub(LOADPERF_DLL_NAME, "UpdatePerfNameFilesW")
}
