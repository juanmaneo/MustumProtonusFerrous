//! Port of the deterministic logic in Wine's `dlls/wmvcore`.
//!
//! Ported:
//! - [`debugstr::debugstr_time`]: the time formatter in `wmvcore_private.h`,
//!   used by the async reader's TRACE calls.
//! - [`current_time`]: `get_current_time` in `async_reader.c`, with the
//!   performance counter reading passed in.
//! - [`clock::ReaderClock::wait_timeout`]: `async_reader_get_wait_timeout`, with
//!   the current time passed in.
//!
//! Skipped: the exported `WMCheckURLExtension`, `WMCheckURLScheme`,
//! `WMCreateEditor`, `WMCreateBackupRestorer` and `WMIsContentProtected`, which
//! are FIXME stubs; the COM objects in `writer.c` and `async_reader.c`; the
//! callback thread and its condition variable wait; and the reads of
//! `QueryPerformanceCounter`, which the caller supplies.
//!
//! Deviations:
//! - `debugstr_time(i64::MIN)` returns `-922337203685.4775808`. Wine negates the
//!   value in signed arithmetic, which is undefined behaviour in C.
//! - `current_time` and `ReaderClock::wait_timeout` wrap where Wine's signed
//!   arithmetic would overflow, which is undefined behaviour in C.
//! - `current_time` panics when `frequency` is zero, where Wine traps.

pub mod clock;
pub mod debugstr;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `get_current_time`: converts a performance counter reading to 100 ns units.
///
/// `counter` is the value from `QueryPerformanceCounter` and `frequency` the
/// value from `QueryPerformanceFrequency`.
pub fn current_time(counter: i64, frequency: i64) -> i64 {
    counter
        .wrapping_mul(1000)
        .wrapping_div(frequency)
        .wrapping_mul(10_000)
}
