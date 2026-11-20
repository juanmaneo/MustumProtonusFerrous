/// `FF_PERIODIC` from `linux/input.h`.
pub const FF_PERIODIC: u16 = 0x51;
/// `FF_CONSTANT` from `linux/input.h`.
pub const FF_CONSTANT: u16 = 0x52;
/// `FF_SPRING` from `linux/input.h`.
pub const FF_SPRING: u16 = 0x53;
/// `FF_FRICTION` from `linux/input.h`.
pub const FF_FRICTION: u16 = 0x54;
/// `FF_DAMPER` from `linux/input.h`.
pub const FF_DAMPER: u16 = 0x55;
/// `FF_INERTIA` from `linux/input.h`.
pub const FF_INERTIA: u16 = 0x56;
/// `FF_RAMP` from `linux/input.h`.
pub const FF_RAMP: u16 = 0x57;
/// `FF_SQUARE` from `linux/input.h`.
pub const FF_SQUARE: u16 = 0x58;
/// `FF_TRIANGLE` from `linux/input.h`.
pub const FF_TRIANGLE: u16 = 0x59;
/// `FF_SINE` from `linux/input.h`.
pub const FF_SINE: u16 = 0x5a;
/// `FF_SAW_UP` from `linux/input.h`.
pub const FF_SAW_UP: u16 = 0x5b;
/// `FF_SAW_DOWN` from `linux/input.h`.
pub const FF_SAW_DOWN: u16 = 0x5c;
/// `FF_CUSTOM` from `linux/input.h`.
pub const FF_CUSTOM: u16 = 0x5d;

const PID_USAGE_ET_CONSTANT_FORCE: u16 = 0x26;
const PID_USAGE_ET_RAMP: u16 = 0x27;
const PID_USAGE_ET_CUSTOM_FORCE_DATA: u16 = 0x28;
const PID_USAGE_ET_SQUARE: u16 = 0x30;
const PID_USAGE_ET_SINE: u16 = 0x31;
const PID_USAGE_ET_TRIANGLE: u16 = 0x32;
const PID_USAGE_ET_SAWTOOTH_UP: u16 = 0x33;
const PID_USAGE_ET_SAWTOOTH_DOWN: u16 = 0x34;
const PID_USAGE_ET_SPRING: u16 = 0x40;
const PID_USAGE_ET_DAMPER: u16 = 0x41;
const PID_USAGE_ET_INERTIA: u16 = 0x42;
const PID_USAGE_ET_FRICTION: u16 = 0x43;

/// The force-feedback effect type that a PID effect usage maps to.
///
/// `waveform` is set only for `FF_PERIODIC` effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FfEffectType {
    pub effect_type: u16,
    pub waveform: Option<u16>,
}

fn periodic(waveform: u16) -> FfEffectType {
    FfEffectType {
        effect_type: FF_PERIODIC,
        waveform: Some(waveform),
    }
}

fn plain(effect_type: u16) -> FfEffectType {
    FfEffectType {
        effect_type,
        waveform: None,
    }
}

/// `set_effect_type_from_usage`. Returns `None` for an unsupported usage
/// (`STATUS_NOT_SUPPORTED` in C).
pub fn ff_effect_type_from_usage(usage: u16) -> Option<FfEffectType> {
    Some(match usage {
        PID_USAGE_ET_SINE => periodic(FF_SINE),
        PID_USAGE_ET_SQUARE => periodic(FF_SQUARE),
        PID_USAGE_ET_TRIANGLE => periodic(FF_TRIANGLE),
        PID_USAGE_ET_SAWTOOTH_UP => periodic(FF_SAW_UP),
        PID_USAGE_ET_SAWTOOTH_DOWN => periodic(FF_SAW_DOWN),
        PID_USAGE_ET_SPRING => plain(FF_SPRING),
        PID_USAGE_ET_DAMPER => plain(FF_DAMPER),
        PID_USAGE_ET_INERTIA => plain(FF_INERTIA),
        PID_USAGE_ET_FRICTION => plain(FF_FRICTION),
        PID_USAGE_ET_CONSTANT_FORCE => plain(FF_CONSTANT),
        PID_USAGE_ET_RAMP => plain(FF_RAMP),
        PID_USAGE_ET_CUSTOM_FORCE_DATA => plain(FF_CUSTOM),
        _ => return None,
    })
}
