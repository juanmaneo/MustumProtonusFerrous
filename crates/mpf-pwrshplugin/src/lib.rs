//! Port of the portable logic in Wine's `dlls/pwrshplugin/pwrshplugin.spec` and `dlls/pwrshplugin/Makefile.in`.
//!
//! The DLL has no `.c`, `.idl`, `.rc` or `.tlb` sources, so the port covers only what the spec and
//! the Makefile define.
//!
//! Ported:
//! - `pwrshplugin.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The twelve `@ stub` exports, one helper each (such as [`ws_man_plugin_command`]), and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//! - `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE`, and the `MODULE` name `pwrshplugin.dll`
//!
//! Skipped:
//! - Any implementation of the WSMan plugin entry points: Wine's spec exports them only as stubs.
//! - The `__cdecl` calling convention and the `ExceptionAddress` field. The address is a location
//!   in the Wine binary, so the port does not reproduce it.
//! - The `ExceptionInformation` entries past the first two, which the C body never sets.
//! - The `for (;;)` loop that re-raises forever: the port returns the record instead.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    get_clr_version_for_ps_version, perform_ws_man_plugin_report_completion,
    wine_spec_unimplemented_stub, ws_man_plugin_command, ws_man_plugin_connect,
    ws_man_plugin_receive, ws_man_plugin_release_command_context,
    ws_man_plugin_release_shell_context, ws_man_plugin_send, ws_man_plugin_shell,
    ws_man_plugin_shutdown, ws_man_plugin_signal, ws_man_plugin_startup, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
