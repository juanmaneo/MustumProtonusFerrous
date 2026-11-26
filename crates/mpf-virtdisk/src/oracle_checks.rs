//! Oracle-derived rows generated from the Wine C bodies and stub shim in `target/canon/virtdisk`.
use super::*;

#[test]
fn oracle_get_storage_dependency_information_rows() {
    // Each tuple is (input size, has info, initial entries, return, used, final entries).
    let rows = [
        (8, false, 77, 87, 8, 77),
        (0, true, 77, 87, 8, 77),
        (1, true, 77, 122, 8, 77),
        (7, true, 77, 122, 8, 77),
        (8, true, 77, 0, 8, 0),
        (u32::MAX, true, 77, 0, 8, 0),
    ];
    for (size, has_info, entries, ret, used_value, final_entries) in rows {
        let mut info = StorageDependencyInfo {
            version: 2,
            number_entries: entries,
        };
        let mut used = 999;
        let result = get_storage_dependency_information(
            size,
            has_info.then_some(&mut info),
            Some(&mut used),
        );
        assert_eq!((result, used), (ret, used_value));
        assert_eq!(info.number_entries, final_entries);
        assert_eq!(info.version, 2);
    }
}

#[test]
fn oracle_open_virtual_disk_rows() {
    let rows = [
        (true, true, 0, 0, Some(1), 120),
        (false, true, 0, 0, Some(1), 87),
        (true, false, 0, 0, Some(1), 87),
        (true, true, 0, 0, None, 87),
        (true, true, 0x00ff_ffff, 0, Some(1), 87),
        (true, true, 0, 4, Some(1), 87),
        (true, true, 0, 0, Some(0), 87),
        (true, true, 0, 3, Some(1), 120),
    ];
    for (has_type, has_path, mask, flags, version, expected) in rows {
        assert_eq!(
            open_virtual_disk(has_type, has_path, mask, flags, version),
            expected
        );
    }
}

type StubExport = (&'static str, fn() -> StubException);

#[test]
fn oracle_detach_and_all_stub_export_rows() {
    assert_eq!(detach_virtual_disk(), 87);
    let exports: [StubExport; 18] = [
        ("AddVirtualDiskParent", add_virtual_disk_parent),
        ("AttachVirtualDisk", attach_virtual_disk),
        ("BreakMirrorVirtualDisk", break_mirror_virtual_disk),
        ("CompactVirtualDisk", compact_virtual_disk),
        ("CreateVirtualDisk", create_virtual_disk),
        ("DeleteVirtualDiskMetadata", delete_virtual_disk_metadata),
        (
            "EnumerateVirtualDiskMetadata",
            enumerate_virtual_disk_metadata,
        ),
        ("ExpandVirtualDisk", expand_virtual_disk),
        (
            "GetAllAttachedVirtualDiskPhysicalPaths",
            get_all_attached_virtual_disk_physical_paths,
        ),
        ("GetVirtualDiskInformation", get_virtual_disk_information),
        ("GetVirtualDiskMetadata", get_virtual_disk_metadata),
        (
            "GetVirtualDiskOperationProgress",
            get_virtual_disk_operation_progress,
        ),
        ("GetVirtualDiskPhysicalPath", get_virtual_disk_physical_path),
        ("MergeVirtualDisk", merge_virtual_disk),
        ("MirrorVirtualDisk", mirror_virtual_disk),
        ("ResizeVirtualDisk", resize_virtual_disk),
        ("SetVirtualDiskInformation", set_virtual_disk_information),
        ("SetVirtualDiskMetadata", set_virtual_disk_metadata),
    ];
    for (name, call) in exports {
        assert_eq!(
            call(),
            StubException {
                code: 0x8000_0100,
                flags: 1,
                number_parameters: 2,
                module: "virtdisk.dll",
                function: name,
            }
        );
    }
}
