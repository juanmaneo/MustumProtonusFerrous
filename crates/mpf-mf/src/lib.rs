//! mf subset for MPF: the decision logic of `dlls/mf` for the growth of its element arrays, the
//! presentation clock, the topology connect methods, the session node states and the sample grabber
//! sink.
//!
//! Reference Wine's `dlls/mf` implements the Media Foundation session, clock, topology loader and
//! sample grabber on top of COM objects, threads, timers, event queues and the sample pipeline. This
//! crate ports the decisions those objects make from their current state and the requested command.
//! It works on enums, integers and slices, with no COM objects, allocation, threads, timers or
//! queues.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `array`: `mf_array_reserve` from `mf_private.h`, the capacity growth policy.
//! - `clock`: `clock_change_state` from `clock.c`: the check order, the notification, the restart
//!   case and the periodic-callback transition.
//! - `connect`: `connect_method_from_mf` from `topology_loader.c`.
//! - `session`: the event-to-object-state map from `session.c`, and the source and output node
//!   predicates.
//! - `sink`: `sample_grabber_set_state` from `samplegrabber.c`: the transition rules and the number
//!   of sample requests.
//! - The crate root holds the `HRESULT` codes, the `PRESENTATION_CURRENT_POSITION` sentinel, the
//!   `Hresult` alias and `failed`, the `FAILED` macro.
//!
//! Deviations from the C:
//! - `array::reserve_capacity` plans the new capacity only. The caller allocates, and a failed
//!   `realloc` is the caller's to report. A zero element size with a count above the capacity reports
//!   `TooLarge`, where the C divides by zero.
//! - `clock::plan_change` returns the notification, the new state and the timer action. The caller
//!   calls the time source's state sink, notifies the other sinks and adds or removes the periodic
//!   callback. An error from the state sink is the caller's to report.
//! - `TimerAction::Stop` means the caller removes the periodic callback if it holds one. The C only
//!   calls `MFRemovePeriodicCallback` when the key is non-zero.
//! - `sink::plan_set_state` returns the steps and runs none of them. The caller cancels the timer,
//!   releases the queued items, sends the requests, queues the events and calls the clock callback.
//!   Event codes are not part of the plan.
//! - `SinkPlan::requests` is a `u8`, as the C `byte` is. A pending count above four wraps the way the
//!   C does: `4 - pending` is an unsigned 32-bit value, and only its low byte is kept.
//!
//! Skipped (reason per item):
//! - Session command gating and the session worker loop: they drive COM objects and a thread.
//! - `copier.c`: its buffer size comes from mfplat's `MFCalculateImageSize`, a cross-crate dependency.
//! - `evr.c`, `sar.c`, the rest of `topology*.c`, `quality.c`, `sac.c`, `scheme_handler.c` and
//!   `main.c`: COM objects, registry access or handles.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub mod array;
pub mod clock;
pub mod connect;
pub mod session;
pub mod sink;

/// `HRESULT`.
pub type Hresult = i32;

/// `MF_E_INVALIDREQUEST`.
pub const MF_E_INVALIDREQUEST: Hresult = 0xC00D_36B2u32 as i32;

/// `MF_E_INVALID_STATE_TRANSITION`.
pub const MF_E_INVALID_STATE_TRANSITION: Hresult = 0xC00D_3E82u32 as i32;

/// `MF_E_CLOCK_NO_TIME_SOURCE`.
pub const MF_E_CLOCK_NO_TIME_SOURCE: Hresult = 0xC00D_9C41u32 as i32;

/// `MF_E_CLOCK_STATE_ALREADY_SET`.
pub const MF_E_CLOCK_STATE_ALREADY_SET: Hresult = 0xC00D_9C42u32 as i32;

/// `PRESENTATION_CURRENT_POSITION`: the clock offset that means "from the current position".
pub const PRESENTATION_CURRENT_POSITION: i64 = 0x7fff_ffff_ffff_ffff;

/// `FAILED`: whether `hr` is a failure, which is the sign bit of an `HRESULT`.
pub const fn failed(hr: Hresult) -> bool {
    hr < 0
}
