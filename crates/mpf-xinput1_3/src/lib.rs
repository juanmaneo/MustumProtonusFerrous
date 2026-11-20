//! Pure keystroke logic from Wine's `dlls/xinput1_3/main.c`.
//!
//! `keystroke_step` is the edge detector behind `XInputGetKeystroke`: it compares the current
//! gamepad state with the last reported one and returns at most one keystroke. The axis scaling
//! (`axis`), joystick-as-keyboard mapping (`joystick`) and button/trigger edges (`buttons`) are
//! ported as separate pure functions. Controller I/O, HID report parsing, the update thread, the
//! critical sections and the static last-state storage stay in Wine.

pub mod axis;
pub mod buttons;
pub mod joystick;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `XINPUT_KEYSTROKE_KEYDOWN`.
pub const KEYSTROKE_KEYDOWN: u16 = 0x0001;
/// `XINPUT_KEYSTROKE_KEYUP`.
pub const KEYSTROKE_KEYUP: u16 = 0x0002;

/// A stick position as `(sThumbLX, sThumbLY)` or `(sThumbRX, sThumbRY)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Thumb {
    pub x: i16,
    pub y: i16,
}

/// The fields of `XINPUT_GAMEPAD` that the keystroke logic reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gamepad {
    pub buttons: u16,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub left_thumb: Thumb,
    pub right_thumb: Thumb,
}

/// `XINPUT_KEYSTROKE` without `Unicode` and `HidCode`, which Wine always zeroes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keystroke {
    pub virtual_key: u16,
    pub flags: u16,
    pub user_index: u8,
}

impl Keystroke {
    fn new(virtual_key: u16, pressed: bool, user_index: u8) -> Self {
        let flags = if pressed {
            KEYSTROKE_KEYDOWN
        } else {
            KEYSTROKE_KEYUP
        };
        Self {
            virtual_key,
            flags,
            user_index,
        }
    }
}

/// Port of the edge detection in `check_for_keystroke`. `last` plays the role of the C
/// `last_state[index]` and is updated the same way. Returns `None` where Wine returns
/// `ERROR_EMPTY`, and `Some` where it returns `ERROR_SUCCESS`.
pub fn keystroke_step(index: u8, cur: &Gamepad, last: &mut Gamepad) -> Option<Keystroke> {
    buttons::digital_step(index, cur, last)
        .or_else(|| {
            joystick::joystick_step(
                index,
                joystick::VK_PAD_LTHUMB_UP,
                cur.left_thumb,
                &mut last.left_thumb,
            )
        })
        .or_else(|| {
            joystick::joystick_step(
                index,
                joystick::VK_PAD_RTHUMB_UP,
                cur.right_thumb,
                &mut last.right_thumb,
            )
        })
}
