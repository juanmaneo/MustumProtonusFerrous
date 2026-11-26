/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;
const VIRTDISK_DLL_NAME: &str = "virtdisk.dll";

/// The exception record raised by Wine for an unimplemented spec export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Models `__wine_spec_unimplemented_stub`; Wine raises this record and never returns.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

macro_rules! stub_export {
    ($name:ident, $export:literal) => {
        pub fn $name() -> StubException {
            wine_spec_unimplemented_stub(VIRTDISK_DLL_NAME, $export)
        }
    };
}

stub_export!(add_virtual_disk_parent, "AddVirtualDiskParent");
stub_export!(attach_virtual_disk, "AttachVirtualDisk");
stub_export!(break_mirror_virtual_disk, "BreakMirrorVirtualDisk");
stub_export!(compact_virtual_disk, "CompactVirtualDisk");
stub_export!(create_virtual_disk, "CreateVirtualDisk");
stub_export!(delete_virtual_disk_metadata, "DeleteVirtualDiskMetadata");
stub_export!(
    enumerate_virtual_disk_metadata,
    "EnumerateVirtualDiskMetadata"
);
stub_export!(expand_virtual_disk, "ExpandVirtualDisk");
stub_export!(
    get_all_attached_virtual_disk_physical_paths,
    "GetAllAttachedVirtualDiskPhysicalPaths"
);
stub_export!(get_virtual_disk_information, "GetVirtualDiskInformation");
stub_export!(get_virtual_disk_metadata, "GetVirtualDiskMetadata");
stub_export!(
    get_virtual_disk_operation_progress,
    "GetVirtualDiskOperationProgress"
);
stub_export!(get_virtual_disk_physical_path, "GetVirtualDiskPhysicalPath");
stub_export!(merge_virtual_disk, "MergeVirtualDisk");
stub_export!(mirror_virtual_disk, "MirrorVirtualDisk");
stub_export!(resize_virtual_disk, "ResizeVirtualDisk");
stub_export!(set_virtual_disk_information, "SetVirtualDiskInformation");
stub_export!(set_virtual_disk_metadata, "SetVirtualDiskMetadata");
