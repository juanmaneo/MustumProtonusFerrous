//! Port of the portable logic in Wine's `dlls/msdrm/main.c` and `dlls/msdrm/msdrm.spec`.
//!
//! Ported:
//! - `DRMRegisterContent`: returns `S_OK` whatever its `BOOL` argument: [`drm_register_content`],
//!   [`S_OK`]
//! - The 85 `@ stub` exports, each as a helper named after the export in snake case (for example
//!   `DRMAcquireLicense` is [`drm_acquire_license`]). Each helper returns the
//!   `__wine_spec_unimplemented_stub` record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: `@ stdcall -private`
//!   exports with no body in `dlls/msdrm/main.c`, so there is no reference behaviour to port.
//! - The `FIXME` trace of the `value` argument.

mod content;
mod stub;

pub use content::{drm_register_content, S_OK};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
