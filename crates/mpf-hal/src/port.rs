/// `READ_PORT_UCHAR` from `dlls/hal/hal.c`. The port is never read: the function returns 0.
pub fn read_port_uchar(_port: *mut u8) -> u8 {
    0
}

/// `READ_PORT_ULONG` from `dlls/hal/hal.c`. The port is never read: the function returns 0.
pub fn read_port_ulong(_port: *mut u32) -> u32 {
    0
}

/// `WRITE_PORT_UCHAR` from `dlls/hal/hal.c`. The C body only traces the call, so nothing is
/// written to the port.
pub fn write_port_uchar(_port: *mut u8, _value: u8) {}

/// `WRITE_PORT_ULONG` from `dlls/hal/hal.c`. The C body only traces the call, so nothing is
/// written to the port.
pub fn write_port_ulong(_port: *mut u32, _value: u32) {}
