/// How `dlls/magnification/magnification.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: ported as [`crate::mag_initialize`] or [`crate::mag_uninitialize`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `magnification.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 21] = [
    ("MagSetFullscreenUseBitmapSmoothing", ExportKind::Stub),
    ("MagSetLensUseBitmapSmoothing", ExportKind::Stub),
    ("MagGetColorEffect", ExportKind::Stub),
    ("MagGetFullscreenColorEffect", ExportKind::Stub),
    ("MagGetFullscreenTransform", ExportKind::Stub),
    ("MagGetImageScalingCallback", ExportKind::Stub),
    ("MagGetInputTransform", ExportKind::Stub),
    ("MagGetWindowFilterList", ExportKind::Stub),
    ("MagGetWindowSource", ExportKind::Stub),
    ("MagGetWindowTransform", ExportKind::Stub),
    ("MagInitialize", ExportKind::Stdcall),
    ("MagSetColorEffect", ExportKind::Stub),
    ("MagSetFullscreenColorEffect", ExportKind::Stub),
    ("MagSetFullscreenTransform", ExportKind::Stub),
    ("MagSetImageScalingCallback", ExportKind::Stub),
    ("MagSetInputTransform", ExportKind::Stub),
    ("MagSetWindowFilterList", ExportKind::Stub),
    ("MagSetWindowSource", ExportKind::Stub),
    ("MagSetWindowTransform", ExportKind::Stub),
    ("MagShowSystemCursor", ExportKind::Stub),
    ("MagUninitialize", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
