/// How `dlls/winbrand/winbrand.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::branding_format_string`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `winbrand.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 15] = [
    ("BrandingFormatString", ExportKind::Stdcall),
    ("BrandingFormatStringForEdition", ExportKind::Stub),
    ("BrandingLoadBitmap", ExportKind::Stub),
    ("BrandingLoadCursor", ExportKind::Stub),
    ("BrandingLoadIcon", ExportKind::Stub),
    ("BrandingLoadImage", ExportKind::Stub),
    ("BrandingLoadNeutralStringForEdition", ExportKind::Stub),
    ("BrandingLoadString", ExportKind::Stub),
    ("BrandingLoadStringForEdition", ExportKind::Stub),
    ("GetEULAFile", ExportKind::Stub),
    ("GetEULAFileEx", ExportKind::Stub),
    ("GetEULAInCurrentUILanguage", ExportKind::Stub),
    ("GetHinstanceByNamespace", ExportKind::Stub),
    ("GetInstalledEULAPath", ExportKind::Stub),
    ("InstallEULA", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
