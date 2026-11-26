/// How `dlls/fontsub/fontsub.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ cdecl`: a real function, ported as [`crate::create_font_package`].
    Cdecl,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `fontsub.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 2] = [
    ("CreateFontPackage", ExportKind::Cdecl),
    ("MergeFontPackage", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
