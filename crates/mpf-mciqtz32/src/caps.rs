//! The capabilities that `MCIQTZ_mciGetDevCaps` reports.

use crate::abi::{
    make_mci_resource, MCIERR_MISSING_PARAMETER, MCIERR_UNSUPPORTED_FUNCTION,
    MCI_DEVTYPE_DIGITAL_VIDEO, MCI_DGV_GETDEVCAPS_CAN_FREEZE, MCI_DGV_GETDEVCAPS_CAN_LOCK,
    MCI_DGV_GETDEVCAPS_CAN_REVERSE, MCI_DGV_GETDEVCAPS_CAN_STRETCH, MCI_DGV_GETDEVCAPS_CAN_STR_IN,
    MCI_DGV_GETDEVCAPS_CAN_TEST, MCI_DGV_GETDEVCAPS_HAS_STILL, MCI_DGV_GETDEVCAPS_MAX_WINDOWS,
    MCI_FALSE, MCI_GETDEVCAPS_CAN_EJECT, MCI_GETDEVCAPS_CAN_PLAY, MCI_GETDEVCAPS_CAN_RECORD,
    MCI_GETDEVCAPS_CAN_SAVE, MCI_GETDEVCAPS_COMPOUND_DEVICE, MCI_GETDEVCAPS_DEVICE_TYPE,
    MCI_GETDEVCAPS_HAS_AUDIO, MCI_GETDEVCAPS_HAS_VIDEO, MCI_GETDEVCAPS_ITEM,
    MCI_GETDEVCAPS_USES_FILES, MCI_RESOURCE_RETURNED, MCI_TRUE,
};

/// The `dwReturn` of `MCI_GETDEVCAPS` and the return value that goes with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    pub value: u32,
    pub reply: u32,
}

/// `MCI_GETDEVCAPS` needs `MCI_GETDEVCAPS_ITEM`. Unknown items and the rate items are
/// [MCIERR_UNSUPPORTED_FUNCTION].
pub fn devcap(flags: u32, item: u32) -> Result<Capability, u32> {
    if flags & MCI_GETDEVCAPS_ITEM == 0 {
        return Err(MCIERR_MISSING_PARAMETER);
    }
    match item {
        MCI_GETDEVCAPS_CAN_RECORD
        | MCI_GETDEVCAPS_CAN_EJECT
        | MCI_GETDEVCAPS_CAN_SAVE
        | MCI_DGV_GETDEVCAPS_CAN_REVERSE
        | MCI_DGV_GETDEVCAPS_CAN_STRETCH
        | MCI_DGV_GETDEVCAPS_CAN_LOCK
        | MCI_DGV_GETDEVCAPS_CAN_FREEZE
        | MCI_DGV_GETDEVCAPS_CAN_STR_IN
        | MCI_DGV_GETDEVCAPS_HAS_STILL
        | MCI_DGV_GETDEVCAPS_CAN_TEST => Ok(yes_no(false)),
        MCI_GETDEVCAPS_HAS_AUDIO
        | MCI_GETDEVCAPS_HAS_VIDEO
        | MCI_GETDEVCAPS_USES_FILES
        | MCI_GETDEVCAPS_COMPOUND_DEVICE
        | MCI_GETDEVCAPS_CAN_PLAY => Ok(yes_no(true)),
        MCI_GETDEVCAPS_DEVICE_TYPE => Ok(Capability {
            value: make_mci_resource(MCI_DEVTYPE_DIGITAL_VIDEO, MCI_DEVTYPE_DIGITAL_VIDEO),
            reply: MCI_RESOURCE_RETURNED,
        }),
        MCI_DGV_GETDEVCAPS_MAX_WINDOWS => Ok(Capability { value: 1, reply: 0 }),
        _ => Err(MCIERR_UNSUPPORTED_FUNCTION),
    }
}

fn yes_no(answer: bool) -> Capability {
    let value = if answer {
        make_mci_resource(1, MCI_TRUE)
    } else {
        make_mci_resource(0, MCI_FALSE)
    };
    Capability {
        value,
        reply: MCI_RESOURCE_RETURNED,
    }
}
