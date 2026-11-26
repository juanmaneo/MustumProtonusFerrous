//! Port of the portable logic in Wine's `dlls/olesvr32/olesvr_main.c` and `dlls/olesvr32/olesvr32.spec`.
//!
//! Ported:
//! - `OLE_OK`, the `OLESTATUS` value that every explicit export returns: [`OLE_OK`], [`OleStatus`]
//! - The handle counter behind `OleRegisterServer` and `OleRegisterServerDoc`: [`HandleCounter`],
//!   [`ole_register_server`], [`ole_register_server_doc`]
//! - `OleUnblockServer`, which stores `FALSE` in `*block`: [`ole_unblock_server`]
//! - The explicit exports that only trace their arguments: [`ole_block_server`],
//!   [`ole_revoke_server`], [`ole_revoke_server_doc`], [`ole_rename_server_doc`],
//!   [`ole_revert_server_doc`], [`ole_saved_server_doc`]
//! - `olesvr32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The three `@ stub` exports, `WEP`, `OleRevokeObject` and `OleQueryServerVersion`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`wep`], [`ole_revoke_object`],
//!   [`ole_query_server_version`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` traces and the parameters that only they read: `svrname`, `olesvr`, `hinst`,
//!   `osu`, `hServer`, `docname`, `document`, `hServerDoc`, `hDoc` and `newName`.
//! - The `OLESTATUS` members other than `OLE_OK`, and the `OLESERVER` and `OLESERVERVTBL`
//!   structures: no export in this DLL returns or reads them.
//! - The `WINAPI` calling convention and the `static` storage of `OLE_current_handle`: the counter
//!   is an explicit [`HandleCounter`] argument.
//! - A NULL `block` or `hRet`: the C code crashes on that write, and a Rust reference cannot be null.

mod exports;
mod server;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use server::{
    ole_block_server, ole_register_server, ole_register_server_doc, ole_rename_server_doc,
    ole_revert_server_doc, ole_revoke_server, ole_revoke_server_doc, ole_saved_server_doc,
    ole_unblock_server, HandleCounter, OleStatus, OLE_OK,
};
pub use stub::{
    ole_query_server_version, ole_revoke_object, wep, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
