//! Port of the portable logic in Wine's `dlls/d3d8thk/d3d8thk.spec` and `dlls/d3d8thk/Makefile.in`.
//!
//! The reference has no C, IDL, RC or TLB sources, and every export in `d3d8thk.spec` is `@ stub`.
//! The port therefore models the export table and the stub records. No function returns a value.
//!
//! Ported:
//! - `d3d8thk.spec` export table: [`EXPORTS`] and [`export_stub`], in spec order (56 names)
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//! - One snake_case helper per export, such as [`os_thunk_d3d_context_create`], which calls
//!   [`wine_spec_unimplemented_stub`] with `d3d8thk.dll` and the export name
//!
//! Skipped:
//! - The `OsThunk*` bodies: the reference defines none, so there is no logic to port
//! - The ordinal form of the `function` parameter, which winebuild uses for exports without a
//!   name. Every `d3d8thk.spec` export is named, so the port takes only names
//! - `VER_FILEDESCRIPTION_STR` and `VER_PRODUCTVERSION` in `Makefile.in`: resource metadata for
//!   the build, not runtime behaviour

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
