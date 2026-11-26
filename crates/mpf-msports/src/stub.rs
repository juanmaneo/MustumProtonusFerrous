/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msports.dll` in `dlls/msports/Makefile.in`.
const MSPORTS_DLL_NAME: &str = "msports.dll";

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

/// `ComDBClaimNextFreePort`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn com_db_claim_next_free_port() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ComDBClaimNextFreePort")
}

/// `ComDBClaimPort`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn com_db_claim_port() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ComDBClaimPort")
}

/// `ComDBGetCurrentPortUsage`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn com_db_get_current_port_usage() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ComDBGetCurrentPortUsage")
}

/// `ComDBReleasePort`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn com_db_release_port() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ComDBReleasePort")
}

/// `ComDBResizeDatabase`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn com_db_resize_database() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ComDBResizeDatabase")
}

/// `ParallelPortPropPageProvider`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn parallel_port_prop_page_provider() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "ParallelPortPropPageProvider")
}

/// `PortsClassInstaller`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn ports_class_installer() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "PortsClassInstaller")
}

/// `SerialDisplayAdvancedSettings`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn serial_display_advanced_settings() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "SerialDisplayAdvancedSettings")
}

/// `SerialPortPropPageProvider`, exported as `@ stub` in `dlls/msports/msports.spec`.
pub fn serial_port_prop_page_provider() -> StubException {
    wine_spec_unimplemented_stub(MSPORTS_DLL_NAME, "SerialPortPropPageProvider")
}
