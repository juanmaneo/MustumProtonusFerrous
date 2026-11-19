//! MCI constants from `mmsystem.h` and `mmddk.h` that `mcicda.c` uses, and the `MAKEMCIRESOURCE`
//! encoding of its return values.

pub const MCI_TO: u32 = 0x0000_0008;
pub const MCI_TRACK: u32 = 0x0000_0010;

pub const MCI_OPEN_SHAREABLE: u32 = 0x0000_0100;
pub const MCI_OPEN_ELEMENT: u32 = 0x0000_0200;
pub const MCI_OPEN_ELEMENT_ID: u32 = 0x0000_0800;

pub const MCI_SEEK_TO_START: u32 = 0x0000_0100;
pub const MCI_SEEK_TO_END: u32 = 0x0000_0200;

pub const MCI_STATUS_ITEM: u32 = 0x0000_0100;
pub const MCI_STATUS_START: u32 = 0x0000_0200;
pub const MCI_STATUS_LENGTH: u32 = 0x0000_0001;
pub const MCI_STATUS_POSITION: u32 = 0x0000_0002;
pub const MCI_STATUS_NUMBER_OF_TRACKS: u32 = 0x0000_0003;
pub const MCI_STATUS_MODE: u32 = 0x0000_0004;
pub const MCI_STATUS_MEDIA_PRESENT: u32 = 0x0000_0005;
pub const MCI_STATUS_TIME_FORMAT: u32 = 0x0000_0006;
pub const MCI_STATUS_READY: u32 = 0x0000_0007;
pub const MCI_STATUS_CURRENT_TRACK: u32 = 0x0000_0008;
pub const MCI_CDA_STATUS_TYPE_TRACK: u32 = 0x0000_4001;

pub const MCI_GETDEVCAPS_ITEM: u32 = 0x0000_0100;
pub const MCI_GETDEVCAPS_CAN_RECORD: u32 = 0x0000_0001;
pub const MCI_GETDEVCAPS_HAS_AUDIO: u32 = 0x0000_0002;
pub const MCI_GETDEVCAPS_HAS_VIDEO: u32 = 0x0000_0003;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: u32 = 0x0000_0004;
pub const MCI_GETDEVCAPS_USES_FILES: u32 = 0x0000_0005;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: u32 = 0x0000_0006;
pub const MCI_GETDEVCAPS_CAN_EJECT: u32 = 0x0000_0007;
pub const MCI_GETDEVCAPS_CAN_PLAY: u32 = 0x0000_0008;
pub const MCI_GETDEVCAPS_CAN_SAVE: u32 = 0x0000_0009;

pub const MCI_INFO_PRODUCT: u32 = 0x0000_0100;
pub const MCI_INFO_MEDIA_UPC: u32 = 0x0000_0400;
pub const MCI_INFO_MEDIA_IDENTITY: u32 = 0x0000_0800;

pub const MCI_SET_DOOR_OPEN: u32 = 0x0000_0100;
pub const MCI_SET_DOOR_CLOSED: u32 = 0x0000_0200;
pub const MCI_SET_TIME_FORMAT: u32 = 0x0000_0400;

pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
pub const MCI_FORMAT_MSF: u32 = 2;
pub const MCI_FORMAT_TMSF: u32 = 10;
pub const MCI_FORMAT_RETURN_BASE: u32 = 533;

pub const MCI_RESOURCE_RETURNED: u32 = 0x0001_0000;
pub const MCI_COLONIZED3_RETURN: u32 = 0x0002_0000;
pub const MCI_COLONIZED4_RETURN: u32 = 0x0004_0000;

pub const MCI_MODE_NOT_READY: u32 = 524;
pub const MCI_MODE_STOP: u32 = 525;
pub const MCI_MODE_PLAY: u32 = 526;
pub const MCI_MODE_PAUSE: u32 = 529;
pub const MCI_MODE_OPEN: u32 = 530;

pub const MCI_FALSE: u32 = 531;
pub const MCI_TRUE: u32 = 532;

pub const MCI_DEVTYPE_CD_AUDIO: u32 = 516;
pub const MCI_CDA_TRACK_AUDIO: u32 = 1088;
pub const MCI_CDA_TRACK_OTHER: u32 = 1089;

/// `MAKEMCIRESOURCE(ret, res)`: the return value in the low word, the resource id in the high word.
pub const fn make_resource(ret: u32, res: u32) -> u32 {
    (ret & 0xffff) | ((res & 0xffff) << 16)
}

/// `MAKEMCIRESOURCE(TRUE, MCI_TRUE)` or `MAKEMCIRESOURCE(FALSE, MCI_FALSE)`. The low word is
/// the C `TRUE`/`FALSE` value (1 or 0), not `MCI_TRUE`/`MCI_FALSE`.
pub const fn boolean_resource(value: bool) -> u32 {
    if value {
        make_resource(1, MCI_TRUE)
    } else {
        make_resource(0, MCI_FALSE)
    }
}
