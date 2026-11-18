//! Checks the public API on values chosen for their edges: the extreme and
//! negative fields, the unhandled query types, and the limits of the capacity rule.

use super::*;
use std::fmt::Debug;

fn check_traits<T: Clone + Copy + Debug + PartialEq + Eq>(value: &T) {
    let copy = *value;
    assert_eq!(&value.clone(), value);
    assert_eq!(copy, *value);
    assert!(!format!("{value:?}").is_empty());
}

fn device() -> UsbDevice {
    UsbDevice {
        vendor: 0x046D,
        product: 0xC077,
        revision: 0x1200,
        usbver: 0x0200,
        class: 0,
        subclass: 0,
        protocol: 0,
        busnum: 1,
        portnum: 3,
        interface: false,
        interface_index: -1,
    }
}

/// The ID strings of `device` for `query_type`, with each NUL shown as `|`.
fn ids_text(device: &UsbDevice, query_type: u32) -> String {
    let units = query_id(device, query_type).expect("handled type");
    units
        .iter()
        .map(|&u| if u == 0 { '|' } else { u as u8 as char })
        .collect()
}

#[test]
fn usb_device_traits() {
    check_traits(&device());
}

#[test]
fn unhandled_query_types_give_none() {
    for query_type in [4, 5, 0x1000, u32::MAX] {
        assert_eq!(query_id(&device(), query_type), None);
    }
}

#[test]
fn negative_interface_index_prints_as_32_bits() {
    let device = UsbDevice {
        interface: true,
        ..device()
    };
    assert_eq!(ids_text(&device, 0), "USB\\VID_046D&PID_C077&MI_FFFFFFFF|");
}

#[test]
fn largest_interface_index_prints_in_full() {
    let device = UsbDevice {
        interface: true,
        interface_index: i16::MAX,
        ..device()
    };
    assert_eq!(
        ids_text(&device, 1),
        "USB\\VID_046D&PID_C077&REV_1200&MI_7FFF|USB\\VID_046D&PID_C077&MI_7FFF||"
    );
}

#[test]
fn hardware_ids_without_interface_end_with_empty_string() {
    assert_eq!(
        ids_text(&device(), 1),
        "USB\\VID_046D&PID_C077&REV_1200|USB\\VID_046D&PID_C077||"
    );
}

#[test]
fn compatible_ids_use_lowercase_hex() {
    let device = UsbDevice {
        class: 0xAB,
        subclass: 0xCD,
        protocol: 0xEF,
        interface_index: 0,
        ..device()
    };
    assert_eq!(
        ids_text(&device, 2),
        "USB\\Class_ab&SubClass_cd&Prot_ef|USB\\Class_ab&SubClass_cd|USB\\Class_ab||"
    );
}

#[test]
fn instance_id_has_no_padding() {
    let device = UsbDevice {
        usbver: 0,
        revision: 7,
        busnum: 0,
        portnum: 12,
        ..device()
    };
    assert_eq!(ids_text(&device, 3), "0&7&0&12|");
}

#[test]
fn reserve_capacity_keeps_capacity_when_count_fits() {
    assert_eq!(reserve_capacity(8, 8, 1), Some(8));
    assert_eq!(reserve_capacity(8, 0, 0), Some(8));
}

#[test]
fn reserve_capacity_grows_from_a_non_power_of_two() {
    assert_eq!(reserve_capacity(6, 7, 8), Some(12));
    assert_eq!(reserve_capacity(4, 16, 8), Some(16));
}

#[test]
fn reserve_capacity_limit_is_32_bit() {
    assert_eq!(
        reserve_capacity(0, u32::MAX as usize, 1),
        Some(u32::MAX as usize)
    );
    assert_eq!(reserve_capacity(0, u32::MAX as usize + 1, 1), None);
}

#[test]
fn reserve_capacity_refuses_count_over_the_element_limit() {
    assert_eq!(reserve_capacity(0, 3, usize::MAX / 2), None);
}

#[test]
fn unmapped_libusb_statuses_give_request_failed() {
    for status in [-1, 1, 6, 99, i32::MIN] {
        assert_eq!(usbd_status_from_libusb(status), 0x8000_0500_u32 as i32);
    }
}
