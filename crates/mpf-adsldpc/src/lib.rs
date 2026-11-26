//! Port of the portable logic in Wine's `dlls/adsldpc/adsldpc.spec` and `dlls/adsldpc/Makefile.in`.
//!
//! `dlls/adsldpc` has no `.c`, `.idl`, `.rc` or `.tlb` sources. Its only deterministic content is
//! the export table, and every one of its 189 exports is `@ stub`, so the port models those stubs.
//!
//! Ported:
//! - `adsldpc.spec` export table: [`EXPORTS`], [`StubExport`], [`Arch`], [`find_export`]
//! - `MODULE = adsldpc.dll` from `Makefile.in`, the module name every stub passes
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - one helper per export, 189 in all, each returning the record its stub raises (re-exported
//!   from the private `stubs` module)
//!
//! Skipped:
//! - The stub bodies themselves: winebuild generates them, and each one raises the record and
//!   never returns, so there is no return value to port.
//! - The `ExceptionRecord` (NULL) and `ExceptionAddress` fields, which the C body sets but the
//!   record shape does not expose.
//! - The `-Wb,--prefer-native` link flag in `Makefile.in`.

mod exports;
mod stub;
mod stubs;

pub use exports::{find_export, Arch, StubExport, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
