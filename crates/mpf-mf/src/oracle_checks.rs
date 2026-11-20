//! Replays of the 25 oracle rows in `target/canon/mf`. The oracle compiles the verbatim ranges of
//! `mf_private.h`, `clock.c`, `topology_loader.c`, `session.c` and `samplegrabber.c` with
//! `gcc -fsanitize=address,undefined -g`, with the Win32 shims in `wine_mock.h`. Each `oracle_oNN`
//! test reproduces row NN, and its expected value is what the C printed for that row. No row trips
//! the sanitizer. The one vector that does, a zero element size with a count above the capacity, is
//! left out because the C divides by zero there. See `edge_checks::zero_element_size_is_too_large`.

use crate::array::{reserve_capacity, Reserve};
use crate::clock::{plan_change, Change, Command, Notification, State, TimerAction};
use crate::connect::connect_method_from_mf;
use crate::session::{
    object_state_for_event, output_nodes_in_state, source_nodes_in_state, CommandState, NodeType,
    ObjectState,
};
use crate::sink::{plan_set_state, SinkPlan, SinkState, SinkStatus};
use crate::{
    MF_E_CLOCK_NO_TIME_SOURCE, MF_E_CLOCK_STATE_ALREADY_SET, MF_E_INVALIDREQUEST,
    MF_E_INVALID_STATE_TRANSITION, PRESENTATION_CURRENT_POSITION,
};

#[test]
fn oracle_o01() {
    assert_eq!(
        reserve_capacity(0usize, 1usize, 8usize),
        Reserve::Grow(4usize)
    );
}

#[test]
fn oracle_o02() {
    assert_eq!(
        reserve_capacity(4usize, 5usize, 8usize),
        Reserve::Grow(8usize)
    );
}

#[test]
fn oracle_o03() {
    assert_eq!(
        reserve_capacity(0usize, 2305843009213693952usize, 8usize),
        Reserve::TooLarge
    );
}

#[test]
fn oracle_o04() {
    assert_eq!(
        reserve_capacity(0usize, 2305843009213693951usize, 8usize),
        Reserve::Grow(2305843009213693951usize)
    );
}

#[test]
fn oracle_o05() {
    assert_eq!(
        plan_change(State::Invalid, Command::Start, true, 0i64),
        Ok(Change {
            notification: Notification::Start,
            state: State::Running,
            timer: TimerAction::Start
        })
    );
}

#[test]
fn oracle_o06() {
    assert_eq!(
        plan_change(
            State::Paused,
            Command::Start,
            true,
            PRESENTATION_CURRENT_POSITION
        ),
        Ok(Change {
            notification: Notification::Restart,
            state: State::Running,
            timer: TimerAction::Start
        })
    );
}

#[test]
fn oracle_o07() {
    assert_eq!(
        plan_change(State::Running, Command::Stop, true, 0i64),
        Ok(Change {
            notification: Notification::Stop,
            state: State::Stopped,
            timer: TimerAction::Stop
        })
    );
}

#[test]
fn oracle_o08() {
    assert_eq!(
        plan_change(State::Stopped, Command::Stop, true, 0i64),
        Err(MF_E_CLOCK_STATE_ALREADY_SET)
    );
}

#[test]
fn oracle_o09() {
    assert_eq!(
        plan_change(State::Stopped, Command::Pause, true, 0i64),
        Err(MF_E_INVALIDREQUEST)
    );
}

#[test]
fn oracle_o10() {
    assert_eq!(
        plan_change(State::Invalid, Command::Pause, true, 0i64),
        Ok(Change {
            notification: Notification::Pause,
            state: State::Paused,
            timer: TimerAction::None
        })
    );
}

#[test]
fn oracle_o11() {
    assert_eq!(
        plan_change(State::Running, Command::SetRate, true, 0i64),
        Ok(Change {
            notification: Notification::SetRate,
            state: State::Running,
            timer: TimerAction::None
        })
    );
}

#[test]
fn oracle_o12() {
    assert_eq!(
        plan_change(State::Invalid, Command::Start, false, 0i64),
        Err(MF_E_CLOCK_NO_TIME_SOURCE)
    );
}

