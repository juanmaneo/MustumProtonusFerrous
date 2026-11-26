/// How `dlls/hrtfapo/hrtfapo.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::create_hrtf_apo`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `hrtfapo.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 5] = [
    ("CreateHrtfApo", ExportKind::Stdcall),
    ("CreateHrtfApoWithDatasetType", ExportKind::Stub),
    ("CreateHrtfEngineFactory", ExportKind::Stub),
    ("IsHrtfApoAvailable", ExportKind::Stub),
    ("GetHrtfEngineMinFrameCount", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
