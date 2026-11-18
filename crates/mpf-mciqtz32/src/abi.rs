//! Win32, COM and MCI values that `mciqtz.c` reads. The numbers come from the Wine headers
//! (`winerror.h`, `mmsystem.h`, `mciavi.h`, `dshow.h`, `winuser.h`, `winbase.h`).

/// A COM `HRESULT`. A negative value is a failure.
pub type HResult = i32;

pub const S_OK: HResult = 0;
pub const E_NOINTERFACE: HResult = 0x8000_4002u32 as i32;
pub const E_FAIL: HResult = 0x8000_4005u32 as i32;

/// `FAILED(hr)`.
pub const fn failed(hr: HResult) -> bool {
    hr < 0
}

/// The MCI error for a COM result: [MCIERR_INTERNAL] when `hr` failed, otherwise 0.
pub const fn com_status(hr: HResult) -> u32 {
    if failed(hr) {
        MCIERR_INTERNAL
    } else {
        0
    }
}

/// `MAKEMCIRESOURCE(lo, hi)`: `lo` in bits 0 to 15 and `hi` in bits 16 to 31.
pub const fn make_mci_resource(lo: u32, hi: u32) -> u32 {
    (lo & 0xffff) | ((hi & 0xffff) << 16)
}

pub const ERROR_CLASS_ALREADY_EXISTS: u32 = 1410;
pub const WAIT_OBJECT_0: u32 = 0;
pub const WAIT_TIMEOUT: u32 = 258;

pub const STATE_STOPPED: i32 = 0;
pub const STATE_PAUSED: i32 = 1;
pub const STATE_RUNNING: i32 = 2;

pub const AM_SEEKING_NO_POSITIONING: u32 = 0;
pub const AM_SEEKING_ABSOLUTE_POSITIONING: u32 = 1;

pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
pub const WS_CLIPCHILDREN: u32 = 0x0200_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const WS_POPUP: u32 = 0x8000_0000;
pub const WS_CHILD: u32 = 0x4000_0000;

pub const MCIERR_INVALID_DEVICE_ID: u32 = 257;
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261;
pub const MCIERR_MISSING_PARAMETER: u32 = 273;
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274;
pub const MCIERR_INTERNAL: u32 = 277;
pub const MCIERR_OUTOFRANGE: u32 = 282;
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293;
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297;

pub const MCI_INTEGER_RETURNED: u32 = 0x80000;
pub const MCI_RESOURCE_RETURNED: u32 = 0x10000;
pub const MCI_FORMAT_RETURN_BASE: u32 = 533;
pub const MCI_TRUE: u32 = 532;
pub const MCI_FALSE: u32 = 531;
pub const MCI_MODE_STOP: u32 = 525;
pub const MCI_MODE_PLAY: u32 = 526;
pub const MCI_MODE_PAUSE: u32 = 529;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 520;

pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
pub const MCI_FORMAT_HMS: u32 = 1;
pub const MCI_FORMAT_FRAMES: u32 = 3;

pub const MCI_NOTIFY: u32 = 0x1;
pub const MCI_FROM: u32 = 0x4;
pub const MCI_TO: u32 = 0x8;
pub const MCI_TEST: u32 = 0x20;
pub const MCI_SET_TIME_FORMAT: u32 = 0x400;
pub const MCI_STATUS_ITEM: u32 = 0x100;
pub const MCI_GETDEVCAPS_ITEM: u32 = 0x100;
pub const MCI_SEEK_TO_START: u32 = 0x100;
pub const MCI_SEEK_TO_END: u32 = 0x200;
pub const MCI_OPEN_ELEMENT: u32 = 0x200;
pub const MCI_OPEN_ELEMENT_ID: u32 = 0x800;
pub const MCI_DGV_RECT: u32 = 0x10000;
pub const MCI_DGV_OPEN_WS: u32 = 0x10000;
pub const MCI_DGV_OPEN_PARENT: u32 = 0x20000;
pub const MCI_DGV_PUT_DESTINATION: u32 = 0x40000;
pub const MCI_DGV_SETAUDIO_ITEM: u32 = 0x800000;
pub const MCI_DGV_SETAUDIO_VALUE: u32 = 0x1000000;

pub const MCI_DGV_SETAUDIO_VOLUME: u32 = 16386;

