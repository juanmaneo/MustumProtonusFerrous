//! Port of the portable logic in Wine's `dlls/setupapi/...`.
//!
//! Ported:
//! - The INF tokenizer, the state machine of `parse_buffer` in `dlls/setupapi/parser.c`:
//!   [`parse_inf_buffer`], [`InfFile`], [`InfSection`], [`InfLine`], [`InfError`], and the
//!   `ERROR_*` codes from `include/setupapi.h`
//! - The 22 functions in `dlls/setupapi/stubs.c` whose C bodies return a fixed value:
//!   [`FixedReturn`] and the `setup_*` / `registry_delnode` functions
//! - The `setupapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 241 `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`], and one
//!   helper per export, named by the snake_case form of the export name
//!
//! Skipped:
//! - `parse_file`, the file encoding detection and `append_inf_file` (file I/O)
//! - The `[Strings]` substitution helpers (`PARSER_string_subst*`, `get_dirid_subst`) and the
//!   lookup accessors (`find_line`, `get_field` and the rest of `parser.c`)
//! - The bodies of the 281 local `@ stdcall` exports that are not fixed-value functions (303
//!   local entries, of which 22 are fixed-value). Their export table entries are ported, not
//!   their code. The 73 `cfgmgr32` forwards are table entries only.
//! - Allocation-failure paths (`ERROR_NOT_ENOUGH_MEMORY`), the `assert`s, and the `FIXME` traces
//! - The `__cdecl` calling convention and the WINAPI wrappers
//! - Locale-dependent character classes. `iswspace` is ASCII-only here, and `wcsicmp` folds
//!   ASCII case only. Wine's tables differ for non-ASCII input.

mod exports;
mod fixed;
mod parser;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use fixed::*;
pub use parser::{
    parse_inf_buffer, InfError, InfFile, InfLine, InfSection, ERROR_BAD_SECTION_NAME_LINE,
    ERROR_EXPECTED_SECTION_NAME, ERROR_SECTION_NAME_TOO_LONG,
};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
