//! Port of the portable logic in Wine's `dlls/softpub/softpub.spec` and `dlls/softpub/Makefile.in`.
//!
//! `dlls/softpub` has no C sources. `Makefile.in` sets `MODULE = softpub.dll` and
//! `IMPORTS = wintrust`, and `softpub.spec` defines the whole DLL.
//!
//! Ported:
//! - `softpub.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`find_export`]
//! - Forward resolution for the 21 `stdcall` lines that forward to `wintrust`: [`forward_target`]
//! - `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE`: [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Modelled:
//! - The three `stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], with one helper per export:
//!   [`softpub_free_def_usage_call_data`], [`softpub_load_def_usage_call_data`],
//!   [`add_personal_trust_db_pages`]
//!
//! Skipped:
//! - The forwarded functions themselves. They live in `wintrust.dll`, which the loader resolves.
//!   This crate records only the target.
//! - The argument lists after each name, such as `ptr ptr long`. winebuild uses them only for the
//!   `@N` decoration on i386.
//! - The ordinal branch of `__wine_spec_unimplemented_stub`, which takes the ordinal when an export
//!   has no name. `softpub.spec` names every export.
//! - The `__cdecl` and `WINAPI` calling conventions, and the `for (;;)` loop. The port returns the
//!   record instead of looping.

mod exports;
mod stub;

pub use exports::{find_export, forward_target, Export, ExportKind, EXPORTS};
pub use stub::{
    add_personal_trust_db_pages, softpub_free_def_usage_call_data,
    softpub_load_def_usage_call_data, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
