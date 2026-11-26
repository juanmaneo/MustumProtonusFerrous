//! Port of the portable logic in Wine's `dlls/loadperf/loadperf_main.c`, plus the `stub`
//! exports declared in `dlls/loadperf/loadperf.spec`.
//!
//! Ported:
//! - `InstallPerfDllA` and `InstallPerfDllW`: [`install_perf_dll_a`], [`install_perf_dll_w`]
//! - `LoadPerfCounterTextStringsA` and `LoadPerfCounterTextStringsW`:
//!   [`load_perf_counter_text_strings_a`], [`load_perf_counter_text_strings_w`]
//! - `UnloadPerfCounterTextStringsA` and `UnloadPerfCounterTextStringsW`:
//!   [`unload_perf_counter_text_strings_a`], [`unload_perf_counter_text_strings_w`]
//! - The `strdupAW` conversion and its return values `ERROR_SUCCESS` and `ERROR_OUTOFMEMORY`:
//!   [`strdup_aw`], [`ERROR_SUCCESS`], [`ERROR_OUTOFMEMORY`]
//! - The `@ stub` exports `BackupPerfRegistryToFileW`, `LoadMofFromInstalledServiceA`/`W`,
//!   `RestorePerfRegistryFromFileW`, `SetServiceAsTrustedA`/`W` and `UpdatePerfNameFilesA`/`W`,
//!   with their `__wine_spec_unimplemented_stub` record: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], and one helper per export, such as [`backup_perf_registry_to_file_w`]
//! - The `loadperf.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The `FIXME` traces, which only log the arguments.
//! - `MultiByteToWideChar` (the ANSI code page) and `HeapAlloc` (the process heap). The `*A`
//!   functions take a `convert` closure in their place, and a `None` from it means the
//!   allocation failed.
//! - The arguments that the `*W` bodies never read. They stay as `_`-prefixed parameters, so the
//!   `*A` functions pass the converted copies on as the C wrappers do.
//! - The `WINAPI` and `__cdecl` calling conventions.

mod exports;
mod loadperf;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use loadperf::{
    install_perf_dll_a, install_perf_dll_w, load_perf_counter_text_strings_a,
    load_perf_counter_text_strings_w, strdup_aw, unload_perf_counter_text_strings_a,
    unload_perf_counter_text_strings_w, ERROR_OUTOFMEMORY, ERROR_SUCCESS,
};
pub use stub::{
    backup_perf_registry_to_file_w, load_mof_from_installed_service_a,
    load_mof_from_installed_service_w, restore_perf_registry_from_file_w, set_service_as_trusted_a,
    set_service_as_trusted_w, update_perf_name_files_a, update_perf_name_files_w,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
