//! Port of the portable logic in Wine's `dlls/dpnlobby/dpnlobby.spec`. The reference has no `.c`,
//! `.idl`, `.rc` or `.tlb` file: its only code is one forwarded export.
//!
//! Ported:
//! - The `dpnlobby.spec` export table: [`EXPORTS`], [`Export`], [`export_by_name`], and the
//!   module name [`DLL_NAME`] that `winebuild` gives the DLL
//! - The forward test `winebuild` applies to a spec entry, which is also its data-only rule:
//!   [`Export::is_forward`]
//! - The `.def` line `winebuild` writes for an entry, with the i386 stdcall `@N` decoration and
//!   the `--kill-at` flag: [`Export::def_line`]
//!
//! Skipped:
//! - The forwarded function itself. `DirectPlay8Create` is in `dlls/dpnet`, and this DLL only
//!   names it.
//! - The PE layout that `winebuild` writes around the export table: section RVAs, the export
//!   directory's address table, and the import library. The oracle covers the export name, the
//!   ordinal and the forwarder string.
//! - The `=link` omission and the non-forward link names. Every entry of this spec is a forward,
//!   so [`Export::def_line`] always writes `=link`.
//! - `@ stub` exports and the `__wine_spec_unimplemented_stub` record. The spec has none, so there
//!   is no stub to model.
//! - The other `winebuild` flags, such as the calling-convention and relay options.

mod exports;

pub use exports::{export_by_name, Export, DLL_NAME, EXPORTS};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
