//! WAVE_mciPlay and WAVE_mciRecord up to the buffer loop, which the caller drives.

use crate::control;
use crate::device::{Env, WaveDevice};
use crate::time::{align_up, round_down, time_to_byte};
use crate::{
    MCIERR_DEVICE_OPEN, MCIERR_FILE_NOT_FOUND, MCIERR_INTERNAL, MCIERR_OUTOFRANGE, MCI_FROM,
    MCI_MODE_PAUSE, MCI_MODE_PLAY, MCI_MODE_RECORD, MCI_MODE_STOP, MCI_TO, MCI_WAIT,
};

/// The outcome of play or record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Playback {
    /// The command finished. `seek` is the byte offset the C code seeks to, if it got that far.
    Done { ret: u32, seek: Option<u32> },
    /// The device opened. The caller moves data from `seek` up to `end`, in
    /// buffers of `buffer` bytes, then calls [WaveDevice::end_playback] with
    /// the command flags and a return code of 0.
    Stream { seek: u32, end: u32, buffer: u32 },
}

fn done(ret: u32) -> Playback {
    Playback::Done { ret, seek: None }
}

/// WAVE_mciPlay with MCI_WAIT. `from` and `to` are read only with their flags.
pub fn play(
    dev: &mut WaveDevice,
    env: &Env,
    flags: u32,
    from: u32,
    to: u32,
    callback: u32,
) -> Playback {
    if !dev.has_file {
        return done(MCIERR_FILE_NOT_FOUND);
    }
    if dev.status == MCI_MODE_PAUSE && !dev.f_input && (flags & (MCI_FROM | MCI_TO)) == 0 {
        return done(control::resume(dev, env, flags, true));
    }
    if dev.status != MCI_MODE_STOP {
        let ret = control::stop(dev, env, MCI_WAIT, false);
        if ret != 0 {
            return done(ret);
        }
    }

    dev.format.normalize_pcm();
    let mut end = dev.cksize;
    if (flags & MCI_TO) != 0 {
        let position = time_to_byte(dev, to);
        if position > end {
            return done(MCIERR_OUTOFRANGE);
        }
        end = position;
    }
    if (flags & MCI_FROM) != 0 {
        let position = time_to_byte(dev, from);
        if position > end {
            return done(MCIERR_OUTOFRANGE);
        }
        let Some(position) = round_down(position, dev.format.block_align) else {
            return done(MCIERR_INTERNAL);
        };
        dev.position = position;
    }
    if end < dev.position {
        return done(MCIERR_OUTOFRANGE);
    }
    if end == dev.position {
        return done(0);
    }

    dev.f_input = false;
    dev.status = MCI_MODE_PLAY;
    dev.swap_callback(flags, callback);
    let seek = dev.position;
    if !env.open_ok {
        return Playback::Done {
            ret: dev.end_playback(flags, MCIERR_DEVICE_OPEN),
            seek: Some(seek),
        };
    }
    let Some(buffer) = align_up(dev.format.avg_bytes_per_sec / 3, dev.format.block_align) else {
        return Playback::Done {
            ret: dev.end_playback(flags, MCIERR_INTERNAL),
            seek: Some(seek),
        };
    };
    Playback::Stream { seek, end, buffer }
}

/// WAVE_mciRecord with MCI_WAIT. `from` and `to` are read only with their flags.
pub fn record(
    dev: &mut WaveDevice,
    env: &Env,
    flags: u32,
    from: u32,
    to: u32,
    callback: u32,
) -> Playback {
    if dev.status == MCI_MODE_PAUSE && dev.f_input {
        return done(control::resume(dev, env, flags, true));
    }
    if dev.status != MCI_MODE_STOP {
        return done(MCIERR_INTERNAL);
    }

    dev.f_input = true;
    dev.status = MCI_MODE_RECORD;
    dev.has_file = true;
    dev.file_name = None;
    dev.riff_skeleton();

    let end = if (flags & MCI_TO) != 0 {
        time_to_byte(dev, to)
    } else {
        0xFFFF_FFFF
    };
    if (flags & MCI_FROM) != 0 {
        let position = time_to_byte(dev, from);
        if dev.cksize < position {
            return done(MCIERR_OUTOFRANGE);
        }
        let Some(position) = round_down(position, dev.format.block_align) else {
            return done(MCIERR_INTERNAL);
        };
        dev.position = position;
    }
    if end == dev.position {
        return done(0);
    }

    let Some(cksize) = align_up(dev.cksize, dev.format.block_align) else {
        return done(MCIERR_INTERNAL);
    };
    dev.swap_callback(flags, callback);
    dev.cksize = cksize;
    let seek = dev.position;
    if !env.open_ok {
        return Playback::Done {
            ret: dev.end_playback(flags, MCIERR_DEVICE_OPEN),
            seek: Some(seek),
        };
    }
    let buffer = align_up(dev.format.avg_bytes_per_sec / 3, dev.format.block_align)
        .expect("the block align was checked above");
    Playback::Stream { seek, end, buffer }
}
