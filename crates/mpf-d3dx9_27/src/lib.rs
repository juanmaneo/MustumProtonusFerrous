//! Ported records for Wine's `dlls/d3dx9_27/d3dx9_27.spec`.
//!
//! Ported:
//! - All 68 `@ stub` exports, represented as the `StubException` records raised by
//!   `__wine_spec_unimplemented_stub` (`stub.rs`).
//!
//! Skipped:
//! - The DLL sets `PARENTSRC = ../d3dx9_36`; its C bodies live in `dlls/d3dx9_36`. Per the
//!   porting scope, only the local spec table and stub records are included here. All non-stub
//!   exports and their parent-source implementations are skipped.
//! - Native DLL loading, pointer ABI details, and the infinite exception-raising loop are modeled
//!   as a returned exception record.

mod stub;

pub use stub::{
    stub_exports, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, MODULE_NAME,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
