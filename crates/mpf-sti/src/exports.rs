/// How `dlls/sti/sti.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`, possibly `-private`: a body in `sti.c` or `sti_main.c`, or a forward to the
    /// midl proxy-stub for `DllGetClassObject`, `DllRegisterServer` and `DllUnregisterServer`.
    Function,
    /// `@ stdcall -private` with no body in `sti`: `DllCanUnloadNow`, which winecrt0 supplies.
    Winecrt0Default,
    /// `@ stdcall NAME(...) TARGET`: the export is the same function as `TARGET`.
    Alias(&'static str),
}

/// The `sti.spec` export table, in spec order. The spec has no `@ stub` entries.
pub const EXPORTS: [(&str, ExportKind); 7] = [
    ("DllCanUnloadNow", ExportKind::Winecrt0Default),
    ("DllGetClassObject", ExportKind::Function),
    ("DllRegisterServer", ExportKind::Function),
    ("DllUnregisterServer", ExportKind::Function),
    ("StiCreateInstance", ExportKind::Alias("StiCreateInstanceW")),
    ("StiCreateInstanceA", ExportKind::Function),
    ("StiCreateInstanceW", ExportKind::Function),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
