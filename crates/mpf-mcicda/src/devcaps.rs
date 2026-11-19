//! `MCICDA_GetDevCaps`: the fixed capabilities of the audio CD device.

use crate::error::*;
use crate::mci::*;

/// `MCI_GETDEVCAPS_PARMS`: the item going in, the value coming out.
pub struct DevCapsParms {
    pub item: u32,
    pub value: u32,
}

/// `MCICDA_GetDevCaps`. The device-ID check is the caller's, as `Driver` is passed in directly.
pub fn get_dev_caps(flags: u32, parms: Option<&mut DevCapsParms>) -> u32 {
    let Some(parms) = parms else {
        return MCIERR_NULL_PARAMETER_BLOCK;
    };
    if (flags & MCI_GETDEVCAPS_ITEM) == 0 {
        return MCIERR_MISSING_PARAMETER;
    }
    parms.value = match parms.item {
        MCI_GETDEVCAPS_CAN_RECORD
        | MCI_GETDEVCAPS_HAS_VIDEO
        | MCI_GETDEVCAPS_USES_FILES
        | MCI_GETDEVCAPS_COMPOUND_DEVICE
        | MCI_GETDEVCAPS_CAN_SAVE => boolean_resource(false),
        MCI_GETDEVCAPS_HAS_AUDIO | MCI_GETDEVCAPS_CAN_EJECT | MCI_GETDEVCAPS_CAN_PLAY => {
            boolean_resource(true)
        }
        MCI_GETDEVCAPS_DEVICE_TYPE => make_resource(MCI_DEVTYPE_CD_AUDIO, MCI_DEVTYPE_CD_AUDIO),
        _ => return MCIERR_UNSUPPORTED_FUNCTION,
    };
    MCI_RESOURCE_RETURNED
}
