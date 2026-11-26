//! Port of the portable logic in Wine's dlls/virtdisk/virtdisk_main.c and virtdisk.spec.
//!
//! Ported:
//! - `GetStorageDependencyInformation`, `OpenVirtualDisk`, and `DetachVirtualDisk` return logic.
//! - Wine stub exception records for every `@ stub` export in `virtdisk.spec`.
//!
//! Skipped:
//! - OS-backed virtual disk operations and native handle/storage behavior.

mod exports;
mod logic;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use logic::{
    detach_virtual_disk, get_storage_dependency_information, open_virtual_disk,
    StorageDependencyInfo, ERROR_CALL_NOT_IMPLEMENTED, ERROR_INSUFFICIENT_BUFFER,
    ERROR_INVALID_PARAMETER, ERROR_SUCCESS, OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE,
    OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS, OPEN_VIRTUAL_DISK_VERSION_1, STORAGE_DEPENDENCY_INFO_SIZE,
    VIRTUAL_DISK_ACCESS_ALL,
};
pub use stub::add_virtual_disk_parent;
pub use stub::attach_virtual_disk;
pub use stub::break_mirror_virtual_disk;
pub use stub::compact_virtual_disk;
pub use stub::create_virtual_disk;
pub use stub::delete_virtual_disk_metadata;
pub use stub::enumerate_virtual_disk_metadata;
pub use stub::expand_virtual_disk;
pub use stub::get_all_attached_virtual_disk_physical_paths;
pub use stub::get_virtual_disk_information;
pub use stub::get_virtual_disk_metadata;
pub use stub::get_virtual_disk_operation_progress;
pub use stub::get_virtual_disk_physical_path;
pub use stub::merge_virtual_disk;
pub use stub::mirror_virtual_disk;
pub use stub::resize_virtual_disk;
pub use stub::set_virtual_disk_information;
pub use stub::set_virtual_disk_metadata;
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
