//! Oracle-derived test vectors for the ported winexinput.sys logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `MulDiv`
//! (`dlls/kernelbase/main.c`) and of `sign_extend`, `scale_value`, `check_value_caps` and
//! `translate_report_to_xinput_state` (`dlls/winexinput.sys/main.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/winexinput.sys/`. The 25 printed
//! rows include three that UBSan flags (a negative shift exponent in `sign_extend`, negation of
//! `INT_MIN` in `MulDiv`, and signed overflow in `LogicalMax - LogicalMin`). They are excluded
//! here, so 22 rows remain.

use super::*;

fn value_caps(bit_size: u16, logical_min: i32, logical_max: i32) -> ValueCaps {
    ValueCaps {
        bit_size,
        logical_min,
        logical_max,
    }
}

/// Caps of the standard gamepad: 16-bit X/Y/RX/RY and 8-bit Z/RZ, all unsigned.
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

/// A report with every value zero, except the given button usages.
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
fn oracle_sign_extend_rows() {
    // sign_8bit_0x80: value=128 bit=8 lmin=-128 -> -128
    assert_eq!(sign_extend(0x80, value_caps(8, -128, 0)), -128);
    // sign_16bit_0xffff: value=65535 bit=16 lmin=-32768 -> -1
    assert_eq!(sign_extend(0xffff, value_caps(16, -32768, 0)), -1);
    // sign_16bit_0x1234_unsigned: value=4660 bit=16 lmin=0 -> 4660
    assert_eq!(sign_extend(0x1234, value_caps(16, 0, 0)), 4660);
    // sign_bit1_passthrough: value=1 bit=1 lmin=-1 -> 1
    assert_eq!(sign_extend(0x1, value_caps(1, -1, 0)), 1);
}

#[test]
fn oracle_mul_div_rows() {
    // muldiv_3_4_5: a=3 b=4 c=5 -> 2
    assert_eq!(mul_div(3, 4, 5), 2);
    // muldiv_neg_3_4_5: a=-3 b=4 c=5 -> -2
    assert_eq!(mul_div(-3, 4, 5), -2);
    // muldiv_zero_divisor: a=7 b=1 c=0 -> -1
    assert_eq!(mul_div(7, 1, 0), -1);
    // muldiv_overflow_pos: a=2147483647 b=2 c=1 -> -1
    assert_eq!(mul_div(2147483647, 2, 1), -1);
    // muldiv_neg_divisor_10_10_-4: a=10 b=10 c=-4 -> -25
    assert_eq!(mul_div(10, 10, -4), -25);
}

#[test]
fn oracle_scale_value_rows() {
    // scale_128_of_255: value=128 bit=8 lmin=0 lmax=255 min=0 max=65535 -> 32896
    assert_eq!(scale_value(128, value_caps(8, 0, 255), 0, 65535), 32896);
    // scale_degenerate_range: value=5 bit=8 lmin=5 lmax=5 -> -1
    assert_eq!(scale_value(5, value_caps(8, 5, 5), 0, 65535), -1);
    // scale_above_max: value=300 bit=8 lmin=0 lmax=255 -> 0
    assert_eq!(scale_value(300, value_caps(8, 0, 255), 0, 65535), 0);
    // scale_min_gt_max: value=5 bit=8 lmin=10 lmax=5 -> 0
    assert_eq!(scale_value(5, value_caps(8, 10, 5), 0, 65535), 0);
}

#[test]
fn oracle_check_value_caps_rows() {
    // caps_usage_x: usage=0x30 bit=16 lmin=0 lmax=65535 -> lx=(16,0,65535), others zero
    let mut caps = DeviceCaps::default();
    caps.check_value_caps(0x30, value_caps(16, 0, 65535));
    assert_eq!(
        caps,
        DeviceCaps {
            lx_caps: value_caps(16, 0, 65535),
            ..DeviceCaps::default()
        }
    );
    // caps_usage_rz: usage=0x35 bit=8 lmin=0 lmax=255 -> rt=(8,0,255), others zero
    let mut caps = DeviceCaps::default();
    caps.check_value_caps(0x35, value_caps(8, 0, 255));
    assert_eq!(
        caps,
        DeviceCaps {
            rt_caps: value_caps(8, 0, 255),
            ..DeviceCaps::default()
        }
    );
}

#[test]
fn oracle_translate_report_rows() {
    let caps = standard_caps();

    // translate_neutral: all values zero
    assert_eq!(
        translate_report_to_xinput_state(&report(&[]), &caps),
        XinputState {
            lx_axis: 0,
            ly_axis: 65535,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 32768,
            buttons: 0,
        }
    );

    // translate_stick_hat_n: buttons=[1,2] hat=1 x=32768 y=32768 z=255 rx=65535 ry=65535 rz=0
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                hat: 1,
                x: 32768,
                y: 32768,
                z: 255,
                rx: 65535,
                ry: 65535,
                ..report(&[1, 2])
            },
            &caps
        ),
        XinputState {
            lx_axis: 32768,
            ly_axis: 32767,
            rx_axis: 65535,
            ry_axis: 0,
            trigger: 65408,
            buttons: 1027,
        }
    );

    // translate_hat9_buttons_filtered: buttons=[0,11,10] hat=9 -> only usage 10 counts
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                hat: 9,
                ..report(&[0, 11, 10])
            },
            &caps
        ),
        XinputState {
            lx_axis: 0,
            ly_axis: 65535,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 32768,
            buttons: 512,
        }
    );

    // translate_hat8_signed_x: hat=8 x=128 with lx caps bit=8 lmin=-128 lmax=127
    let signed_x = DeviceCaps {
        lx_caps: value_caps(8, -128, 127),
        ..standard_caps()
    };
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                hat: 8,
                x: 0x80,
                ..report(&[])
            },
            &signed_x
        ),
        XinputState {
            lx_axis: 0,
            ly_axis: 65535,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 32768,
            buttons: 8192,
        }
    );

    // translate_trigger_rt_only: rz=255 -> rt full, lt zero
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                rz: 255,
                ..report(&[])
            },
            &caps
        ),
        XinputState {
            lx_axis: 0,
            ly_axis: 65535,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 128,
            buttons: 0,
        }
    );

    // translate_inverted_lx_caps: lx caps bit=16 lmin=10 lmax=5 -> lx 0
    let inverted = DeviceCaps {
        lx_caps: value_caps(16, 10, 5),
        ..standard_caps()
    };
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                x: 1234,
                ..report(&[])
            },
            &inverted
        ),
        XinputState {
            lx_axis: 0,
            ly_axis: 65535,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 32768,
            buttons: 0,
        }
    );

    // translate_degenerate_axes: lx and ly caps bit=16 lmin=5 lmax=5 with x=5 y=5.
    // MulDiv returns -1, so lx wraps to 65535 and ly becomes 65535 - (-1) = 65536 -> 0.
    let degenerate = DeviceCaps {
        lx_caps: value_caps(16, 5, 5),
        ly_caps: value_caps(16, 5, 5),
        ..standard_caps()
    };
    assert_eq!(
        translate_report_to_xinput_state(
            &Report {
                x: 5,
                y: 5,
                ..report(&[])
            },
            &degenerate
        ),
        XinputState {
            lx_axis: 65535,
            ly_axis: 0,
            rx_axis: 0,
            ry_axis: 65535,
            trigger: 32768,
            buttons: 0,
        }
    );
}
