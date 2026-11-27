/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = srclient.dll` in `dlls/srclient/Makefile.in`.
const SRCLIENT_DLL_NAME: &str = "srclient.dll";

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

/// `SysprepCleanup`, exported as `1 stub` in `dlls/srclient/srclient.spec`.
pub fn sysprep_cleanup() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SysprepCleanup")
}

/// `SysprepGeneralize`, exported as `2 stub` in `dlls/srclient/srclient.spec`.
pub fn sysprep_generalize() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SysprepGeneralize")
}

/// `DisableSR`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn disable_sr() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "DisableSR")
}

/// `DisableSRInternal`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn disable_sr_internal() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "DisableSRInternal")
}

/// `EnableSR`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn enable_sr() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "EnableSR")
}

/// `EnableSREx`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn enable_sr_ex() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "EnableSREx")
}

/// `EnableSRInternal`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn enable_sr_internal() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "EnableSRInternal")
}

/// `SRNewSystemId`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn sr_new_system_id() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SRNewSystemId")
}

/// `SRRemoveRestorePoint`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn sr_remove_restore_point() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SRRemoveRestorePoint")
}

/// `SRSetRestorePointInternal`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn sr_set_restore_point_internal() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SRSetRestorePointInternal")
}

/// `SetSRStateAfterSetup`, exported as `@ stub` in `dlls/srclient/srclient.spec`.
pub fn set_sr_state_after_setup() -> StubException {
    wine_spec_unimplemented_stub(SRCLIENT_DLL_NAME, "SetSRStateAfterSetup")
}
