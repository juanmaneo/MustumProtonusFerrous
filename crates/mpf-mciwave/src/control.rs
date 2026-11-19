//! WAVE_mciPause, WAVE_mciResume, WAVE_mciStop, WAVE_mciCue and WAVE_mciSeek.

use crate::device::{Env, WaveDevice};
use crate::time::{round_down, time_to_byte};
use crate::{
    MCIERR_FLAGS_NOT_COMPATIBLE, MCIERR_INTERNAL, MCIERR_MISSING_PARAMETER,
    MCIERR_NONAPPLICABLE_FUNCTION, MCIERR_OUTOFRANGE, MCI_MODE_PAUSE, MCI_MODE_PLAY,
    MCI_MODE_RECORD, MCI_MODE_STOP, MCI_SEEK_TO_END, MCI_SEEK_TO_START, MCI_TO, MCI_WAIT,
};

/// Moves the device into `next` when the winmm call succeeded, and reports
/// MCIERR_INTERNAL with the state unchanged when it did not.
fn enter_mode(dev: &mut WaveDevice, env: &Env, next: u32) -> u32 {
    if env.control_ret == 0 {
        dev.status = next;
        0
    } else {
        MCIERR_INTERNAL
    }
}

/// WAVE_mciPause.
pub fn pause(dev: &mut WaveDevice, env: &Env, flags: u32, has_params: bool) -> u32 {
    let ret = match dev.status {
        MCI_MODE_PLAY | MCI_MODE_RECORD => enter_mode(dev, env, MCI_MODE_PAUSE),
        MCI_MODE_PAUSE => 0,
        _ => MCIERR_NONAPPLICABLE_FUNCTION,
    };
    dev.notify_success(flags, ret == 0 && has_params);
    ret
}

/// WAVE_mciResume. A paused record resumes recording, a paused play resumes playing.
pub fn resume(dev: &mut WaveDevice, env: &Env, flags: u32, has_params: bool) -> u32 {
    let ret = match dev.status {
        MCI_MODE_PAUSE => {
            let next = if dev.f_input {
                MCI_MODE_RECORD
            } else {
                MCI_MODE_PLAY
            };
            enter_mode(dev, env, next)
        }
        MCI_MODE_PLAY | MCI_MODE_RECORD => 0,
        _ => MCIERR_NONAPPLICABLE_FUNCTION,
    };
    dev.notify_success(flags, ret == 0 && has_params);
    ret
}

/// WAVE_mciStop. The pending callback is aborted when the device is not
/// stopped. Only a playing, recording or paused device can fail to stop.
pub fn stop(dev: &mut WaveDevice, env: &Env, flags: u32, has_params: bool) -> u32 {
    if dev.status != MCI_MODE_STOP {
        dev.abort_callback();
    }
    let ret = match dev.status {
        MCI_MODE_PAUSE | MCI_MODE_PLAY | MCI_MODE_RECORD => env.control_ret,
        _ => 0,
    };
    dev.status = MCI_MODE_STOP;
    dev.notify_success(flags, ret == 0 && has_params);
    ret
}

/// WAVE_mciCue: a no-op on a stopped device that only reports success.
pub fn cue(dev: &mut WaveDevice, flags: u32, has_params: bool) -> u32 {
    if dev.status != MCI_MODE_STOP {
        return MCIERR_NONAPPLICABLE_FUNCTION;
    }
    dev.notify_success(flags, has_params);
    0
}

/// WAVE_mciSeek. `to` is read only with MCI_TO. The position is rounded down to
/// a whole block unless it is the end of the data.
pub fn seek(dev: &mut WaveDevice, env: &Env, flags: u32, to: u32) -> u32 {
    let selector = flags & (MCI_SEEK_TO_START | MCI_SEEK_TO_END | MCI_TO);
    if selector == 0 {
        return MCIERR_MISSING_PARAMETER;
    }
    if (selector & (selector - 1)) != 0 {
        return MCIERR_FLAGS_NOT_COMPATIBLE;
    }
    let ret = stop(dev, env, MCI_WAIT, false);
    if ret != 0 {
        return ret;
    }

    let position = if (flags & MCI_TO) != 0 {
        let position = time_to_byte(dev, to);
        if position > dev.cksize {
            return MCIERR_OUTOFRANGE;
        }
        position
    } else if (flags & MCI_SEEK_TO_START) != 0 {
        0
    } else {
        dev.cksize
    };
    let position = if position == dev.cksize {
        position
    } else {
        let Some(rounded) = round_down(position, dev.format.block_align) else {
            return MCIERR_INTERNAL;
        };
        rounded
    };
    dev.position = position;
    dev.notify_success(flags, true);
    0
}
