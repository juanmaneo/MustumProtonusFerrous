//! Port of the portable logic in Wine's `dlls/tdh/tdh_main.c` and `dlls/tdh/tdh.spec`.
//!
//! Ported:
//! - `TdhEnumerateProviders` buffer and size contract: [`tdh_enumerate_providers`], with the
//!   `ERROR_*` values from `include/winerror.h`
//! - `TdhLoadManifest` and `TdhLoadManifestFromBinary`, which return `STATUS_SUCCESS`:
//!   [`tdh_load_manifest`], [`tdh_load_manifest_from_binary`]
//! - `tdh.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 31 `@ stub` exports, each modelled as the `__wine_spec_unimplemented_stub` record it
//!   raises: [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export
//!   (`tdh_aggregate_payload_filters` and so on) in `stub.rs`
//!
//! Skipped:
//! - The `FIXME` traces, which only log the arguments.
//! - The `WINAPI` calling convention. The `LPWSTR` arguments are `Option<&[u16]>`, and
//!   `PROVIDER_ENUMERATION_INFO` is a byte buffer whose first four bytes are written.
//! - Any body for the `@ stub` exports: Wine never implemented them, so the only behaviour
//!   to model is the exception record.

mod enumerate;
mod exports;
mod manifest;
mod stub;

pub use enumerate::{
    tdh_enumerate_providers, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER, ERROR_SUCCESS,
    PROVIDER_ENUMERATION_HEADER_SIZE,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use manifest::{tdh_load_manifest, tdh_load_manifest_from_binary, STATUS_SUCCESS};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
