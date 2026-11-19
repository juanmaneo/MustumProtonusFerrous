//! Headless subset of Wine's dlls/mciwave/mciwave.c, the MCI waveform-audio device.
//!
//! The ported functions compute what Wine's C bodies compute from their
//! arguments. The winmm device, the RIFF file, the playback and record threads
//! and the notification delivery are not modelled. [device::Env] stands in for
//! the device return codes, and [device::WaveDevice::sent] records the
//! notifications the C code would send.
//!
//! Ported:
//! - WAVE_ConvertByteToTimeFormat and WAVE_ConvertTimeFormatToByte: [time::byte_to_time], [time::time_to_byte].
//! - MulDiv from dlls/kernelbase/main.c: [time::mul_div].
//! - WAVE_ALIGN_ON_BLOCK and the round-down in play, record and seek: [time::align_up], [time::round_down].
//! - WAVE_mciDefaultFmt: [device::WaveDevice::set_default_format].
//! - PCM normalisation in WAVE_mciPlay and WAVE_mciCreateRIFFSkeleton: [format::WaveFormat::normalize_pcm].
//! - WAVE_mciCreateRIFFSkeleton, state part: [device::WaveDevice::riff_skeleton] and [format::WaveFormat::chunk_len].
//! - WAVE_mciNotify, the callback swap and the playback end: [device::WaveDevice].
//! - WAVE_mciPlay and WAVE_mciRecord: [playback::play], [playback::record].
//! - WAVE_mciPause, WAVE_mciResume, WAVE_mciStop, WAVE_mciCue and WAVE_mciSeek: [control].
//! - WAVE_mciSet: [settings::set].
//! - WAVE_mciStatus, WAVE_mciGetDevCaps and WAVE_mciInfo: [query].
//!
//! Skipped:
//! - WAVE_mciOpen, WAVE_mciOpenFile, WAVE_mciReadFmt, WAVE_mciClose and WAVE_mciSave: file and driver handles.
//! - create_tmp_file: the temporary path. Record sets the file flag and clears the name.
//! - MCI_SendCommandAsync and the thread entry points: the port runs the synchronous (MCI_WAIT) body only.
//! - The per-buffer loops of play and record: the port returns [playback::Playback::Stream] for the caller to drive.
//! - The thread re-entry exception in play and record (status PLAY or RECORD with MCI_WAIT and no wave handle).
//! - The NULL parameter-block checks, because the port takes parameters by value.
//! - The lpstrReturn NULL check in info: the buffer is represented by its size only.
//! - The audio channel switch in set, which Wine also leaves as a no-op.
//! - WARN, ERR, FIXME and TRACE output.
//!
//! Divergence: C divides by nBlockAlign, which is zero for a non-PCM format with
//! a zero block align. The port returns MCIERR_INTERNAL there, where the C is
//! undefined behaviour. Those vectors are excluded from the oracle.

pub mod control;
pub mod device;
pub mod format;
pub mod playback;
pub mod query;
pub mod settings;
pub mod time;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// MCIERR_UNRECOGNIZED_KEYWORD.
pub const MCIERR_UNRECOGNIZED_KEYWORD: u32 = 259;
/// MCIERR_UNRECOGNIZED_COMMAND.
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261;
/// MCIERR_DEVICE_OPEN.
pub const MCIERR_DEVICE_OPEN: u32 = 265;
/// MCIERR_PARAM_OVERFLOW.
pub const MCIERR_PARAM_OVERFLOW: u32 = 268;
/// MCIERR_BAD_INTEGER.
pub const MCIERR_BAD_INTEGER: u32 = 270;
/// MCIERR_MISSING_PARAMETER.
pub const MCIERR_MISSING_PARAMETER: u32 = 273;
/// MCIERR_UNSUPPORTED_FUNCTION.
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274;
/// MCIERR_FILE_NOT_FOUND.
pub const MCIERR_FILE_NOT_FOUND: u32 = 275;
/// MCIERR_INTERNAL.
pub const MCIERR_INTERNAL: u32 = 277;
/// MCIERR_OUTOFRANGE.
pub const MCIERR_OUTOFRANGE: u32 = 282;
/// MCIERR_FLAGS_NOT_COMPATIBLE.
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284;
/// MCIERR_BAD_TIME_FORMAT.
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293;
/// MCIERR_NONAPPLICABLE_FUNCTION.
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 302;

