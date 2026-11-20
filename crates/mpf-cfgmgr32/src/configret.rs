//! `CONFIGRET` codes and the Win32 error mapping of `cfgmgr32.c`.

pub const CR_SUCCESS: u32 = 0x00;
pub const CR_OUT_OF_MEMORY: u32 = 0x02;
pub const CR_INVALID_POINTER: u32 = 0x03;
pub const CR_INVALID_FLAG: u32 = 0x04;
pub const CR_INVALID_DEVNODE: u32 = 0x05;
pub const CR_NO_SUCH_DEVNODE: u32 = 0x0d;
pub const CR_ALREADY_SUCH_DEVNODE: u32 = 0x10;
pub const CR_FAILURE: u32 = 0x13;
pub const CR_BUFFER_SMALL: u32 = 0x1a;
pub const CR_INVALID_DATA: u32 = 0x1f;
pub const CR_NO_REGISTRY_HANDLE: u32 = 0x1c;
pub const CR_REGISTRY_ERROR: u32 = 0x1d;
pub const CR_INVALID_DEVICE_ID: u32 = 0x1e;
pub const CR_NO_SUCH_REGISTRY_KEY: u32 = 0x2e;
pub const CR_INVALID_MACHINENAME: u32 = 0x2f;
pub const CR_REMOTE_COMM_FAILURE: u32 = 0x30;
pub const CR_MACHINE_UNAVAILABLE: u32 = 0x31;
pub const CR_NO_CM_SERVICES: u32 = 0x32;
pub const CR_ACCESS_DENIED: u32 = 0x33;
pub const CR_CALL_NOT_IMPLEMENTED: u32 = 0x34;
pub const CR_INVALID_PROPERTY: u32 = 0x35;
pub const CR_NO_SUCH_VALUE: u32 = 0x25;
pub const CR_NO_SUCH_DEVICE_INTERFACE: u32 = 0x37;
pub const CR_INVALID_REFERENCE_STRING: u32 = 0x38;

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_HANDLE: u32 = 6;
pub const ERROR_NOT_ENOUGH_MEMORY: u32 = 8;
pub const ERROR_INVALID_DATA: u32 = 13;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
pub const ERROR_ALREADY_EXISTS: u32 = 183;
pub const ERROR_MORE_DATA: u32 = 234;
pub const ERROR_NO_MORE_ITEMS: u32 = 259;
pub const ERROR_INVALID_FLAGS: u32 = 1004;
pub const ERROR_REGISTRY_CORRUPT: u32 = 1015;
pub const ERROR_SERVICE_NOT_ACTIVE: u32 = 1062;
pub const ERROR_NOT_FOUND: u32 = 1168;
pub const ERROR_UNKNOWN_PROPERTY: u32 = 1608;
pub const ERROR_INVALID_USER_BUFFER: u32 = 1784;

/// `CM_MapCrToWin32Err`: the Win32 error for a `CONFIGRET`, or `default_error` when no
/// case applies.
pub fn cr_to_win32_error(code: u32, default_error: u32) -> u32 {
    match code {
        CR_SUCCESS => ERROR_SUCCESS,
        CR_OUT_OF_MEMORY => ERROR_NOT_ENOUGH_MEMORY,
        CR_INVALID_POINTER => ERROR_INVALID_USER_BUFFER,
        CR_INVALID_FLAG => ERROR_INVALID_FLAGS,
        CR_INVALID_DEVNODE
        | CR_INVALID_DEVICE_ID
        | CR_INVALID_MACHINENAME
        | CR_INVALID_PROPERTY
        | CR_INVALID_REFERENCE_STRING => ERROR_INVALID_DATA,
        CR_NO_SUCH_DEVNODE | CR_NO_SUCH_VALUE | CR_NO_SUCH_DEVICE_INTERFACE => ERROR_NOT_FOUND,
        CR_ALREADY_SUCH_DEVNODE => ERROR_ALREADY_EXISTS,
        CR_BUFFER_SMALL => ERROR_INSUFFICIENT_BUFFER,
        CR_NO_REGISTRY_HANDLE => ERROR_INVALID_HANDLE,
        CR_REGISTRY_ERROR => ERROR_REGISTRY_CORRUPT,
        CR_NO_SUCH_REGISTRY_KEY => ERROR_FILE_NOT_FOUND,
        CR_REMOTE_COMM_FAILURE | CR_MACHINE_UNAVAILABLE | CR_NO_CM_SERVICES => {
            ERROR_SERVICE_NOT_ACTIVE
        }
        CR_ACCESS_DENIED => ERROR_ACCESS_DENIED,
        CR_CALL_NOT_IMPLEMENTED => ERROR_CALL_NOT_IMPLEMENTED,
        _ => default_error,
    }
}

/// `map_error`: the `CONFIGRET` for a registry `LSTATUS`. An unlisted error gives
/// `CR_FAILURE` (the C version also logs it).
pub fn map_error(err: u32) -> u32 {
    match err {
        ERROR_INVALID_PARAMETER => CR_FAILURE,
        ERROR_INVALID_USER_BUFFER => CR_INVALID_POINTER,
        ERROR_FILE_NOT_FOUND => CR_NO_SUCH_REGISTRY_KEY,
        ERROR_MORE_DATA => CR_BUFFER_SMALL,
        ERROR_NO_MORE_ITEMS | ERROR_NOT_FOUND => CR_NO_SUCH_VALUE,
        ERROR_SUCCESS => CR_SUCCESS,
        ERROR_UNKNOWN_PROPERTY => CR_INVALID_PROPERTY,
        _ => CR_FAILURE,
    }
}

/// `map_error_node`: `map_error` for a device node lookup, where a missing or invalid
/// node has its own codes.
pub fn map_error_node(err: u32) -> u32 {
    match err {
        ERROR_INVALID_DATA => CR_INVALID_DEVNODE,
        ERROR_NOT_FOUND | ERROR_FILE_NOT_FOUND => CR_NO_SUCH_DEVNODE,
        _ => map_error(err),
    }
}
