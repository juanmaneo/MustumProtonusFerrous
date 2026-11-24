//! `MCIAVI_mciSet`.

use crate::abi::*;
use crate::Device;

/// The fields of `MCI_DGV_SET_PARMS` that the command reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetParams {
    /// `dwTimeFormat`, read when `MCI_SET_TIME_FORMAT` is set.
    pub time_format: u32,
    /// `dwAudio`, read when `MCI_SET_AUDIO` is set.
    pub audio: u32,
}

/// `params` is `None` when the parameter block is NULL, and `device` is `None` when the device id
/// is not open. The return value is the MCI return code.
pub fn mci_set(device: Option<&mut Device>, flags: u32, params: Option<SetParams>) -> u32 {
    let Some(params) = params else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    let Some(device) = device else {
        return MCIERR_INVALID_DEVICE_ID;
    };
    if flags & MCI_TEST != 0 {
        return 0;
    }

    if flags & MCI_SET_TIME_FORMAT != 0 {
        match params.time_format {
            MCI_FORMAT_MILLISECONDS | MCI_FORMAT_FRAMES => device.time_format = params.time_format,
            _ => return MCIERR_BAD_TIME_FORMAT,
        }
    }
    if flags & (MCI_SET_DOOR_OPEN | MCI_SET_DOOR_CLOSED) != 0 {
        return MCIERR_UNSUPPORTED_FUNCTION;
    }

    if flags & MCI_SET_ON != 0 {
        if flags & MCI_SET_VIDEO != 0 {
            device.set |= SET_VIDEO_BIT;
        }
        if flags & MCI_SET_AUDIO != 0 {
            match params.audio {
                MCI_SET_AUDIO_ALL => device.set |= SET_AUDIO_BITS,
                MCI_SET_AUDIO_LEFT => device.set |= SET_AUDIO_LEFT_BIT,
                MCI_SET_AUDIO_RIGHT => device.set |= SET_AUDIO_RIGHT_BIT,
                _ => {}
            }
        }
    }

    if flags & MCI_SET_OFF != 0 {
        if flags & MCI_SET_VIDEO != 0 {
            device.set &= !SET_VIDEO_BIT;
        }
        if flags & MCI_SET_AUDIO != 0 {
            match params.audio {
                MCI_SET_AUDIO_ALL => device.set &= !SET_AUDIO_BITS,
                // Wine clears the right channel bit for `MCI_SET_AUDIO_LEFT` too. The state
                // matches Wine, so the bug is kept.
                MCI_SET_AUDIO_LEFT | MCI_SET_AUDIO_RIGHT => device.set &= !SET_AUDIO_RIGHT_BIT,
                _ => {}
            }
        }
    }

    0
}
