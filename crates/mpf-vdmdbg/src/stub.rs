/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = vdmdbg.dll` in `dlls/vdmdbg/Makefile.in`.
const VDMDBG_DLL_NAME: &str = "vdmdbg.dll";

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

/// `VDMBreakThread`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_break_thread() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMBreakThread")
}

/// `VDMDetectWOW`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_detect_wow() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMDetectWOW")
}

/// `VDMEnumTaskWOWEx`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_enum_task_wow_ex() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMEnumTaskWOWEx")
}

/// `VDMGetModuleSelector`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_get_module_selector() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGetModuleSelector")
}

/// `VDMGetPointer`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_get_pointer() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGetPointer")
}

/// `VDMGetSelectorModule`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_get_selector_module() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGetSelectorModule")
}

/// `VDMGetThreadContext`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_get_thread_context() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGetThreadContext")
}

/// `VDMGetThreadSelectorEntry`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_get_thread_selector_entry() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGetThreadSelectorEntry")
}

/// `VDMGlobalFirst`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_global_first() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGlobalFirst")
}

/// `VDMGlobalNext`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_global_next() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMGlobalNext")
}

/// `VDMKillWOW`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_kill_wow() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMKillWOW")
}

/// `VDMModuleFirst`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_module_first() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMModuleFirst")
}

/// `VDMModuleNext`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_module_next() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMModuleNext")
}

/// `VDMProcessException`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_process_exception() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMProcessException")
}

/// `VDMSetThreadContext`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_set_thread_context() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMSetThreadContext")
}

/// `VDMStartTaskInWOW`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_start_task_in_wow() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMStartTaskInWOW")
}

/// `VDMTerminateTaskWOW`, exported as `@ stub` in `dlls/vdmdbg/vdmdbg.spec`.
pub fn vdm_terminate_task_wow() -> StubException {
    wine_spec_unimplemented_stub(VDMDBG_DLL_NAME, "VDMTerminateTaskWOW")
}
