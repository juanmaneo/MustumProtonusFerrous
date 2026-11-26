/// How `dlls/msftedit/msftedit.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `extern IID_IRichEditOle` or `extern IID_IRichEditOleCallback`: GUID data this DLL exports
    /// with no forward target in the spec.
    Data,
    /// A `module.name` target, such as `riched20.CreateTextServices`: forwarded to `riched20`.
    /// This covers both code and `extern` data, and this DLL has no code for either.
    Forward(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `stdcall -private DllGetVersion(ptr)`: the `libs/winecrt0` default, see [`crate::dll_get_version`].
    WinecrtDefault,
}

/// The `msftedit.spec` export table, in spec order. The export ordinals are not included.
pub const EXPORTS: [(&str, ExportKind); 14] = [
    ("IID_IRichEditOle", ExportKind::Data),
    ("IID_IRichEditOleCallback", ExportKind::Data),
    (
        "CreateTextServices",
        ExportKind::Forward("riched20.CreateTextServices"),
    ),
    (
        "IID_ITextServices",
        ExportKind::Forward("riched20.IID_ITextServices"),
    ),
    (
        "IID_ITextHost",
        ExportKind::Forward("riched20.IID_ITextHost"),
    ),
    (
        "IID_ITextHost2",
        ExportKind::Forward("riched20.IID_ITextHost2"),
    ),
    (
        "REExtendedRegisterClass",
        ExportKind::Forward("riched20.REExtendedRegisterClass"),
    ),
    (
        "RichEdit10ANSIWndProc",
        ExportKind::Forward("riched20.RichEdit10ANSIWndProc"),
    ),
    (
        "RichEditANSIWndProc",
        ExportKind::Forward("riched20.RichEditANSIWndProc"),
    ),
    ("SetCustomTextOutHandlerEx", ExportKind::Stub),
    ("DllGetVersion", ExportKind::WinecrtDefault),
    ("RichEditWndProc", ExportKind::Stub),
    ("RichListBoxWndProc", ExportKind::Stub),
    ("RichComboBoxWndProc", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