/// MCI_MODE_NOT_READY.
pub const MCI_MODE_NOT_READY: u32 = 524;
/// MCI_MODE_STOP.
pub const MCI_MODE_STOP: u32 = 525;
/// MCI_MODE_PLAY.
pub const MCI_MODE_PLAY: u32 = 526;
/// MCI_MODE_RECORD.
pub const MCI_MODE_RECORD: u32 = 527;
/// MCI_MODE_PAUSE.
pub const MCI_MODE_PAUSE: u32 = 529;
/// MCI_FALSE.
pub const MCI_FALSE: u32 = 531;
/// MCI_TRUE.
pub const MCI_TRUE: u32 = 532;
/// MCI_DEVTYPE_WAVEFORM_AUDIO.
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: u32 = 522;
/// MCI_FORMAT_RETURN_BASE, the string id of the time-format names.
pub const MCI_FORMAT_RETURN_BASE: u32 = 533;
/// MCI_RESOURCE_RETURNED.
pub const MCI_RESOURCE_RETURNED: u32 = 0x0001_0000;

/// MCI_FORMAT_MILLISECONDS.
pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
/// MCI_FORMAT_BYTES.
pub const MCI_FORMAT_BYTES: u32 = 8;
/// MCI_FORMAT_SAMPLES.
pub const MCI_FORMAT_SAMPLES: u32 = 9;

/// MCI_NOTIFY.
pub const MCI_NOTIFY: u32 = 0x0000_0001;
/// MCI_WAIT.
pub const MCI_WAIT: u32 = 0x0000_0002;
/// MCI_FROM.
pub const MCI_FROM: u32 = 0x0000_0004;
/// MCI_TO.
pub const MCI_TO: u32 = 0x0000_0008;

/// MCI_NOTIFY_SUCCESSFUL.
pub const MCI_NOTIFY_SUCCESSFUL: u32 = 0x0001;
/// MCI_NOTIFY_SUPERSEDED.
pub const MCI_NOTIFY_SUPERSEDED: u32 = 0x0002;
/// MCI_NOTIFY_ABORTED.
pub const MCI_NOTIFY_ABORTED: u32 = 0x0004;
/// MCI_NOTIFY_FAILURE.
pub const MCI_NOTIFY_FAILURE: u32 = 0x0008;

/// WAVE_MAPPER, the device id that selects the default wave device.
pub const WAVE_MAPPER: u16 = 0xFFFF;
/// WAVE_FORMAT_PCM.
pub const WAVE_FORMAT_PCM: u16 = 1;
/// WAVE_FORMAT_PCM_S, the string id of the PCM format name.
pub const WAVE_FORMAT_PCM_S: u32 = 1152;
/// WAVE_MAPPER_S, the string id of the mapper device name.
pub const WAVE_MAPPER_S: u32 = 1153;

/// MCI_SET_DOOR_OPEN.
pub const MCI_SET_DOOR_OPEN: u32 = 0x0000_0100;
/// MCI_SET_DOOR_CLOSED.
pub const MCI_SET_DOOR_CLOSED: u32 = 0x0000_0200;
/// MCI_SET_TIME_FORMAT.
pub const MCI_SET_TIME_FORMAT: u32 = 0x0000_0400;
/// MCI_SET_AUDIO.
pub const MCI_SET_AUDIO: u32 = 0x0000_0800;
/// MCI_SET_VIDEO.
pub const MCI_SET_VIDEO: u32 = 0x0000_1000;
/// MCI_SET_ON.
pub const MCI_SET_ON: u32 = 0x0000_2000;
/// MCI_SET_OFF.
pub const MCI_SET_OFF: u32 = 0x0000_4000;
/// MCI_WAVE_SET_FORMATTAG.
pub const MCI_WAVE_SET_FORMATTAG: u32 = 0x0001_0000;
/// MCI_WAVE_SET_CHANNELS.
pub const MCI_WAVE_SET_CHANNELS: u32 = 0x0002_0000;
/// MCI_WAVE_SET_SAMPLESPERSEC.
pub const MCI_WAVE_SET_SAMPLESPERSEC: u32 = 0x0004_0000;
/// MCI_WAVE_SET_AVGBYTESPERSEC.
pub const MCI_WAVE_SET_AVGBYTESPERSEC: u32 = 0x0008_0000;
/// MCI_WAVE_SET_BLOCKALIGN.
pub const MCI_WAVE_SET_BLOCKALIGN: u32 = 0x0010_0000;
/// MCI_WAVE_SET_BITSPERSAMPLE.
pub const MCI_WAVE_SET_BITSPERSAMPLE: u32 = 0x0020_0000;
/// MCI_WAVE_INPUT, also a status and info item.
pub const MCI_WAVE_INPUT: u32 = 0x0040_0000;
/// MCI_WAVE_OUTPUT, also a status and info item.
pub const MCI_WAVE_OUTPUT: u32 = 0x0080_0000;
/// MCI_WAVE_SET_ANYINPUT.
pub const MCI_WAVE_SET_ANYINPUT: u32 = 0x0400_0000;
/// MCI_WAVE_SET_ANYOUTPUT.
pub const MCI_WAVE_SET_ANYOUTPUT: u32 = 0x0800_0000;

