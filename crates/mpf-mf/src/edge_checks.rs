//! Edge tests for the mf decision logic: the capacity limits, the check order of the clock, the
//! session event groups and the sink's request count at its boundaries. They cover the branches
//! that the oracle rows do not reach.

use crate::array::{reserve_capacity, Reserve};
use crate::clock::{plan_change, Change, Command, Notification, State, TimerAction};
use crate::connect::{connect_method_from_mf, CONNECT_CONVERTER, CONNECT_DECODER, CONNECT_DIRECT};
use crate::session::{
    object_state_for_event, output_nodes_in_state, source_nodes_in_state, CommandState, NodeType,
    ObjectState,
};
use crate::sink::{plan_set_state, SinkPlan, SinkState, SinkStatus};
use crate::{
    failed, MF_E_CLOCK_NO_TIME_SOURCE, MF_E_CLOCK_STATE_ALREADY_SET, MF_E_INVALIDREQUEST,
    PRESENTATION_CURRENT_POSITION,
};

fn status(is_shut_down: bool, state: SinkState, rate: f32, pending: u32) -> SinkStatus {
    SinkStatus {
        is_shut_down,
        state,
        rate,
        pending_sample_deliveries: pending,
    }
}

fn plan_of(status: SinkStatus, requested: SinkState, offset: i64) -> SinkPlan {
    plan_set_state(&status, requested, offset)
        .ok()
        .flatten()
        .expect("a live sink has a plan")
}

#[test]
fn reserve_fits_when_the_count_is_within_capacity() {
    assert_eq!(reserve_capacity(8, 8, 8), Reserve::Fits);
    assert_eq!(reserve_capacity(8, 0, 8), Reserve::Fits);
}

#[test]
fn zero_element_size_fits_within_capacity() {
    assert_eq!(reserve_capacity(4, 4, 0), Reserve::Fits);
}

#[test]
fn zero_element_size_is_too_large() {
    assert_eq!(reserve_capacity(0, 1, 0), Reserve::TooLarge);
}

#[test]
fn capacity_doubles_from_four() {
    assert_eq!(reserve_capacity(4, 5, 1), Reserve::Grow(8));
    assert_eq!(reserve_capacity(8, 9, 1), Reserve::Grow(16));
    assert_eq!(reserve_capacity(4, 16, 1), Reserve::Grow(16));
}

#[test]
fn capacity_is_capped_at_the_largest_addressable_count() {
    assert_eq!(
        reserve_capacity(0, usize::MAX, 1),
        Reserve::Grow(usize::MAX)
    );
    assert_eq!(
        reserve_capacity(0, usize::MAX / 2, 2),
        Reserve::Grow(usize::MAX / 2)
    );
    assert_eq!(
        reserve_capacity(0, usize::MAX / 2 + 1, 2),
        Reserve::TooLarge
    );
}

#[test]
fn missing_time_source_wins_over_the_other_checks() {
    assert_eq!(
        plan_change(State::Stopped, Command::Stop, false, 0),
        Err(MF_E_CLOCK_NO_TIME_SOURCE)
    );
    assert_eq!(
        plan_change(State::Stopped, Command::Pause, false, 0),
        Err(MF_E_CLOCK_NO_TIME_SOURCE)
    );
}

#[test]
fn already_set_wins_over_an_invalid_request() {
    assert_eq!(
        plan_change(State::Paused, Command::Pause, true, 0),
        Err(MF_E_CLOCK_STATE_ALREADY_SET)
    );
    assert_eq!(
        plan_change(State::Stopped, Command::Pause, true, 0),
        Err(MF_E_INVALIDREQUEST)
    );
}

#[test]
fn start_from_running_notifies_without_touching_the_timer() {
    assert_eq!(
        plan_change(State::Running, Command::Start, true, 0),
        Ok(Change {
            notification: Notification::Start,
            state: State::Running,
            timer: TimerAction::None,
        })
    );
}

#[test]
fn start_from_paused_at_an_explicit_offset_is_a_plain_start() {
    assert_eq!(
        plan_change(State::Paused, Command::Start, true, 1000),
        Ok(Change {
            notification: Notification::Start,
            state: State::Running,
            timer: TimerAction::Start,
        })
    );
}

#[test]
fn pause_from_running_removes_the_timer() {
    assert_eq!(
        plan_change(State::Running, Command::Pause, true, 0),
        Ok(Change {
            notification: Notification::Pause,
            state: State::Paused,
            timer: TimerAction::Stop,
        })
    );
}

#[test]
fn stop_from_paused_leaves_the_timer_alone() {
    assert_eq!(
        plan_change(State::Paused, Command::Stop, true, 0),
        Ok(Change {
            notification: Notification::Stop,
            state: State::Stopped,
            timer: TimerAction::None,
        })
    );
}

#[test]
fn set_rate_keeps_the_state_and_the_timer() {
    assert_eq!(
        plan_change(State::Paused, Command::SetRate, true, 0),
        Ok(Change {
            notification: Notification::SetRate,
            state: State::Paused,
            timer: TimerAction::None,
        })
    );
    assert_eq!(
        plan_change(State::Running, Command::SetRate, true, 0),
        Ok(Change {
            notification: Notification::SetRate,
            state: State::Running,
            timer: TimerAction::None,
        })
    );
}

#[test]
fn connect_methods_follow_the_allow_bits() {
    assert_eq!(
        connect_method_from_mf(1),
        CONNECT_DIRECT | CONNECT_CONVERTER
    );
    assert_eq!(connect_method_from_mf(2), CONNECT_DIRECT);
    assert_eq!(
        connect_method_from_mf(3),
        CONNECT_DIRECT | CONNECT_CONVERTER | CONNECT_DECODER
    );
}

