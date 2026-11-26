/// `FILE_DEVICE_KEYBOARD` from `include/winioctl.h`, the device type of every HID control code.
pub const FILE_DEVICE_KEYBOARD: u32 = 0x0000_000b;
/// `METHOD_BUFFERED` from `include/winioctl.h`.
pub const METHOD_BUFFERED: u32 = 0;
/// `METHOD_IN_DIRECT` from `include/winioctl.h`.
pub const METHOD_IN_DIRECT: u32 = 1;
/// `METHOD_OUT_DIRECT` from `include/winioctl.h`.
pub const METHOD_OUT_DIRECT: u32 = 2;
/// `METHOD_NEITHER` from `include/winioctl.h`.
pub const METHOD_NEITHER: u32 = 3;
/// `FILE_ANY_ACCESS` from `include/winioctl.h`.
pub const FILE_ANY_ACCESS: u32 = 0;

/// `CTL_CODE` from `include/winioctl.h`. Like the macro, the fields are not masked.
pub const fn ctl_code(device_type: u32, function: u32, method: u32, access: u32) -> u32 {
    (device_type << 16) | (access << 14) | (function << 2) | method
}

/// `HID_CTL_CODE` from `include/ddk/hidclass.h`.
pub const fn hid_ctl_code(id: u32) -> u32 {
    ctl_code(FILE_DEVICE_KEYBOARD, id, METHOD_NEITHER, FILE_ANY_ACCESS)
}

/// `HID_BUFFER_CTL_CODE` from `include/ddk/hidclass.h`.
pub const fn hid_buffer_ctl_code(id: u32) -> u32 {
    ctl_code(FILE_DEVICE_KEYBOARD, id, METHOD_BUFFERED, FILE_ANY_ACCESS)
}

/// `HID_IN_CTL_CODE` from `include/ddk/hidclass.h`.
pub const fn hid_in_ctl_code(id: u32) -> u32 {
    ctl_code(FILE_DEVICE_KEYBOARD, id, METHOD_IN_DIRECT, FILE_ANY_ACCESS)
}

/// `HID_OUT_CTL_CODE` from `include/ddk/hidclass.h`.
pub const fn hid_out_ctl_code(id: u32) -> u32 {
    ctl_code(FILE_DEVICE_KEYBOARD, id, METHOD_OUT_DIRECT, FILE_ANY_ACCESS)
}

/// The IOCTLs `internal_ioctl` forwards to the lower driver, in the order of its `case` labels:
/// `IOCTL_GET_PHYSICAL_DESCRIPTOR`, `IOCTL_HID_ACTIVATE_DEVICE`, `IOCTL_HID_DEACTIVATE_DEVICE`,
/// `IOCTL_HID_GET_INDEXED_STRING`, `IOCTL_HID_GET_DEVICE_ATTRIBUTES`,
/// `IOCTL_HID_GET_DEVICE_DESCRIPTOR`, `IOCTL_HID_GET_REPORT_DESCRIPTOR`, `IOCTL_HID_GET_STRING`,
/// `IOCTL_HID_GET_INPUT_REPORT`, `IOCTL_HID_READ_REPORT`, `IOCTL_HID_SET_OUTPUT_REPORT`,
/// `IOCTL_HID_WRITE_REPORT`, `IOCTL_HID_GET_FEATURE`, `IOCTL_HID_SET_FEATURE`.
pub const PASS_THROUGH_IOCTLS: [u32; 14] = [
    hid_out_ctl_code(102),
    hid_ctl_code(7),
    hid_ctl_code(8),
    hid_out_ctl_code(120),
    hid_ctl_code(9),
    hid_ctl_code(0),
    hid_ctl_code(1),
    hid_ctl_code(4),
    hid_out_ctl_code(104),
    hid_ctl_code(2),
    hid_in_ctl_code(101),
    hid_ctl_code(3),
    hid_out_ctl_code(100),
    hid_in_ctl_code(100),
];

/// The fields of an IOCTL code, as the `FIXME` in `internal_ioctl` prints them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IoctlFields {
    pub device: u32,
    pub access: u32,
    pub function: u32,
    pub method: u32,
}

/// Splits an IOCTL code into the fields the `FIXME` prints.
pub const fn ioctl_fields(code: u32) -> IoctlFields {
    IoctlFields {
        device: code >> 16,
        access: (code >> 14) & 3,
        function: (code >> 2) & 0xfff,
        method: code & 3,
    }
}

/// The `FIXME` text `internal_ioctl` prints for an unsupported code, without the trailing newline.
pub fn unsupported_ioctl_message(code: u32) -> String {
    let fields = ioctl_fields(code);
    format!(
        "Unsupported ioctl {} (device={:x} access={:x} func={:x} method={:x})",
        c_alt_hex(code),
        fields.device,
        fields.access,
        fields.function,
        fields.method
    )
}

/// C's `%#lx`, which prints 0 as `0` and any other value with a `0x` prefix.
fn c_alt_hex(value: u32) -> String {
    if value == 0 {
        "0".to_string()
    } else {
        format!("{value:#x}")
    }
}
