/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = rtutils.dll` in `dlls/rtutils/Makefile.in`.
const RTUTILS_DLL_NAME: &str = "rtutils.dll";

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

/// `CreateWaitEvent`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn create_wait_event() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "CreateWaitEvent")
}

/// `CreateWaitEventBinding`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn create_wait_event_binding() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "CreateWaitEventBinding")
}

/// `CreateWaitTimer`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn create_wait_timer() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "CreateWaitTimer")
}

/// `DeRegisterWaitEventBinding`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn de_register_wait_event_binding() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "DeRegisterWaitEventBinding")
}

/// `DeRegisterWaitEventBindingSelf`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn de_register_wait_event_binding_self() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "DeRegisterWaitEventBindingSelf")
}

/// `DeRegisterWaitEventsTimers`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn de_register_wait_events_timers() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "DeRegisterWaitEventsTimers")
}

/// `DeRegisterWaitEventsTimersSelf`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn de_register_wait_events_timers_self() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "DeRegisterWaitEventsTimersSelf")
}

/// `DebugPrintWaitWorkerThreads`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn debug_print_wait_worker_threads() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "DebugPrintWaitWorkerThreads")
}

/// `LogErrorA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn log_error_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "LogErrorA")
}

/// `LogErrorW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn log_error_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "LogErrorW")
}

/// `LogEventA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn log_event_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "LogEventA")
}

/// `LogEventW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn log_event_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "LogEventW")
}

/// `MprSetupProtocolEnum`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn mpr_setup_protocol_enum() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "MprSetupProtocolEnum")
}

/// `MprSetupProtocolFree`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn mpr_setup_protocol_free() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "MprSetupProtocolFree")
}

/// `QueueWorkItem`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn queue_work_item() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "QueueWorkItem")
}

/// `RegisterWaitEventBinding`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn register_wait_event_binding() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RegisterWaitEventBinding")
}

/// `RegisterWaitEventsTimers`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn register_wait_events_timers() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RegisterWaitEventsTimers")
}

/// `RouterAssert`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_assert() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterAssert")
}

/// `RouterGetErrorStringA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_get_error_string_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterGetErrorStringA")
}

/// `RouterGetErrorStringW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_get_error_string_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterGetErrorStringW")
}

/// `RouterLogDeregisterA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_deregister_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogDeregisterA")
}

/// `RouterLogDeregisterW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_deregister_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogDeregisterW")
}

/// `RouterLogEventA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventA")
}

/// `RouterLogEventDataA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_data_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventDataA")
}

/// `RouterLogEventDataW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_data_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventDataW")
}

/// `RouterLogEventExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventExA")
}

/// `RouterLogEventExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventExW")
}

/// `RouterLogEventStringA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_string_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventStringA")
}

/// `RouterLogEventStringW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_string_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventStringW")
}

/// `RouterLogEventValistExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_valist_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventValistExA")
}

/// `RouterLogEventValistExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_valist_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventValistExW")
}

/// `RouterLogEventW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_event_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogEventW")
}

/// `RouterLogRegisterA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_register_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogRegisterA")
}

/// `RouterLogRegisterW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn router_log_register_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "RouterLogRegisterW")
}

/// `SetIoCompletionProc`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn set_io_completion_proc() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "SetIoCompletionProc")
}

/// `TraceDeregisterA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_deregister_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDeregisterA")
}

/// `TraceDeregisterExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_deregister_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDeregisterExA")
}

/// `TraceDeregisterExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_deregister_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDeregisterExW")
}

/// `TraceDeregisterW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_deregister_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDeregisterW")
}

/// `TraceDumpExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_dump_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDumpExA")
}

/// `TraceDumpExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_dump_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceDumpExW")
}

/// `TraceGetConsoleA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_get_console_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceGetConsoleA")
}

/// `TraceGetConsoleW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_get_console_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceGetConsoleW")
}

/// `TracePrintfA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_printf_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePrintfA")
}

/// `TracePrintfExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_printf_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePrintfExA")
}

/// `TracePrintfExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_printf_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePrintfExW")
}

/// `TracePrintfW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_printf_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePrintfW")
}

/// `TracePutsExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_puts_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePutsExA")
}

/// `TracePutsExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_puts_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TracePutsExW")
}

/// `TraceVprintfExA`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_vprintf_ex_a() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceVprintfExA")
}

/// `TraceVprintfExW`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn trace_vprintf_ex_w() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "TraceVprintfExW")
}

/// `UpdateWaitTimer`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn update_wait_timer() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "UpdateWaitTimer")
}

/// `WTFreeEvent`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn wt_free_event() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "WTFreeEvent")
}

/// `WTFreeTimer`, exported as `@ stub` in `dlls/rtutils/rtutils.spec`.
pub fn wt_free_timer() -> StubException {
    wine_spec_unimplemented_stub(RTUTILS_DLL_NAME, "WTFreeTimer")
}
