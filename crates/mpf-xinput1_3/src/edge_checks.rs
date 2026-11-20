use crate::axis::{mul_div, scale_value, sign_extend, ValueCaps};
use crate::buttons::{digital_step, trigger_is_on};
use crate::joystick::{joystick_state, joystick_step, js_vk_offs, JoyState};
use crate::{keystroke_step, Gamepad, Thumb, KEYSTROKE_KEYDOWN, KEYSTROKE_KEYUP};

#[test]
fn axis_sign_extension_handles_documented_edges() {
    let signed = ValueCaps {
        bit_size: 8,
        logical_min: -128,
        logical_max: 127,
    };
    assert_eq!(sign_extend(0x80, signed), -128);
    assert_eq!(sign_extend(0x7f, signed), 127);
    assert_eq!(
        sign_extend(
            1,
            ValueCaps {
                bit_size: 1,
                logical_min: -1,
                logical_max: 0
            }
        ),
        1
    );
    assert_eq!(
        sign_extend(
            9,
            ValueCaps {
                bit_size: 0,
                logical_min: -1,
                logical_max: 1
            }
        ),
        9
    );
    assert_eq!(
        sign_extend(
            9,
            ValueCaps {
                bit_size: 33,
                logical_min: -1,
                logical_max: 1
            }
        ),
        9
    );
    assert_eq!(
        sign_extend(
            0x8000_0000,
            ValueCaps {
                bit_size: 32,
                logical_min: i32::MIN,
                logical_max: i32::MAX
            }
        ),
        i32::MIN
    );
    assert_eq!(
        sign_extend(
            255,
            ValueCaps {
                bit_size: 8,
                logical_min: 0,
                logical_max: 255
            }
        ),
        255
    );
}

#[test]
fn scaling_rejects_ranges_and_handles_zero_width() {
    let range = ValueCaps {
        bit_size: 8,
        logical_min: 0,
        logical_max: 255,
    };
    assert_eq!(scale_value(0, range, 0, 255), 0);
    assert_eq!(scale_value(255, range, 0, 255), 255);
    assert_eq!(scale_value(256, range, 0, 255), 0);
    assert_eq!(scale_value(0, range, 0, 255), 0);
    assert_eq!(
        scale_value(
            5,
            ValueCaps {
                bit_size: 8,
                logical_min: 10,
                logical_max: 0
            },
            0,
            255
        ),
        0
    );
    assert_eq!(
        scale_value(
            5,
            ValueCaps {
                bit_size: 8,
                logical_min: 5,
                logical_max: 5
            },
            12,
            20
        ),
        11
    );
    assert_eq!(
        scale_value(
            0x80,
            ValueCaps {
                bit_size: 8,
                logical_min: -128,
                logical_max: 127
            },
            -32768,
            32767
        ),
        -32768
    );
}

#[test]
fn mul_div_covers_rounding_divisor_and_overflow_paths() {
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(-1, 1, 2), -1);
    assert_eq!(mul_div(-1, -1, 2), 1);
    assert_eq!(mul_div(7, 9, -2), -32);
    assert_eq!(mul_div(1, 2, 0), -1);
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
}

#[test]
fn trigger_threshold_is_strict() {
    assert!(!trigger_is_on(0));
    assert!(!trigger_is_on(30));
    assert!(trigger_is_on(31));
    assert!(trigger_is_on(u8::MAX));
}

#[test]
fn every_joystick_direction_and_threshold_is_mapped() {
    assert_eq!(joystick_state(20000), JoyState::Off);
    assert_eq!(joystick_state(20001), JoyState::High);
    assert_eq!(joystick_state(-20000), JoyState::Off);
    assert_eq!(joystick_state(-20001), JoyState::Low);
    assert_eq!(
        [
            js_vk_offs(JoyState::Off, JoyState::High),
            js_vk_offs(JoyState::Off, JoyState::Low),
            js_vk_offs(JoyState::High, JoyState::Off),
            js_vk_offs(JoyState::Low, JoyState::Off),
            js_vk_offs(JoyState::Low, JoyState::High),
            js_vk_offs(JoyState::High, JoyState::High),
            js_vk_offs(JoyState::High, JoyState::Low),
            js_vk_offs(JoyState::Low, JoyState::Low),
            js_vk_offs(JoyState::Off, JoyState::Off),
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 2]
    );
}

