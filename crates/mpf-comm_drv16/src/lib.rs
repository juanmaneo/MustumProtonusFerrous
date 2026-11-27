//! Port of the portable logic in Wine's `dlls/comm.drv16/comm.drv16.spec`.
//!
//! `dlls/comm.drv16` has no C sources, so it has no deterministic logic to port. Every export in
//! its spec file is `@ stub`, and the crate models those stubs instead.
//!
//! Ported:
//! - The `@ stub` exports of `comm.drv16.spec` and the `__wine_spec_unimplemented_stub` record each
//!   one raises: one helper per export in [`stub`], with [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//! - `comm.drv16.spec` export table: [`EXPORTS`] and [`export_stub`]
//!
//! Skipped:
//! - `#16 stub WEP`: the spec comments it out, so it is not exported and not modelled.
//! - The `__cdecl` and `WINAPI` calling conventions: the stub loops forever, so the port returns
//!   the record instead of raising it.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
