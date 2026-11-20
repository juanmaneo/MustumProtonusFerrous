//! Joystick-as-keyboard mapping from `joystick_state`, `js_vk_offs` and
//! `check_joystick_keystroke` in Wine's `dlls/xinput1_3/main.c`.

use crate::{Keystroke, Thumb};

/// Base virtual key for the left stick, `VK_PAD_LTHUMB_UP`.
pub const VK_PAD_LTHUMB_UP: u16 = 0x5820;
/// Base virtual key for the right stick, `VK_PAD_RTHUMB_UP`.
pub const VK_PAD_RTHUMB_UP: u16 = 0x5830;

const THUMB_THRESHOLD: i16 = 20000;

/// One axis of a stick, quantised against the 20000 threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoyState {
    Off,
    Low,
    High,
}

/// Port of `joystick_state`.
pub fn joystick_state(value: i16) -> JoyState {
    if value > THUMB_THRESHOLD {
        JoyState::High
    } else if value < -THUMB_THRESHOLD {
        JoyState::Low
    } else {
        JoyState::Off
    }
}

/// Port of `js_vk_offs`: the direction offset from the base key, with 0 = up, 1 = down,
/// 2 = right, 3 = left, 4 = up-left, 5 = up-right, 6 = down-right, 7 = down-left. `(Off, Off)`
/// is never requested by the caller; it yields 2, as the C code does.
pub fn js_vk_offs(x: JoyState, y: JoyState) -> u16 {
    match y {
        JoyState::Off => {
            if x == JoyState::Low {
                3
            } else {
                2
            }
        }
        JoyState::High => match x {
            JoyState::Off => 0,
            JoyState::Low => 4,
            JoyState::High => 5,
        },
        JoyState::Low => match x {
            JoyState::Off => 1,
            JoyState::Low => 7,
            JoyState::High => 6,
        },
    }
}

/// Port of `check_joystick_keystroke`. `last` is the stick position from the previous call and
/// is updated in place. A change from a held direction sends a key-up and recentres `last`, so
/// the matching key-down follows on the next call.
pub fn joystick_step(index: u8, base_vk: u16, cur: Thumb, last: &mut Thumb) -> Option<Keystroke> {
    let cur_vk = direction_key(base_vk, cur);
    let last_vk = direction_key(base_vk, *last);

    if cur_vk != last_vk {
        if last_vk != 0 {
            *last = Thumb::default();
            return Some(Keystroke::new(last_vk, false, index));
        }
        *last = cur;
        return Some(Keystroke::new(cur_vk, true, index));
    }

    *last = cur;
    None
}

fn direction_key(base_vk: u16, thumb: Thumb) -> u16 {
    let x = joystick_state(thumb.x);
    let y = joystick_state(thumb.y);
    if x == JoyState::Off && y == JoyState::Off {
        0
    } else {
        base_vk + js_vk_offs(x, y)
    }
}
