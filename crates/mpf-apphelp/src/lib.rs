//! Port of the portable logic in Wine's `dlls/apphelp/apphelp.c`, plus the
//! stub-export behaviour declared in `dlls/apphelp/apphelp.spec`.
//!
//! Ported:
//! - `ApphelpCheckInstallShieldPackage`, `ApphelpCheckMsiPackage`: always TRUE:
//!   [`apphelp_check_install_shield_package`], [`apphelp_check_msi_package`]
//! - `ApphelpCheckShellObject`: TRUE, and `*flags = 0` when the pointer is non-NULL:
//!   [`apphelp_check_shell_object`]
//! - `ShimFlushCache`: TRUE with `ERROR_CALL_NOT_IMPLEMENTED`: [`shim_flush_cache`]
//! - `SdbCreateDatabase`, `SdbInitDatabase`, `SdbOpenDatabase`: NULL handle with
//!   `ERROR_CALL_NOT_IMPLEMENTED`: [`sdb_create_database`], [`sdb_init_database`],
//!   [`sdb_open_database`]
//! - `SdbGetFirstChild` (always `TAGID_NULL`) and `SdbCloseDatabase` (no effect):
//!   [`sdb_get_first_child`], [`sdb_close_database`]
//! - `SdbGetAppPatchDir`: writes NUL to `path[0]` when `size` and `path` are non-zero:
//!   [`sdb_get_app_patch_dir`]
//! - `SetPermLayerState`, `SdbSetPermLayerKeys`, `SdbGetPermLayerKeys`: TRUE with
//!   `ERROR_CALL_NOT_IMPLEMENTED`: [`set_perm_layer_state`], [`sdb_set_perm_layer_keys`],
//!   [`sdb_get_perm_layer_keys`]
//! - Unimplemented `@ stub` exports (the 155 names in [`STUB_EXPORTS`]): the
//!   `__wine_spec_unimplemented_stub` path from `libs/winecrt0/stub.c`, which never
//!   returns and raises `EXCEPTION_WINE_STUB` non-continuably: [`unimplemented_stub`],
//!   [`StubException`]
//!
//! Skipped:
//! - `FIXME` and `TRACE` output, and the `debugstr_*` helpers.
//! - Arguments that only reach that tracing (`PATH_TYPE`, the handles, the paths,
//!   `clsid`, `shim`, `cmdline`, `cmd`, and the `size` out-pointer of
//!   `SdbGetPermLayerKeys`). No result depends on them, so the port does not take them.
//! - The SDB tag, index and database-file functions and the other spec exports
//!   without a body in `apphelp.c`. They are `@ stub` entries in the spec, so
//!   [`unimplemented_stub`] covers them.

mod checks;
mod database;
mod perm_layers;
mod stub_exports;
mod stub_result;

pub use checks::{
    apphelp_check_install_shield_package, apphelp_check_msi_package, apphelp_check_shell_object,
    shim_flush_cache,
};
pub use database::{
    sdb_close_database, sdb_create_database, sdb_get_app_patch_dir, sdb_get_first_child,
    sdb_init_database, sdb_open_database, Handle, TagId, NULL_HANDLE, TAGID_NULL,
};
pub use perm_layers::{sdb_get_perm_layer_keys, sdb_set_perm_layer_keys, set_perm_layer_state};
pub use stub_exports::{
    unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, STUB_EXPORTS,
};
pub use stub_result::{StubResult, ERROR_CALL_NOT_IMPLEMENTED};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
