//! Headless subset of Wine's `dlls/rasapi32/rasapi.c`.
//!
//! Ported (the `FIXME` trace is skipped):
//! - `RasEnumDevicesA`: `ras_enum_devices_a`; the argument checks, the `dwSize` check, the buffer-size query and the virtual modem entry.
//!
//! Skipped (each export is a `FIXME` stub unless noted):
//! - `RasEnumDevicesW`: stub returning 0 without writing its outputs.
//! - `RasEnumConnectionsA`/`W`: stubs that write 0 to the buffer size and connection count, then return 0. RAS connections are network state.
//! - `RasEnumEntriesA`/`W`: stubs that write 0 to the entry count, then return 0. The phonebook is a file.
//! - The other exports (`RasDialA`/`W`, `RasHangUpA`/`W`, `RasGetConnectStatusA`/`W`, `RasGetEntryDialParamsA`/`W`, `RasGetProjectionInfoA`/`W`, `RasSetEntryPropertiesA`/`W`, the autodial and credential functions, and the rest): stubs that return a constant (0, 1 or `ERROR_UNKNOWN`). They need RAS connection or phonebook state.

const RASBASE: u32 = 600;

/// `ERROR_INVALID_PARAMETER`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `ERROR_BUFFER_TOO_SMALL` from `raserror.h`, which is `RASBASE + 3`, not the Win32 value 122.
pub const ERROR_BUFFER_TOO_SMALL: u32 = RASBASE + 3;
/// `ERROR_INVALID_SIZE` from `raserror.h`.
pub const ERROR_INVALID_SIZE: u32 = RASBASE + 32;

/// `sizeof(RASDEVINFOA)` on Wine: the `dwSize` field plus two fixed-size strings, padded to 4 bytes.
pub const RAS_DEVINFO_A_SIZE: u32 = 152;
const RAS_MAX_DEVICE_TYPE: usize = 16;
const RAS_MAX_DEVICE_NAME: usize = 128;
const RASDT_MODEM: &[u8] = b"modem";
const VIRTUAL_MODEM_NAME: &[u8] = b"WINE virtmodem";

/// `RASDEVINFOA`: the `dwSize` field, then the NUL-terminated device type and name.
#[repr(C)]
pub struct RasDevInfoA {
    pub dw_size: u32,
    pub device_type: [u8; RAS_MAX_DEVICE_TYPE + 1],
    pub device_name: [u8; RAS_MAX_DEVICE_NAME + 1],
}

/// Copies `text` and its NUL terminator to the start of `dst`, as `strcpy` does. The bytes after the terminator are left alone.
fn strcpy_into<const N: usize>(dst: &mut [u8; N], text: &[u8]) {
    dst[..text.len()].copy_from_slice(text);
    dst[text.len()] = 0;
}

/// Ports `RasEnumDevicesA`. `devinfo` is the optional `RASDEVINFOA` buffer, `cb` the in/out buffer size and `devices` the device count.
///
/// A null `cb` or `devices` fails with `ERROR_INVALID_PARAMETER`. A buffer whose `dw_size` is not `RAS_DEVINFO_A_SIZE` fails with `ERROR_INVALID_SIZE`; in both cases nothing is written. Otherwise `devices` becomes 1. With no buffer, or one smaller than `RAS_DEVINFO_A_SIZE`, `cb` is set to `RAS_DEVINFO_A_SIZE` and `ERROR_BUFFER_TOO_SMALL` is returned. With a large enough buffer, it receives the virtual modem and the call returns 0.
pub fn ras_enum_devices_a(
    devinfo: Option<&mut RasDevInfoA>,
    cb: Option<&mut u32>,
    devices: Option<&mut u32>,
) -> u32 {
    let (Some(cb), Some(devices)) = (cb, devices) else {
        return ERROR_INVALID_PARAMETER;
    };
    if devinfo
        .as_deref()
        .is_some_and(|info| info.dw_size != RAS_DEVINFO_A_SIZE)
    {
        return ERROR_INVALID_SIZE;
    }
    *devices = 1;
    match devinfo {
        Some(info) if *cb >= RAS_DEVINFO_A_SIZE => {
            strcpy_into(&mut info.device_type, RASDT_MODEM);
            strcpy_into(&mut info.device_name, VIRTUAL_MODEM_NAME);
            0
        }
        _ => {
            *cb = RAS_DEVINFO_A_SIZE;
            ERROR_BUFFER_TOO_SMALL
        }
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
