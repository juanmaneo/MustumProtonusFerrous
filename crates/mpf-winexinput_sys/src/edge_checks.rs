//! Boundary and branch tests for the ported winexinput.sys logic.
//!
//! Expectations marked "source-derived" follow from reading the Wine C code. They are not
//! oracle runs. Inputs where the C code is undefined behaviour are marked "not oracle-derived".

use super::*;

fn value_caps(bit_size: u16, logical_min: i32, logical_max: i32) -> ValueCaps {
    ValueCaps {
        bit_size,
        logical_min,
        logical_max,
    }
}

fn standard_caps() -> DeviceCaps {
    DeviceCaps {
        lx_caps: value_caps(16, 0, 65535),
        ly_caps: value_caps(16, 0, 65535),
        lt_caps: value_caps(8, 0, 255),
        rx_caps: value_caps(16, 0, 65535),
        ry_caps: value_caps(16, 0, 65535),
        rt_caps: value_caps(8, 0, 255),
    }
}

fn report(buttons: &[u16]) -> Report<'_> {
    Report {
        buttons,
        hat: 0,
        x: 0,
        y: 0,
        z: 0,
        rx: 0,
        ry: 0,
        rz: 0,
    }
}

#[test]
fn sign_extend_passes_unsigned_and_one_bit_values_through() {
    // Source-derived: sign <= 1 (bit_size 1) or logical_min >= 0 returns the value unchanged.
    assert_eq!(sign_extend(0xff, value_caps(8, 0, 255)), 255);
    assert_eq!(sign_extend(0x0, value_caps(1, -1, 0)), 0);
}

#[test]
fn sign_extend_clear_sign_bit_keeps_positive_value() {
    // Source-derived: with the sign bit clear, value - 0 is the value itself.
    assert_eq!(sign_extend(0x7f, value_caps(8, -128, 127)), 127);
}

#[test]
fn sign_extend_32_bit_is_twos_complement() {
    // Source-derived: bit_size 32 gives sign = 0x8000_0000, so the value reads as i32.
    assert_eq!(sign_extend(0x8000_0000, value_caps(32, -1, 0)), i32::MIN);
}

#[test]
fn sign_extend_out_of_range_bit_size_returns_value() {
    // Not oracle-derived: bit_size 0 and 33 make the C shift undefined (the oracle run flags
    // bit_size 0). This port follows the sibling mpf-xinput1_3: with no valid sign bit the
    // value is returned unchanged.
    assert_eq!(sign_extend(0x80, value_caps(0, -128, 0)), 128);
    assert_eq!(sign_extend(0x80, value_caps(33, -128, 0)), 128);
}

#[test]
fn scale_value_endpoints_map_to_min_and_max() {
    // Source-derived: a value at logical_min maps to min, and at logical_max to max.
    assert_eq!(scale_value(0, value_caps(8, 0, 255), 0, 65535), 0);
    assert_eq!(scale_value(255, value_caps(8, 0, 255), 0, 65535), 65535);
}

#[test]
fn scale_value_adds_min_after_scaling() {
    // Source-derived: MulDiv(128, 100, 255) = 50 (12927 / 255), and min 100 is added: 150.
    assert_eq!(scale_value(128, value_caps(8, 0, 255), 100, 200), 150);
}

#[test]
fn scale_value_below_logical_min_is_zero() {
    // Source-derived: the second guard rejects a value under logical_min.
    assert_eq!(scale_value(3, value_caps(8, 5, 255), 0, 65535), 0);
}

#[test]
fn scale_value_scales_the_sign_extended_value() {
    // Source-derived: 0xff with a signed 8-bit range is -1. MulDiv(127, 255, 255) = 127.
    assert_eq!(scale_value(0xff, value_caps(8, -128, 127), 0, 255), 127);
}

#[test]
fn mul_div_rounds_half_away_from_zero() {
    // Source-derived: the rounding term is c / 2, added for a positive product and subtracted
    // for a negative one.
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(-1, 1, 2), -1);
}

#[test]
fn mul_div_negative_operand_pairs_take_the_matching_branch() {
    // Source-derived: two negative operands take the add branch. Mixed signs take the subtract
    // branch and truncate toward zero.
    assert_eq!(mul_div(-3, -4, 5), 2);
    assert_eq!(mul_div(3, -4, 5), -2);
}

