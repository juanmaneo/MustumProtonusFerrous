//! Oracle-derived test vectors for the ported rtutils logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `TraceRegisterExW` and
//! `TraceRegisterExA` (`dlls/rtutils/tracing.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/rtutils/`, and running the harness. The harness printed 63 rows
//! (one per export and one per `TraceRegisterEx` case). ASan and UBSan reported no errors,
//! so no row is excluded.

use super::*;

const WIDE_TRACE: &[u16] = &[0x54, 0x72, 0x61, 0x63, 0x65];

/// Calls `trace_register_ex_a` and returns the result with the size its allocator was asked for.
fn call_a(name: Option<&[u8]>, flags: u32, alloc_ok: bool) -> (TraceRegistration, usize) {
    let mut asked = 0;
    let reg = trace_register_ex_a(name, flags, |size| {
        asked = size;
        alloc_ok
    });
    (reg, asked)
}

/// `(helper, code, flags, number of parameters, module, function)` for each `@ stub` export.
type StubRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

const STUB_ROWS: [StubRow; 54] = [
    (
        create_wait_event,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "CreateWaitEvent",
    ), // row 01
    (
        create_wait_event_binding,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "CreateWaitEventBinding",
    ), // row 02
    (
        create_wait_timer,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "CreateWaitTimer",
    ), // row 03
    (
        de_register_wait_event_binding,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "DeRegisterWaitEventBinding",
    ), // row 04
    (
        de_register_wait_event_binding_self,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "DeRegisterWaitEventBindingSelf",
    ), // row 05
    (
        de_register_wait_events_timers,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "DeRegisterWaitEventsTimers",
    ), // row 06
    (
        de_register_wait_events_timers_self,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "DeRegisterWaitEventsTimersSelf",
    ), // row 07
    (
        debug_print_wait_worker_threads,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "DebugPrintWaitWorkerThreads",
    ), // row 08
    (log_error_a, 0x8000_0100, 0x1, 2, "rtutils.dll", "LogErrorA"), // row 09
    (log_error_w, 0x8000_0100, 0x1, 2, "rtutils.dll", "LogErrorW"), // row 10
    (log_event_a, 0x8000_0100, 0x1, 2, "rtutils.dll", "LogEventA"), // row 11
    (log_event_w, 0x8000_0100, 0x1, 2, "rtutils.dll", "LogEventW"), // row 12
    (
        mpr_setup_protocol_enum,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "MprSetupProtocolEnum",
    ), // row 13
    (
        mpr_setup_protocol_free,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "MprSetupProtocolFree",
    ), // row 14
    (
        queue_work_item,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "QueueWorkItem",
    ), // row 15
    (
        register_wait_event_binding,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RegisterWaitEventBinding",
    ), // row 16
    (
        register_wait_events_timers,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RegisterWaitEventsTimers",
    ), // row 17
    (
        router_assert,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterAssert",
    ), // row 18
    (
        router_get_error_string_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterGetErrorStringA",
    ), // row 19
    (
        router_get_error_string_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterGetErrorStringW",
    ), // row 20
    (
        router_log_deregister_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogDeregisterA",
    ), // row 21
    (
        router_log_deregister_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogDeregisterW",
    ), // row 22
    (
        router_log_event_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventA",
    ), // row 23
    (
        router_log_event_data_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventDataA",
    ), // row 24
    (
        router_log_event_data_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventDataW",
    ), // row 25
    (
        router_log_event_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventExA",
    ), // row 26
    (
        router_log_event_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventExW",
    ), // row 27
    (
        router_log_event_string_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventStringA",
    ), // row 28
    (
        router_log_event_string_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventStringW",
    ), // row 29
    (
        router_log_event_valist_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventValistExA",
    ), // row 30
    (
        router_log_event_valist_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventValistExW",
    ), // row 31
    (
        router_log_event_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogEventW",
    ), // row 32
    (
        router_log_register_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogRegisterA",
    ), // row 33
    (
        router_log_register_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "RouterLogRegisterW",
    ), // row 34
    (
        set_io_completion_proc,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "SetIoCompletionProc",
    ), // row 35
    (
        trace_deregister_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDeregisterA",
    ), // row 36
    (
        trace_deregister_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDeregisterExA",
    ), // row 37
    (
        trace_deregister_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDeregisterExW",
    ), // row 38
    (
        trace_deregister_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDeregisterW",
    ), // row 39
    (
        trace_dump_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDumpExA",
    ), // row 40
    (
        trace_dump_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceDumpExW",
    ), // row 41
    (
        trace_get_console_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceGetConsoleA",
    ), // row 42
    (
        trace_get_console_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceGetConsoleW",
    ), // row 43
    (
        trace_printf_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePrintfA",
    ), // row 44
    (
        trace_printf_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePrintfExA",
    ), // row 45
    (
        trace_printf_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePrintfExW",
    ), // row 46
    (
        trace_printf_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePrintfW",
    ), // row 47
    (
        trace_puts_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePutsExA",
    ), // row 48
    (
        trace_puts_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TracePutsExW",
    ), // row 49
    (
        trace_vprintf_ex_a,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceVprintfExA",
    ), // row 50
    (
        trace_vprintf_ex_w,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "TraceVprintfExW",
    ), // row 51
    (
        update_wait_timer,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "UpdateWaitTimer",
    ), // row 52
    (
        wt_free_event,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "WTFreeEvent",
    ), // row 53
    (
        wt_free_timer,
        0x8000_0100,
        0x1,
        2,
        "rtutils.dll",
        "WTFreeTimer",
    ), // row 54
];

