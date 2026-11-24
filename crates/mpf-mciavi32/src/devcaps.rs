//! `MCIAVI_mciGetDevCaps`.

use crate::abi::*;
use crate::{Device, Reply};

/// `item` is `None` when the parameter block is NULL, and `device` is `None` when the device id
/// is not open.
pub fn mci_get_dev_caps(device: Option<&Device>, flags: u32, item: Option<u32>) -> Reply {
    let Some(item) = item else {
        return Reply::code(MCIERR_NULL_PARAMETER_BLOCK);
    };
    if device.is_none() {
        return Reply::code(MCIERR_INVALID_DEVICE_ID);
    }
    if flags & MCI_GETDEVCAPS_ITEM == 0 {
        return Reply::code(MCIERR_MISSING_PARAMETER);
    }
    if flags & MCI_TEST != 0 {
        return Reply::code(0);
    }
    capability(item)
}

fn capability(item: u32) -> Reply {
    match item {
        MCI_GETDEVCAPS_DEVICE_TYPE => Reply::resource(make_resource(
            MCI_DEVTYPE_DIGITAL_VIDEO,
            MCI_DEVTYPE_DIGITAL_VIDEO,
        )),
        MCI_GETDEVCAPS_HAS_AUDIO
        | MCI_GETDEVCAPS_HAS_VIDEO
        | MCI_GETDEVCAPS_USES_FILES
        | MCI_GETDEVCAPS_COMPOUND_DEVICE
        | MCI_GETDEVCAPS_CAN_PLAY
        | MCI_DGV_GETDEVCAPS_CAN_TEST => Reply::resource(bool_resource(true)),
        MCI_GETDEVCAPS_CAN_EJECT
        | MCI_GETDEVCAPS_CAN_RECORD
        | MCI_GETDEVCAPS_CAN_SAVE
        | MCI_DGV_GETDEVCAPS_CAN_REVERSE
        | MCI_DGV_GETDEVCAPS_CAN_STRETCH
        | MCI_DGV_GETDEVCAPS_CAN_LOCK
        | MCI_DGV_GETDEVCAPS_CAN_FREEZE
        | MCI_DGV_GETDEVCAPS_CAN_STR_IN
        | MCI_DGV_GETDEVCAPS_HAS_STILL
        | MCI_DGV_GETDEVCAPS_PALETTES => Reply::resource(bool_resource(false)),
        _ => Reply::code(MCIERR_UNSUPPORTED_FUNCTION),
    }
}