#[test]
fn mul_div_negative_divisor_flips_operand_sign() {
    // Source-derived: c < 0 negates a and c. mul_div(6, 2, -3) is (-12 - 1) / 3 = -4.
    assert_eq!(mul_div(6, 2, -3), -4);
}

#[test]
fn mul_div_result_limits_are_inclusive() {
    // Source-derived: results in -2147483647..=2147483647 are kept.
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
    // Source-derived: -2147483648 is below the lower limit, so the result is -1.
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
}

#[test]
fn check_value_caps_selects_one_slot_per_axis_usage() {
    // Source-derived: the switch maps X, Y, Z, RX, RY and RZ to lx, ly, lt, rx, ry and rt.
    let mut caps = DeviceCaps::default();
    caps.check_value_caps(0x30, value_caps(1, 0, 0));
    caps.check_value_caps(0x31, value_caps(2, 0, 0));
    caps.check_value_caps(0x32, value_caps(3, 0, 0));
    caps.check_value_caps(0x33, value_caps(4, 0, 0));
    caps.check_value_caps(0x34, value_caps(5, 0, 0));
    caps.check_value_caps(0x35, value_caps(6, 0, 0));
    assert_eq!(
        caps,
        DeviceCaps {
            lx_caps: value_caps(1, 0, 0),
            ly_caps: value_caps(2, 0, 0),
            lt_caps: value_caps(3, 0, 0),
            rx_caps: value_caps(4, 0, 0),
            ry_caps: value_caps(5, 0, 0),
            rt_caps: value_caps(6, 0, 0),
        }
    );
}

#[test]
fn check_value_caps_ignores_other_usages() {
    // Source-derived: the hat switch (0x39) matches no case, so the caps are unchanged.
    let mut caps = standard_caps();
    let before = caps;
    caps.check_value_caps(0x39, value_caps(9, 9, 9));
    assert_eq!(caps, before);
}

#[test]
fn translate_hat_values_select_the_direction_bits() {
    // Source-derived: hat 1..=8 shifts into bits 10..13. Hat 0 gives no hat bits.
    let caps = standard_caps();
    let hat = |value| {
        translate_report_to_xinput_state(
            &Report {
                hat: value,
                ..report(&[])
            },
            &caps,
        )
        .buttons
    };
    assert_eq!(hat(0), 0);
    assert_eq!(hat(1), 1024);
    assert_eq!(hat(8), 8192);
}

#[test]
fn translate_button_usages_set_bits_one_to_ten_only() {
    // Source-derived: usage u in 1..=10 sets bit u - 1. Usage 0 and anything above 10 is skipped.
    let caps = standard_caps();
    let buttons = |list: &[u16]| translate_report_to_xinput_state(&report(list), &caps).buttons;
    assert_eq!(buttons(&[1]), 1);
    assert_eq!(buttons(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 0x3ff);
    assert_eq!(buttons(&[u16::MAX, 0]), 0);
}

#[test]
fn translate_hat_bits_come_before_button_bits() {
    // Source-derived: the hat group is assigned first, then button bits are OR-ed in.
    let caps = standard_caps();
    let state = translate_report_to_xinput_state(
        &Report {
            hat: 8,
            ..report(&[1])
        },
        &caps,
    );
    assert_eq!(state.buttons, 0x2001);
}

#[test]
fn translate_y_axes_are_inverted() {
    // Source-derived: LY and RY are 65535 - scaled value, so full deflection maps to 0.
    let caps = standard_caps();
    let state = translate_report_to_xinput_state(
        &Report {
            y: 65535,
            ry: 65535,
            ..report(&[])
        },
        &caps,
    );
    assert_eq!((state.ly_axis, state.ry_axis), (0, 0));
}

#[test]
fn translate_trigger_falls_below_neutral_when_rt_exceeds_lt() {
    // Source-derived: rt = 128 and lt = 0, so (lt - rt) * 128 = -16384 and 0x8000 - 16384 = 0x4000.
    let caps = standard_caps();
    let state = translate_report_to_xinput_state(
        &Report {
            rz: 128,
            ..report(&[])
        },
        &caps,
    );
    assert_eq!(state.trigger, 0x4000);
}
