use std::ffi::c_void;

/// `HalGetBusData` from `dlls/hal/hal.c`. It claims there is no such bus, so it returns 0 and
/// does not touch `buffer`.
pub fn hal_get_bus_data(
    _bus_data_type: i32,
    _bus_number: u32,
    _slot_number: u32,
    _buffer: *mut c_void,
    _length: u32,
) -> u32 {
    0
}

/// `HalGetBusDataByOffset` from `dlls/hal/hal.c`. It claims there is no such bus, so it returns 0
/// and does not touch `buffer`.
pub fn hal_get_bus_data_by_offset(
    _bus_data_type: i32,
    _bus_number: u32,
    _slot_number: u32,
    _buffer: *mut c_void,
    _offset: u32,
    _length: u32,
) -> u32 {
    0
}

/// `HalTranslateBusAddress` from `dlls/hal/hal.c`. It always fails, and it writes neither
/// `address_space` nor `translated_address`.
pub fn hal_translate_bus_address(
    _interface_type: i32,
    _bus_number: u32,
    _bus_address: i64,
    _address_space: *mut u32,
    _translated_address: *mut i64,
) -> bool {
    false
}
