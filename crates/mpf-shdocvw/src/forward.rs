//! Exports that hand their work to another module by ordinal: the `StopWatch*FORWARD` exports to
//! shlwapi, and `ShellDDEInit` to shell32. The C code caches the resolved pointer in a `static`.
//! The port takes the resolver as an argument, so the cache is not modelled.

use crate::fixed::ERROR_CALL_NOT_IMPLEMENTED;

/// The shlwapi ordinal that `StopWatchModeFORWARD` resolves.
pub const SHLWAPI_STOP_WATCH_MODE: u16 = 241;
/// The shlwapi ordinal that `StopWatchFlushFORWARD` resolves.
pub const SHLWAPI_STOP_WATCH_FLUSH: u16 = 242;
/// The shlwapi ordinal that `StopWatchAFORWARD` resolves.
pub const SHLWAPI_STOP_WATCH_A: u16 = 243;
/// The shlwapi ordinal that `StopWatchWFORWARD` resolves.
pub const SHLWAPI_STOP_WATCH_W: u16 = 244;
/// The shell32 ordinal that `ShellDDEInit` resolves.
pub const SHELL32_SHELL_DDE_INIT: u16 = 188;

/// `StopWatchModeFORWARD`: calls the target that `resolve` finds for ordinal 241, if any.
pub fn stop_watch_mode_forward<F: FnOnce()>(resolve: impl FnOnce(u16) -> Option<F>) {
    if let Some(target) = resolve(SHLWAPI_STOP_WATCH_MODE) {
        target();
    }
}

/// `StopWatchFlushFORWARD`: calls the target that `resolve` finds for ordinal 242, if any.
pub fn stop_watch_flush_forward<F: FnOnce()>(resolve: impl FnOnce(u16) -> Option<F>) {
    if let Some(target) = resolve(SHLWAPI_STOP_WATCH_FLUSH) {
        target();
    }
}

/// `StopWatchAFORWARD`: returns the target's result for ordinal 243, or
/// `ERROR_CALL_NOT_IMPLEMENTED` when there is no target.
pub fn stop_watch_a_forward<S, F>(
    class: u32,
    text: S,
    unknown: u32,
    mode: u32,
    timestamp: u32,
    resolve: impl FnOnce(u16) -> Option<F>,
) -> u32
where
    F: FnOnce(u32, S, u32, u32, u32) -> u32,
{
    match resolve(SHLWAPI_STOP_WATCH_A) {
        Some(target) => target(class, text, unknown, mode, timestamp),
        None => ERROR_CALL_NOT_IMPLEMENTED,
    }
}

/// `StopWatchWFORWARD`: returns the target's result for ordinal 244, or
/// `ERROR_CALL_NOT_IMPLEMENTED` when there is no target.
pub fn stop_watch_w_forward<S, F>(
    class: u32,
    text: S,
    unknown: u32,
    mode: u32,
    timestamp: u32,
    resolve: impl FnOnce(u16) -> Option<F>,
) -> u32
where
    F: FnOnce(u32, S, u32, u32, u32) -> u32,
{
    match resolve(SHLWAPI_STOP_WATCH_W) {
        Some(target) => target(class, text, unknown, mode, timestamp),
        None => ERROR_CALL_NOT_IMPLEMENTED,
    }
}

/// `ShellDDEInit`: returns `FALSE` when shell32 is not available or has no ordinal 188. Otherwise
/// it returns the target's result.
pub fn shell_dde_init<F: FnOnce(bool) -> bool>(
    start: bool,
    shell32_available: bool,
    resolve: impl FnOnce(u16) -> Option<F>,
) -> bool {
    if !shell32_available {
        return false;
    }
    match resolve(SHELL32_SHELL_DDE_INIT) {
        Some(target) => target(start),
        None => false,
    }
}
