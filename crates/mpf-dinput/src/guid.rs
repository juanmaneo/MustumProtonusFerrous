//! The `GUID_*` values and the usage mappings of `dlls/dinput/joystick_hid.c`.
//!
//! The GUID values are copied from `include/dinput.h`.

/// A `GUID` with the layout of the Windows structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

const OBJECT_DATA4: [u8; 8] = [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00];
const EFFECT_DATA4: [u8; 8] = [0x9A, 0xD0, 0x00, 0xA0, 0xC9, 0xA0, 0x6E, 0x35];

/// `GUID_XAxis`.
pub const GUID_X_AXIS: Guid = Guid {
    data1: 0xA36D_02E0,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_YAxis`.
pub const GUID_Y_AXIS: Guid = Guid {
    data1: 0xA36D_02E1,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_ZAxis`.
pub const GUID_Z_AXIS: Guid = Guid {
    data1: 0xA36D_02E2,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_RxAxis`.
pub const GUID_RX_AXIS: Guid = Guid {
    data1: 0xA36D_02F4,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_RyAxis`.
pub const GUID_RY_AXIS: Guid = Guid {
    data1: 0xA36D_02F5,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_RzAxis`.
pub const GUID_RZ_AXIS: Guid = Guid {
    data1: 0xA36D_02E3,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_Slider`.
pub const GUID_SLIDER: Guid = Guid {
    data1: 0xA36D_02E4,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_Button`.
pub const GUID_BUTTON: Guid = Guid {
    data1: 0xA36D_02F0,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_POV`.
pub const GUID_POV: Guid = Guid {
    data1: 0xA36D_02F2,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};
/// `GUID_Unknown`.
pub const GUID_UNKNOWN: Guid = Guid {
    data1: 0xA36D_02F3,
    data2: 0xC9F3,
    data3: 0x11CF,
    data4: OBJECT_DATA4,
};

/// `GUID_CustomForce`.
pub const GUID_CUSTOM_FORCE: Guid = Guid {
    data1: 0x1354_1C2B,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_ConstantForce`.
pub const GUID_CONSTANT_FORCE: Guid = Guid {
    data1: 0x1354_1C20,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_RampForce`.
pub const GUID_RAMP_FORCE: Guid = Guid {
    data1: 0x1354_1C21,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Square`.
pub const GUID_SQUARE: Guid = Guid {
    data1: 0x1354_1C22,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Sine`.
pub const GUID_SINE: Guid = Guid {
    data1: 0x1354_1C23,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Triangle`.
pub const GUID_TRIANGLE: Guid = Guid {
    data1: 0x1354_1C24,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_SawtoothUp`.
pub const GUID_SAWTOOTH_UP: Guid = Guid {
    data1: 0x1354_1C25,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_SawtoothDown`.
pub const GUID_SAWTOOTH_DOWN: Guid = Guid {
    data1: 0x1354_1C26,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Spring`.
pub const GUID_SPRING: Guid = Guid {
    data1: 0x1354_1C27,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Damper`.
pub const GUID_DAMPER: Guid = Guid {
    data1: 0x1354_1C28,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Inertia`.
pub const GUID_INERTIA: Guid = Guid {
    data1: 0x1354_1C29,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};
/// `GUID_Friction`.
pub const GUID_FRICTION: Guid = Guid {
    data1: 0x1354_1C2A,
    data2: 0x8E33,
    data3: 0x11D0,
    data4: EFFECT_DATA4,
};

const PAGE_GENERIC: u16 = 0x01;
const PAGE_SIMULATION: u16 = 0x02;
const PAGE_BUTTON: u16 = 0x09;

const GENERIC_X: u16 = 0x30;
const GENERIC_Y: u16 = 0x31;
const GENERIC_Z: u16 = 0x32;
const GENERIC_RX: u16 = 0x33;
const GENERIC_RY: u16 = 0x34;
const GENERIC_RZ: u16 = 0x35;
const GENERIC_SLIDER: u16 = 0x36;
const GENERIC_DIAL: u16 = 0x37;
const GENERIC_WHEEL: u16 = 0x38;
const GENERIC_HATSWITCH: u16 = 0x39;

const SIMULATION_RUDDER: u16 = 0xBA;
const SIMULATION_THROTTLE: u16 = 0xBB;
const SIMULATION_ACCELERATOR: u16 = 0xC4;
const SIMULATION_BRAKE: u16 = 0xC5;
const SIMULATION_STEERING: u16 = 0xC8;

const PID_CONSTANT_FORCE: u16 = 0x26;
const PID_RAMP: u16 = 0x27;
const PID_CUSTOM_FORCE: u16 = 0x28;
const PID_SQUARE: u16 = 0x30;
const PID_SINE: u16 = 0x31;
const PID_TRIANGLE: u16 = 0x32;
const PID_SAWTOOTH_UP: u16 = 0x33;
const PID_SAWTOOTH_DOWN: u16 = 0x34;
const PID_SPRING: u16 = 0x40;
const PID_DAMPER: u16 = 0x41;
const PID_INERTIA: u16 = 0x42;
const PID_FRICTION: u16 = 0x43;

/// The PID effect usage for each effect GUID, in the order the C code checks them.
const EFFECT_USAGES: [(u16, Guid); 12] = [
    (PID_CUSTOM_FORCE, GUID_CUSTOM_FORCE),
    (PID_CONSTANT_FORCE, GUID_CONSTANT_FORCE),
    (PID_RAMP, GUID_RAMP_FORCE),
    (PID_SQUARE, GUID_SQUARE),
    (PID_SINE, GUID_SINE),
    (PID_TRIANGLE, GUID_TRIANGLE),
    (PID_SAWTOOTH_UP, GUID_SAWTOOTH_UP),
    (PID_SAWTOOTH_DOWN, GUID_SAWTOOTH_DOWN),
    (PID_SPRING, GUID_SPRING),
    (PID_DAMPER, GUID_DAMPER),
    (PID_INERTIA, GUID_INERTIA),
    (PID_FRICTION, GUID_FRICTION),
];

/// Maps a HID usage page and usage to the object GUID that DirectInput reports.
pub fn object_usage_to_guid(usage_page: u16, usage: u16) -> Guid {
    match (usage_page, usage) {
        (PAGE_BUTTON, _) => GUID_BUTTON,
        (PAGE_SIMULATION, SIMULATION_STEERING) => GUID_X_AXIS,
        (PAGE_SIMULATION, SIMULATION_ACCELERATOR) => GUID_Y_AXIS,
        (PAGE_SIMULATION, SIMULATION_BRAKE | SIMULATION_RUDDER) => GUID_RZ_AXIS,
        (PAGE_SIMULATION, SIMULATION_THROTTLE) => GUID_SLIDER,
        (PAGE_GENERIC, GENERIC_X) => GUID_X_AXIS,
        (PAGE_GENERIC, GENERIC_Y) => GUID_Y_AXIS,
        (PAGE_GENERIC, GENERIC_Z | GENERIC_WHEEL) => GUID_Z_AXIS,
        (PAGE_GENERIC, GENERIC_RX) => GUID_RX_AXIS,
        (PAGE_GENERIC, GENERIC_RY) => GUID_RY_AXIS,
        (PAGE_GENERIC, GENERIC_RZ) => GUID_RZ_AXIS,
        (PAGE_GENERIC, GENERIC_SLIDER | GENERIC_DIAL) => GUID_SLIDER,
        (PAGE_GENERIC, GENERIC_HATSWITCH) => GUID_POV,
        _ => GUID_UNKNOWN,
    }
}

/// Maps an effect GUID to its PID effect usage, or 0 when the GUID is not a PID effect.
pub fn effect_guid_to_usage(guid: &Guid) -> u16 {
    EFFECT_USAGES
        .iter()
        .find(|(_, candidate)| candidate == guid)
        .map_or(0, |&(usage, _)| usage)
}

/// Maps a PID effect usage to its effect GUID, or `GUID_UNKNOWN` for an unknown usage.
pub fn effect_usage_to_guid(usage: u16) -> Guid {
    EFFECT_USAGES
        .iter()
        .find(|(candidate, _)| *candidate == usage)
        .map_or(GUID_UNKNOWN, |&(_, guid)| guid)
}
