//! `DefDriverProc` from `dlls/winmm/driver.c`. The C body reads only the message, so the port takes
//! only the message. The driver identifier, the driver handle and the two `LPARAM` values are not
//! modelled.

/// `DRV_LOAD`, `DRV_ENABLE`, `DRV_DISABLE`, `DRV_FREE`, `DRV_INSTALL`, `DRV_REMOVE` and
/// `DRV_SUCCESS`, from `include/mmddk.h`.
const DRV_LOAD: u32 = 0x0001;
const DRV_ENABLE: u32 = 0x0002;
const DRV_DISABLE: u32 = 0x0005;
const DRV_FREE: u32 = 0x0006;
const DRV_INSTALL: u32 = 0x0009;
const DRV_REMOVE: u32 = 0x000a;
const DRV_SUCCESS: isize = 0x0001;

/// `DefDriverProc`: the load, enable, disable and free messages return 1, the install and remove
/// messages return `DRV_SUCCESS`, and every other message returns 0.
pub fn def_driver_proc(msg: u32) -> isize {
    match msg {
        DRV_LOAD | DRV_FREE | DRV_ENABLE | DRV_DISABLE => 1,
        DRV_INSTALL | DRV_REMOVE => DRV_SUCCESS,
        _ => 0,
    }
}
