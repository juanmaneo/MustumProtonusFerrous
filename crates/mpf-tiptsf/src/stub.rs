//! The `@ stub` exports of `dlls/tiptsf/tiptsf.spec` and the `__wine_spec_unimplemented_stub`
//! record they raise.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = tiptsf.dll` in `dlls/tiptsf/Makefile.in`.
const TIPTSF_DLL_NAME: &str = "tiptsf.dll";

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

/// `AdviseHook`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn advise_hook() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "AdviseHook")
}

/// `EndCaretTracking`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn end_caret_tracking() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "EndCaretTracking")
}

/// `ProcessCaretEvents`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn process_caret_events() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "ProcessCaretEvents")
}

/// `ProcessCiceroCaretEvent`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn process_cicero_caret_event() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "ProcessCiceroCaretEvent")
}

/// `RestrictedModeMsgWndProc`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn restricted_mode_msg_wnd_proc() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "RestrictedModeMsgWndProc")
}

/// `StartCaretTracking`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn start_caret_tracking() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "StartCaretTracking")
}

/// `UnadviseHook`, exported as `@ stub` in `dlls/tiptsf/tiptsf.spec`.
pub fn unadvise_hook() -> StubException {
    wine_spec_unimplemented_stub(TIPTSF_DLL_NAME, "UnadviseHook")
}
