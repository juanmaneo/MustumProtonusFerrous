/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = softpub.dll` in `dlls/softpub/Makefile.in`.
const SOFTPUB_DLL_NAME: &str = "softpub.dll";

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

/// `SoftpubFreeDefUsageCallData`, exported as `stub` in `dlls/softpub/softpub.spec`.
pub fn softpub_free_def_usage_call_data() -> StubException {
    wine_spec_unimplemented_stub(SOFTPUB_DLL_NAME, "SoftpubFreeDefUsageCallData")
}

/// `SoftpubLoadDefUsageCallData`, exported as `stub` in `dlls/softpub/softpub.spec`.
pub fn softpub_load_def_usage_call_data() -> StubException {
    wine_spec_unimplemented_stub(SOFTPUB_DLL_NAME, "SoftpubLoadDefUsageCallData")
}

/// `AddPersonalTrustDBPages`, exported as `stub` in `dlls/softpub/softpub.spec`.
pub fn add_personal_trust_db_pages() -> StubException {
    wine_spec_unimplemented_stub(SOFTPUB_DLL_NAME, "AddPersonalTrustDBPages")
}
