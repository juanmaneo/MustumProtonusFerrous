//! Port of the portable logic in Wine's `dlls/sfc_os/sfc_os.c` and `dlls/sfc_os/sfc_os.spec`.
//!
//! Ported:
//! - The fixed results of the explicit C functions, with the value each one passes to
//!   `SetLastError`: [`sfc_get_next_protected_file`], [`sfc_is_file_protected`],
//!   [`sfc_is_key_protected`], [`sfc_connect_to_server`], [`sfp_verify_file`]
//! - `SRSetRestorePointA` and `SRSetRestorePointW`, which write `ERROR_SUCCESS` into
//!   `STATEMGRSTATUS.nStatus` and return FALSE: [`sr_set_restore_point_a`],
//!   [`sr_set_restore_point_w`], [`StateMgrStatus`]
//! - `sfc_os.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export
//!
//! Skipped:
//! - `FIXME` and `TRACE` output, and the static `reported` flag that only chooses between them.
//! - Arguments that the C bodies only pass to `FIXME` or `TRACE`, or never read: the handle and
//!   `PROTECTED_FILE_DATA` of `SfcGetNextProtectedFile`, `RpcHandle`, the file and key names,
//!   the `samDesired` value, `unknown`, the `SfpVerifyFile` buffers, and the restore point.
//!   Only the NULL-ness of `hKey` is kept.
//! - `SRSetRestorePoint*` with a NULL `status`: the C body dereferences it and crashes. A Rust
//!   `&mut` cannot be NULL, so that edge is not representable.
//! - `STATEMGRSTATUS.llSequenceNumber`, which Wine never writes.
//! - The `__cdecl` calling convention and the `for (;;)` raise loop of the stub: the port
//!   returns the record instead.

mod exports;
mod outcome;
mod protect;
mod restore;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use outcome::{
    Outcome, ERROR_CALL_NOT_IMPLEMENTED, ERROR_FILE_NOT_FOUND, ERROR_INVALID_HANDLE,
    ERROR_NO_MORE_FILES, ERROR_SUCCESS,
};
pub use protect::{
    sfc_connect_to_server, sfc_get_next_protected_file, sfc_is_file_protected,
    sfc_is_key_protected, sfp_verify_file,
};
pub use restore::{sr_set_restore_point_a, sr_set_restore_point_w, StateMgrStatus};
pub use stub::{
    begin_file_map_enumeration, close_file_map_enumeration, get_next_file_map_content, sfc_close,
    sfc_file_exception, sfc_init_prot, sfc_initiate_scan, sfc_install_protected_files,
    sfc_terminate_watcher_thread, sfp_delete_catalog, sfp_install_catalog,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
