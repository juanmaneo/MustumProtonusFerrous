/// How `dlls/wofutil/wofutil.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a C function in `main.c`, ported as a free function in [`crate::calls`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Stdcall, Stub};

/// The `wofutil.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 11] = [
    ("WofEnumEntries", Stub),
    ("WofFileEnumFiles", Stub),
    ("WofGetDriverVersion", Stub),
    ("WofIsExternalFile", Stdcall),
    ("WofSetFileDataLocation", Stub),
    ("WofShouldCompressBinaries", Stdcall),
    ("WofWimAddEntry", Stub),
    ("WofWimEnumFiles", Stub),
    ("WofWimRemoveEntry", Stub),
    ("WofWimSuspendEntry", Stub),
    ("WofWimUpdateEntry", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
