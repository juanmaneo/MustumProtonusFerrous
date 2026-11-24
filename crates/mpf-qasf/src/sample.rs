//! The times and flags a sample gets in Wine's `dlls/qasf`: `OnSample` of the ASF reader, which
//! stamps a sample from the WM reader before it goes downstream, and `Receive` of the DMO wrapper,
//! which turns a sample's time into the `ProcessInput` arguments.

use crate::SampleTime;

/// `WM_SF_CLEANPOINT`.
pub const WM_SF_CLEANPOINT: u32 = 0x1;
/// `WM_SF_DISCONTINUITY`.
pub const WM_SF_DISCONTINUITY: u32 = 0x2;

/// `DMO_INPUT_DATA_BUFFERF_SYNCPOINT`.
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: u32 = 0x1;
/// `DMO_INPUT_DATA_BUFFERF_TIME`.
pub const DMO_INPUT_DATA_BUFFERF_TIME: u32 = 0x2;
/// `DMO_INPUT_DATA_BUFFERF_TIMELENGTH`.
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: u32 = 0x4;

/// A sample from the WM reader, stamped with the time and flags `OnSample` sets on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamSample {
    pub time: SampleTime,
    pub discontinuity: bool,
    pub sync_point: bool,
}

/// `OnSample` for one output. An output with no connected pin takes no sample (`None`). The stop
/// is the time plus the duration, and the flags are the `WM_SF_*` bits of `flags`.
pub fn stream_sample(
    connected: bool,
    time: u64,
    duration: u64,
    flags: u32,
) -> Option<StreamSample> {
    if !connected {
        return None;
    }
    Some(StreamSample {
        time: SampleTime::span(time as i64, duration as i64),
        discontinuity: flags & WM_SF_DISCONTINUITY != 0,
        sync_point: flags & WM_SF_CLEANPOINT != 0,
    })
}

/// What `GetTime` reported for a sample on the DMO input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputTime {
    /// The call failed. The start and stop keep their zero initial values.
    Unset,
    /// `VFW_S_NO_STOP_TIME`: a start with no stop.
    NoStop { start: i64 },
    /// `S_OK`: a start and a stop.
    Range { start: i64, stop: i64 },
}

/// The arguments of `ProcessInput` for one sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputBuffer {
    pub flags: u32,
    pub start: i64,
    pub length: i64,
}

/// The `ProcessInput` flags and time for a sample. A sample with no stop gets the stop one unit
/// after its start, so its length is 1.
pub fn input_buffer(sync_point: bool, time: InputTime) -> InputBuffer {
    let mut flags = if sync_point {
        DMO_INPUT_DATA_BUFFERF_SYNCPOINT
    } else {
        0
    };
    let (start, stop) = match time {
        InputTime::Unset => (0, 0),
        InputTime::NoStop { start } => {
            flags |= DMO_INPUT_DATA_BUFFERF_TIME | DMO_INPUT_DATA_BUFFERF_TIMELENGTH;
            (start, start.wrapping_add(1))
        }
        InputTime::Range { start, stop } => {
            flags |= DMO_INPUT_DATA_BUFFERF_TIME | DMO_INPUT_DATA_BUFFERF_TIMELENGTH;
            (start, stop)
        }
    };
    InputBuffer {
        flags,
        start,
        length: stop.wrapping_sub(start),
    }
}