#[test]
fn oracle_stub_rows() {
    for (helper, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_trace_register_ex_w_rows() {
    // row 01 w_null_flags0 -> ret=0xffffffff
    assert_eq!(trace_register_ex_w(None, 0), 0xFFFF_FFFF);
    // row 02 w_trace_flags_max -> ret=0xffffffff
    assert_eq!(
        trace_register_ex_w(Some(WIDE_TRACE), 0xFFFF_FFFF),
        0xFFFF_FFFF
    );
}

#[test]
fn oracle_trace_register_ex_a_rows() {
    // row 03 a_null_ok alloc_ok=1 -> ret=0xffffffff err=0 asize=0
    assert_eq!(
        call_a(None, 0, true),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: None
            },
            0
        )
    );
    // row 04 a_empty_ok alloc_ok=1 -> ret=0xffffffff err=0 asize=2
    assert_eq!(
        call_a(Some(b""), 0, true),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: None
            },
            2
        )
    );
    // row 05 a_trace_ok alloc_ok=1 -> ret=0xffffffff err=0 asize=12
    assert_eq!(
        call_a(Some(b"Trace"), 0xFFFF_FFFF, true),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: None
            },
            12
        )
    );
    // row 06 a_high_byte_ok alloc_ok=1 -> ret=0xffffffff err=0 asize=8
    assert_eq!(
        call_a(Some(b"\xe9t\xe9"), 0, true),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: None
            },
            8
        )
    );
    // row 07 a_embedded_nul_ok alloc_ok=1 -> ret=0xffffffff err=0 asize=6
    assert_eq!(
        call_a(Some(b"ab\0cd"), 0, true),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: None
            },
            6
        )
    );
    // row 08 a_trace_alloc_fail alloc_ok=0 -> ret=0xffffffff err=14 asize=12
    assert_eq!(
        call_a(Some(b"Trace"), 0, false),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: Some(14)
            },
            12
        )
    );
    // row 09 a_empty_alloc_fail alloc_ok=0 -> ret=0xffffffff err=14 asize=2
    assert_eq!(
        call_a(Some(b""), 0, false),
        (
            TraceRegistration {
                id: 0xFFFF_FFFF,
                last_error: Some(14)
            },
            2
        )
    );
}