#[test]
fn connect_methods_ignore_the_bits_above_the_allow_flags() {
    assert_eq!(connect_method_from_mf(0xffff_fffc), CONNECT_DIRECT);
    assert_eq!(
        connect_method_from_mf(u32::MAX),
        CONNECT_DIRECT | CONNECT_CONVERTER | CONNECT_DECODER
    );
}

#[test]
fn failed_is_the_sign_bit_of_an_hresult() {
    assert!(failed(MF_E_INVALIDREQUEST));
    assert!(!failed(0));
    assert!(!failed(1));
}

#[test]
fn unknown_events_are_invalid() {
    for event in [0, 200, 308, u32::MAX] {
        assert_eq!(
            object_state_for_event(event, CommandState::Complete),
            ObjectState::Invalid
        );
    }
}

#[test]
fn stopped_sink_event_is_invalid_while_the_sinks_are_starting() {
    assert_eq!(
        object_state_for_event(302, CommandState::PrerollingSinks),
        ObjectState::Invalid
    );
    assert_eq!(
        object_state_for_event(302, CommandState::StartingSinks),
        ObjectState::Invalid
    );
}

#[test]
fn stopped_sink_event_is_stopped_in_other_command_states() {
    assert_eq!(
        object_state_for_event(302, CommandState::Submitted),
        ObjectState::Stopped
    );
    assert_eq!(
        object_state_for_event(302, CommandState::StoppingSinks),
        ObjectState::Stopped
    );
}

#[test]
fn started_and_paused_events_ignore_the_command_state() {
    for event in [201, 202, 203, 204, 301] {
        assert_eq!(
            object_state_for_event(event, CommandState::StartingSinks),
            ObjectState::Started
        );
    }
    for event in [209, 210, 303] {
        assert_eq!(
            object_state_for_event(event, CommandState::PausingSinks),
            ObjectState::Paused
        );
    }
}

#[test]
fn source_stopped_events_and_prerolled_events_map_directly() {
    for event in [207, 208] {
        assert_eq!(
            object_state_for_event(event, CommandState::StartingSinks),
            ObjectState::Stopped
        );
    }
    assert_eq!(
        object_state_for_event(307, CommandState::Complete),
        ObjectState::Prerolled
    );
}

#[test]
fn empty_node_lists_are_in_every_state() {
    assert!(source_nodes_in_state(&[], &[], ObjectState::Started));
    assert!(output_nodes_in_state(&[], ObjectState::Started));
}

#[test]
fn one_source_out_of_state_fails_the_source_check() {
    assert!(!source_nodes_in_state(
        &[ObjectState::Started, ObjectState::Paused],
        &[],
        ObjectState::Started
    ));
}

#[test]
fn transform_and_tee_nodes_are_not_checked() {
    let nodes = [
        (NodeType::Transform, ObjectState::Paused),
        (NodeType::Tee, ObjectState::Paused),
    ];
    assert!(source_nodes_in_state(
        &[ObjectState::Started],
        &nodes,
        ObjectState::Started
    ));
    assert!(output_nodes_in_state(&nodes, ObjectState::Started));
}

#[test]
fn one_output_out_of_state_fails_the_output_check() {
    assert!(!output_nodes_in_state(
        &[
            (NodeType::Output, ObjectState::Started),
            (NodeType::Output, ObjectState::Stopped),
        ],
        ObjectState::Started
    ));
}

#[test]
fn pending_deliveries_reduce_the_requests_for_a_start() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Stopped, 1.0, 3),
            SinkState::Running,
            PRESENTATION_CURRENT_POSITION
        ),
        SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 1,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Running,
        }
    );
}

#[test]
fn pending_deliveries_past_the_queue_wrap_the_request_byte() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Stopped, 1.0, 5),
            SinkState::Running,
            PRESENTATION_CURRENT_POSITION
        )
        .requests,
        255
    );
}

#[test]
fn pausing_a_paused_sink_does_not_notify() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Paused, 1.0, 0),
            SinkState::Paused,
            PRESENTATION_CURRENT_POSITION
        ),
        SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 0,
            scrub_complete: false,
            notify: false,
            next_state: SinkState::Paused,
        }
    );
}

#[test]
fn start_at_rate_zero_scrubs_before_the_state_event() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Stopped, 0.0, 0),
            SinkState::Running,
            PRESENTATION_CURRENT_POSITION
        ),
        SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 4,
            scrub_complete: true,
            notify: true,
            next_state: SinkState::Running,
        }
    );
}

#[test]
fn pausing_from_running_notifies_without_a_scrub() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Running, 0.0, 0),
            SinkState::Paused,
            PRESENTATION_CURRENT_POSITION
        ),
        SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 0,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Paused,
        }
    );
}

#[test]
fn stopping_a_stopped_sink_resets_the_deliveries() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Stopped, 1.0, 2),
            SinkState::Stopped,
            PRESENTATION_CURRENT_POSITION
        ),
        SinkPlan {
            cancel_timer: true,
            release_pending: true,
            reset_deliveries: true,
            requests: 0,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Stopped,
        }
    );
}

#[test]
fn start_at_an_explicit_offset_from_paused_cancels_and_requests() {
    assert_eq!(
        plan_of(
            status(false, SinkState::Paused, 1.0, 1),
            SinkState::Running,
            1000
        ),
        SinkPlan {
            cancel_timer: true,
            release_pending: true,
            reset_deliveries: false,
            requests: 3,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Running,
        }
    );
}

#[test]
fn shut_down_sink_changes_nothing_even_for_an_invalid_pause() {
    assert_eq!(
        plan_set_state(
            &status(true, SinkState::Stopped, 1.0, 0),
            SinkState::Paused,
            PRESENTATION_CURRENT_POSITION
        ),
        Ok(None)
    );
}
