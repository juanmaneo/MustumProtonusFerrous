//! Headless subset of Wine's `dlls/usbd.sys` (`usbd.c`).
//!
//! The ported functions compute from their arguments what the C bodies compute.
//! Nothing is allocated from a pool; all operations are pure over bytes.
//!
//! Ported:
//! - `parse_descriptors`: the descriptor item reader, walking by `bLength`.
//! - `parse_configuration_descriptor_ex`: the interface descriptor finder with
//!   filtering on interface number, alternate setting, class, subclass, protocol.
//! - `parse_configuration_descriptor`: the simpler interface finder.
//! - `get_interface_length`: the total length of an interface and its endpoints.
//! - `validate_configuration_descriptor`: the configuration descriptor validator.
//! - `get_usbd_i_version`: the USBDI version constants.
//!
//! Skipped:
//! - `USBD_CreateConfigurationRequest`, `USBD_CreateConfigurationRequestEx`:
//!   they allocate from a pool and build `URB` structures with opaque handles.
//! - `USBD_AllocateDeviceName`, `USBD_CalculateUsbBandwidth`, `USBD_CompleteRequest`,
//!   `USBD_CreateDevice`, `USBD_Debug_*`, `USBD_Dispatch`, `USBD_FreeDeviceMutex`,
//!   `USBD_FreeDeviceName`, `USBD_GetDeviceInformation`, `USBD_GetPdoRegistryParameter`,
//!   `USBD_GetRegistryKeyValue`, `USBD_GetSuspendPowerState`, `USBD_InitializeDevice`,
//!   `USBD_MakePdoName`, `USBD_QueryBusTime`, `USBD_RegisterHcDeviceCapabilities`,
//!   `USBD_RegisterHcFilter`, `USBD_RegisterHostController`, `USBD_RemoveDevice`,
//!   `USBD_RestoreDevice`, `USBD_SetSuspendPowerState`, `USBD_WaitDeviceMutex`:
//!   they need OS, registry, files, network, COM or handles.
//! - `DriverEntry`: a stub.
//!
//! Divergences from the C:
//! - A descriptor whose `bLength` is 0 would hang the C loop; `parse_descriptors`
//!   and `parse_configuration_descriptor_ex` return `None`, and
//!   `get_interface_length` stops summing there.
//! - The `offset` out-parameter of `validate_configuration_descriptor` is not
//!   modeled: the partial stub always writes `NULL` to it.

/// USB descriptor type constants (from Wine's `ddk/usb100.h`).
pub const USB_DEVICE_DESCRIPTOR_TYPE: i32 = 0x01;
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: i32 = 0x02;
pub const USB_STRING_DESCRIPTOR_TYPE: i32 = 0x03;
pub const USB_INTERFACE_DESCRIPTOR_TYPE: i32 = 0x04;
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: i32 = 0x05;

/// USBD status constants (from Wine's `ddk/usb.h`).
pub const USBD_STATUS_SUCCESS: i32 = 0x00000000;
pub const USBD_STATUS_ERROR: i32 = 0x8000_0000_u32 as i32;

/// Size of the packed `USB_COMMON_DESCRIPTOR` header.
const USB_COMMON_DESCRIPTOR_SIZE: usize = 2;
/// Size of the packed `USB_INTERFACE_DESCRIPTOR`.
const USB_INTERFACE_DESCRIPTOR_SIZE: usize = 9;
/// Size of the packed `USB_CONFIGURATION_DESCRIPTOR`.
const USB_CONFIGURATION_DESCRIPTOR_SIZE: usize = 9;

/// USBD version information returned by [`get_usbd_i_version`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbdVersion {
    pub usbdi_version: u32,
    pub supported_usb_version: u32,
}

/// Walks a USB descriptor buffer and returns the offset of the first descriptor
/// of `descriptor_type` at or after `start`, within the first `total_length` bytes.
///
/// Returns `None` when no such descriptor is found, when `start` is beyond the
/// buffer, or when a descriptor has `bLength == 0` (which would hang the C loop).
pub fn parse_descriptors(
    buffer: &[u8],
    total_length: usize,
    start: usize,
    descriptor_type: i32,
) -> Option<usize> {
    let mut off = 0;
    while off + USB_COMMON_DESCRIPTOR_SIZE <= total_length {
        let b_length = buffer[off] as usize;
        let b_type = buffer[off + 1] as i32;
        if off >= start && b_type == descriptor_type {
            return Some(off);
        }
        if b_length == 0 {
            return None;
        }
        off += b_length;
    }
    None
}

