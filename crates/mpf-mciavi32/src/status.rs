//! `MCIAVI_mciStatus` and `MulDiv`.

use crate::abi::*;
use crate::time::frame_to_time_format;
use crate::{Device, Reply};

const NO_FILE: Reply = Reply {
    ret: MCIERR_UNSUPPORTED_FUNCTION,
    dw_return: Some(0),
};

/// `device` is `None` when the device id is not open, and `item` is `None` when the parameter
/// block is NULL. The window items return `None`, because they query OS windows and the caller
/// answers them. `MCI_NOTIFY` is left to the caller too.
pub fn mci_status(device: Option<&Device>, flags: u32, item: Option<u32>) -> Option<Reply> {
    let Some(item) = item else {
        return Some(Reply::code(MCIERR_NULL_PARAMETER_BLOCK));
    };
    let Some(device) = device else {
        return Some(Reply::code(MCIERR_INVALID_DEVICE_ID));
    };
    if flags & MCI_STATUS_ITEM == 0 {
        return Some(Reply::code(MCIERR_MISSING_PARAMETER));
    }
    if flags & MCI_TEST != 0 {
        return Some(Reply::code(0));
    }
    match item {
        MCI_DGV_STATUS_HWND
        | MCI_DGV_STATUS_WINDOW_VISIBLE
        | MCI_DGV_STATUS_WINDOW_MINIMIZED
        | MCI_DGV_STATUS_WINDOW_MAXIMIZED => None,
        _ => Some(item_reply(device, flags, item)),
    }
}

fn item_reply(device: &Device, flags: u32, item: u32) -> Reply {
    match item {
        MCI_STATUS_CURRENT_TRACK | MCI_STATUS_NUMBER_OF_TRACKS => Reply::value(1),
        MCI_STATUS_LENGTH if !device.has_file => NO_FILE,
        MCI_STATUS_LENGTH => Reply::value(frame_to_time_format(device, device.total_frames)),
        MCI_STATUS_POSITION if !device.has_file => NO_FILE,
        MCI_STATUS_POSITION => {
            let frame = if flags & MCI_STATUS_START != 0 {
                0
            } else {
                device.current_frame
            };
            Reply::value(frame_to_time_format(device, frame))
        }
        MCI_STATUS_MODE => Reply::resource(make_resource(device.status, device.status)),
        MCI_STATUS_MEDIA_PRESENT | MCI_DGV_STATUS_FORWARD => Reply::resource(bool_resource(true)),
        MCI_STATUS_READY => Reply::resource(bool_resource(device.status != MCI_MODE_NOT_READY)),
        MCI_STATUS_TIME_FORMAT => {
            let format = device.time_format;
            Reply::resource(make_resource(
                format,
                format.wrapping_add(MCI_FORMAT_RETURN_BASE),
            ))
        }
        MCI_DGV_STATUS_AUDIO => driver_resource(device.set & SET_AUDIO_BITS != 0),
        MCI_DGV_STATUS_VIDEO => driver_resource(device.set & SET_VIDEO_BIT != 0),
        MCI_DGV_STATUS_BITSPERPEL => Reply::value(u32::from(device.bit_count)),
        MCI_DGV_STATUS_HPAL => Reply::value(0),
        MCI_DGV_STATUS_SPEED => Reply::value(1000),
        MCI_DGV_STATUS_FRAME_RATE => {
            Reply::value(mul_div(1_000_000, 1000, device.micro_sec_per_frame as i32) as u32)
        }
        MCI_DGV_STATUS_PAUSE_MODE if device.status != MCI_MODE_PAUSE => {
            Reply::code(MCIERR_NONAPPLICABLE_FUNCTION)
        }
        MCI_DGV_STATUS_PAUSE_MODE => Reply::resource(make_resource(MCI_MODE_PLAY, MCI_MODE_PLAY)),
        MCI_DGV_STATUS_AUDIO_STREAM => Reply::value(device.audio_stream),
        _ => Reply::code(MCIERR_UNSUPPORTED_FUNCTION),
    }
}

/// The audio and video items return their state as a driver-owned resource.
fn driver_resource(on: bool) -> Reply {
    let state = if on {
        make_resource(MCI_ON, MCI_ON_S)
    } else {
        make_resource(MCI_OFF, MCI_OFF_S)
    };
    Reply {
        ret: MCI_RESOURCE_RETURNED | MCI_RESOURCE_DRIVER,
        dw_return: Some(state),
    }
}

/// Wine's `MulDiv`: `a * b / c` rounded half away from zero. It returns -1 when `c` is zero or
/// the result does not fit in an `INT`. The product uses 64 bits, so no operand overflows.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (mut a, mut c) = (i64::from(a), i64::from(c));
    if c < 0 {
        a = -a;
        c = -c;
    }
    let b = i64::from(b);
    let product = a * b;
    let rounded = if (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        (product + c / 2) / c
    } else {
        (product - c / 2) / c
    };
    if rounded > i64::from(i32::MAX) || rounded < -i64::from(i32::MAX) {
        -1
    } else {
        rounded as i32
    }
}
