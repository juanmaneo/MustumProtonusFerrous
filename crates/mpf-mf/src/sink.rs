//! `sample_grabber_set_state` from `dlls/mf/samplegrabber.c`: the transition rules of the sample
//! grabber sink and the number of sample requests that a start asks for. The C sends the requests,
//! cancels the timer, releases the queued items and queues the events itself. This module returns
//! those steps as a plan and runs none of them.

use crate::{Hresult, MF_E_INVALID_STATE_TRANSITION, PRESENTATION_CURRENT_POSITION};

/// `MAX_SAMPLE_QUEUE_LENGTH`: the number of sample requests that the sink keeps outstanding.
pub const MAX_SAMPLE_QUEUE_LENGTH: u32 = 4;

/// `enum sink_state`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkState {
    Stopped,
    Paused,
    Running,
}

/// The sink fields that the transition reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SinkStatus {
    pub is_shut_down: bool,
    pub state: SinkState,
    pub rate: f32,
    pub pending_sample_deliveries: u32,
}

/// The steps of a state change, in the order the C runs them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SinkPlan {
    /// Cancel the scheduled sample timer.
    pub cancel_timer: bool,
    /// Release the queued sample items.
    pub release_pending: bool,
    /// Set the pending sample deliveries to zero.
    pub reset_deliveries: bool,
    /// Sample requests to send. This is a `byte` in the C, so the count wraps at 256.
    pub requests: u8,
    /// Queue the scrub-sample-complete event before the state event.
    pub scrub_complete: bool,
    /// Queue the state event and call the clock callback.
    pub notify: bool,
    pub next_state: SinkState,
}

/// Plans a change to `requested`. A shut-down sink changes nothing, so the plan is `None`. A pause
/// from a stopped sink is an error. `offset` is the start position, and `PRESENTATION_CURRENT_POSITION`
/// means the current one.
pub fn plan_set_state(
    status: &SinkStatus,
    requested: SinkState,
    offset: i64,
) -> Result<Option<SinkPlan>, Hresult> {
    if status.is_shut_down {
        return Ok(None);
    }
    if requested == SinkState::Paused && status.state == SinkState::Stopped {
        return Err(MF_E_INVALID_STATE_TRANSITION);
    }

    let is_current = offset == PRESENTATION_CURRENT_POSITION;
    let stop = requested == SinkState::Stopped;
    let refill =
        requested == SinkState::Running && (status.state != SinkState::Running || !is_current);
    let cancel_timer = stop || (refill && !is_current);
    let requests = if refill {
        MAX_SAMPLE_QUEUE_LENGTH.wrapping_sub(status.pending_sample_deliveries) as u8
    } else {
        0
    };
    let notify = requested != status.state || requested != SinkState::Paused;

    Ok(Some(SinkPlan {
        cancel_timer,
        release_pending: cancel_timer,
        reset_deliveries: stop,
        requests,
        scrub_complete: notify && status.rate == 0.0 && requested == SinkState::Running,
        notify,
        next_state: requested,
    }))
}
