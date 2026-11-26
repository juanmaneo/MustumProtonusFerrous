/// How `dlls/rtutils/rtutils.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::trace_register_ex_a`] or
    /// [`crate::trace_register_ex_w`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `rtutils.spec` export table, in spec order (56 entries).
pub const EXPORTS: [(&str, ExportKind); 56] = [
    ("CreateWaitEvent", ExportKind::Stub),
    ("CreateWaitEventBinding", ExportKind::Stub),
    ("CreateWaitTimer", ExportKind::Stub),
    ("DeRegisterWaitEventBinding", ExportKind::Stub),
    ("DeRegisterWaitEventBindingSelf", ExportKind::Stub),
    ("DeRegisterWaitEventsTimers", ExportKind::Stub),
    ("DeRegisterWaitEventsTimersSelf", ExportKind::Stub),
    ("DebugPrintWaitWorkerThreads", ExportKind::Stub),
    ("LogErrorA", ExportKind::Stub),
    ("LogErrorW", ExportKind::Stub),
    ("LogEventA", ExportKind::Stub),
    ("LogEventW", ExportKind::Stub),
    ("MprSetupProtocolEnum", ExportKind::Stub),
    ("MprSetupProtocolFree", ExportKind::Stub),
    ("QueueWorkItem", ExportKind::Stub),
    ("RegisterWaitEventBinding", ExportKind::Stub),
    ("RegisterWaitEventsTimers", ExportKind::Stub),
    ("RouterAssert", ExportKind::Stub),
    ("RouterGetErrorStringA", ExportKind::Stub),
    ("RouterGetErrorStringW", ExportKind::Stub),
    ("RouterLogDeregisterA", ExportKind::Stub),
    ("RouterLogDeregisterW", ExportKind::Stub),
    ("RouterLogEventA", ExportKind::Stub),
    ("RouterLogEventDataA", ExportKind::Stub),
    ("RouterLogEventDataW", ExportKind::Stub),
    ("RouterLogEventExA", ExportKind::Stub),
    ("RouterLogEventExW", ExportKind::Stub),
    ("RouterLogEventStringA", ExportKind::Stub),
    ("RouterLogEventStringW", ExportKind::Stub),
    ("RouterLogEventValistExA", ExportKind::Stub),
    ("RouterLogEventValistExW", ExportKind::Stub),
    ("RouterLogEventW", ExportKind::Stub),
    ("RouterLogRegisterA", ExportKind::Stub),
    ("RouterLogRegisterW", ExportKind::Stub),
    ("SetIoCompletionProc", ExportKind::Stub),
    ("TraceDeregisterA", ExportKind::Stub),
    ("TraceDeregisterExA", ExportKind::Stub),
    ("TraceDeregisterExW", ExportKind::Stub),
    ("TraceDeregisterW", ExportKind::Stub),
    ("TraceDumpExA", ExportKind::Stub),
    ("TraceDumpExW", ExportKind::Stub),
    ("TraceGetConsoleA", ExportKind::Stub),
    ("TraceGetConsoleW", ExportKind::Stub),
    ("TracePrintfA", ExportKind::Stub),
    ("TracePrintfExA", ExportKind::Stub),
    ("TracePrintfExW", ExportKind::Stub),
    ("TracePrintfW", ExportKind::Stub),
    ("TracePutsExA", ExportKind::Stub),
    ("TracePutsExW", ExportKind::Stub),
    ("TraceRegisterExA", ExportKind::Stdcall),
    ("TraceRegisterExW", ExportKind::Stdcall),
    ("TraceVprintfExA", ExportKind::Stub),
    ("TraceVprintfExW", ExportKind::Stub),
    ("UpdateWaitTimer", ExportKind::Stub),
    ("WTFreeEvent", ExportKind::Stub),
    ("WTFreeTimer", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
