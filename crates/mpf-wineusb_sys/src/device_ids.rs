//! The ID strings of a USB device, as `query_id` builds them.

/// The fields of `struct usb_device` that the ID strings are made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbDevice {
    pub vendor: u16,
    pub product: u16,
    pub revision: u16,
    pub usbver: u16,
    pub class: u8,
    pub subclass: u8,
    pub protocol: u8,
    pub busnum: u8,
    pub portnum: u8,
    pub interface: bool,
    pub interface_index: i16,
}

/// `BUS_QUERY_DEVICE_ID`.
const BUS_QUERY_DEVICE_ID: u32 = 0;
/// `BUS_QUERY_HARDWARE_IDS`.
const BUS_QUERY_HARDWARE_IDS: u32 = 1;
/// `BUS_QUERY_COMPATIBLE_IDS`.
const BUS_QUERY_COMPATIBLE_IDS: u32 = 2;
/// `BUS_QUERY_INSTANCE_ID`.
const BUS_QUERY_INSTANCE_ID: u32 = 3;

/// `query_id`: the ID strings of `device` for `query_type`, or `None` when the
/// type is not handled. Each string is NUL-terminated. The hardware and
/// compatible lists end with an empty string.
pub fn query_id(device: &UsbDevice, query_type: u32) -> Option<Vec<u16>> {
    let mut buffer = Vec::new();
    match query_type {
        BUS_QUERY_DEVICE_ID => append(&mut buffer, &device_id(device)),
        BUS_QUERY_INSTANCE_ID => append(&mut buffer, &instance_id(device)),
        BUS_QUERY_HARDWARE_IDS => hardware_ids(device, &mut buffer),
        BUS_QUERY_COMPATIBLE_IDS => compatible_ids(device, &mut buffer),
        _ => return None,
    }
    Some(buffer)
}

/// Appends `id` and its terminating NUL, as `append_id` does.
fn append(buffer: &mut Vec<u16>, id: &str) {
    buffer.extend(id.encode_utf16());
    buffer.push(0);
}

/// The `&MI_xx` suffix of an interface. The index is printed as the C `int` it
/// is promoted to, so a negative index shows as 32 bits.
fn interface_suffix(device: &UsbDevice) -> String {
    if device.interface {
        format!("&MI_{:02X}", device.interface_index as i32 as u32)
    } else {
        String::new()
    }
}

fn device_id(device: &UsbDevice) -> String {
    format!(
        "USB\\VID_{:04X}&PID_{:04X}{}",
        device.vendor,
        device.product,
        interface_suffix(device)
    )
}

fn instance_id(device: &UsbDevice) -> String {
    format!(
        "{}&{}&{}&{}",
        device.usbver, device.revision, device.busnum, device.portnum
    )
}

fn hardware_ids(device: &UsbDevice, buffer: &mut Vec<u16>) {
    append(
        buffer,
        &format!(
            "USB\\VID_{:04X}&PID_{:04X}&REV_{:04X}{}",
            device.vendor,
            device.product,
            device.revision,
            interface_suffix(device)
        ),
    );
    append(buffer, &device_id(device));
    append(buffer, "");
}

fn compatible_ids(device: &UsbDevice, buffer: &mut Vec<u16>) {
    let kind = if device.interface_index != -1 {
        "Class"
    } else {
        "DevClass"
    };
    let (class, subclass, protocol) = (device.class, device.subclass, device.protocol);
    append(
        buffer,
        &format!("USB\\{kind}_{class:02x}&SubClass_{subclass:02x}&Prot_{protocol:02x}"),
    );
    append(
        buffer,
        &format!("USB\\{kind}_{class:02x}&SubClass_{subclass:02x}"),
    );
    append(buffer, &format!("USB\\{kind}_{class:02x}"));
    append(buffer, "");
}
