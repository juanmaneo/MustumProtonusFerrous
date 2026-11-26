//! Port of the portable logic in Wine's `dlls/qwave/main.c`.
//!
//! Ported:
//! - `QOSCreateHandle` argument validation, which picks the value passed to
//!   `SetLastError`: [`qos_create_handle`]
//!
//! Skipped:
//! - `QOSCreateHandle` handle creation: Wine never creates a handle. Its success
//!   return of `TRUE` is marked `todo_wine` in `tests/qos.c`, so the BOOL result is
//!   always FALSE here.
//! - `QOSAddSocketToFlow` and `QOSCloseHandle`: fixed-result stubs
//!   (`ERROR_NOT_SUPPORTED` / `FALSE`).
//! - Spec exports with no code in `main.c` (`QDLHPathDiagnostics`, `QOSCancel`,
//!   `QOSEnumerateFlows`, ..., `ServiceMain`).
//! - `FIXME` tracing.

/// `ERROR_INVALID_PARAMETER` from `winerror.h`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_SERVICE_ALREADY_RUNNING` from `winerror.h`. Wine's placeholder result for valid input.
pub const ERROR_SERVICE_ALREADY_RUNNING: u32 = 1056;

/// `QOS_VERSION` from `qos2.h`. A NULL `PQOS_VERSION` is `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QosVersion {
    pub major: u16,
    pub minor: u16,
}

/// `QOSCreateHandle` without the OS calls.
///
/// `version` is the `PQOS_VERSION` argument (`None` for NULL). `handle_present`
/// says whether the `PHANDLE` output pointer is non-NULL. Returns the value Wine
/// passes to `SetLastError`. The BOOL result is always FALSE.
pub fn qos_create_handle(version: Option<QosVersion>, handle_present: bool) -> u32 {
    match version {
        Some(QosVersion { major: 1, minor: 0 }) if handle_present => ERROR_SERVICE_ALREADY_RUNNING,
        _ => ERROR_INVALID_PARAMETER,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
