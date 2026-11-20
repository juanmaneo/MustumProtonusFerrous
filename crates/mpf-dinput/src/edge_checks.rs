//! Edge checks for the cases that the oracle rows do not reach. Each expected value is a
//! probe recorded from the verbatim Wine C under ASan and UBSan
//! (`target/canon/dinput/rows.out`). Rows marked `port-only` have no C result: the C
//! code is undefined there, or traps, and the port defines the result.

use super::*;

type ConvertCase = (u32, &'static [i32], u32, Option<&'static [i32]>);

fn range(physical_min: i32, physical_max: i32, logical_min: i32, logical_max: i32) -> ValueRange {
    ValueRange {
        logical_min,
        logical_max,
        physical_min,
        physical_max,
    }
}

fn props(
    bit_size: i32,
    logical_min: i32,
    logical_max: i32,
    range_min: i32,
    range_max: i32,
    deadzone: i32,
    saturation: i32,
) -> ObjectProperties {
    ObjectProperties {
        bit_size,
        logical_min,
        logical_max,
        range_min,
        range_max,
        deadzone,
        saturation,
    }
}

#[test]
fn object_usage_covers_every_arm() {
    // obj_guid <page> <usage>
    let cases = [
        (0x0c, 0x01, GUID_UNKNOWN),
        (0x01, 0x31, GUID_Y_AXIS),
        (0x01, 0x32, GUID_Z_AXIS),
        (0x01, 0x33, GUID_RX_AXIS),
        (0x01, 0x34, GUID_RY_AXIS),
        (0x01, 0x35, GUID_RZ_AXIS),
        (0x01, 0x36, GUID_SLIDER),
        (0x01, 0x37, GUID_SLIDER),
        (0x01, 0x38, GUID_Z_AXIS),
        (0x01, 0x3a, GUID_UNKNOWN),
        (0x01, 0x00, GUID_UNKNOWN),
        (0x02, 0xba, GUID_RZ_AXIS),
        (0x02, 0xbb, GUID_SLIDER),
        (0x02, 0xc4, GUID_Y_AXIS),
        (0x02, 0xc8, GUID_X_AXIS),
        (0x02, 0x00, GUID_UNKNOWN),
        (0x09, 0x01, GUID_BUTTON),
    ];
    for (page, usage, expected) in cases {
        assert_eq!(
            object_usage_to_guid(page, usage),
            expected,
            "page {page:#x} usage {usage:#x}"
        );
    }
}

#[test]
fn effect_guid_to_usage_covers_every_effect() {
    // eff_usage <name>
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    let cases = [
        (GUID_CONSTANT_FORCE, 38),
        (GUID_RAMP_FORCE, 39),
        (GUID_CUSTOM_FORCE, 40),
        (GUID_SQUARE, 48),
        (GUID_TRIANGLE, 50),
        (GUID_SAWTOOTH_UP, 51),
        (GUID_SAWTOOTH_DOWN, 52),
        (GUID_SPRING, 64),
        (GUID_DAMPER, 65),
        (GUID_INERTIA, 66),
        (GUID_FRICTION, 67),
        (GUID_X_AXIS, 0),
        (zero, 0),
    ];
    for (guid, expected) in cases {
        assert_eq!(effect_guid_to_usage(&guid), expected, "{guid:?}");
    }
}

#[test]
fn effect_usage_to_guid_covers_every_usage() {
    // eff_guid <usage>
    let cases = [
        (0x26, GUID_CONSTANT_FORCE),
        (0x27, GUID_RAMP_FORCE),
        (0x28, GUID_CUSTOM_FORCE),
        (0x30, GUID_SQUARE),
        (0x31, GUID_SINE),
        (0x32, GUID_TRIANGLE),
        (0x33, GUID_SAWTOOTH_UP),
        (0x34, GUID_SAWTOOTH_DOWN),
        (0x40, GUID_SPRING),
        (0x41, GUID_DAMPER),
        (0x42, GUID_INERTIA),
        (0x43, GUID_FRICTION),
        (0x44, GUID_UNKNOWN),
        (0x00, GUID_UNKNOWN),
    ];
    for (usage, expected) in cases {
        assert_eq!(effect_usage_to_guid(usage), expected, "usage {usage:#x}");
    }
}

