//! Button and trigger edge detection from `check_for_keystroke` in Wine's
//! `dlls/xinput1_3/main.c`.

use crate::{Gamepad, Keystroke};

const VK_PAD_LTRIGGER: u16 = 0x5806;
const VK_PAD_RTRIGGER: u16 = 0x5807;

/// Button mask and virtual key, in the order Wine checks them. The guide button sends no event.
const BUTTONS: [(u16, u16); 14] = [
    (0x0001, 0x5810),
    (0x0002, 0x5811),
    (0x0004, 0x5812),
    (0x0008, 0x5813),
    (0x0010, 0x5814),
    (0x0020, 0x5815),
    (0x0040, 0x5816),
    (0x0080, 0x5817),
    (0x0100, 0x5805),
    (0x0200, 0x5804),
    (0x1000, 0x5800),
    (0x2000, 0x5801),
    (0x4000, 0x5802),
    (0x8000, 0x5803),
];

/// Port of `trigger_is_on`: a trigger counts as held above 30.
pub fn trigger_is_on(value: u8) -> bool {
    value > 30
}

/// Buttons first, then the two triggers. Returns the first change found, and updates `last`
/// for that change only, as the C code does.
pub fn digital_step(index: u8, cur: &Gamepad, last: &mut Gamepad) -> Option<Keystroke> {
    for (mask, virtual_key) in BUTTONS {
        if ((cur.buttons & mask) ^ (last.buttons & mask)) != 0 {
            let pressed = cur.buttons & mask != 0;
            if pressed {
                last.buttons |= mask;
            } else {
                last.buttons &= !mask;
            }
            return Some(Keystroke::new(virtual_key, pressed, index));
        }
    }

    if trigger_is_on(cur.left_trigger) != trigger_is_on(last.left_trigger) {
        last.left_trigger = cur.left_trigger;
        return Some(Keystroke::new(
            VK_PAD_LTRIGGER,
            trigger_is_on(cur.left_trigger),
            index,
        ));
    }

    if trigger_is_on(cur.right_trigger) != trigger_is_on(last.right_trigger) {
        last.right_trigger = cur.right_trigger;
        return Some(Keystroke::new(
            VK_PAD_RTRIGGER,
            trigger_is_on(cur.right_trigger),
            index,
        ));
    }

    None
}
