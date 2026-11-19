//! WAVE_mciStatus, WAVE_mciGetDevCaps and WAVE_mciInfo.
//!
//! The info text is cut to the caller's buffer in UTF-16 units, as lstrcpynW
//! does. A surrogate pair split by the cut is returned as U+FFFD.

use crate::device::{Env, WaveDevice};
use crate::time::byte_to_time;
use crate::{
    bool_resource, make_resource, MCIERR_MISSING_PARAMETER, MCIERR_PARAM_OVERFLOW,
    MCIERR_UNRECOGNIZED_COMMAND, MCIERR_UNRECOGNIZED_KEYWORD, MCIERR_UNSUPPORTED_FUNCTION,
    MCI_DEVTYPE_WAVEFORM_AUDIO, MCI_FORMAT_RETURN_BASE, MCI_GETDEVCAPS_CAN_EJECT,
    MCI_GETDEVCAPS_CAN_PLAY, MCI_GETDEVCAPS_CAN_RECORD, MCI_GETDEVCAPS_CAN_SAVE,
    MCI_GETDEVCAPS_COMPOUND_DEVICE, MCI_GETDEVCAPS_DEVICE_TYPE, MCI_GETDEVCAPS_HAS_AUDIO,
    MCI_GETDEVCAPS_HAS_VIDEO, MCI_GETDEVCAPS_ITEM, MCI_GETDEVCAPS_USES_FILES, MCI_INFO_FILE,
    MCI_INFO_PRODUCT, MCI_MODE_NOT_READY, MCI_NOTIFY, MCI_RESOURCE_RETURNED,
    MCI_STATUS_CURRENT_TRACK, MCI_STATUS_ITEM, MCI_STATUS_LENGTH, MCI_STATUS_MEDIA_PRESENT,
    MCI_STATUS_MODE, MCI_STATUS_NUMBER_OF_TRACKS, MCI_STATUS_POSITION, MCI_STATUS_READY,
    MCI_STATUS_START, MCI_STATUS_TIME_FORMAT, MCI_WAIT, MCI_WAVE_GETDEVCAPS_INPUTS,
    MCI_WAVE_GETDEVCAPS_OUTPUTS, MCI_WAVE_INPUT, MCI_WAVE_OUTPUT, MCI_WAVE_STATUS_AVGBYTESPERSEC,
    MCI_WAVE_STATUS_BITSPERSAMPLE, MCI_WAVE_STATUS_BLOCKALIGN, MCI_WAVE_STATUS_CHANNELS,
    MCI_WAVE_STATUS_FORMATTAG, MCI_WAVE_STATUS_LEVEL, MCI_WAVE_STATUS_SAMPLESPERSEC,
    WAVE_FORMAT_PCM, WAVE_FORMAT_PCM_S, WAVE_MAPPER, WAVE_MAPPER_S,
};

/// The return value and the dwReturn field of an MCI query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reply {
    pub ret: u32,
    pub value: u32,
}

fn ok(value: u32) -> Reply {
    Reply { ret: 0, value }
}

fn resource(value: u32) -> Reply {
    Reply {
        ret: MCI_RESOURCE_RETURNED,
        value,
    }
}

fn fail(ret: u32) -> Reply {
    Reply { ret, value: 0 }
}

/// An input or output device id. The mapper is returned as a resource string.
fn wave_device(id: u16) -> Reply {
    if id == WAVE_MAPPER {
        resource(make_resource(u32::from(WAVE_MAPPER), WAVE_MAPPER_S))
    } else {
        ok(u32::from(id))
    }
}

