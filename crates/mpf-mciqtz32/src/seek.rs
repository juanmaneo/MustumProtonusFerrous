//! The ranges and checks of `MCIQTZ_mciPlay` and `MCIQTZ_mciSeek`.

use crate::abi::{
    failed, HResult, AM_SEEKING_ABSOLUTE_POSITIONING, AM_SEEKING_NO_POSITIONING, MCIERR_INTERNAL,
    MCIERR_MISSING_PARAMETER, MCI_FROM, MCI_SEEK_TO_END, MCI_SEEK_TO_START, MCI_TO,
};

/// The arguments of `IMediaSeeking::SetPositions` that `MCI_PLAY` passes. The stop value comes
/// from the duration when `MCI_TO` is not set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayRange {
    pub start: i64,
    pub start_flags: u32,
    pub stop: i64,
}

/// The play range. `from` and `to` are scaled from milliseconds to 100 ns units when the device
/// uses media time.
pub fn play_range(flags: u32, from: u32, to: u32, media_time: bool, duration: i64) -> PlayRange {
    let (start, start_flags) = if flags & MCI_FROM != 0 {
        (scaled(from, media_time), AM_SEEKING_ABSOLUTE_POSITIONING)
    } else {
        (0, AM_SEEKING_NO_POSITIONING)
    };
    let stop = if flags & MCI_TO != 0 {
        scaled(to, media_time)
    } else {
        duration
    };
    PlayRange {
        start,
        start_flags,
        stop,
    }
}

fn scaled(value: u32, media_time: bool) -> i64 {
    if media_time {
        i64::from(value) * 10000
    } else {
        i64::from(value)
    }
}

/// The position for `MCI_SEEK`. Only `MCI_SEEK_TO_START` is supported. `MCI_SEEK_TO_END` and
/// `MCI_TO` return [MCIERR_INTERNAL], and no flag returns [MCIERR_MISSING_PARAMETER].
pub fn seek_target(flags: u32) -> Result<i64, u32> {
    if flags & MCI_SEEK_TO_START != 0 {
        Ok(0)
    } else if flags & (MCI_SEEK_TO_END | MCI_TO) != 0 {
        Err(MCIERR_INTERNAL)
    } else {
        Err(MCIERR_MISSING_PARAMETER)
    }
}

/// The reply of `MCI_PLAY`. A failed `Run` or a thread that could not be started returns
/// [MCIERR_INTERNAL]. `thread_ok` is true when the thread exists after the call.
pub fn play_reply(run_hr: HResult, thread_ok: bool) -> u32 {
    if failed(run_hr) || !thread_ok {
        MCIERR_INTERNAL
    } else {
        0
    }
}
