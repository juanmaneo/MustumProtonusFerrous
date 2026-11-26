/// How `dlls/windowscodecsext/windowscodecsext.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function in `main.c`.
    Stdcall,
    /// `@ stdcall NAME(args) IMPL`: `NAME` is exported and implemented by `IMPL` in `main.c`.
    Alias,
    /// `@ stdcall -private`: a C function in `main.c` that winebuild leaves out of the import library.
    Private,
}

use ExportKind::{Alias, Private, Stdcall};

/// The `windowscodecsext.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 3] = [
    ("DllGetClassObject", Private),
    ("IWICColorTransform_Initialize_Proxy", Alias),
    ("WICCreateColorTransform_Proxy", Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