#[test]
fn joystick_changes_emit_up_then_down_and_track_quantised_sticks() {
    let mut last = Thumb::default();
    let right = joystick_step(1, 0x5820, Thumb { x: 30000, y: 0 }, &mut last).unwrap();
    assert_eq!(
        (right.virtual_key, right.flags, right.user_index),
        (0x5822, KEYSTROKE_KEYDOWN, 1)
    );
    let same_sector = joystick_step(1, 0x5820, Thumb { x: 31000, y: 0 }, &mut last);
    assert_eq!(same_sector, None);
    assert_eq!(last, Thumb { x: 31000, y: 0 });
    let up = joystick_step(1, 0x5820, Thumb { x: 0, y: 30000 }, &mut last).unwrap();
    assert_eq!((up.virtual_key, up.flags), (0x5822, KEYSTROKE_KEYUP));
    assert_eq!(last, Thumb::default());
    let next_down = joystick_step(1, 0x5820, Thumb { x: 0, y: 30000 }, &mut last).unwrap();
    assert_eq!(
        (next_down.virtual_key, next_down.flags),
        (0x5820, KEYSTROKE_KEYDOWN)
    );
    let neutral = joystick_step(1, 0x5820, Thumb { x: 0, y: 1000 }, &mut last).unwrap();
    assert_eq!(
        (neutral.virtual_key, neutral.flags),
        (0x5820, KEYSTROKE_KEYUP)
    );
    assert_eq!(last, Thumb::default());
    assert_eq!(
        joystick_step(1, 0x5820, Thumb { x: 0, y: 1000 }, &mut last),
        None
    );
    assert_eq!(last, Thumb { x: 0, y: 1000 });
}

#[test]
fn digital_button_table_priority_and_guide_button_behavior() {
    let keys = [
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
    for (mask, virtual_key) in keys {
        let cur = Gamepad {
            buttons: mask,
            ..Gamepad::default()
        };
        let mut last = Gamepad::default();
        let event = digital_step(4, &cur, &mut last).unwrap();
        assert_eq!(
            (event.virtual_key, event.flags, event.user_index),
            (virtual_key, KEYSTROKE_KEYDOWN, 4)
        );
        assert_eq!(last.buttons, mask);
    }

    let cur = Gamepad {
        buttons: 0x0003,
        ..Gamepad::default()
    };
    let mut last = Gamepad::default();
    let event = digital_step(0, &cur, &mut last).unwrap();
    assert_eq!(event.virtual_key, 0x5810);
    assert_eq!(last.buttons, 1);

    let guide = Gamepad {
        buttons: 0x0400,
        ..Gamepad::default()
    };
    let mut last = Gamepad::default();
    assert_eq!(digital_step(0, &guide, &mut last), None);
    assert_eq!(last.buttons, 0);
    assert_eq!(digital_step(0, &Gamepad::default(), &mut last), None);
}

#[test]
fn trigger_edges_follow_buttons_and_update_raw_state_only_on_edges() {
    let mut last = Gamepad::default();
    let cur = Gamepad {
        left_trigger: 31,
        right_trigger: 31,
        ..Gamepad::default()
    };
    let left_down = digital_step(0, &cur, &mut last).unwrap();
    assert_eq!(
        (left_down.virtual_key, left_down.flags),
        (0x5806, KEYSTROKE_KEYDOWN)
    );
    assert_eq!(last.left_trigger, 31);
    let right_down = digital_step(0, &cur, &mut last).unwrap();
    assert_eq!(
        (right_down.virtual_key, right_down.flags),
        (0x5807, KEYSTROKE_KEYDOWN)
    );
    assert_eq!(last.right_trigger, 31);

    let changed_raw = Gamepad {
        left_trigger: 200,
        right_trigger: 200,
        ..Gamepad::default()
    };
    assert_eq!(digital_step(0, &changed_raw, &mut last), None);
    assert_eq!(last.left_trigger, 31);
    assert_eq!(last.right_trigger, 31);

    let released = Gamepad {
        left_trigger: 30,
        right_trigger: 30,
        ..Gamepad::default()
    };
    let left_up = digital_step(0, &released, &mut last).unwrap();
    assert_eq!(
        (left_up.virtual_key, left_up.flags),
        (0x5806, KEYSTROKE_KEYUP)
    );
    assert_eq!(last.left_trigger, 30);
    let right_up = digital_step(0, &released, &mut last).unwrap();
    assert_eq!(
        (right_up.virtual_key, right_up.flags),
        (0x5807, KEYSTROKE_KEYUP)
    );
    assert_eq!(last.right_trigger, 30);
    assert_eq!(digital_step(0, &released, &mut last), None);
}

#[test]
fn keystroke_step_checks_buttons_then_left_then_right_stick() {
    let mut last = Gamepad::default();
    let button = Gamepad {
        buttons: 1,
        ..Gamepad::default()
    };
    let event = keystroke_step(2, &button, &mut last).unwrap();
    assert_eq!(
        (event.virtual_key, event.flags),
        (0x5810, KEYSTROKE_KEYDOWN)
    );

    last = Gamepad::default();
    let left = Gamepad {
        left_thumb: Thumb { x: 0, y: 30000 },
        ..Gamepad::default()
    };
    let event = keystroke_step(2, &left, &mut last).unwrap();
    assert_eq!(
        (event.virtual_key, event.flags),
        (0x5820, KEYSTROKE_KEYDOWN)
    );

    last = Gamepad::default();
    let right = Gamepad {
        right_thumb: Thumb { x: 30000, y: 0 },
        ..Gamepad::default()
    };
    let event = keystroke_step(2, &right, &mut last).unwrap();
    assert_eq!(
        (event.virtual_key, event.flags),
        (0x5832, KEYSTROKE_KEYDOWN)
    );
    assert_eq!(
        keystroke_step(2, &Gamepad::default(), &mut Gamepad::default()),
        None
    );
}
