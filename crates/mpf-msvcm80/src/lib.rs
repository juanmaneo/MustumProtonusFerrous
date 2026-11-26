//! Port of the portable logic in Wine's `dlls/msvcm80/msvcm_main.c` and `dlls/msvcm80/msvcm80.spec`.
//!
//! Ported:
//! - The six C functions in `msvcm_main.c` behind the `@ cdecl` exports. Each one only writes a
//!   `FIXME` trace and returns, so each helper has no effect, for example [`setusermatherr_m`].
//! - Every `@ stub` export (296 lines): one helper per export in `stub_exports`, each built on
//!   [`wine_spec_unimplemented_stub`] and returning the [`StubException`] that the stub raises.
//!   Both `-arch=win32` and `-arch=win64` variants are kept.
//!
//! Skipped:
//! - The `FIXME` trace that each C body writes: debug output, with no return value or state.
//! - The `WINE_DEFAULT_DEBUG_CHANNEL` setup and the `__cdecl` calling convention.
//! - Arch selection: a build exports only one variant of each `-arch` pair, which this crate does
//!   not model.
//! - The C++ runtime behaviour behind the stubbed exports: Wine has no implementation, so the port
//!   returns the exception instead of looping on it.

mod msvcm_main;
mod stub;
mod stub_exports;

pub use msvcm_main::*;
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stub_exports::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
