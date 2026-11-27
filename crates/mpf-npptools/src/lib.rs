//! Port of the portable logic in Wine's `dlls/npptools/npptools.spec` and `dlls/npptools/Makefile.in`.
//!
//! `dlls/npptools` has no C source. Its only files are the spec and the makefile, and all 60 of its
//! exports are `@ stub`, so the port models what those two files define.
//!
//! Ported:
//! - `npptools.spec` export table: [`is_export`], [`call_export`], [`STUB_EXPORTS`]
//! - The 60 `@ stub` exports, one helper each such as [`create_blob`], and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//! - The values the record uses: [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`] and
//!   [`NPPTOOLS_DLL_NAME`]
//!
//! Skipped:
//! - The bodies of the exports. Wine does not implement them here, and the stub loop never
//!   returns a value, so there is no return value to model.
//! - `Makefile.in`: `EXTRADLLFLAGS = -Wb,--prefer-native` is a link-time setting with no
//!   runtime effect.

mod exports;
mod stub;

pub use exports::{call_export, is_export};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
