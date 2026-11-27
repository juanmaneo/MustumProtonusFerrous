/// How `dlls/srclient/srclient.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall ... dll.name`: forwarded to the `dll.name` export of another DLL.
    Forward(&'static str),
}

/// The `srclient.spec` export table as `(ordinal, name, kind)`, in spec order. `winebuild` keeps the
/// explicit ordinals of `1 stub` and `2 stub`, and gives each `@` entry the lowest free ordinal from 1.
pub const EXPORTS: [(u32, &str, ExportKind); 13] = [
    (1, "SysprepCleanup", ExportKind::Stub),
    (2, "SysprepGeneralize", ExportKind::Stub),
    (3, "DisableSR", ExportKind::Stub),
    (4, "DisableSRInternal", ExportKind::Stub),
    (5, "EnableSR", ExportKind::Stub),
    (6, "EnableSREx", ExportKind::Stub),
    (7, "EnableSRInternal", ExportKind::Stub),
    (8, "SRNewSystemId", ExportKind::Stub),
    (9, "SRRemoveRestorePoint", ExportKind::Stub),
    (
        10,
        "SRSetRestorePointA",
        ExportKind::Forward("sfc_os.SRSetRestorePointA"),
    ),
    (11, "SRSetRestorePointInternal", ExportKind::Stub),
    (
        12,
        "SRSetRestorePointW",
        ExportKind::Forward("sfc_os.SRSetRestorePointW"),
    ),
    (13, "SetSRStateAfterSetup", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(_, export, _)| *export == name)
        .map(|&(_, _, kind)| kind)
}
