//! Port of the portable logic in Wine's `dlls/profapi/profapi.spec` and `dlls/profapi/Makefile.in`.
//!
//! `dlls/profapi` has no C sources, IDL, resources or type libraries. Its only code is the spec
//! file, which exports 17 unnamed `@ stub` ordinals, 101 to 117.
//!
//! Ported:
//! - `profapi.spec` export table: [`EXPORTS`], [`is_export`]
//! - The 17 unnamed `@ stub` ordinals and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`ordinal_101`] to [`ordinal_117`], [`wine_spec_unimplemented_stub`], [`StubException`],
//!   [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The endless loop in `__wine_spec_unimplemented_stub`: the port returns the record instead.
//! - The `ExceptionRecord` and `ExceptionAddress` fields, which the record does not carry, and
//!   the `__cdecl` calling convention.

mod exports;
mod stub;

pub use exports::{is_export, EXPORTS};
pub use stub::{
    ordinal_101, ordinal_102, ordinal_103, ordinal_104, ordinal_105, ordinal_106, ordinal_107,
    ordinal_108, ordinal_109, ordinal_110, ordinal_111, ordinal_112, ordinal_113, ordinal_114,
    ordinal_115, ordinal_116, ordinal_117, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
