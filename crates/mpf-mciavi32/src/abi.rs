//! Constants and packing helpers from `mmsystem.h`, `mmddk.h` and `digitalv.h` that `info.c` uses.

pub const MCI_TEST: u32 = 0x20;
pub const MCI_NOTIFY: u32 = 0x1;

pub const MCI_GETDEVCAPS_ITEM: u32 = 0x100;
pub const MCI_INFO_PRODUCT: u32 = 0x100;
pub const MCI_INFO_FILE: u32 = 0x200;
pub const MCI_INFO_VERSION: u32 = 0x400;

pub const MCI_SET_DOOR_OPEN: u32 = 0x100;
pub const MCI_SET_DOOR_CLOSED: u32 = 0x200;
pub const MCI_SET_TIME_FORMAT: u32 = 0x400;
pub const MCI_SET_AUDIO: u32 = 0x800;
pub const MCI_SET_VIDEO: u32 = 0x1000;
pub const MCI_SET_ON: u32 = 0x2000;
pub const MCI_SET_OFF: u32 = 0x4000;
pub const MCI_DGV_SET_SEEK_EXACTLY: u32 = 0x10000;
pub const MCI_DGV_SET_SPEED: u32 = 0x20000;
pub const MCI_DGV_SET_FILEFORMAT: u32 = 0x80000;

pub const MCI_SET_AUDIO_ALL: u32 = 0;
pub const MCI_SET_AUDIO_LEFT: u32 = 0x1;
pub const MCI_SET_AUDIO_RIGHT: u32 = 0x2;

pub const MCI_STATUS_ITEM: u32 = 0x100;
pub const MCI_STATUS_START: u32 = 0x200;

pub const MCI_GETDEVCAPS_CAN_RECORD: u32 = 0x1;
pub const MCI_GETDEVCAPS_HAS_AUDIO: u32 = 0x2;
pub const MCI_GETDEVCAPS_HAS_VIDEO: u32 = 0x3;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: u32 = 0x4;
pub const MCI_GETDEVCAPS_USES_FILES: u32 = 0x5;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: u32 = 0x6;
pub const MCI_GETDEVCAPS_CAN_EJECT: u32 = 0x7;
pub const MCI_GETDEVCAPS_CAN_PLAY: u32 = 0x8;
pub const MCI_GETDEVCAPS_CAN_SAVE: u32 = 0x9;
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: u32 = 0x4000;
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: u32 = 0x4001;
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: u32 = 0x4002;
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: u32 = 0x4004;
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: u32 = 0x4005;
pub const MCI_DGV_GETDEVCAPS_PALETTES: u32 = 0x4006;
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: u32 = 0x4008;
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: u32 = 0x4009;

pub const MCI_STATUS_LENGTH: u32 = 0x1;
pub const MCI_STATUS_POSITION: u32 = 0x2;
pub const MCI_STATUS_NUMBER_OF_TRACKS: u32 = 0x3;
pub const MCI_STATUS_MODE: u32 = 0x4;
pub const MCI_STATUS_MEDIA_PRESENT: u32 = 0x5;
pub const MCI_STATUS_TIME_FORMAT: u32 = 0x6;
pub const MCI_STATUS_READY: u32 = 0x7;
pub const MCI_STATUS_CURRENT_TRACK: u32 = 0x8;
pub const MCI_DGV_STATUS_HWND: u32 = 0x4001;
pub const MCI_DGV_STATUS_SPEED: u32 = 0x4003;
pub const MCI_DGV_STATUS_HPAL: u32 = 0x4004;
pub const MCI_DGV_STATUS_AUDIO: u32 = 0x4014;
pub const MCI_DGV_STATUS_VIDEO: u32 = 0x4018;
pub const MCI_DGV_STATUS_FRAME_RATE: u32 = 0x400e;
pub const MCI_DGV_STATUS_WINDOW_VISIBLE: u32 = 0x4021;
pub const MCI_DGV_STATUS_WINDOW_MINIMIZED: u32 = 0x4022;
pub const MCI_DGV_STATUS_WINDOW_MAXIMIZED: u32 = 0x4023;
pub const MCI_DGV_STATUS_PAUSE_MODE: u32 = 0x4026;
pub const MCI_DGV_STATUS_FORWARD: u32 = 0x402c;
pub const MCI_DGV_STATUS_AUDIO_STREAM: u32 = 0x402d;
pub const MCI_DGV_STATUS_BITSPERPEL: u32 = 0x402b;

pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 0x208;

pub const MCI_MODE_NOT_READY: u32 = 0x20c;
pub const MCI_MODE_STOP: u32 = 0x20d;
pub const MCI_MODE_PLAY: u32 = 0x20e;
pub const MCI_MODE_PAUSE: u32 = 0x211;

pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
pub const MCI_FORMAT_FRAMES: u32 = 0x3;
pub const MCI_FORMAT_RETURN_BASE: u32 = 0x215;

pub const MCI_FALSE: u32 = 0x213;
pub const MCI_TRUE: u32 = 0x214;
pub const MCI_OFF: u32 = 0;
pub const MCI_ON: u32 = 0x1;
pub const MCI_OFF_S: u32 = 0x8001;
pub const MCI_ON_S: u32 = 0x8000;
pub const MCI_RESOURCE_RETURNED: u32 = 0x10000;
pub const MCI_RESOURCE_DRIVER: u32 = 0x100000;

pub const MCIERR_INVALID_DEVICE_ID: u32 = 0x101;
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 0x105;
pub const MCIERR_MISSING_PARAMETER: u32 = 0x111;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 0x112;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 0x125;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 0x129;
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 0x12e;

/// The `dwSet` bit that `MCI_SET_VIDEO` turns on.
pub const SET_VIDEO_BIT: u32 = 4;
/// The `dwSet` bit that `MCI_SET_AUDIO_LEFT` turns on.
pub const SET_AUDIO_LEFT_BIT: u32 = 1;
/// The `dwSet` bit that `MCI_SET_AUDIO_RIGHT` turns on.
pub const SET_AUDIO_RIGHT_BIT: u32 = 2;
/// The `dwSet` bits that `MCI_SET_AUDIO_ALL` turns on.
pub const SET_AUDIO_BITS: u32 = SET_AUDIO_LEFT_BIT | SET_AUDIO_RIGHT_BIT;

/// `MAKEMCIRESOURCE`: packs the low word `lo` and the high word `hi` into one `DWORD`.
pub const fn make_resource(lo: u32, hi: u32) -> u32 {
    (lo & 0xffff) | ((hi & 0xffff) << 16)
}

/// `MAKEMCIRESOURCE(TRUE, MCI_TRUE)` or `MAKEMCIRESOURCE(FALSE, MCI_FALSE)`.
pub const fn bool_resource(flag: bool) -> u32 {
    if flag {
        make_resource(1, MCI_TRUE)
    } else {
        make_resource(0, MCI_FALSE)
    }
}
