/// Whether a virtdisk spec export has a C implementation or is a Wine stub.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    CFunction,
    Stub,
}

/// `virtdisk.spec` exports in spec order.
pub const EXPORTS: [(&str, ExportKind); 21] = [
    ("AddVirtualDiskParent", ExportKind::Stub),
    ("AttachVirtualDisk", ExportKind::Stub),
    ("BreakMirrorVirtualDisk", ExportKind::Stub),
    ("CompactVirtualDisk", ExportKind::Stub),
    ("CreateVirtualDisk", ExportKind::Stub),
    ("DeleteVirtualDiskMetadata", ExportKind::Stub),
    ("DetachVirtualDisk", ExportKind::CFunction),
    ("EnumerateVirtualDiskMetadata", ExportKind::Stub),
    ("ExpandVirtualDisk", ExportKind::Stub),
    ("GetAllAttachedVirtualDiskPhysicalPaths", ExportKind::Stub),
    ("GetStorageDependencyInformation", ExportKind::CFunction),
    ("GetVirtualDiskInformation", ExportKind::Stub),
    ("GetVirtualDiskMetadata", ExportKind::Stub),
    ("GetVirtualDiskOperationProgress", ExportKind::Stub),
    ("GetVirtualDiskPhysicalPath", ExportKind::Stub),
    ("MergeVirtualDisk", ExportKind::Stub),
    ("MirrorVirtualDisk", ExportKind::Stub),
    ("OpenVirtualDisk", ExportKind::CFunction),
    ("ResizeVirtualDisk", ExportKind::Stub),
    ("SetVirtualDiskInformation", ExportKind::Stub),
    ("SetVirtualDiskMetadata", ExportKind::Stub),
];

/// Returns the export kind for an exact, case-sensitive spec name.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(export, _)| *export == name)
        .map(|(_, kind)| *kind)
}
