//! Boundary checks beyond the captured C-oracle vectors.
use super::*;

#[test]
fn dependency_header_size_edges_and_optional_used() {
    // Not oracle-derived: this additional row checks NULL `used` and values immediately around the header size.
    for (size, expected) in [
        (0, ERROR_INVALID_PARAMETER),
        (7, ERROR_INSUFFICIENT_BUFFER),
        (8, ERROR_SUCCESS),
    ] {
        let mut info = StorageDependencyInfo {
            version: 9,
            number_entries: 5,
        };
        assert_eq!(
            get_storage_dependency_information(size, Some(&mut info), None),
            expected
        );
        assert_eq!(info.version, 9);
        assert_eq!(
            info.number_entries,
            if expected == ERROR_SUCCESS { 0 } else { 5 }
        );
    }
    assert_eq!(
        get_storage_dependency_information(STORAGE_DEPENDENCY_INFO_SIZE, None, None),
        ERROR_INVALID_PARAMETER
    );
}

#[test]
fn open_disk_masks_and_version_boundaries() {
    // Not oracle-derived: validates the complement mask and rejects every other version value.
    for flags in [
        OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS,
        OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE,
        3,
    ] {
        assert_eq!(
            open_virtual_disk(true, true, VIRTUAL_DISK_ACCESS_ALL, flags, Some(1)),
            ERROR_CALL_NOT_IMPLEMENTED
        );
    }
    for mask in [1, 0x0040_0000, u32::MAX] {
        assert_eq!(
            open_virtual_disk(true, true, mask, 0, Some(1)),
            ERROR_INVALID_PARAMETER
        );
    }
    for version in [None, Some(0), Some(2), Some(u32::MAX)] {
        assert_eq!(
            open_virtual_disk(true, true, 0, 0, version),
            ERROR_INVALID_PARAMETER
        );
    }
}

#[test]
fn stub_builder_and_export_table_edges() {
    // Not oracle-derived: generic pass-through behavior and exact, case-sensitive spec lookup.
    let record = wine_spec_unimplemented_stub("another.dll", "OddExport");
    assert_eq!(
        (record.code, record.flags, record.number_parameters),
        (EXCEPTION_WINE_STUB, EXCEPTION_NONCONTINUABLE, 2)
    );
    assert_eq!(
        (record.module, record.function),
        ("another.dll", "OddExport")
    );
    assert_eq!(EXPORTS.len(), 21);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        18
    );
    assert_eq!(export_kind("OpenVirtualDisk"), Some(ExportKind::CFunction));
    assert_eq!(export_kind("AddVirtualDiskParent"), Some(ExportKind::Stub));
    assert_eq!(export_kind("openvirtualdisk"), None);
    assert_eq!(export_kind(""), None);
}
