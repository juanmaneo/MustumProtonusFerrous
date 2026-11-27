//! Port of the portable logic in Wine's `dlls/msdelta/msdelta.spec` and `dlls/msdelta/Makefile.in`.
//!
//! `dlls/msdelta` has no `.c`, `.idl`, `.rc` or `.tlb` file. Its spec exports 15 names, all `@ stub`,
//! so this port models those stubs and the export table. The crate is partial by design.
//!
//! Ported:
//! - `msdelta.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The 15 `@ stub` exports, one helper each (for example [`apply_delta_a`]), and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The delta logic (`ApplyDelta*`, `CreateDelta*`, `GetDeltaInfo*`, `GetDeltaSignature*`,
//!   `DeltaFree`, `DeltaNormalizeProvidedB`). Wine ships only the stubs, so the reference has no
//!   code to port.
//! - `EXTRADLLFLAGS = -Wb,--prefer-native` in `Makefile.in`. It changes the build, not runtime
//!   behaviour.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    apply_delta_a, apply_delta_b, apply_delta_provided_b, apply_delta_w, create_delta_a,
    create_delta_b, create_delta_w, delta_free, delta_normalize_provided_b, get_delta_info_a,
    get_delta_info_b, get_delta_info_w, get_delta_signature_a, get_delta_signature_b,
    get_delta_signature_w, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
