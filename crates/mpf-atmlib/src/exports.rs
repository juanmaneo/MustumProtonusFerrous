//! Export table of `dlls/atmlib/atmlib.spec`, in spec order.

/// How an export of `atmlib.spec` is implemented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function in `dlls/atmlib/main.c`.
    Stdcall,
    /// `@ stub`: calls `__wine_spec_unimplemented_stub`.
    Stub,
}

/// Every export of `atmlib.spec`, in spec order.
pub const EXPORTS: &[(&str, ExportKind)] = &[
    ("ATMAddFont", ExportKind::Stub),
    ("ATMAddFontA", ExportKind::Stub),
    ("ATMAddFontEx", ExportKind::Stub),
    ("ATMAddFontExA", ExportKind::Stub),
    ("ATMAddFontExW", ExportKind::Stub),
    ("ATMAddFontW", ExportKind::Stub),
    ("ATMBBoxBaseXYShowText", ExportKind::Stub),
    ("ATMBBoxBaseXYShowTextA", ExportKind::Stub),
    ("ATMBBoxBaseXYShowTextW", ExportKind::Stub),
    ("ATMBeginFontChange", ExportKind::Stub),
    ("ATMClient", ExportKind::Stub),
    ("ATMEndFontChange", ExportKind::Stub),
    ("ATMEnumFonts", ExportKind::Stub),
    ("ATMEnumFontsA", ExportKind::Stub),
    ("ATMEnumFontsW", ExportKind::Stub),
    ("ATMEnumMMFonts", ExportKind::Stub),
    ("ATMEnumMMFontsA", ExportKind::Stub),
    ("ATMEnumMMFontsW", ExportKind::Stub),
    ("ATMFinish", ExportKind::Stdcall),
    ("ATMFontAvailable", ExportKind::Stub),
    ("ATMFontAvailableA", ExportKind::Stub),
    ("ATMFontAvailableW", ExportKind::Stub),
    ("ATMFontSelected", ExportKind::Stub),
    ("ATMFontStatus", ExportKind::Stub),
    ("ATMFontStatusA", ExportKind::Stub),
    ("ATMFontStatusW", ExportKind::Stub),
    ("ATMForceFontChange", ExportKind::Stub),
    ("ATMGetBuildStr", ExportKind::Stub),
    ("ATMGetBuildStrA", ExportKind::Stub),
    ("ATMGetBuildStrW", ExportKind::Stub),
    ("ATMGetFontBBox", ExportKind::Stub),
    ("ATMGetFontInfo", ExportKind::Stub),
    ("ATMGetFontInfoA", ExportKind::Stub),
    ("ATMGetFontInfoW", ExportKind::Stub),
    ("ATMGetFontPaths", ExportKind::Stub),
    ("ATMGetFontPathsA", ExportKind::Stub),
    ("ATMGetFontPathsW", ExportKind::Stub),
    ("ATMGetGlyphList", ExportKind::Stub),
    ("ATMGetGlyphListA", ExportKind::Stub),
    ("ATMGetGlyphListW", ExportKind::Stub),
    ("ATMGetMenuName", ExportKind::Stub),
    ("ATMGetMenuNameA", ExportKind::Stub),
    ("ATMGetMenuNameW", ExportKind::Stub),
    ("ATMGetNtmFields", ExportKind::Stub),
    ("ATMGetNtmFieldsA", ExportKind::Stub),
    ("ATMGetNtmFieldsW", ExportKind::Stub),
    ("ATMGetOutline", ExportKind::Stub),
    ("ATMGetOutlineA", ExportKind::Stub),
    ("ATMGetOutlineW", ExportKind::Stub),
    ("ATMGetPostScriptName", ExportKind::Stub),
    ("ATMGetPostScriptNameA", ExportKind::Stub),
    ("ATMGetPostScriptNameW", ExportKind::Stub),
    ("ATMGetVersion", ExportKind::Stub),
    ("ATMGetVersionEx", ExportKind::Stub),
    ("ATMGetVersionExA", ExportKind::Stub),
    ("ATMGetVersionExW", ExportKind::Stub),
    ("ATMInstallSubstFontA", ExportKind::Stub),
    ("ATMInstallSubstFontW", ExportKind::Stub),
    ("ATMMakePFM", ExportKind::Stub),
    ("ATMMakePFMA", ExportKind::Stub),
    ("ATMMakePFMW", ExportKind::Stub),
    ("ATMMakePSS", ExportKind::Stub),
    ("ATMMakePSSA", ExportKind::Stub),
    ("ATMMakePSSW", ExportKind::Stub),
    ("ATMProperlyLoaded", ExportKind::Stdcall),
    ("ATMRemoveFont", ExportKind::Stub),
    ("ATMRemoveFontA", ExportKind::Stub),
    ("ATMRemoveFontW", ExportKind::Stub),
    ("ATMRemoveSubstFontA", ExportKind::Stub),
    ("ATMRemoveSubstFontW", ExportKind::Stub),
    ("ATMSelectEncoding", ExportKind::Stub),
    ("ATMSelectObject", ExportKind::Stub),
    ("ATMSetFlags", ExportKind::Stub),
    ("ATMXYShowText", ExportKind::Stub),
    ("ATMXYShowTextA", ExportKind::Stub),
    ("ATMXYShowTextW", ExportKind::Stub),
];

/// Looks up an export by its exact, case-sensitive name.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(export, _)| *export == name)
        .map(|&(_, kind)| kind)
}
