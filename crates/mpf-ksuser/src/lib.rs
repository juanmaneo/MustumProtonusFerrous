//! Port of the portable logic in Wine's `dlls/ksuser/ksuser.spec` and `dlls/ksuser/Makefile.in`.
//!
//! `dlls/ksuser` has no C, IDL, resource or type library sources, so it has no executable code
//! of its own. Its export table has four entries, and all four are `@ stub`.
//!
//! Ported:
//! - `ksuser.spec` export table: [`EXPORTS`], [`stub_for_export`]
//! - The four `@ stub` exports: [`ks_create_allocator`], [`ks_create_clock`], [`ks_create_pin`],
//!   [`ks_create_topology_node`]
//! - `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) and the record it raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - Ordinals. The spec gives no `@ N` numbers, and the port takes the function name, so the
//!   ordinal form of the stub's second argument is not modelled.
//! - The PE export directory and the `RtlRaiseException` call. The port returns the record,
//!   because the C function never returns.
//! - `VER_PRODUCTVERSION` in `Makefile.in`, which is version resource metadata.

mod exports;
mod stub;

pub use exports::{stub_for_export, EXPORTS};
pub use stub::{
    ks_create_allocator, ks_create_clock, ks_create_pin, ks_create_topology_node,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
