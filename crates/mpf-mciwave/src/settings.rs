//! WAVE_mciSet: the time format, the wave device ids and the default format.

use crate::control::stop;
use crate::device::{Env, WaveDevice};
use crate::{
    MCIERR_BAD_INTEGER, MCIERR_BAD_TIME_FORMAT, MCIERR_NONAPPLICABLE_FUNCTION, MCIERR_OUTOFRANGE,
    MCIERR_UNSUPPORTED_FUNCTION, MCI_FORMAT_BYTES, MCI_FORMAT_MILLISECONDS, MCI_FORMAT_SAMPLES,
    MCI_SET_AUDIO, MCI_SET_DOOR_CLOSED, MCI_SET_DOOR_OPEN, MCI_SET_OFF, MCI_SET_ON,
    MCI_SET_TIME_FORMAT, MCI_SET_VIDEO, MCI_WAIT, MCI_WAVE_INPUT, MCI_WAVE_OUTPUT,
    MCI_WAVE_SET_ANYINPUT, MCI_WAVE_SET_ANYOUTPUT, MCI_WAVE_SET_AVGBYTESPERSEC,
    MCI_WAVE_SET_BITSPERSAMPLE, MCI_WAVE_SET_BLOCKALIGN, MCI_WAVE_SET_CHANNELS,
    MCI_WAVE_SET_FORMATTAG, MCI_WAVE_SET_SAMPLESPERSEC, WAVE_FORMAT_PCM, WAVE_MAPPER,
};

/// The format-changing MCI_WAVE_SET_* flags.
const FORMAT_FLAGS: u32 = MCI_WAVE_SET_FORMATTAG
    | MCI_WAVE_SET_CHANNELS
    | MCI_WAVE_SET_SAMPLESPERSEC
    | MCI_WAVE_SET_AVGBYTESPERSEC
    | MCI_WAVE_SET_BLOCKALIGN
    | MCI_WAVE_SET_BITSPERSAMPLE;

/// The MCI_WAVE_SET_PARMS fields.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetParams {
    pub time_format: u32,
    pub input: u16,
    pub output: u16,
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

/// WAVE_mciSet with MCI_WAIT and a parameter block.
pub fn set(dev: &mut WaveDevice, env: &Env, flags: u32, params: &SetParams) -> u32 {
    if (flags & MCI_SET_TIME_FORMAT) != 0 {
        match params.time_format {
            MCI_FORMAT_MILLISECONDS | MCI_FORMAT_BYTES | MCI_FORMAT_SAMPLES => {
                dev.time_format = params.time_format;
            }
            _ => return MCIERR_BAD_TIME_FORMAT,
        }
    }
    if (flags & (MCI_SET_VIDEO | MCI_SET_DOOR_OPEN | MCI_SET_DOOR_CLOSED)) != 0 {
        return MCIERR_UNSUPPORTED_FUNCTION;
    }
    // The audio switch is accepted and does nothing, as in Wine.
    if (flags & MCI_SET_AUDIO) != 0 && (flags & (MCI_SET_ON | MCI_SET_OFF)) == 0 {
        return MCIERR_BAD_INTEGER;
    }

    if (flags & MCI_WAVE_INPUT) != 0 {
        if u32::from(params.input) >= env.wave_in_devs {
            return MCIERR_OUTOFRANGE;
        }
        if dev.input != params.input {
            stop(dev, env, MCI_WAIT, false);
        }
        dev.input = params.input;
    }
    if (flags & MCI_WAVE_OUTPUT) != 0 {
        if u32::from(params.output) >= env.wave_out_devs {
            return MCIERR_OUTOFRANGE;
        }
        if dev.output != params.output {
            stop(dev, env, MCI_WAIT, false);
        }
        dev.output = params.output;
    }
    if (flags & MCI_WAVE_SET_ANYINPUT) != 0 {
        if dev.input != params.input {
            stop(dev, env, MCI_WAIT, false);
        }
        dev.input = WAVE_MAPPER;
    }
    if (flags & MCI_WAVE_SET_ANYOUTPUT) != 0 {
        if dev.output != params.output {
            stop(dev, env, MCI_WAIT, false);
        }
        dev.output = WAVE_MAPPER;
    }

    if (flags & FORMAT_FLAGS) != 0 {
        if !dev.format_is_default {
            return MCIERR_NONAPPLICABLE_FUNCTION;
        }
        // Wine checks the tag but never stores it, so the format stays as it was.
        if (flags & MCI_WAVE_SET_FORMATTAG) != 0 && params.format_tag != WAVE_FORMAT_PCM {
            return MCIERR_OUTOFRANGE;
        }
    }
    if (flags & MCI_WAVE_SET_AVGBYTESPERSEC) != 0 {
        dev.format.avg_bytes_per_sec = params.avg_bytes_per_sec;
    }
    if (flags & MCI_WAVE_SET_BITSPERSAMPLE) != 0 {
        dev.format.bits_per_sample = params.bits_per_sample;
    }
    if (flags & MCI_WAVE_SET_BLOCKALIGN) != 0 {
        dev.format.block_align = params.block_align;
    }
    if (flags & MCI_WAVE_SET_CHANNELS) != 0 {
        dev.format.channels = params.channels;
    }
    if (flags & MCI_WAVE_SET_SAMPLESPERSEC) != 0 {
        dev.format.samples_per_sec = params.samples_per_sec;
    }

    dev.notify_success(flags, true);
    0
}
