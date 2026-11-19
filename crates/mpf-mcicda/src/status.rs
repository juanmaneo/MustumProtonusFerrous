//! `MCICDA_Status`: the status items, and the play mode that `MCICDA_GetStatus` reads from the
//! Q channel.

use crate::device::{q_or_error, toc_or_error, CdRom};
use crate::error::*;
use crate::mci::*;
use crate::time::calc_time;
use crate::Driver;

/// `MCI_STATUS_PARMS`: the item and track going in, the value coming out.
pub struct StatusParms {
    pub item: u32,
    pub track: u32,
    pub value: u32,
}

/// The item that `mcicda.c` also accepts as `MCI_CDA_STATUS_TYPE_TRACK`, marked as a bogus FullCD
/// value in the C source. Decimal, as in the C `case` label.
const BOGUS_FULLCD_TYPE_TRACK: u32 = 4001;

const AUDIO_STATUS_IN_PROGRESS: u8 = 0x11;
const AUDIO_STATUS_PAUSED: u8 = 0x12;
const AUDIO_STATUS_PLAY_COMPLETE: u8 = 0x13;
const AUDIO_STATUS_NO_STATUS: u8 = 0x15;

/// `MCICDA_Status`.
pub fn status(
    drv: &Driver,
    dev: &mut dyn CdRom,
    flags: u32,
    parms: Option<&mut StatusParms>,
) -> u32 {
    let Some(parms) = parms else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    if !drv.is_open() {
        return MCIERR_INVALID_DEVICE_ID;
    }
    if (flags & MCI_STATUS_ITEM) == 0 {
        return MCIERR_MISSING_PARAMETER;
    }
    match status_item(drv, dev, flags, parms) {
        Ok((value, ret)) => {
            parms.value = value;
            ret
        }
        Err(ret) => ret,
    }
}

/// The item switch of `MCICDA_Status`. `Ok((value, ret))` is the value to store and the return
/// code. `Err` returns before any value is stored, as the C code does.
fn status_item(
    drv: &Driver,
    dev: &mut dyn CdRom,
    flags: u32,
    parms: &StatusParms,
) -> Result<(u32, u32), u32> {
    match parms.item {
        MCI_STATUS_CURRENT_TRACK => {
            let q = q_or_error(dev)?;
            Ok((u32::from(q.track), 0))
        }
        MCI_STATUS_LENGTH => {
            let toc = toc_or_error(dev)?;
            let frames = if (flags & MCI_TRACK) != 0 {
                if !toc.contains_track(parms.track) {
                    return Err(MCIERR_OUTOFRANGE);
                }
                let length = toc
                    .frame_of(parms.track + 1)
                    .wrapping_sub(toc.frame_of(parms.track));
                if parms.track == u32::from(toc.last_track) {
                    length.wrapping_sub(1)
                } else {
                    length
                }
            } else {
                toc.lead_out_frame()
                    .wrapping_sub(toc.first_frame())
                    .wrapping_sub(1)
            };
            let tf = if drv.time_format == MCI_FORMAT_TMSF {
                MCI_FORMAT_MSF
            } else {
                drv.time_format
            };
            Ok(calc_time(dev, tf, frames))
        }
        MCI_STATUS_MODE => {
            let mode = current_mode(dev);
            Ok((make_resource(mode, mode), MCI_RESOURCE_RETURNED))
        }
        MCI_STATUS_MEDIA_PRESENT => Ok((
            boolean_resource(current_mode(dev) != MCI_MODE_OPEN),
            MCI_RESOURCE_RETURNED,
        )),
        MCI_STATUS_NUMBER_OF_TRACKS => {
            let toc = toc_or_error(dev)?;
            Ok((toc.track_count() as u32, 0))
        }
        MCI_STATUS_POSITION => {
            let frame = match flags & (MCI_STATUS_START | MCI_TRACK) {
                MCI_STATUS_START => toc_or_error(dev)?.first_frame(),
                MCI_TRACK => {
                    let toc = toc_or_error(dev)?;
                    if !toc.contains_track(parms.track) {
                        return Err(MCIERR_OUTOFRANGE);
                    }
                    toc.frame_of(parms.track)
                }
                0 => q_or_error(dev)?.absolute.frames(),
                _ => return Err(MCIERR_FLAGS_NOT_COMPATIBLE),
            };
            Ok(calc_time(dev, drv.time_format, frame))
        }
        MCI_STATUS_READY => {
            let ready = !matches!(current_mode(dev), MCI_MODE_NOT_READY | MCI_MODE_OPEN);
            Ok((boolean_resource(ready), MCI_RESOURCE_RETURNED))
        }
        MCI_STATUS_TIME_FORMAT => {
            let tf = drv.time_format;
            Ok((
                make_resource(tf, MCI_FORMAT_RETURN_BASE.wrapping_add(tf)),
                MCI_RESOURCE_RETURNED,
            ))
        }
        BOGUS_FULLCD_TYPE_TRACK | MCI_CDA_STATUS_TYPE_TRACK => {
            if (flags & MCI_TRACK) == 0 {
                return Err(MCIERR_MISSING_PARAMETER);
            }
            let toc = toc_or_error(dev)?;
            if !toc.contains_track(parms.track) {
                return Err(MCIERR_OUTOFRANGE);
            }
            let value = if toc.track_data(parms.track).is_data_track() {
                MCI_CDA_TRACK_OTHER
            } else {
                MCI_CDA_TRACK_AUDIO
            };
            Ok((value, 0))
        }
        _ => Err(MCIERR_UNSUPPORTED_FUNCTION),
    }
}

/// The mode that `MCICDA_GetStatus` reports when no playback thread runs. It reads the Q channel:
/// a not-ready error from the drive means the tray is open.
fn current_mode(dev: &mut dyn CdRom) -> u32 {
    match dev.read_q_channel() {
        Ok(q) => match q.audio_status {
            AUDIO_STATUS_IN_PROGRESS => MCI_MODE_PLAY,
            AUDIO_STATUS_PAUSED => MCI_MODE_PAUSE,
            AUDIO_STATUS_PLAY_COMPLETE | AUDIO_STATUS_NO_STATUS => MCI_MODE_STOP,
            _ => MCI_MODE_NOT_READY,
        },
        Err(ERROR_NOT_READY) => MCI_MODE_OPEN,
        Err(_) => MCI_MODE_NOT_READY,
    }
}