/// WAVE_mciStatus.
pub fn status(dev: &mut WaveDevice, flags: u32, item: u32) -> Reply {
    if (flags & MCI_STATUS_ITEM) == 0 {
        return fail(MCIERR_MISSING_PARAMETER);
    }
    let reply = match item {
        MCI_STATUS_CURRENT_TRACK | MCI_STATUS_NUMBER_OF_TRACKS => ok(1),
        MCI_STATUS_LENGTH => {
            if dev.has_file {
                ok(byte_to_time(dev, dev.cksize))
            } else {
                fail(MCIERR_UNSUPPORTED_FUNCTION)
            }
        }
        MCI_STATUS_MODE => resource(make_resource(dev.status, dev.status)),
        MCI_STATUS_MEDIA_PRESENT => resource(bool_resource(true)),
        MCI_STATUS_POSITION => {
            if dev.has_file {
                let position = if (flags & MCI_STATUS_START) != 0 {
                    0
                } else {
                    dev.position
                };
                ok(byte_to_time(dev, position))
            } else {
                fail(MCIERR_UNSUPPORTED_FUNCTION)
            }
        }
        MCI_STATUS_READY => resource(bool_resource(dev.status != MCI_MODE_NOT_READY)),
        MCI_STATUS_TIME_FORMAT => resource(make_resource(
            dev.time_format,
            MCI_FORMAT_RETURN_BASE + dev.time_format,
        )),
        MCI_WAVE_INPUT => wave_device(dev.input),
        MCI_WAVE_OUTPUT => wave_device(dev.output),
        MCI_WAVE_STATUS_FORMATTAG => {
            if dev.format.is_pcm() {
                resource(make_resource(u32::from(WAVE_FORMAT_PCM), WAVE_FORMAT_PCM_S))
            } else {
                ok(u32::from(dev.format.format_tag))
            }
        }
        MCI_WAVE_STATUS_AVGBYTESPERSEC => ok(dev.format.avg_bytes_per_sec),
        MCI_WAVE_STATUS_BITSPERSAMPLE => ok(u32::from(dev.format.bits_per_sample)),
        MCI_WAVE_STATUS_BLOCKALIGN => ok(u32::from(dev.format.block_align)),
        MCI_WAVE_STATUS_CHANNELS => ok(u32::from(dev.format.channels)),
        MCI_WAVE_STATUS_SAMPLESPERSEC => ok(dev.format.samples_per_sec),
        MCI_WAVE_STATUS_LEVEL => ok(0xAAAA_5555),
        _ => fail(MCIERR_UNSUPPORTED_FUNCTION),
    };
    dev.notify_success(flags, (reply.ret & 0xFFFF) == 0);
    reply
}

/// WAVE_mciGetDevCaps.
pub fn dev_caps(dev: &mut WaveDevice, env: &Env, flags: u32, item: u32) -> Reply {
    if (flags & MCI_GETDEVCAPS_ITEM) == 0 {
        return fail(MCIERR_UNRECOGNIZED_COMMAND);
    }
    let reply = match item {
        MCI_GETDEVCAPS_DEVICE_TYPE => resource(make_resource(
            MCI_DEVTYPE_WAVEFORM_AUDIO,
            MCI_DEVTYPE_WAVEFORM_AUDIO,
        )),
        MCI_GETDEVCAPS_HAS_AUDIO
        | MCI_GETDEVCAPS_USES_FILES
        | MCI_GETDEVCAPS_COMPOUND_DEVICE
        | MCI_GETDEVCAPS_CAN_RECORD
        | MCI_GETDEVCAPS_CAN_PLAY
        | MCI_GETDEVCAPS_CAN_SAVE => resource(bool_resource(true)),
        MCI_GETDEVCAPS_HAS_VIDEO | MCI_GETDEVCAPS_CAN_EJECT => resource(bool_resource(false)),
        MCI_WAVE_GETDEVCAPS_INPUTS => ok(env.wave_in_devs),
        MCI_WAVE_GETDEVCAPS_OUTPUTS => ok(env.wave_out_devs),
        _ => fail(MCIERR_UNRECOGNIZED_COMMAND),
    };
    dev.notify_success(flags, (reply.ret & 0xFFFF) == 0);
    reply
}

/// WAVE_mciInfo: the return code and the text copied into a buffer of `retsize` units.
pub fn info(dev: &mut WaveDevice, flags: u32, retsize: u32) -> (u32, String) {
    let text = match flags & !(MCI_WAIT | MCI_NOTIFY) {
        MCI_INFO_PRODUCT => "Wine's audio player".to_string(),
        MCI_INFO_FILE => dev.file_name.clone().unwrap_or_default(),
        MCI_WAVE_INPUT => "Wine Wave In".to_string(),
        MCI_WAVE_OUTPUT => "Wine Wave Out".to_string(),
        _ => return (MCIERR_UNRECOGNIZED_KEYWORD, String::new()),
    };
    if retsize == 0 {
        return (MCIERR_PARAM_OVERFLOW, String::new());
    }
    dev.notify_success(flags, true);
    let units: Vec<u16> = text.encode_utf16().take(retsize as usize - 1).collect();
    (0, String::from_utf16_lossy(&units))
}
