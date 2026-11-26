/// How `dlls/rstrtmgr/rstrtmgr.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as the matching `rm_*` function in `api.rs`.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `rstrtmgr.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 12] = [
    ("RmAddFilter", ExportKind::Stdcall),
    ("RmCancelCurrentTask", ExportKind::Stub),
    ("RmEndSession", ExportKind::Stdcall),
    ("RmGetFilterList", ExportKind::Stub),
    ("RmGetList", ExportKind::Stdcall),
    ("RmJoinSession", ExportKind::Stub),
    ("RmRegisterResources", ExportKind::Stdcall),
    ("RmRemoveFilter", ExportKind::Stdcall),
    ("RmReserveHeap", ExportKind::Stub),
    ("RmRestart", ExportKind::Stdcall),
    ("RmShutdown", ExportKind::Stdcall),
    ("RmStartSession", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