#[test]
fn map_dik_code_remaps_japan106_and_version_gates_the_scan_code() {
    // map_dik <scan> <vk> <sub> <version> <vk_to_scan>; the vk argument is not an input here.
    let cases = [
        (0x0d, 10, 0x0800, 0, 144),
        (0x1a, 10, 0x0800, 0, 145),
        (0x1b, 10, 0x0800, 0, 26),
        (0x28, 10, 0x0800, 0, 146),
        (0x2b, 10, 0x0800, 0, 27),
        (0x73, 10, 0x0800, 0, 43),
        (0x29, 4, 0x0800, 0, 41),
        (0x1ff, 0, 0x0800, 0, 255),
        (0x1e, 10, 0x0800, 0, 30),
        (0, 0, 0x0800, 0x1e, 0),
        (0, 0, 0x0700, 0, 0),
    ];
    for (scan, sub_type, version, vk_to_scan, expected) in cases {
        assert_eq!(
            map_dik_code(scan, sub_type, version, vk_to_scan),
            expected,
            "scan {scan:#x} sub {sub_type} version {version:#x}"
        );
    }
}

#[test]
fn mul_div_matches_muldiv_including_its_limits() {
    // muldiv <a> <b> <c>
    let cases = [
        (7, 3, 2, 11),
        (-7, 3, 2, -11),
        (7, -3, 2, -11),
        (7, 3, -2, -11),
        (-7, -3, 2, 11),
        (7, -3, -2, 11),
        (5, 5, 0, -1),
        (2147483647, 2, 1, -1),
        (-2147483647, 1, 1, -2147483647),
        (-2147483648, 1, 1, -1),
        (3, -5, 2147483647, 0),
        (2147483647, -2, 1, -1),
        (-2147483647, -1, 1, 2147483647),
        (2147483647, -1, 1, -2147483647),
    ];
    for (a, b, c, expected) in cases {
        assert_eq!(mul_div(a, b, c), expected, "muldiv {a} {b} {c}");
    }
}

