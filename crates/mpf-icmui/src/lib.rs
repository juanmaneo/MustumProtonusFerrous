//! Port of the portable logic in Wine's `dlls/icmui/icmui.c`, plus the stub-export behaviour
//! declared in `dlls/icmui/icmui.spec`.
//!
//! Ported:
//! - `SetupColorMatchingW` (`@ stdcall`): always `FALSE` with `ERROR_INVALID_PARAMETER`:
//!   [`setup_color_matching_w`], [`SetupColorMatchingWResult`], [`ERROR_INVALID_PARAMETER`]
//! - `SetupColorMatchingA` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`setup_color_matching_a`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` trace of `cms`, and the `__stdcall` calling convention. The C body only prints
//!   `cms` and never dereferences it, so the port takes no argument.
//! - The `WINE_DEFAULT_DEBUG_CHANNEL` setup.

mod setup;
mod stub;

pub use setup::{setup_color_matching_w, SetupColorMatchingWResult, ERROR_INVALID_PARAMETER};
pub use stub::{
    setup_color_matching_a, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
