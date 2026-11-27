//! Port of the portable logic in Wine's `dlls/apisetschema/apisetschema.spec`: the winebuild parser for
//! `apiset` records (`tools/winebuild/parser.c`) and the ApiSetSchema version 6 section writer
//! (`tools/winebuild/spec32.c`). The DLL has no C sources, so its behaviour comes from those two files.
//!
//! Ported:
//! - `apiset_hash` and `apiset_hash_len`: [`apiset_hash`]
//! - `GetToken` on one spec line: separators, comments, whitespace and backslash escapes
//! - `apiset_add_str`, `add_apiset` and `add_apiset_value`, with the string pool they fill
//! - `parse_spec_apiset` and the top-level line loop of `parse_spec`: [`Schema::parse_spec`],
//!   [`Schema::parse_line`] and [`SpecError`]
//! - `output_apiset_section`: [`Schema::section`]
//! - the 672 records of `apisetschema.spec`: [`RECORDS`] and [`apisetschema`]
//!
//! Skipped:
//! - Ordinal and `@` declarations, which Wine parses as exports. They are rejected as
//!   [`SpecError::UnknownDirective`]. `apisetschema.spec` has no `@ stub` lines, so the stub model does not apply.
//! - Backslash-newline line continuation. A line is parsed without its newline, so a backslash at the end
//!   of a line ends the token.
//! - Wine's error recovery. winebuild counts errors and keeps parsing, and this port stops at the first error.
//! - The data-only check: `apiset` is only allowed in data-only mode, and the port is always data-only.
//! - The `api-` and `ext-` prefix check. Wine ANDs two `strncmp` tests, so it never rejects a name.
//! - The PE wrapper, the import library (`output_apiset_lib`) and the loader that reads the section.

mod hash;
mod pool;
mod records;
mod schema;
mod section;
mod tokens;

pub use hash::apiset_hash;
pub use records::{apisetschema, RECORDS};
pub use schema::{Entry, Schema, SpecError, Value};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
