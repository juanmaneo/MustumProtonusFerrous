//! Port of the portable logic in Wine's `dlls/dbgeng/dbgeng.c`.
//!
//! Ported:
//! - `debug_target_return_string`: [`return_string`], with [`S_OK`] and [`S_FALSE`]
//! - The base-address search in `debug_target_get_module_info_by_base`:
//!   [`module_index_by_base`]
//!
//! Skipped:
//! - `debug_target_init_modules_info`, `debug_target_get_module_info`,
//!   `debug_target_get_module_machine`, `debug_target_get_module_timestamp`:
//!   read loaded-module data from a live target process (OS memory and PE headers).
//! - `debug_client_*`, `debug_target_free`, `debug_client_detach_target`, and the
//!   `impl_from_IDebug*` helpers: COM object, list and refcount plumbing.
//! - All `IDebug*` vtable entries (`debugclient_*`, `debugdataspaces_*`,
//!   `debugsymbols_*`, `debugcontrol_*`, `debugadvanced_*`, `debugsystemobjects_*`):
//!   process control, dump files, memory, symbols and output callbacks.
//! - `FIXME`-only stubs that return `E_NOTIMPL`.

mod module;
mod string_return;

pub use module::module_index_by_base;
pub use string_return::{return_string, S_FALSE, S_OK};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
