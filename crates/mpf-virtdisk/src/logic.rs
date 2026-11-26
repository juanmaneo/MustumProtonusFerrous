/// `ERROR_SUCCESS` from `include/winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
/// `ERROR_INSUFFICIENT_BUFFER` from `include/winerror.h`.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
/// `VIRTUAL_DISK_ACCESS_ALL` from `include/virtdisk.h`.
pub const VIRTUAL_DISK_ACCESS_ALL: u32 = 0x003f_0000;
/// `OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS` from `include/virtdisk.h`.
pub const OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS: u32 = 0x01;
/// `OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE` from `include/virtdisk.h`.
pub const OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE: u32 = 0x02;
/// `OPEN_VIRTUAL_DISK_VERSION_1` from `include/virtdisk.h`.
pub const OPEN_VIRTUAL_DISK_VERSION_1: u32 = 1;
/// `sizeof(STORAGE_DEPENDENCY_INFO)` for the Wine build's version/entry-count header.
pub const STORAGE_DEPENDENCY_INFO_SIZE: u32 = 8;

/// The fixed header fields used by `STORAGE_DEPENDENCY_INFO`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StorageDependencyInfo {
    pub version: u32,
    pub number_entries: u32,
}

/// Portable behavior of Wine's `GetStorageDependencyInformation` C body.
pub fn get_storage_dependency_information(
    size: u32,
    info: Option<&mut StorageDependencyInfo>,
    used: Option<&mut u32>,
) -> u32 {
    if let Some(used) = used {
        *used = STORAGE_DEPENDENCY_INFO_SIZE;
    }
    let Some(info) = info else {
        return ERROR_INVALID_PARAMETER;
    };
    if size == 0 {
        return ERROR_INVALID_PARAMETER;
    }
    if size < STORAGE_DEPENDENCY_INFO_SIZE {
        return ERROR_INSUFFICIENT_BUFFER;
    }
    info.number_entries = 0;
    ERROR_SUCCESS
}

/// Portable parameter-validation and return behavior of Wine's `OpenVirtualDisk` C body.
pub fn open_virtual_disk(
    has_type: bool,
    has_path: bool,
    access_mask: u32,
    flags: u32,
    parameters_version: Option<u32>,
) -> u32 {
    if !has_type
        || !has_path
        || access_mask & !VIRTUAL_DISK_ACCESS_ALL != 0
        || flags & !(OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS | OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE) != 0
    {
        return ERROR_INVALID_PARAMETER;
    }
    let Some(version) = parameters_version else {
        return ERROR_INVALID_PARAMETER;
    };
    if version != OPEN_VIRTUAL_DISK_VERSION_1 {
        return ERROR_INVALID_PARAMETER;
    }
    ERROR_CALL_NOT_IMPLEMENTED
}

/// Wine's `DetachVirtualDisk` body always returns `ERROR_INVALID_PARAMETER`.
pub fn detach_virtual_disk() -> u32 {
    ERROR_INVALID_PARAMETER
}
