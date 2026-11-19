//! `MCICDA_Seek` and `MCICDA_SkipDataTracks`: the seek position, and the step over data tracks.

use crate::device::{toc_or_error, CdRom};
use crate::error::*;
use crate::frame::Msf;
use crate::mci::*;
use crate::time::calc_frame;
use crate::Driver;

/// `MCI_SEEK_PARMS`: the position for `MCI_TO`.
pub struct SeekParms {
    pub to: u32,
}

/// `MCICDA_Seek`.
pub fn seek(drv: &Driver, dev: &mut dyn CdRom, flags: u32, parms: Option<&SeekParms>) -> u32 {
    if !drv.is_open() {
        return MCIERR_INVALID_DEVICE_ID;
    }
    let Some(parms) = parms else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    let position = flags & (MCI_SEEK_TO_START | MCI_SEEK_TO_END | MCI_TO);
    if position == 0 {
        return MCIERR_MISSING_PARAMETER;
    }
    if (position & (position - 1)) != 0 {
        return MCIERR_FLAGS_NOT_COMPATIBLE;
    }
    dev.stop_audio();
    let toc = match toc_or_error(dev) {
        Ok(toc) => toc,
        Err(code) => return code,
    };
    let mut at = match position {
        MCI_SEEK_TO_START => toc.first_frame(),
        MCI_SEEK_TO_END => toc.lead_out_frame().wrapping_sub(1),
        _ => calc_frame(dev, drv.time_format, parms.to),
    };
    let ret = skip_data_tracks(dev, &mut at);
    if ret != 0 {
        return ret;
    }
    if dev.seek_audio(Msf::from_frames(at)) {
        0
    } else {
        MCIERR_HARDWARE
    }
}

/// `MCICDA_SkipDataTracks`: moves `frame` on to the next audio track when it lies on a data
/// track. `frame` is left alone when the call fails.
pub(crate) fn skip_data_tracks(dev: &mut dyn CdRom, frame: &mut u32) -> u32 {
    let toc = match toc_or_error(dev) {
        Ok(toc) => toc,
        Err(code) => return code,
    };
    match toc.skip_data_tracks(*frame) {
        Some(next) => {
            *frame = next;
            0
        }
        None => MCIERR_OUTOFRANGE,
    }
}