/// MCI_SEEK_TO_START.
pub const MCI_SEEK_TO_START: u32 = 0x0000_0100;
/// MCI_SEEK_TO_END.
pub const MCI_SEEK_TO_END: u32 = 0x0000_0200;

/// MCI_STATUS_ITEM.
pub const MCI_STATUS_ITEM: u32 = 0x0000_0100;
/// MCI_STATUS_START.
pub const MCI_STATUS_START: u32 = 0x0000_0200;
/// MCI_STATUS_LENGTH.
pub const MCI_STATUS_LENGTH: u32 = 1;
/// MCI_STATUS_POSITION.
pub const MCI_STATUS_POSITION: u32 = 2;
/// MCI_STATUS_NUMBER_OF_TRACKS.
pub const MCI_STATUS_NUMBER_OF_TRACKS: u32 = 3;
/// MCI_STATUS_MODE.
pub const MCI_STATUS_MODE: u32 = 4;
/// MCI_STATUS_MEDIA_PRESENT.
pub const MCI_STATUS_MEDIA_PRESENT: u32 = 5;
/// MCI_STATUS_TIME_FORMAT.
pub const MCI_STATUS_TIME_FORMAT: u32 = 6;
/// MCI_STATUS_READY.
pub const MCI_STATUS_READY: u32 = 7;
/// MCI_STATUS_CURRENT_TRACK.
pub const MCI_STATUS_CURRENT_TRACK: u32 = 8;
/// MCI_WAVE_STATUS_FORMATTAG.
pub const MCI_WAVE_STATUS_FORMATTAG: u32 = 0x0000_4001;
/// MCI_WAVE_STATUS_CHANNELS.
pub const MCI_WAVE_STATUS_CHANNELS: u32 = 0x0000_4002;
/// MCI_WAVE_STATUS_SAMPLESPERSEC.
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: u32 = 0x0000_4003;
/// MCI_WAVE_STATUS_AVGBYTESPERSEC.
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: u32 = 0x0000_4004;
/// MCI_WAVE_STATUS_BLOCKALIGN.
pub const MCI_WAVE_STATUS_BLOCKALIGN: u32 = 0x0000_4005;
/// MCI_WAVE_STATUS_BITSPERSAMPLE.
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: u32 = 0x0000_4006;
/// MCI_WAVE_STATUS_LEVEL.
pub const MCI_WAVE_STATUS_LEVEL: u32 = 0x0000_4007;

/// MCI_GETDEVCAPS_ITEM.
pub const MCI_GETDEVCAPS_ITEM: u32 = 0x0000_0100;
/// MCI_GETDEVCAPS_CAN_RECORD.
pub const MCI_GETDEVCAPS_CAN_RECORD: u32 = 1;
/// MCI_GETDEVCAPS_HAS_AUDIO.
pub const MCI_GETDEVCAPS_HAS_AUDIO: u32 = 2;
/// MCI_GETDEVCAPS_HAS_VIDEO.
pub const MCI_GETDEVCAPS_HAS_VIDEO: u32 = 3;
/// MCI_GETDEVCAPS_DEVICE_TYPE.
pub const MCI_GETDEVCAPS_DEVICE_TYPE: u32 = 4;
/// MCI_GETDEVCAPS_USES_FILES.
pub const MCI_GETDEVCAPS_USES_FILES: u32 = 5;
/// MCI_GETDEVCAPS_COMPOUND_DEVICE.
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: u32 = 6;
/// MCI_GETDEVCAPS_CAN_EJECT.
pub const MCI_GETDEVCAPS_CAN_EJECT: u32 = 7;
/// MCI_GETDEVCAPS_CAN_PLAY.
pub const MCI_GETDEVCAPS_CAN_PLAY: u32 = 8;
/// MCI_GETDEVCAPS_CAN_SAVE.
pub const MCI_GETDEVCAPS_CAN_SAVE: u32 = 9;
/// MCI_WAVE_GETDEVCAPS_INPUTS.
pub const MCI_WAVE_GETDEVCAPS_INPUTS: u32 = 0x0000_4001;
/// MCI_WAVE_GETDEVCAPS_OUTPUTS.
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: u32 = 0x0000_4002;

/// MCI_INFO_PRODUCT.
pub const MCI_INFO_PRODUCT: u32 = 0x0000_0100;
/// MCI_INFO_FILE.
pub const MCI_INFO_FILE: u32 = 0x0000_0200;

/// MAKEMCIRESOURCE: the value in the low word, the string id in the high word.
pub fn make_resource(value: u32, string: u32) -> u32 {
    (value & 0xFFFF) | ((string & 0xFFFF) << 16)
}

/// The MCI boolean resource: TRUE with MCI_TRUE, or FALSE with MCI_FALSE.
pub fn bool_resource(value: bool) -> u32 {
    if value {
        make_resource(1, MCI_TRUE)
    } else {
        make_resource(0, MCI_FALSE)
    }
}