/// Finds an interface descriptor matching the given criteria.
///
/// The search starts at byte offset `start` and walks descriptors by `bLength`,
/// bounded by the configuration descriptor's `wTotalLength` (read from the buffer).
///
/// Returns the offset of the matching interface descriptor, or `None`.
pub fn parse_configuration_descriptor_ex(
    buffer: &[u8],
    start: usize,
    interface_number: i32,
    alternate_setting: i32,
    interface_class: i32,
    interface_subclass: i32,
    interface_protocol: i32,
) -> Option<usize> {
    let total_length = u16::from_le_bytes([buffer[2], buffer[3]]) as usize;
    let mut interface =
        parse_descriptors(buffer, total_length, start, USB_INTERFACE_DESCRIPTOR_TYPE);
    while let Some(off) = interface {
        let inum = buffer[off + 2] as i32;
        let alt = buffer[off + 3] as i32;
        let cls = buffer[off + 5] as i32;
        let sub = buffer[off + 6] as i32;
        let proto = buffer[off + 7] as i32;
        if (interface_number == -1 || inum == interface_number)
            && (alternate_setting == -1 || alt == alternate_setting)
            && (interface_class == -1 || cls == interface_class)
            && (interface_subclass == -1 || sub == interface_subclass)
            && (interface_protocol == -1 || proto == interface_protocol)
        {
            return Some(off);
        }
        interface = parse_descriptors(
            buffer,
            total_length,
            off + USB_INTERFACE_DESCRIPTOR_SIZE,
            USB_INTERFACE_DESCRIPTOR_TYPE,
        );
    }
    None
}

/// Finds an interface descriptor by number and alternate setting only.
///
/// Equivalent to [`parse_configuration_descriptor_ex`] with class, subclass,
/// and protocol set to -1 (wildcard).
pub fn parse_configuration_descriptor(
    buffer: &[u8],
    interface_number: u8,
    alternate_setting: u8,
) -> Option<usize> {
    parse_configuration_descriptor_ex(
        buffer,
        0,
        interface_number as i32,
        alternate_setting as i32,
        -1,
        -1,
        -1,
    )
}

/// Computes the total length of an interface descriptor and all following
/// descriptors up to (but not including) the next interface descriptor or
/// `buffer_end`.
///
/// Returns the sum of the `bLength` of the interface descriptor and of all
/// subsequent non-interface descriptors before `buffer_end`.
pub fn get_interface_length(buffer: &[u8], interface_offset: usize, buffer_end: usize) -> usize {
    let mut total = buffer[interface_offset] as usize;
    let mut common = interface_offset + USB_INTERFACE_DESCRIPTOR_SIZE;
    while common + USB_COMMON_DESCRIPTOR_SIZE <= buffer_end
        && buffer[common + 1] as i32 != USB_INTERFACE_DESCRIPTOR_TYPE
    {
        let b_length = buffer[common] as usize;
        total += b_length;
        if b_length == 0 {
            break;
        }
        common += b_length;
    }
    total
}

/// Validates a USB configuration descriptor.
///
/// Returns [`USBD_STATUS_SUCCESS`] when the descriptor passes the basic checks,
/// otherwise [`USBD_STATUS_ERROR`].
///
/// The checks are:
/// - The buffer is at least [`USB_CONFIGURATION_DESCRIPTOR_SIZE`] bytes.
/// - The descriptor's own `bLength` is at least [`USB_CONFIGURATION_DESCRIPTOR_SIZE`].
/// - The `wTotalLength` field is at least `bNumInterfaces * USB_CONFIGURATION_DESCRIPTOR_SIZE`.
///
/// The `level` and `tag` parameters are accepted for signature compatibility but
/// are unused (the Wine implementation is a partial stub).
pub fn validate_configuration_descriptor(buffer: &[u8], _level: u16, _tag: u32) -> i32 {
    if buffer.len() < USB_CONFIGURATION_DESCRIPTOR_SIZE {
        return USBD_STATUS_ERROR;
    }
    if buffer[0] < USB_CONFIGURATION_DESCRIPTOR_SIZE as u8 {
        return USBD_STATUS_ERROR;
    }
    let w_total_length = u16::from_le_bytes([buffer[2], buffer[3]]) as u32;
    let b_num_interfaces = buffer[4] as u32;
    if w_total_length < b_num_interfaces * (USB_CONFIGURATION_DESCRIPTOR_SIZE as u32) {
        return USBD_STATUS_ERROR;
    }
    USBD_STATUS_SUCCESS
}

/// Returns the USBDI version information.
///
/// Emulates Windows 2000: `USBDI_Version = 0x300`, `Supported_USB_Version = 0x200`.
pub fn get_usbd_i_version() -> UsbdVersion {
    UsbdVersion {
        usbdi_version: 0x300,
        supported_usb_version: 0x200,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
