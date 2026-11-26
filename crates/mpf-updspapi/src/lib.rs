//! Port of the portable logic in Wine's `dlls/updspapi/main.c` and `dlls/updspapi/updspapi.spec`.
//!
//! Ported:
//! - `UpdSpSetDynamicStringA`, whose C body only traces its arguments and returns 1:
//!   [`upd_sp_set_dynamic_string_a`]
//! - `updspapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The four `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`upd_sp_set_dynamic_string_ex_a`],
//!   [`upd_sp_set_dynamic_string_ex_w`], [`upd_sp_set_dynamic_string_w`],
//!   [`upd_sp_string_table_enum`]
//!
//! Skipped:
//! - The `FIXME` trace of the arguments, which the C body prints and nothing else.
//! - The 73 `@ stdcall ... setupapi.<name>` forwards. They resolve to setupapi.dll, so this DLL
//!   runs none of their code. Only their names and targets are in [`EXPORTS`].
//! - The `WINAPI` calling convention and the `HINF` handle: the handle is an opaque `usize`
//!   that the port never dereferences.

mod dynamic_string;
mod exports;
mod stub;

pub use dynamic_string::upd_sp_set_dynamic_string_a;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    upd_sp_set_dynamic_string_ex_a, upd_sp_set_dynamic_string_ex_w, upd_sp_set_dynamic_string_w,
    upd_sp_string_table_enum, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
