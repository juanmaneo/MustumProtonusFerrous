/// How `dlls/winnls32/winnls32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a C function in `winnls.c`, ported as a free function in [`crate::winnls32_enable_ime`]
    /// or [`crate::winnls32_get_enable_status`].
    Stdcall,
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

use ExportKind::{Stdcall, Stub};

/// The `winnls32.spec` export table, in spec order (by ordinal).
pub const EXPORTS: [(&str, ExportKind); 7] = [
    ("WINNLS32EnableIME", Stdcall),
    ("WINNLS32GetEnableStatus", Stdcall),
    ("WINNLS32GetIMEHotKey", Stub),
    ("IMP32GetIME", Stub),
    ("IMP32QueryIME", Stub),
    ("IMP32SetIME", Stub),
    ("IME32SendIMEMessageEx", Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
