//! Edge-case tests for the ported winehid.sys logic.
//!
//! These cover the IOCTL bit fields, the pass-through table boundary, the `FIXME` text and the
//! registration size. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winioctl.h, include/ntstatus.h and include/ddk/wdm.h.
    assert_eq!(FILE_DEVICE_KEYBOARD, 0x0b);
    assert_eq!(METHOD_BUFFERED, 0);
    assert_eq!(METHOD_IN_DIRECT, 1);
    assert_eq!(METHOD_OUT_DIRECT, 2);
    assert_eq!(METHOD_NEITHER, 3);
    assert_eq!(FILE_ANY_ACCESS, 0);
    assert_eq!(STATUS_SUCCESS, 0);
    assert_eq!(IRP_MJ_INTERNAL_DEVICE_CONTROL, 0x0f);
    assert_eq!(IRP_MJ_PNP, 0x1b);
}

#[test]
fn ctl_code_packs_each_field_into_its_bits() {
    // Not oracle-derived: CTL_CODE is DeviceType << 16 | Access << 14 | Function << 2 | Method.
    assert_eq!(ctl_code(0x0022, 0, 0, 0), 0x0022_0000);
    assert_eq!(ctl_code(0, 1, 0, 0), 0x0000_0004);
    assert_eq!(ctl_code(0, 0, 1, 0), 0x0000_0001);
    assert_eq!(ctl_code(0, 0, 0, 1), 0x0000_4000);
    assert_eq!(ctl_code(0xffff, 0xfff, 3, 3), 0xffff_ffff);
}

#[test]
fn ctl_code_does_not_mask_the_function_field() {
    // Not oracle-derived: the C macro does not mask Function, so bits above 12 run into the
    // Access and DeviceType fields. The port keeps that.
    assert_eq!(ctl_code(0, 0x1000, 0, 0), 0x0000_4000);
}

#[test]
fn hid_macros_use_their_method_and_any_access() {
    // Not oracle-derived: each HID_*_CTL_CODE macro is CTL_CODE with a fixed method.
    assert_eq!(hid_ctl_code(4), 0x000b_0013);
    assert_eq!(hid_buffer_ctl_code(4), 0x000b_0010);
    assert_eq!(hid_in_ctl_code(4), 0x000b_0011);
    assert_eq!(hid_out_ctl_code(4), 0x000b_0012);
}

#[test]
fn ioctl_fields_split_the_code() {
    // Not oracle-derived: the field split of the FIXME, checked on the maximum and minimum codes.
    assert_eq!(
        ioctl_fields(0xFFFF_FFFF),
        IoctlFields {
            device: 0xffff,
            access: 3,
            function: 0xfff,
            method: 3,
        }
    );
    assert_eq!(
        ioctl_fields(0),
        IoctlFields {
            device: 0,
            access: 0,
            function: 0,
            method: 0,
        }
    );
}

#[test]
fn pass_through_table_has_fourteen_distinct_codes() {
    // Not oracle-derived: the 14 case labels of the switch in internal_ioctl.
    assert_eq!(PASS_THROUGH_IOCTLS.len(), 14);
    for (i, code) in PASS_THROUGH_IOCTLS.iter().enumerate() {
        assert!(!PASS_THROUGH_IOCTLS[..i].contains(code));
    }
}

#[test]
fn every_pass_through_code_is_forwarded_whatever_the_irp_status() {
    // Not oracle-derived: each case label of the switch, with two irp statuses.
    let codes = [
        hid_out_ctl_code(102),
        hid_ctl_code(7),
        hid_ctl_code(8),
        hid_out_ctl_code(120),
        hid_ctl_code(9),
        hid_ctl_code(0),
        hid_ctl_code(1),
        hid_ctl_code(4),
        hid_out_ctl_code(104),
        hid_ctl_code(2),
        hid_in_ctl_code(101),
        hid_ctl_code(3),
        hid_out_ctl_code(100),
        hid_in_ctl_code(100),
    ];
    for code in codes {
        assert_eq!(internal_ioctl(code, 0), IrpOutcome::ForwardToNextDevice);
        assert_eq!(
            internal_ioctl(code, 0xC000_0001),
            IrpOutcome::ForwardToNextDevice
        );
    }
}

#[test]
fn near_miss_codes_are_unsupported() {
    // Not oracle-derived: codes that differ from a pass-through code by one field.
    let near_misses = [
        // Same function as IOCTL_HID_GET_STRING, but the buffered method.
        hid_buffer_ctl_code(4),
        // Same function as IOCTL_HID_GET_STRING, but the in-direct method.
        hid_in_ctl_code(4),
        // Function 5 and 6 are not in the switch.
        hid_ctl_code(5),
        hid_ctl_code(6),
        // Same code as IOCTL_HID_GET_STRING, but a read access bit.
        ctl_code(FILE_DEVICE_KEYBOARD, 4, METHOD_NEITHER, 1),
        // Same code as IOCTL_HID_GET_STRING, but another device type.
        ctl_code(0x22, 4, METHOD_NEITHER, FILE_ANY_ACCESS),
    ];
    for code in near_misses {
        assert_eq!(
            internal_ioctl(code, 0xC000_0002),
            IrpOutcome::CompleteUnsupported {
                status: 0xC000_0002
            }
        );
    }
}

#[test]
fn unsupported_message_prints_nonzero_codes_with_a_0x_prefix() {
    // Not oracle-derived: the non-zero branch of C's %#lx, which the oracle rows only reach
    // through the large codes.
    assert_eq!(
        unsupported_ioctl_message(1),
        "Unsupported ioctl 0x1 (device=0 access=0 func=0 method=1)"
    );
    assert_eq!(
        unsupported_ioctl_message(0x0000_0004),
        "Unsupported ioctl 0x4 (device=0 access=0 func=1 method=0)"
    );
}

#[test]
fn add_device_always_succeeds() {
    // Not oracle-derived: add_device has no inputs, so one call covers its only path.
    assert_eq!(add_device(), STATUS_SUCCESS);
}

#[test]
fn driver_pnp_forwards_every_irp() {
    // Not oracle-derived: driver_pnp has no branch; the oracle row covers its only path.
    assert_eq!(driver_pnp(), IrpOutcome::ForwardToNextDevice);
}

#[test]
fn registration_size_follows_the_pointer_size() {
    // Not oracle-derived: the 32-bit size. The oracle harness ran on a 64-bit target only.
    assert_eq!(hid_device_extension_size(4), 12);
    assert_eq!(minidriver_registration(4).device_extension_size, 12);
    assert_eq!(hid_device_extension_size(8), 24);
}

#[test]
fn dispatch_setup_uses_slots_below_the_major_function_count() {
    // Not oracle-derived: DRIVER_OBJECT.MajorFunction has 28 slots (IRP_MJ_MAXIMUM_FUNCTION + 1).
    for (slot, _) in DISPATCH_SETUP.major_functions {
        assert!(slot < 28);
    }
}
