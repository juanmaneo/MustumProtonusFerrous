//! Port of the portable logic in Wine's `dlls/rsabase/`, which has no C sources: the
//! `rsabase.spec` export table, and the `get_args_size` rule from `tools/winebuild/utils.c`
//! that the `.def` output uses for its `@N` suffix.
//!
//! Ported:
//! - `rsabase.spec` export table: [`EXPORTS`], [`Export`], [`find_export`], [`importable`]
//! - the `EXPORTS` line `winebuild --def` writes per export: [`def_line`]
//! - `get_args_size`, the stdcall argument bytes: [`args_size`], [`ArgType`], [`Cpu`]
//!
//! Skipped:
//! - The forwarded calls. Every export is `rsaenh.<name>`, resolved by the loader at run time,
//!   so this DLL has no code of its own to port.
//! - `@ stub` modelling. The spec has no `@ stub` lines, so `__wine_spec_unimplemented_stub`
//!   is never raised from this DLL.
//! - The `.spec` parser, the ordinal numbering it does, and the `DATA`, `NONAME`, `FASTCALL` and
//!   `-k` output paths, which this spec does not use.

mod exports;
mod stdcall;

pub use exports::{def_line, find_export, importable, Export, EXPORTS};
pub use stdcall::{args_size, ArgType, Cpu};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
