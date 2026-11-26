/// How `dlls/sfc_os/sfc_os.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as one of the explicit `sfc_os.c` functions.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `sfc_os.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 18] = [
    ("BeginFileMapEnumeration", ExportKind::Stub),
    ("CloseFileMapEnumeration", ExportKind::Stub),
    ("GetNextFileMapContent", ExportKind::Stub),
    ("SRSetRestorePointA", ExportKind::Stdcall),
    ("SRSetRestorePointW", ExportKind::Stdcall),
    ("SfcClose", ExportKind::Stub),
    ("SfcConnectToServer", ExportKind::Stdcall),
    ("SfcFileException", ExportKind::Stub),
    ("SfcGetNextProtectedFile", ExportKind::Stdcall),
    ("SfcInitProt", ExportKind::Stub),
    ("SfcInitiateScan", ExportKind::Stub),
    ("SfcInstallProtectedFiles", ExportKind::Stub),
    ("SfcIsFileProtected", ExportKind::Stdcall),
    ("SfcIsKeyProtected", ExportKind::Stdcall),
    ("SfcTerminateWatcherThread", ExportKind::Stub),
    ("SfpDeleteCatalog", ExportKind::Stub),
    ("SfpInstallCatalog", ExportKind::Stub),
    ("SfpVerifyFile", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
