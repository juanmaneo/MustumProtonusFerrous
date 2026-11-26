//! Port of the portable logic in Wine's `dlls/dcomp/device.c`, plus the stub
//! exports declared in `dlls/dcomp/dcomp.spec`.
//!
//! Ported:
//! - `DCompositionCreateDevice`, `DCompositionCreateDevice2` and
//!   `DCompositionCreateDevice3`: each logs `FIXME` and returns `E_NOTIMPL` without
//!   reading or writing its arguments: [`dcomposition_create_device`],
//!   [`dcomposition_create_device2`], [`dcomposition_create_device3`], [`E_NOTIMPL`]
//! - `stub` exports in `dcomp.spec` that have a name: their `__wine_spec_unimplemented_stub`
//!   behaviour (`ntdll/exception.c`) raises `EXCEPTION_WINE_STUB` and never returns:
//!   [`unimplemented_stub_exception`], [`is_spec_stub`], [`SPEC_STUBS`]
//!
//! Skipped:
//! - `DllCanUnloadNow` (`-private`): no source in `device.c`.
//! - The ten `stub @` ordinal-only exports (1017 to 1042): they have no name, so
//!   there is nothing to bind to.
//! - `FIXME` logging, `WINE_DEFAULT_DEBUG_CHANNEL`, and the `IDXGIDevice` and
//!   `IUnknown` objects, which the C code never dereferences.

mod device;
mod stub;

pub use device::{
    dcomposition_create_device, dcomposition_create_device2, dcomposition_create_device3, E_NOTIMPL,
};
pub use stub::{
    is_spec_stub, unimplemented_stub_exception, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, SPEC_STUBS,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
