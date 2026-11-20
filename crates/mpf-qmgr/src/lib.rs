//! Deterministic logic ported from Wine's `dlls/qmgr` (BITS).
//!
//! This crate contains the job-state helpers from `job.c`, HTTP response mapping and
//! authentication index tables from `file.c` and `job.c`, notify-flag validation from `job.c`,
//! and the shared `Next`/`Skip` index arithmetic from `enum_files.c` and `enum_jobs.c`.
//!
//! COM objects, callbacks, worker threads, registry/service integration, credential ownership,
//! and FIXME/stub exports are intentionally excluded.
//!
//! Wine uses different authentication-scheme orderings in `index_from_scheme` and
//! `scheme_from_index`; the port preserves both tables, including their non-round-tripping values.
//! Invalid auth indices return `None` instead of Wine's ambiguous zero. Enumerator remaining
//! counts saturate for a corrupt index past the end; Wine's unsigned subtraction wraps there.

pub mod auth;
pub mod enumerate;
pub mod http;
pub mod notify;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub type HResult = i32;

pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const E_NOTIMPL: HResult = 0x8000_4001_u32 as HResult;
pub const E_INVALIDARG: HResult = 0x8007_0057_u32 as HResult;
pub const BG_E_INVALID_STATE: HResult = 0x8020_0002_u32 as HResult;

/// `BG_JOB_STATE`, with values from Wine's `bits.idl`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobState {
    Queued = 0,
    Connecting = 1,
    Transferring = 2,
    Suspended = 3,
    Error = 4,
    TransientError = 5,
    Transferred = 6,
    Acknowledged = 7,
    Cancelled = 8,
}

/// A job is done once it is cancelled or acknowledged.
pub fn is_job_done(state: JobState) -> bool {
    matches!(state, JobState::Cancelled | JobState::Acknowledged)
}

/// Move from `from` to `to` only if the current state is `from`.
pub fn transition_job_state(state: &mut JobState, from: JobState, to: JobState) -> bool {
    if *state == from {
        *state = to;
        true
    } else {
        false
    }
}
