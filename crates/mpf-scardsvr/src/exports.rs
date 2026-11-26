/// How `dlls/scardsvr/scardsvr.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private`: a real function, ported as [`crate::calais_main`].
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `scardsvr.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 2] = [
    ("CalaisMain", ExportKind::Private),
    ("InitSmartCardService", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
