//! The dispatch of `MCIQTZ_DriverProc`, and the replies of its driver-level messages.

use crate::abi::*;

pub use crate::route;

/// The device id of the session instance, which takes the messages no device owns.
pub const SESSION_DEVICE: u32 = 0xFFFF_FFFF;

/// The MCI commands that `MCIQTZ_DriverProc` sends on to a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Open,
    Close,
    Play,
    Seek,
    Stop,
    Pause,
    Resume,
    GetDevCaps,
    Set,
    Status,
    Where,
    SetAudio,
    Update,
    Window,
    Put,
}

/// Where a driver message goes. `Reply` returns its value straight away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Reply(u32),
    DrvOpen,
    DrvClose,
    DrvConfigure,
    Command(Command),
    /// A command that `mciqtz.c` does not implement. It returns [MCIERR_UNRECOGNIZED_COMMAND].
    Unrecognized,
    /// A message the driver does not handle. It goes to `DefDriverProc`.
    Default,
}

/// The return value of `MCIQTZ_drvOpen`. The session instance gets [SESSION_DEVICE]. A failed
/// class registration or allocation returns 0. Otherwise the new device id is returned.
pub fn drv_open_reply(has_params: bool, class_ready: bool, alloc_ok: bool, device_id: u32) -> u32 {
    if !has_params {
        SESSION_DEVICE
    } else if !class_ready || !alloc_ok {
        0
    } else {
        device_id
    }
}

/// The return value of `MCIQTZ_drvClose`. It is 1 for an open device, or for the session.
pub fn drv_close_reply(device_exists: bool, dev_id: u32) -> u32 {
    u32::from(device_exists || dev_id == SESSION_DEVICE)
}

/// The return value of `MCIQTZ_drvConfigure`. It is 1 when the device exists.
pub fn drv_configure_reply(device_exists: bool) -> u32 {
    u32::from(device_exists)
}

/// Whether the window class is usable. `RegisterClassW` may fail when the class already exists,
/// which Wine accepts.
pub fn class_ok(registered: bool, last_error: u32) -> bool {
    registered || last_error == ERROR_CLASS_ALREADY_EXISTS
}
