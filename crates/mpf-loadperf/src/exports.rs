/// How `dlls/loadperf/loadperf.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported in `loadperf.rs`.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `loadperf.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 14] = [
    ("BackupPerfRegistryToFileW", ExportKind::Stub),
    ("InstallPerfDllA", ExportKind::Stdcall),
    ("InstallPerfDllW", ExportKind::Stdcall),
    ("LoadMofFromInstalledServiceA", ExportKind::Stub),
    ("LoadMofFromInstalledServiceW", ExportKind::Stub),
    ("LoadPerfCounterTextStringsA", ExportKind::Stdcall),
    ("LoadPerfCounterTextStringsW", ExportKind::Stdcall),
    ("RestorePerfRegistryFromFileW", ExportKind::Stub),
    ("SetServiceAsTrustedA", ExportKind::Stub),
    ("SetServiceAsTrustedW", ExportKind::Stub),
    ("UnloadPerfCounterTextStringsA", ExportKind::Stdcall),
    ("UnloadPerfCounterTextStringsW", ExportKind::Stdcall),
    ("UpdatePerfNameFilesA", ExportKind::Stub),
    ("UpdatePerfNameFilesW", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