#[test]
fn oracle_o13() {
    assert_eq!(connect_method_from_mf(0x0u32), 1u32);
}

#[test]
fn oracle_o14() {
    assert_eq!(connect_method_from_mf(0x10003u32), 7u32);
}

#[test]
fn oracle_o15() {
    assert_eq!(
        object_state_for_event(201u32, CommandState::Complete),
        ObjectState::Started
    );
}

#[test]
fn oracle_o16() {
    assert_eq!(
        object_state_for_event(302u32, CommandState::StartingSinks),
        ObjectState::Invalid
    );
}

#[test]
fn oracle_o17() {
    assert_eq!(
        object_state_for_event(302u32, CommandState::Complete),
        ObjectState::Stopped
    );
}

#[test]
fn oracle_o18() {
    assert!(source_nodes_in_state(
        &[ObjectState::Started],
        &[
            (NodeType::SourceStream, ObjectState::Started),
            (NodeType::Output, ObjectState::Stopped)
        ],
        ObjectState::Started
    ));
}

#[test]
fn oracle_o19() {
    assert!(!source_nodes_in_state(
        &[ObjectState::Started],
        &[(NodeType::SourceStream, ObjectState::Paused)],
        ObjectState::Started
    ));
}

#[test]
fn oracle_o20() {
    assert!(!output_nodes_in_state(
        &[
            (NodeType::Output, ObjectState::Paused),
            (NodeType::SourceStream, ObjectState::Started)
        ],
        ObjectState::Started
    ));
}

#[test]
fn oracle_o21() {
    assert_eq!(
        plan_set_state(
            &SinkStatus {
                is_shut_down: false,
                state: SinkState::Stopped,
                rate: 1.0,
                pending_sample_deliveries: 0u32
            },
            SinkState::Running,
            PRESENTATION_CURRENT_POSITION
        ),
        Ok(Some(SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 4u8,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Running
        }))
    );
}

#[test]
fn oracle_o22() {
    assert_eq!(
        plan_set_state(
            &SinkStatus {
                is_shut_down: false,
                state: SinkState::Running,
                rate: 0.0,
                pending_sample_deliveries: 2u32
            },
            SinkState::Running,
            1000i64
        ),
        Ok(Some(SinkPlan {
            cancel_timer: true,
            release_pending: true,
            reset_deliveries: false,
            requests: 2u8,
            scrub_complete: true,
            notify: true,
            next_state: SinkState::Running
        }))
    );
}

#[test]
fn oracle_o23() {
    assert_eq!(
        plan_set_state(
            &SinkStatus {
                is_shut_down: false,
                state: SinkState::Running,
                rate: 1.0,
                pending_sample_deliveries: 1u32
            },
            SinkState::Running,
            PRESENTATION_CURRENT_POSITION
        ),
        Ok(Some(SinkPlan {
            cancel_timer: false,
            release_pending: false,
            reset_deliveries: false,
            requests: 0u8,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Running
        }))
    );
}

#[test]
fn oracle_o24() {
    assert_eq!(
        plan_set_state(
            &SinkStatus {
                is_shut_down: false,
                state: SinkState::Stopped,
                rate: 1.0,
                pending_sample_deliveries: 0u32
            },
            SinkState::Paused,
            PRESENTATION_CURRENT_POSITION
        ),
        Err(MF_E_INVALID_STATE_TRANSITION)
    );
}

#[test]
fn oracle_o25() {
    assert_eq!(
        plan_set_state(
            &SinkStatus {
                is_shut_down: false,
                state: SinkState::Paused,
                rate: 1.0,
                pending_sample_deliveries: 3u32
            },
            SinkState::Stopped,
            PRESENTATION_CURRENT_POSITION
        ),
        Ok(Some(SinkPlan {
            cancel_timer: true,
            release_pending: true,
            reset_deliveries: true,
            requests: 0u8,
            scrub_complete: false,
            notify: true,
            next_state: SinkState::Stopped
        }))
    );
}