#[test]
fn scale_to_logical_value_maps_and_rejects() {
    // scale_logical <value> <phy_min,phy_max,log_min,log_max>; None is "-".
    let cases = [
        (0, None, 0),
        (5, Some(range(0, 0, 0, 10)), 5),
        (0, Some(range(-10, 0, 0, 100)), 100),
        (3, Some(range(0, 10, 0, -1)), 0),
        (150, Some(range(0, 100, 0, 200)), -1),
        (-7, Some(range(0, 10, 0, 3)), -1),
        // port-only: the C code divides by zero for phy_min == phy_max.
        (10, Some(range(10, 10, 0, 100)), -1),
    ];
    for (value, caps, expected) in cases {
        assert_eq!(
            scale_to_logical_value(value, caps),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn clamp_to_physical_value_clamps_or_passes_through() {
    // clamp <value> <phy_min,phy_max,log_min,log_max>; None is "-".
    let cases = [
        (500, None, 500),
        (-5, Some(range(0, 100, 0, 200)), 0),
        (50, Some(range(100, 0, 0, 200)), 100),
    ];
    for (value, caps, expected) in cases {
        assert_eq!(
            clamp_to_physical_value(value, caps),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn sign_extend_reads_negative_ranges_as_signed() {
    // sign_extend <value> <bit_size> <logical_min>
    let cases = [
        (0x80, props(8, 0, 0, 0, 0, 0, 0), 128),
        (0x7f, props(8, -128, 0, 0, 0, 0, 0), 127),
        (0xfe, props(8, -128, 0, 0, 0, 0, 0), -2),
        (0xffff, props(16, -32768, 0, 0, 0, 0, 0), -1),
        (0x1, props(1, -1, 0, 0, 0, 0, 0), 1),
        (0x8000, props(16, -1, 0, 0, 0, 0, 0), -32768),
        // port-only: C shifts 1 by 31 (flagged); the port extends the sign bit as written.
        (0x8000_0000, props(32, -1, 0, 0, 0, 0, 0), i32::MIN),
    ];
    for (value, properties, expected) in cases {
        assert_eq!(
            sign_extend(value, &properties),
            expected,
            "value {value:#x}"
        );
    }
}

#[test]
fn scale_value_maps_the_logical_range_onto_the_physical_range() {
    // scale_value <value> <bit_size> <log_min> <log_max> <range_min> <range_max>
    let cases = [
        (0x80, props(8, 0, 100, 0, 1000, 0, 0), -1),
        (0, props(8, 0, 0, 0, 0, 0, 0), -1),
        (5, props(8, 0, 10, -100, 100, 0, 0), 0),
        (1, props(8, 0, 10, 0, 100, 0, 0), 10),
        (
            65535,
            props(16, 0, 65535, -1_000_000_000, 1_000_000_000, 0, 0),
            1_000_000_000,
        ),
        (5, props(8, 5, 5, 0, 100, 0, 0), -1),
        (0, props(8, 5, 10, 0, 100, 0, 0), -1),
    ];
    for (value, properties, expected) in cases {
        assert_eq!(
            scale_value(value, &properties),
            expected,
            "value {value:#x}"
        );
    }
}

#[test]
fn scale_axis_value_covers_deadzone_and_saturation_arms() {
    // scale_axis <value> <bit_size> <log_min> <log_max> <range_min> <range_max> <deadzone> <saturation>
    let cases = [
        (0, props(8, 0, 255, 0, 65535, 0, 10000), 0),
        (255, props(8, 0, 255, 0, 65535, 0, 10000), 65535),
        (200, props(8, 0, 255, 0, 65535, 0, 5000), 65535),
        (0x80, props(8, -128, 127, -32768, 32767, 0, 10000), -32768),
        (0x8000, props(16, 0, -1, 0, 65535, 0, 10000), 32768),
        (
            2_000_000_000,
            props(30, 0, 2147483647, 0, 65535, 0, 30000),
            65535,
        ),
        // port-only: C shifts by -1 for bit_size 0 (flagged); the port clamps the shift.
        (0, props(0, 0, 255, 0, 65535, 0, 10000), 0),
    ];
    for (value, properties, expected) in cases {
        assert_eq!(
            scale_axis_value(value, &properties),
            expected,
            "value {value:#x}"
        );
    }
}

#[test]
fn device_type_for_version_maps_legacy_types() {
    // devtype <type> <version>
    let cases = [
        (0x11, 0x0800, 17),
        (0x14, 0x0700, 66052),
        (0x0114, 0x0700, 65796),
        (0x1c, 0x0700, 65796),
        (0x0d1c, 0x0700, 66820),
        (0x16, 0x0700, 67076),
        (0x17, 0x0700, 66308),
        (0x18, 0x0700, 65796),
        (0x12, 0x0700, 65537),
        (0x1a, 0x0700, 65537),
        (0x0100, 0x0700, 65537),
    ];
    for (device_type, version, expected) in cases {
        assert_eq!(
            device_type_for_version(device_type, version),
            expected,
            "{device_type:#x}"
        );
    }
}

#[test]
fn convert_directions_converts_between_frames() {
    // convdir <in_flags> <axes> <out_flags>; "-" is an empty axis list, and a missing
    // result is "none".
    let cases: &[ConvertCase] = &[
        (0x40, &[], 0x40, Some(&[])),
        (0x40, &[], 0x10, Some(&[])),
        (0x40, &[], 0x20, Some(&[9000, 0])),
        (0x20, &[], 0x40, Some(&[])),
        (0x10, &[], 0x10, Some(&[])),
        (0x10, &[], 0x40, Some(&[])),
        (0x40, &[0], 0x10, Some(&[10000])),
        (0x20, &[500], 0x10, Some(&[871])),
        (
            0x40,
            &[0, 0, 0, 0, 0, 0],
            0x10,
            Some(&[10000, 0, 0, 0, 0, 0]),
        ),
        (
            0x10,
            &[1, 2, 3, 4, 5, 6],
            0x40,
            Some(&[6343, 5330, 4691, 4239, 3897, 0]),
        ),
        (0x20, &[-35999], 0x40, Some(&[-8999])),
        (0x20, &[500], 0x20, Some(&[500])),
        (0x10, &[3000], 0x40, Some(&[0])),
        (0x40, &[-1000, 2000], 0x40, Some(&[35000, 0])),
        (0x10, &[3000, 4000], 0x10, Some(&[3000, 4000])),
        (0x10, &[3000, 4000], 0x20, Some(&[14313, 0])),
        (0x20, &[1000, 2000], 0x20, Some(&[1000, 2000])),
        (0x20, &[1000, 2000], 0x10, Some(&[1736, -9848])),
        (0x20, &[1000, 2000], 0x40, Some(&[28000, 0])),
        (0x40, &[1000, 2000], 0x20, Some(&[10000, 0])),
        (0x40, &[1000, 2000], 0x10, Some(&[9253, 1631])),
        (0x0, &[3000], 0x10, None),
        (0x10, &[3000, 4000], 0x0, None),
        (0x30, &[3000], 0x10, None),
    ];
    for &(in_flags, axes, out_flags, expected) in cases {
        assert_eq!(
            convert_directions(in_flags, axes, out_flags).as_deref(),
            expected,
            "in {in_flags:#x} out {out_flags:#x} axes {axes:?}"
        );
    }
}

// The derived `Clone` impls are the code under test, so `clone_on_copy` does not apply.
#[test]
#[allow(clippy::clone_on_copy)]
fn derived_traits_copy_and_format() {
    let guid = GUID_X_AXIS;
    assert_eq!(Clone::clone(&guid), guid);
    assert!(format!("{guid:?}").starts_with("Guid {"));

    let properties = ObjectProperties::default();
    assert_eq!(Clone::clone(&properties).bit_size, 0);
    assert!(format!("{properties:?}").contains("bit_size: 0"));

    let value_range = range(2, 3, 0, 1);
    assert_eq!(Clone::clone(&value_range).physical_max, 3);
    assert!(format!("{value_range:?}").contains("physical_max: 3"));
}
