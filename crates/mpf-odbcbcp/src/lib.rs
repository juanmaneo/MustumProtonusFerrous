//! Port of the portable logic in Wine's `dlls/odbcbcp/odbcbcp.spec` and `dlls/odbcbcp/Makefile.in`.
//!
//! Wine's odbcbcp has no C sources, and no `.idl`, `.rc` or `.tlb` files. Its spec file lists
//! only `@ stub` exports, so the port covers the export table and the stub exception records.
//!
//! Ported:
//! - `odbcbcp.spec` export table: [`EXPORTS`] and [`export_stub`], in spec order
//! - `__wine_spec_unimplemented_stub`, the record each `@ stub` raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//! - one helper per export that raises its record with the export's name, for example
//!   [`bcp_init_a`] for `bcp_initA`
//!
//! Skipped:
//! - Any behaviour behind the stubs. The C stub never returns: it raises the record in a loop,
//!   so there is no return value to model.
//! - Ordinals. Every export has a name, so winebuild passes names and no ordinals.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `Makefile.in`. It is a build flag with no runtime code.

mod exports;
mod stub;

pub use exports::{
    bcp_batch, bcp_bind, bcp_colfmt, bcp_collen, bcp_colptr, bcp_columns, bcp_control, bcp_done,
    bcp_exec, bcp_getcolfmt, bcp_init_a, bcp_init_w, bcp_moretext, bcp_readfmt_a, bcp_readfmt_w,
    bcp_sendrow, bcp_setcolfmt, bcp_writefmt_a, bcp_writefmt_w, dbprtype_a, dbprtype_w,
    export_stub, lib_main, sql_close_enum_servers, sql_get_next_enumeration, sql_init_enum_servers,
    sql_linked_catalogs_a, sql_linked_catalogs_w, sql_linked_servers, EXPORTS,
};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
