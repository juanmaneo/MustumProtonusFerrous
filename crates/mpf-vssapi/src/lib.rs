//! Port of the portable logic in Wine's `dlls/vssapi/main.c` and `dlls/vssapi/vssapi.spec`.
//!
//! Ported:
//! - Deterministic `CVssWriter` behavior and the `S_OK`/`E_NOTIMPL` return paths in `main.c`.
//! - The complete `.spec` export table and all `@ stub` exception records.
//!
//! Skipped:
//! - Windows calling conventions, pointer ABI details, and the `FIXME` diagnostics.

mod exports;
mod logic;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use logic::{
    create_vss_backup_components, create_vss_backup_components_internal, cvss_writer_default_ctor,
    cvss_writer_dtor, cvss_writer_initialize, cvss_writer_subscribe, cvss_writer_unsubscribe,
    VssWriter, E_NOTIMPL, S_OK,
};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
    STUB_EXPORTS, STUB_HELPERS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
