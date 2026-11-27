//! Port of the portable logic in Wine's `dlls/security/security.spec` and `dlls/security/Makefile.in`.
//!
//! Ported:
//! - The `security.spec` export table: [`EXPORTS`], [`Export`], [`export_by_name`] and
//!   [`export_by_ordinal`]. Every line is `@ stdcall` and forwards to `secur32`. The ordinals and
//!   argument counts are the ones `winebuild` derives in `parse_spec_export` and `assign_ordinals`
//!   (`tools/winebuild/parser.c`).
//! - The forward split: [`split_forward`].
//! - `MODULE` and `IMPORTS` from `Makefile.in`: [`MODULE`], [`IMPORTS`].
//!
//! Skipped:
//! - `security.dll` has no `.c` sources. Its only code is the forwards to `secur32`, which live in
//!   another DLL, so there is no function body to port.
//! - `@ stub` exports and the `__wine_spec_unimplemented_stub` record. The spec has none.
//! - The argument types (`ptr`, `str`, `long`, ...). Only the count is kept. The spec has no
//!   `-flag` tokens, explicit ordinals, or `thiscall` and `fastcall` entries.
//! - The `winebuild` lexer (`GetToken`) and its parse errors.

mod exports;

pub use exports::{
    export_by_name, export_by_ordinal, split_forward, Export, EXPORTS, IMPORTS, MODULE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