pub const MCI_STATUS_LENGTH: u32 = 1;
pub const MCI_STATUS_POSITION: u32 = 2;
pub const MCI_STATUS_NUMBER_OF_TRACKS: u32 = 3;
pub const MCI_STATUS_MODE: u32 = 4;
pub const MCI_STATUS_MEDIA_PRESENT: u32 = 5;
pub const MCI_STATUS_TIME_FORMAT: u32 = 6;
pub const MCI_STATUS_READY: u32 = 7;
pub const MCI_STATUS_CURRENT_TRACK: u32 = 8;

pub const MCI_GETDEVCAPS_CAN_RECORD: u32 = 1;
pub const MCI_GETDEVCAPS_HAS_AUDIO: u32 = 2;
pub const MCI_GETDEVCAPS_HAS_VIDEO: u32 = 3;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: u32 = 4;
pub const MCI_GETDEVCAPS_USES_FILES: u32 = 5;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: u32 = 6;
pub const MCI_GETDEVCAPS_CAN_EJECT: u32 = 7;
pub const MCI_GETDEVCAPS_CAN_PLAY: u32 = 8;
pub const MCI_GETDEVCAPS_CAN_SAVE: u32 = 9;
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: u32 = 16384;
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: u32 = 16385;
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: u32 = 16386;
pub const MCI_DGV_GETDEVCAPS_MAX_WINDOWS: u32 = 16387;
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: u32 = 16388;
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: u32 = 16389;
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: u32 = 16392;
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: u32 = 16393;
pub const MCI_DGV_GETDEVCAPS_MAXIMUM_RATE: u32 = 16394;
pub const MCI_DGV_GETDEVCAPS_MINIMUM_RATE: u32 = 16395;

pub const DRV_LOAD: u32 = 1;
pub const DRV_ENABLE: u32 = 2;
pub const DRV_OPEN: u32 = 3;
pub const DRV_CLOSE: u32 = 4;
pub const DRV_DISABLE: u32 = 5;
pub const DRV_FREE: u32 = 6;
pub const DRV_CONFIGURE: u32 = 7;
pub const DRV_QUERYCONFIGURE: u32 = 8;
pub const DRV_INSTALL: u32 = 9;
pub const DRV_REMOVE: u32 = 10;
pub const DRV_EXITSESSION: u32 = 11;
pub const DRVCNF_RESTART: u32 = 2;

pub const MCI_OPEN_DRIVER: u32 = 2049;
pub const MCI_CLOSE_DRIVER: u32 = 2050;
pub const MCI_OPEN: u32 = 2051;
pub const MCI_CLOSE: u32 = 2052;
pub const MCI_ESCAPE: u32 = 2053;
pub const MCI_PLAY: u32 = 2054;
pub const MCI_SEEK: u32 = 2055;
pub const MCI_STOP: u32 = 2056;
pub const MCI_PAUSE: u32 = 2057;
pub const MCI_INFO: u32 = 2058;
pub const MCI_GETDEVCAPS: u32 = 2059;
pub const MCI_SPIN: u32 = 2060;
pub const MCI_SET: u32 = 2061;
pub const MCI_STEP: u32 = 2062;
pub const MCI_RECORD: u32 = 2063;
pub const MCI_SAVE: u32 = 2067;
pub const MCI_STATUS: u32 = 2068;
pub const MCI_CUE: u32 = 2096;
pub const MCI_REALIZE: u32 = 2112;
pub const MCI_WINDOW: u32 = 2113;
pub const MCI_PUT: u32 = 2114;
pub const MCI_WHERE: u32 = 2115;
pub const MCI_FREEZE: u32 = 2116;
pub const MCI_UNFREEZE: u32 = 2117;
pub const MCI_LOAD: u32 = 2128;
pub const MCI_CUT: u32 = 2129;
pub const MCI_COPY: u32 = 2130;
pub const MCI_PASTE: u32 = 2131;
pub const MCI_UPDATE: u32 = 2132;
pub const MCI_RESUME: u32 = 2133;
pub const MCI_DELETE: u32 = 2134;
pub const MCI_CAPTURE: u32 = 2160;
pub const MCI_MONITOR: u32 = 2161;
pub const MCI_RESERVE: u32 = 2162;
pub const MCI_SETAUDIO: u32 = 2163;
pub const MCI_SIGNAL: u32 = 2165;
pub const MCI_SETVIDEO: u32 = 2166;
pub const MCI_QUALITY: u32 = 2167;
pub const MCI_LIST: u32 = 2168;
pub const MCI_UNDO: u32 = 2169;
pub const MCI_CONFIGURE: u32 = 2170;
pub const MCI_RESTORE: u32 = 2171;
