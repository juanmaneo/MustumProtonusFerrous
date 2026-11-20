//! The device type that `device_type_for_version` reports to pre-0x0800 callers, from
//! `dlls/dinput/joystick_hid.c`.

const DIDEVTYPE_DEVICE: u32 = 1;
const DIDEVTYPE_JOYSTICK: u32 = 4;
const DIDEVTYPE_HID: u32 = 0x0001_0000;

const DIDEVTYPEJOYSTICK_TRADITIONAL: u32 = 2;
const DIDEVTYPEJOYSTICK_FLIGHTSTICK: u32 = 3;
const DIDEVTYPEJOYSTICK_GAMEPAD: u32 = 4;
const DIDEVTYPEJOYSTICK_RUDDER: u32 = 5;
const DIDEVTYPEJOYSTICK_WHEEL: u32 = 6;
const DIDEVTYPEJOYSTICK_HEADTRACKER: u32 = 7;
const DIDEVTYPEJOYSTICK_UNKNOWN: u32 = 1;

const DI8DEVTYPE_JOYSTICK: u32 = 0x14;
const DI8DEVTYPE_GAMEPAD: u32 = 0x15;
const DI8DEVTYPE_DRIVING: u32 = 0x16;
const DI8DEVTYPE_FLIGHT: u32 = 0x17;
const DI8DEVTYPE_1STPERSON: u32 = 0x18;
const DI8DEVTYPE_SUPPLEMENTAL: u32 = 0x1c;

const DI8DEVTYPEJOYSTICK_LIMITED: u32 = 1;
const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: u32 = 4;
const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: u32 = 13;

/// Returns the device type for `version`. From 0x0800 on, the type is unchanged.
/// Before that, the DirectInput 8 type maps onto a joystick subtype.
pub fn device_type_for_version(device_type: u32, version: u32) -> u32 {
    if version >= 0x0800 {
        return device_type;
    }
    let kind = device_type & 0xff;
    let subtype = (device_type >> 8) & 0xff;
    match kind {
        DI8DEVTYPE_JOYSTICK => {
            if subtype == DI8DEVTYPEJOYSTICK_LIMITED {
                joystick(DIDEVTYPEJOYSTICK_UNKNOWN)
            } else {
                joystick(DIDEVTYPEJOYSTICK_TRADITIONAL)
            }
        }
        DI8DEVTYPE_GAMEPAD => joystick(DIDEVTYPEJOYSTICK_GAMEPAD),
        DI8DEVTYPE_DRIVING => joystick(DIDEVTYPEJOYSTICK_WHEEL),
        DI8DEVTYPE_FLIGHT => joystick(DIDEVTYPEJOYSTICK_FLIGHTSTICK),
        DI8DEVTYPE_SUPPLEMENTAL => match subtype {
            DI8DEVTYPESUPPLEMENTAL_HEADTRACKER => joystick(DIDEVTYPEJOYSTICK_HEADTRACKER),
            DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS => joystick(DIDEVTYPEJOYSTICK_RUDDER),
            _ => joystick(DIDEVTYPEJOYSTICK_UNKNOWN),
        },
        DI8DEVTYPE_1STPERSON => joystick(DIDEVTYPEJOYSTICK_UNKNOWN),
        _ => DIDEVTYPE_DEVICE | DIDEVTYPE_HID,
    }
}

fn joystick(subtype: u32) -> u32 {
    DIDEVTYPE_JOYSTICK | (subtype << 8) | DIDEVTYPE_HID
}
