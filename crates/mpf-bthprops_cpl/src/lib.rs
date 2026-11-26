//! Port of the portable logic in Wine's `dlls/bthprops.cpl/bthprops.cpl.spec`. The reference has
//! no C sources: `Makefile.in` only sets `MODULE` and `IMPORTS`.
//!
//! Ported:
//! - `bthprops.cpl.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 42 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`bluetooth_address_to_string`]
//!
//! Skipped:
//! - The 20 `@ stdcall -import` exports. Winebuild emits thunks that jump to the same functions in
//!   `bluetoothapis` (`IMPORTS` in `Makefile.in`), and the reference has no code for them.
//! - `BluetoothGetDeviceInfo` (`@ stdcall`) and `DllCanUnloadNow` (`-private`): their bodies would
//!   be in a C source that this reference does not contain.
//! - `#@ stub CPlApplet`: commented out in the spec, so it is not exported.
//! - The calling conventions and the ordinal form of the stub `function` argument: every `@ stub`
//!   export in the spec has a name, and the stub helpers take no arguments.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
