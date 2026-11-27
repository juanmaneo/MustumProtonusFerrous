/// How `dlls/ctl3dv2.dll16/ctl3dv2.dll16.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `pascal`: a real function in `ctl3d.c`, ported as the function of the same name in this crate.
    Pascal,
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `ctl3dv2.dll16.spec` export table, in spec order. Ordinals 14 and 15 are not in the spec.
pub const EXPORTS: [(&str, ExportKind); 25] = [
    ("Ctl3dGetVer", ExportKind::Pascal),
    ("Ctl3dSubclassDlg", ExportKind::Pascal),
    ("Ctl3dSubclassCtl", ExportKind::Pascal),
    ("Ctl3dCtlColor", ExportKind::Pascal),
    ("Ctl3dEnabled", ExportKind::Pascal),
    ("Ctl3dColorChange", ExportKind::Pascal),
    ("BtnWndProc3d", ExportKind::Pascal),
    ("EditWndProc3d", ExportKind::Pascal),
    ("ListWndProc3d", ExportKind::Pascal),
    ("ComboWndProc3d", ExportKind::Pascal),
    ("StaticWndProc3d", ExportKind::Pascal),
    ("Ctl3dRegister", ExportKind::Pascal),
    ("Ctl3dUnregister", ExportKind::Pascal),
    ("Ctl3dAutoSubclass", ExportKind::Pascal),
    ("Ctl3dDlgProc", ExportKind::Pascal),
    ("Ctl3dCtlColorEx", ExportKind::Pascal),
    ("Ctl3dSetStyle", ExportKind::Stub),
    ("Ctl3dDlgFramePaint", ExportKind::Pascal),
    ("Ctl3dSubclassDlgEx", ExportKind::Pascal),
    ("Ctl3dWinIniChange", ExportKind::Pascal),
    ("Ctl3dIsAutoSubclass", ExportKind::Pascal),
    ("Ctl3dUnAutoSubclass", ExportKind::Pascal),
    ("Ctl3dSubclassCtlEx", ExportKind::Pascal),
    ("Ctl3dUnsubclassCtl", ExportKind::Pascal),
    ("Ctl3dAutoSubclassEx", ExportKind::Pascal),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
