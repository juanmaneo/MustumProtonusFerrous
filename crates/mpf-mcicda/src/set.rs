//! `MCICDA_Set` and `MCICDA_SetDoor`: the tray, and the time format.

use crate::device::CdRom;
use crate::error::*;
use crate::mci::*;
use crate::Driver;

/// `MCI_SET_PARMS`: the time format for `MCI_SET_TIME_FORMAT`.
pub struct SetParms {
    pub time_format: u32,
}

/// `MCICDA_Set`. The tray moves before the parameter block is checked, as in the C code, and a
/// failed tray move is not reported.
pub fn set(drv: &mut Driver, dev: &mut dyn CdRom, flags: u32, parms: Option<&SetParms>) -> u32 {
    if !drv.is_open() {
        return MCIERR_INVALID_DEVICE_ID;
    }
    if (flags & MCI_SET_DOOR_OPEN) != 0 {
        dev.set_door(true);
    }
    if (flags & MCI_SET_DOOR_CLOSED) != 0 {
        dev.set_door(false);
    }
    let Some(parms) = parms else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    if (flags & MCI_SET_TIME_FORMAT) != 0 {
        if !matches!(
            parms.time_format,
            MCI_FORMAT_MILLISECONDS | MCI_FORMAT_MSF | MCI_FORMAT_TMSF
        ) {
            return MCIERR_BAD_TIME_FORMAT;
        }
        drv.time_format = parms.time_format;
    }
    0
}
