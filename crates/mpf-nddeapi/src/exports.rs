/// How `dlls/nddeapi/nddeapi.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a real function, ported as the `ndde_*` helper that returns its fixed value.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `nddeapi.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 28] = [
    ("NDdeShareAddA", ExportKind::Stdcall),
    ("NDdeShareDelA", ExportKind::Stdcall),
    ("NDdeShareEnumA", ExportKind::Stdcall),
    ("NDdeShareGetInfoA", ExportKind::Stdcall),
    ("NDdeShareSetInfoA", ExportKind::Stdcall),
    ("NDdeGetErrorStringA", ExportKind::Stdcall),
    ("NDdeIsValidShareNameA", ExportKind::Stdcall),
    ("NDdeIsValidAppTopicListA", ExportKind::Stdcall),
    ("NDdeGetShareSecurityA", ExportKind::Stdcall),
    ("NDdeSetShareSecurityA", ExportKind::Stdcall),
    ("NDdeGetTrustedShareA", ExportKind::Stdcall),
    ("NDdeSetTrustedShareA", ExportKind::Stdcall),
    ("NDdeTrustedShareEnumA", ExportKind::Stdcall),
    ("NDdeShareAddW", ExportKind::Stdcall),
    ("NDdeShareDelW", ExportKind::Stdcall),
    ("NDdeShareEnumW", ExportKind::Stdcall),
    ("NDdeShareGetInfoW", ExportKind::Stdcall),
    ("NDdeShareSetInfoW", ExportKind::Stdcall),
    ("NDdeGetErrorStringW", ExportKind::Stdcall),
    ("NDdeIsValidShareNameW", ExportKind::Stdcall),
    ("NDdeIsValidAppTopicListW", ExportKind::Stdcall),
    ("NDdeGetShareSecurityW", ExportKind::Stdcall),
    ("NDdeSetShareSecurityW", ExportKind::Stdcall),
    ("NDdeGetTrustedShareW", ExportKind::Stdcall),
    ("NDdeSetTrustedShareW", ExportKind::Stdcall),
    ("NDdeTrustedShareEnumW", ExportKind::Stdcall),
    ("NDdeSpecialCommandA", ExportKind::Stub),
    ("NDdeSpecialCommandW", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
