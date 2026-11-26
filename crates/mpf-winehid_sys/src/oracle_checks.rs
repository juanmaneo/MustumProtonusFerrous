//! Oracle-derived test vectors for the ported winehid.sys logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `internal_ioctl`, `driver_pnp`,
//! `add_device` and `DriverEntry` (`dlls/winehid.sys/main.c`, lines 34 to 99) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/winehid.sys/`, with the IOCTL macros
//! copied from the Wine headers. The 17 printed rows ran with no ASan or UBSan report, so no
//! vector is excluded. Where the harness prints the lower driver's status (`ret=c0000002`) for a
//! forwarded IOCTL, the port does not model it and the check is on the outcome only.

use super::*;

#[test]
fn oracle_internal_ioctl_rows() {
    // row 01 IOCTL_HID_GET_STRING code=000b0013 irp=00000000 calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_ctl_code(4), 0x0000_0000),
        IrpOutcome::ForwardToNextDevice
    );
    // row 02 IOCTL_GET_PHYSICAL_DESCRIPTOR code=000b019a irp=00000103 calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_out_ctl_code(102), 0x0000_0103),
        IrpOutcome::ForwardToNextDevice
    );
    // row 03 IOCTL_HID_SET_FEATURE code=000b0191 irp=c00000bb calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_in_ctl_code(100), 0xC000_00BB),
        IrpOutcome::ForwardToNextDevice
    );
    // row 04 IOCTL_HID_ACTIVATE_DEVICE code=000b001f irp=00000000 calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_ctl_code(7), 0x0000_0000),
        IrpOutcome::ForwardToNextDevice
    );
    // row 05 IOCTL_HID_GET_INPUT_REPORT code=000b01a2 irp=00000000 calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_out_ctl_code(104), 0x0000_0000),
        IrpOutcome::ForwardToNextDevice
    );
    // row 06 IOCTL_HID_GET_FEATURE code=000b0192 irp=00000000 calls=SC next_ok=1
    assert_eq!(
        internal_ioctl(hid_out_ctl_code(100), 0x0000_0000),
        IrpOutcome::ForwardToNextDevice
    );

    // row 07 IOCTL_HID_FLUSH_QUEUE code=000b0197 irp=c0000001 ret=c0000001 calls=K next_ok=0
    assert_eq!(
        internal_ioctl(hid_ctl_code(101), 0xC000_0001),
        IrpOutcome::CompleteUnsupported {
            status: 0xC000_0001
        }
    );
    assert_eq!(
        unsupported_ioctl_message(hid_ctl_code(101)),
        "Unsupported ioctl 0xb0197 (device=b access=0 func=65 method=3)"
    );
    // row 08 code=00000000 irp=00000000 ret=00000000 calls=K fixme=[... 0 ...]
    assert_eq!(
        internal_ioctl(0x0000_0000, 0x0000_0000),
        IrpOutcome::CompleteUnsupported { status: 0 }
    );
    assert_eq!(
        unsupported_ioctl_message(0x0000_0000),
        "Unsupported ioctl 0 (device=0 access=0 func=0 method=0)"
    );
    // row 09 code=ffffffff irp=00000005 ret=00000005 calls=K
    assert_eq!(
        internal_ioctl(0xFFFF_FFFF, 0x0000_0005),
        IrpOutcome::CompleteUnsupported { status: 5 }
    );
    assert_eq!(
        unsupported_ioctl_message(0xFFFF_FFFF),
        "Unsupported ioctl 0xffffffff (device=ffff access=3 func=fff method=3)"
    );
    // row 10 IOCTL_HID_GET_HARDWARE_ID code=000b019e irp=c0000010 ret=c0000010 calls=K
    assert_eq!(
        internal_ioctl(hid_out_ctl_code(103), 0xC000_0010),
        IrpOutcome::CompleteUnsupported {
            status: 0xC000_0010
        }
    );
    assert_eq!(
        unsupported_ioctl_message(hid_out_ctl_code(103)),
        "Unsupported ioctl 0xb019e (device=b access=0 func=67 method=2)"
    );
    // row 11 IOCTL_HID_GET_POLL_FREQUENCY_MSEC code=000b0198 irp=00000000 ret=00000000 calls=K
    assert_eq!(
        internal_ioctl(hid_buffer_ctl_code(102), 0x0000_0000),
        IrpOutcome::CompleteUnsupported { status: 0 }
    );
    assert_eq!(
        unsupported_ioctl_message(hid_buffer_ctl_code(102)),
        "Unsupported ioctl 0xb0198 (device=b access=0 func=66 method=0)"
    );
    // row 12 IOCTL_HID_GET_COLLECTION_DESCRIPTOR code=000b0193 irp=00000000 ret=00000000 calls=K
    assert_eq!(
        internal_ioctl(hid_ctl_code(100), 0x0000_0000),
        IrpOutcome::CompleteUnsupported { status: 0 }
    );
    assert_eq!(
        unsupported_ioctl_message(hid_ctl_code(100)),
        "Unsupported ioctl 0xb0193 (device=b access=0 func=64 method=3)"
    );
}

#[test]
fn oracle_constant_rows() {
    // row 13 IOCTL_HID_GET_DEVICE_DESCRIPTOR=000b0003
    assert_eq!(hid_ctl_code(0), 0x000b_0003);
    // row 14 IOCTL_HID_SET_OUTPUT_REPORT=000b0195
    assert_eq!(hid_in_ctl_code(101), 0x000b_0195);
}

#[test]
fn oracle_add_device_row() {
    // row 15 add_device -> ret=00000000
    assert_eq!(add_device(), 0x0000_0000);
}

#[test]
fn oracle_driver_pnp_row() {
    // row 16 driver_pnp -> ret=c0000002 calls=SC next_ok=1; the lower status is not modelled
    assert_eq!(driver_pnp(), IrpOutcome::ForwardToNextDevice);
}

#[test]
fn oracle_driver_entry_row() {
    // row 17 DriverEntry -> ret=40000001 calls=R mj0f=internal_ioctl mj1b=driver_pnp
    // add_device=add_device nonnull_mj=2 revision=0 ext_size=24 polled=0 reserved=000000
    // driver_ok=1 path_ok=1. The ret is HidRegisterMinidriver's and is not modelled.
    assert_eq!(
        DISPATCH_SETUP.major_functions,
        [
            (IRP_MJ_INTERNAL_DEVICE_CONTROL, Handler::InternalIoctl),
            (IRP_MJ_PNP, Handler::Pnp),
        ]
    );
    assert_eq!(DISPATCH_SETUP.add_device, Handler::AddDevice);
    assert_eq!(
        minidriver_registration(8),
        MinidriverRegistration {
            revision: 0,
            device_extension_size: 24,
            devices_are_polled: false,
            reserved: [0, 0, 0],
        }
    );
}
