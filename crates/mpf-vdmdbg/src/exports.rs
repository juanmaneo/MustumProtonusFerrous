/// How `dlls/vdmdbg/vdmdbg.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real C function, ported as [`crate::vdm_enum_process_wow`] or [`crate::vdm_enum_task_wow`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `vdmdbg.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 19] = [
    ("VDMBreakThread", ExportKind::Stub),
    ("VDMDetectWOW", ExportKind::Stub),
    ("VDMEnumProcessWOW", ExportKind::Stdcall),
    ("VDMEnumTaskWOW", ExportKind::Stdcall),
    ("VDMEnumTaskWOWEx", ExportKind::Stub),
    ("VDMGetModuleSelector", ExportKind::Stub),
    ("VDMGetPointer", ExportKind::Stub),
    ("VDMGetSelectorModule", ExportKind::Stub),
    ("VDMGetThreadContext", ExportKind::Stub),
    ("VDMGetThreadSelectorEntry", ExportKind::Stub),
    ("VDMGlobalFirst", ExportKind::Stub),
    ("VDMGlobalNext", ExportKind::Stub),
    ("VDMKillWOW", ExportKind::Stub),
    ("VDMModuleFirst", ExportKind::Stub),
    ("VDMModuleNext", ExportKind::Stub),
    ("VDMProcessException", ExportKind::Stub),
    ("VDMSetThreadContext", ExportKind::Stub),
    ("VDMStartTaskInWOW", ExportKind::Stub),
    ("VDMTerminateTaskWOW", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
