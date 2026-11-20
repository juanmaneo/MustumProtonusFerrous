//! `clock_change_state` from `dlls/mf/clock.c`: the presentation clock's state machine. A command
//! is checked against the current state. The plan gives the notification for the clock's sinks, the
//! state the clock moves to, and whether the periodic callback starts or stops.

use crate::{
    Hresult, MF_E_CLOCK_NO_TIME_SOURCE, MF_E_CLOCK_STATE_ALREADY_SET, MF_E_INVALIDREQUEST,
    PRESENTATION_CURRENT_POSITION,
};

/// `MFCLOCK_STATE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// `MFCLOCK_STATE_INVALID`: the clock has not run yet.
    Invalid,
    /// `MFCLOCK_STATE_RUNNING`.
    Running,
    /// `MFCLOCK_STATE_STOPPED`.
    Stopped,
    /// `MFCLOCK_STATE_PAUSED`.
    Paused,
}

/// `enum clock_command`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Start,
    Stop,
    Pause,
    /// Changes the rate and keeps the state.
    SetRate,
}

/// `enum clock_notification`: what the clock's sinks are told.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notification {
    Start,
    Stop,
    Pause,
    /// A start from the paused state at the current position.
    Restart,
    SetRate,
}

/// What the periodic callback does when the clock changes state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerAction {
    None,
    /// The clock entered the running state, so the callback is added.
    Start,
    /// The clock left the running state, so the callback is removed if the caller holds one.
    Stop,
}

/// The steps of a state change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Change {
    pub notification: Notification,
    pub state: State,
    pub timer: TimerAction,
}

/// Plans a clock command. The checks run in the C order: a missing time source, a command that
/// would set the state it already has, then a pause from a stopped or paused clock. `offset` is the
/// start position, and `PRESENTATION_CURRENT_POSITION` means the current one.
pub fn plan_change(
    state: State,
    command: Command,
    has_time_source: bool,
    offset: i64,
) -> Result<Change, Hresult> {
    if !has_time_source {
        return Err(MF_E_CLOCK_NO_TIME_SOURCE);
    }

    let target = match command {
        Command::Start => State::Running,
        Command::Stop => State::Stopped,
        Command::Pause => State::Paused,
        Command::SetRate => state,
    };
    if command != Command::SetRate && target == state && state != State::Running {
        return Err(MF_E_CLOCK_STATE_ALREADY_SET);
    }
    if command == Command::Pause && matches!(state, State::Stopped | State::Paused) {
        return Err(MF_E_INVALIDREQUEST);
    }

    let notification = match command {
        Command::Start if state == State::Paused && offset == PRESENTATION_CURRENT_POSITION => {
            Notification::Restart
        }
        Command::Start => Notification::Start,
        Command::Stop => Notification::Stop,
        Command::Pause => Notification::Pause,
        Command::SetRate => Notification::SetRate,
    };

    let timer = match (target == State::Running, state == State::Running) {
        (true, false) => TimerAction::Start,
        (false, true) => TimerAction::Stop,
        _ => TimerAction::None,
    };

    Ok(Change {
        notification,
        state: target,
        timer,
    })
}
