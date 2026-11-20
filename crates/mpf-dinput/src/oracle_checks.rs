//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/dinput/harness.c`, built with ASan and UBSan). Each row is one harness
//! command, quoted in the comment above it, with the arguments and the recorded output.
//! Generated from `rows.out` by `target/canon/dinput/gen.py`.

use super::*;

type ConvertCase = (u32, &'static [i32], u32, Option<&'static [i32]>);

#[test]
fn map_dik_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u32, u32, u32, u32, u8)] = &[
        // map_dik 0 0x41 0 0x0700 0x1e
        (0, 0, 1792, 30, 30),
        // map_dik 0x29 0 10 0x0800 0
        (41, 10, 2048, 0, 148),
        // map_dik 0x100 0 0 0x0800 0
        (256, 0, 2048, 0, 128),
    ];
    for row in ROWS {
        let &(scan, sub_type, version, mapped, expected) = row;
        assert_eq!(
            map_dik_code(scan, sub_type, version, mapped),
            expected,
            "{row:?}"
        );
    }
}

#[test]
fn obj_guid_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u16, u16, Guid)] = &[
        // obj_guid 0x01 0x30
        (1, 48, GUID_X_AXIS),
        // obj_guid 0x02 0xc5
        (2, 197, GUID_RZ_AXIS),
        // obj_guid 0x09 0x05
        (9, 5, GUID_BUTTON),
        // obj_guid 0x01 0x39
        (1, 57, GUID_POV),
    ];
    for row in ROWS {
        let &(page, usage, expected) = row;
        assert_eq!(object_usage_to_guid(page, usage), expected, "{row:?}");
    }
}

#[test]
fn eff_usage_rows() {
    #[rustfmt::skip]
    const ROWS: &[(Guid, u16)] = &[
        // eff_usage Sine
        (GUID_SINE, 49),
    ];
    for row in ROWS {
        let &(guid, expected) = row;
        assert_eq!(effect_guid_to_usage(&guid), expected, "{row:?}");
    }
}

#[test]
fn eff_guid_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u16, Guid)] = &[
        // eff_guid 0x28
        (40, GUID_CUSTOM_FORCE),
    ];
    for row in ROWS {
        let &(usage, expected) = row;
        assert_eq!(effect_usage_to_guid(usage), expected, "{row:?}");
    }
}

#[test]
fn scale_logical_rows() {
    #[rustfmt::skip]
    const ROWS: &[(i32, Option<ValueRange>, i32)] = &[
        // scale_logical 50 0,100,0,200
        (50, Some(ValueRange { logical_min: 0, logical_max: 200, physical_min: 0, physical_max: 100 }), 100),
        // scale_logical 5 -1000,1000,-32768,32767
        (5, Some(ValueRange { logical_min: -32768, logical_max: 32767, physical_min: -1000, physical_max: 1000 }), 163),
    ];
    for row in ROWS {
        let &(value, caps, expected) = row;
        assert_eq!(scale_to_logical_value(value, caps), expected, "{row:?}");
    }
}

#[test]
fn clamp_rows() {
    #[rustfmt::skip]
    const ROWS: &[(i32, Option<ValueRange>, i32)] = &[
        // clamp 500 0,100,0,200
        (500, Some(ValueRange { logical_min: 0, logical_max: 200, physical_min: 0, physical_max: 100 }), 100),
    ];
    for row in ROWS {
        let &(value, caps, expected) = row;
        assert_eq!(clamp_to_physical_value(value, caps), expected, "{row:?}");
    }
}

#[test]
fn sign_extend_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u32, ObjectProperties, i32)] = &[
        // sign_extend 0xff 8 -128
        (255, ObjectProperties { bit_size: 8, logical_min: -128, logical_max: 0, range_min: 0, range_max: 0, deadzone: 0, saturation: 0 }, -1),
    ];
    for row in ROWS {
        let &(value, ref props, expected) = row;
        assert_eq!(sign_extend(value, props), expected, "{row:?}");
    }
}

#[test]
fn scale_value_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u32, ObjectProperties, i32)] = &[
        // scale_value 0xff 8 -128 127 -100 100
        (255, ObjectProperties { bit_size: 8, logical_min: -128, logical_max: 127, range_min: -100, range_max: 100, deadzone: 0, saturation: 0 }, 0),
        // scale_value 0x80 8 0 255 0 100
        (128, ObjectProperties { bit_size: 8, logical_min: 0, logical_max: 255, range_min: 0, range_max: 100, deadzone: 0, saturation: 0 }, 50),
    ];
    for row in ROWS {
        let &(value, ref props, expected) = row;
        assert_eq!(scale_value(value, props), expected, "{row:?}");
    }
}

#[test]
fn scale_axis_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u32, ObjectProperties, i32)] = &[
        // scale_axis 0x80 8 0 255 0 65535 0 10000
        (128, ObjectProperties { bit_size: 8, logical_min: 0, logical_max: 255, range_min: 0, range_max: 65535, deadzone: 0, saturation: 10000 }, 33023),
        // scale_axis 127 8 0 255 0 65535 1000 10000
        (127, ObjectProperties { bit_size: 8, logical_min: 0, logical_max: 255, range_min: 0, range_max: 65535, deadzone: 1000, saturation: 10000 }, 32767),
        // scale_axis 200 8 0 -1 -32768 32767 0 10000
        (200, ObjectProperties { bit_size: 8, logical_min: 0, logical_max: -1, range_min: -32768, range_max: 32767, deadzone: 0, saturation: 10000 }, 18687),
    ];
    for row in ROWS {
        let &(value, ref props, expected) = row;
        assert_eq!(scale_axis_value(value, props), expected, "{row:?}");
    }
}

#[test]
fn devtype_rows() {
    #[rustfmt::skip]
    const ROWS: &[(u32, u32, u32)] = &[
        // devtype 0x15 0x0700
        (21, 1792, 66564),
        // devtype 0x041c 0x0700
        (1052, 1792, 67332),
        // devtype 0x11 0x0700
        (17, 1792, 65537),
    ];
    for row in ROWS {
        let &(device_type, version, expected) = row;
        assert_eq!(
            device_type_for_version(device_type, version),
            expected,
            "{row:?}"
        );
    }
}

#[test]
fn convdir_rows() {
    #[rustfmt::skip]
    const ROWS: &[ConvertCase] = &[
        // convdir 0x10 3000,4000 0x40
        (16, &[3000, 4000], 64, Some(&[5313, 0])),
        // convdir 0x40 5313,0 0x10
        (64, &[5313, 0], 16, Some(&[6000, 7999])),
        // convdir 0x20 1000 0x10
        (32, &[1000], 16, Some(&[1736])),
    ];
    for row in ROWS {
        let &(in_flags, axes, out_flags, expected) = row;
        assert_eq!(
            convert_directions(in_flags, axes, out_flags).as_deref(),
            expected,
            "{row:?}"
        );
    }
}

#[test]
fn muldiv_rows() {
    #[rustfmt::skip]
    const ROWS: &[(i64, i64, i64, i32)] = &[
        // muldiv 7 3 -2
        (7, 3, -2, -11),
    ];
    for row in ROWS {
        let &(a, b, c, expected) = row;
        assert_eq!(mul_div(a, b, c), expected, "{row:?}");
    }
}
