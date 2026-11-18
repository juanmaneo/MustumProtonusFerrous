//! The status items that `MCIQTZ_mciStatus` answers, and the `dwReturn` each one sets.

use crate::abi::{
    failed, make_mci_resource, HResult, MCIERR_INTERNAL, MCIERR_UNRECOGNIZED_COMMAND,
    MCI_INTEGER_RETURNED, MCI_MODE_PAUSE, MCI_MODE_PLAY, MCI_MODE_STOP, MCI_RESOURCE_RETURNED,
    MCI_STATUS_ITEM, MCI_STATUS_LENGTH, MCI_STATUS_MODE, MCI_STATUS_POSITION,
    MCI_STATUS_TIME_FORMAT, STATE_PAUSED, STATE_RUNNING, STATE_STOPPED, WAIT_OBJECT_0,
};
use crate::time::TimeFormat;

/// The status items that have a reply in `MCIQTZ_mciStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusItem {
    Length,
    Position,
    Mode,
    Format,
}

impl StatusItem {
    /// The return value that goes with `dwReturn`: an integer or a resource string.
    pub const fn reply(self) -> u32 {
        match self {
            Self::Length | Self::Position => MCI_INTEGER_RETURNED,
            Self::Mode | Self::Format => MCI_RESOURCE_RETURNED,
        }
    }
}

/// `MCI_STATUS` needs `MCI_STATUS_ITEM`. Of the items, length, position, mode and time format
/// have replies. The rest return [MCIERR_UNRECOGNIZED_COMMAND] in Wine.
pub fn status_item(flags: u32, item: u32) -> Result<StatusItem, u32> {
    if flags & MCI_STATUS_ITEM == 0 {
        return Err(MCIERR_UNRECOGNIZED_COMMAND);
    }
    match item {
        MCI_STATUS_LENGTH => Ok(StatusItem::Length),
        MCI_STATUS_POSITION => Ok(StatusItem::Position),
        MCI_STATUS_MODE => Ok(StatusItem::Mode),
        MCI_STATUS_TIME_FORMAT => Ok(StatusItem::Format),
        _ => Err(MCIERR_UNRECOGNIZED_COMMAND),
    }
}

/// The length of the media. Any failed call or a negative duration gives 0. Media time is in
/// 100 ns units and is divided down to milliseconds. Frames are returned as they are.
pub fn length(
    format: TimeFormat,
    set_format_hr: HResult,
    duration_hr: HResult,
    duration: i64,
) -> u64 {
    if failed(set_format_hr) || failed(duration_hr) || duration < 0 {
        return 0;
    }
    match format {
        TimeFormat::Milliseconds => (duration / 10000) as u64,
        TimeFormat::Frames => duration as u64,
    }
}

/// The current position in milliseconds. A failed call returns [MCIERR_INTERNAL]. A negative
/// position keeps its sign in the 64-bit `dwReturn`, as in Wine.
pub fn position(hr: HResult, current: i64) -> Result<u64, u32> {
    if failed(hr) {
        Err(MCIERR_INTERNAL)
    } else {
        Ok((current / 10000) as u64)
    }
}

/// Whether the notify thread of the device is still running, from `WaitForSingleObject` on its
/// handle with a zero timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyThread {
    /// No thread was started.
    Absent,
    /// The handle is not signalled, so the thread still runs.
    Alive,
    /// The handle is signalled, so the thread has finished.
    Finished,
}

impl NotifyThread {
    /// Reads the result of `WaitForSingleObject(thread, 0)`.
    pub const fn from_wait(wait: u32) -> Self {
        if wait == WAIT_OBJECT_0 {
            Self::Finished
        } else {
            Self::Alive
        }
    }
}

/// The `dwReturn` of `MCI_STATUS_MODE` for a filter-graph state. Stopped, and running with no
/// live thread, read as stop. Paused reads as pause. Any other state leaves `dwReturn` unchanged,
/// which is `None` here.
pub const fn mode(state: i32, thread: NotifyThread) -> Option<u32> {
    match state {
        STATE_STOPPED => Some(make_mci_resource(MCI_MODE_STOP, MCI_MODE_STOP)),
        STATE_RUNNING => match thread {
            NotifyThread::Alive => Some(make_mci_resource(MCI_MODE_PLAY, MCI_MODE_PLAY)),
            NotifyThread::Absent | NotifyThread::Finished => {
                Some(make_mci_resource(MCI_MODE_STOP, MCI_MODE_STOP))
            }
        },
        STATE_PAUSED => Some(make_mci_resource(MCI_MODE_PAUSE, MCI_MODE_PAUSE)),
        _ => None,
    }
}
