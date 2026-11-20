//! Twenty-five rows captured from the ASan/UBSan Wine C oracle under
//! `target/canon/xinput1_3/oracle.c`. The oracle uses the portable routines from
//! Wine's xinput1_3 `main.c` and `kernelbase`'s `MulDiv`.

use crate::axis::{mul_div, scale_value, sign_extend, ValueCaps};
use crate::buttons::{digital_step, trigger_is_on};
use crate::joystick::{joystick_state, joystick_step, js_vk_offs, JoyState};
use crate::{Gamepad, Thumb};

const SIGNED_BYTE: ValueCaps = ValueCaps {
    bit_size: 8,
    logical_min: -128,
    logical_max: 127,
};
const BYTE_RANGE: ValueCaps = ValueCaps {
    bit_size: 8,
    logical_min: 0,
    logical_max: 255,
};
const INVERTED: ValueCaps = ValueCaps {
    bit_size: 8,
    logical_min: 10,
    logical_max: 0,
};

/// Format a C-oracle row's corresponding Rust result.
fn run(row: &str) -> String {
    match row {
        "sign_7f" => sign_extend(0x7f, SIGNED_BYTE).to_string(),
        "sign_80" => sign_extend(0x80, SIGNED_BYTE).to_string(),
        "sign_ff" => sign_extend(0xff, SIGNED_BYTE).to_string(),
        "sign_unsigned" => sign_extend(255, BYTE_RANGE).to_string(),
        "axis_min" => scale_value(0, BYTE_RANGE, -32768, 32767).to_string(),
        "axis_midlow" => scale_value(127, BYTE_RANGE, -32768, 32767).to_string(),
        "axis_midhi" => scale_value(128, BYTE_RANGE, -32768, 32767).to_string(),
        "axis_max" => scale_value(255, BYTE_RANGE, -32768, 32767).to_string(),
        "axis_outside" => scale_value(256, BYTE_RANGE, -32768, 32767).to_string(),
        "axis_inverted" => scale_value(5, INVERTED, -32768, 32767).to_string(),
        "muldiv_pos_half" => mul_div(1, 1, 2).to_string(),
        "muldiv_neg_half" => mul_div(-1, 1, 2).to_string(),
        "muldiv_neg_div" => mul_div(7, 9, -2).to_string(),
        "muldiv_overflow" => mul_div(i32::MAX, 2, 1).to_string(),
        "joy_threshold_hi" => joy_state_number(joystick_state(20001)).to_string(),
        "joy_below_lo" => joy_state_number(joystick_state(-20001)).to_string(),
        "offs_up_left" => js_vk_offs(JoyState::Low, JoyState::High).to_string(),
        "joy_down" => {
            let mut last = Thumb::default();
            let event = joystick_step(2, 0x5820, Thumb { x: 30000, y: 0 }, &mut last)
                .expect("oracle row is a key-down");
            format!(
                "{},{},{},{}",
                event.virtual_key, event.flags, last.x, last.y
            )
        }
        "joy_up" => {
            let mut last = Thumb { x: 30000, y: 0 };
            let event = joystick_step(2, 0x5820, Thumb::default(), &mut last)
                .expect("oracle row is a key-up");
            format!(
                "{},{},{},{}",
                event.virtual_key, event.flags, last.x, last.y
            )
        }
        "trigger_30" => i32::from(trigger_is_on(30)).to_string(),
        "trigger_31" => i32::from(trigger_is_on(31)).to_string(),
        "button_priority" => {
            let cur = Gamepad {
                buttons: 1,
                left_trigger: 31,
                ..Gamepad::default()
            };
            let mut last = Gamepad::default();
            let event = digital_step(3, &cur, &mut last).expect("button press");
            format!(
                "{},{},{},{}",
                event.virtual_key, event.flags, last.buttons, last.left_trigger
            )
        }
        "button_release" => {
            let cur = Gamepad::default();
            let mut last = Gamepad {
                buttons: 1,
                ..Gamepad::default()
            };
            let event = digital_step(3, &cur, &mut last).expect("button release");
            format!("{},{},{}", event.virtual_key, event.flags, last.buttons)
        }
        "left_trigger" => {
            let cur = Gamepad {
                left_trigger: 31,
                right_trigger: 31,
                ..Gamepad::default()
            };
            let mut last = Gamepad::default();
            let event = digital_step(3, &cur, &mut last).expect("left trigger press");
            format!(
                "{},{},{}",
                event.virtual_key, event.flags, last.left_trigger
            )
        }
        "right_trigger" => {
            let cur = Gamepad {
                left_trigger: 31,
                right_trigger: 31,
                ..Gamepad::default()
            };
            let mut last = Gamepad {
                left_trigger: 31,
                ..Gamepad::default()
            };
            let event = digital_step(3, &cur, &mut last).expect("right trigger press");
            format!(
                "{},{},{}",
                event.virtual_key, event.flags, last.right_trigger
            )
        }
        _ => String::new(),
    }
}

fn joy_state_number(state: JoyState) -> u8 {
    match state {
        JoyState::Off => 0,
        JoyState::Low => 1,
        JoyState::High => 2,
    }
}

#[test]
fn oracle_rows_match_the_port() {
    let rows = [
        ("sign_7f", "127"),
        ("sign_80", "-128"),
        ("sign_ff", "-1"),
        ("sign_unsigned", "255"),
        ("axis_min", "-32768"),
        ("axis_midlow", "-129"),
        ("axis_midhi", "128"),
        ("axis_max", "32767"),
        ("axis_outside", "0"),
        ("axis_inverted", "0"),
        ("muldiv_pos_half", "1"),
        ("muldiv_neg_half", "-1"),
        ("muldiv_neg_div", "-32"),
        ("muldiv_overflow", "-1"),
        ("joy_threshold_hi", "2"),
        ("joy_below_lo", "1"),
        ("offs_up_left", "4"),
        ("joy_down", "22562,1,30000,0"),
        ("joy_up", "22562,2,0,0"),
        ("trigger_30", "0"),
        ("trigger_31", "1"),
        ("button_priority", "22544,1,1,0"),
        ("button_release", "22544,2,0"),
        ("left_trigger", "22534,1,31"),
        ("right_trigger", "22535,1,31"),
    ];
    for (row, expected) in rows {
        assert_eq!(run(row), expected, "oracle row {row}");
    }
    assert_eq!(joy_state_number(joystick_state(20000)), 0);
}

#[test]
fn unknown_oracle_row_has_an_empty_representation() {
    assert_eq!(run("not-a-row"), "");
}
