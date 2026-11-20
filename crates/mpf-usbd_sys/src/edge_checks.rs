//! Edge cases of the descriptor walkers and the validator. These cover bounds,
//! the `wTotalLength` limit and the zero-`bLength` divergence from the C loops.

use super::*;

/// A configuration with two interfaces and one endpoint on the first.
/// Offsets: configuration 0, interface 0 at 9, endpoint at 18, interface 1 at 25.
fn two_interfaces(total_length: u8) -> Vec<u8> {
    vec![
        9,
        2,
        total_length,
        0,
        2,
        1,
        0,
        0x80,
        50, //
        9,
        4,
        0,
        0,
        1,
        3,
        1,
        1,
        0, //
        7,
        5,
        0x81,
        3,
        0x40,
        0,
        0, //
        9,
        4,
        1,
        0,
        0,
        8,
        6,
        80,
        0,
    ]
}

#[test]
fn parse_descriptors_empty_buffer_is_none() {
    assert_eq!(
        parse_descriptors(&[], 0, 0, USB_INTERFACE_DESCRIPTOR_TYPE),
        None
    );
}

#[test]
fn parse_descriptors_ignores_bytes_past_total_length() {
    let buffer = [2, 4, 2, 4];
    assert_eq!(
        parse_descriptors(&buffer, 1, 0, USB_INTERFACE_DESCRIPTOR_TYPE),
        None
    );
    assert_eq!(
        parse_descriptors(&buffer, 2, 0, USB_INTERFACE_DESCRIPTOR_TYPE),
        Some(0)
    );
}

#[test]
fn parse_descriptors_start_skips_earlier_matches() {
    let buffer = [2, 4, 2, 4];
    assert_eq!(
        parse_descriptors(&buffer, 4, 1, USB_INTERFACE_DESCRIPTOR_TYPE),
        Some(2)
    );
    assert_eq!(
        parse_descriptors(&buffer, 4, 3, USB_INTERFACE_DESCRIPTOR_TYPE),
        None
    );
}

#[test]
fn parse_descriptors_zero_length_ends_the_walk() {
    // The C loop never advances past a zero bLength; the port stops with None.
    let buffer = [0, 1, 2, 4];
    assert_eq!(
        parse_descriptors(&buffer, 4, 0, USB_INTERFACE_DESCRIPTOR_TYPE),
        None
    );
}

#[test]
fn parse_configuration_descriptor_ex_is_bounded_by_total_length() {
    let full = two_interfaces(34);
    assert_eq!(
        parse_configuration_descriptor_ex(&full, 0, 1, -1, -1, -1, -1),
        Some(25)
    );

    let truncated = two_interfaces(25);
    assert_eq!(
        parse_configuration_descriptor_ex(&truncated, 0, 1, -1, -1, -1, -1),
        None
    );
}

#[test]
fn parse_configuration_descriptor_ex_start_past_last_interface_is_none() {
    let buffer = two_interfaces(34);
    assert_eq!(
        parse_configuration_descriptor_ex(&buffer, 26, -1, -1, -1, -1, -1),
        None
    );
}

#[test]
fn parse_configuration_descriptor_finds_by_number_and_alternate() {
    let buffer = two_interfaces(34);
    assert_eq!(parse_configuration_descriptor(&buffer, 1, 0), Some(25));
    assert_eq!(parse_configuration_descriptor(&buffer, 0, 1), None);
}

#[test]
fn get_interface_length_stops_at_the_next_interface() {
    let buffer = two_interfaces(34);
    assert_eq!(get_interface_length(&buffer, 9, 34), 16);
    assert_eq!(get_interface_length(&buffer, 25, 34), 9);
}

#[test]
fn get_interface_length_zero_descriptor_ends_the_sum() {
    // A zero bLength would hang the C loop; the port keeps the total so far.
    let buffer = [9, 4, 0, 0, 0, 3, 0, 0, 0, 0, 5];
    assert_eq!(get_interface_length(&buffer, 0, 11), 9);
}

#[test]
fn validate_accepts_minimal_descriptor_and_ignores_level_and_tag() {
    let descriptor = [9, 2, 9, 0, 0, 1, 0, 0x80, 50];
    assert_eq!(
        validate_configuration_descriptor(&descriptor, 0, 0),
        USBD_STATUS_SUCCESS
    );
    assert_eq!(
        validate_configuration_descriptor(&descriptor, 7, 0xDEAD),
        USBD_STATUS_SUCCESS
    );
}

#[test]
fn validate_total_length_boundary_scales_with_interfaces() {
    let mut descriptor = [9, 2, 27, 0, 3, 1, 0, 0x80, 50];
    assert_eq!(
        validate_configuration_descriptor(&descriptor, 0, 0),
        USBD_STATUS_SUCCESS
    );
    descriptor[2] = 26;
    assert_eq!(
        validate_configuration_descriptor(&descriptor, 0, 0),
        USBD_STATUS_ERROR
    );
}

#[test]
fn validate_rejects_b_length_below_nine() {
    let descriptor = [8, 2, 9, 0, 0, 1, 0, 0x80, 50];
    assert_eq!(
        validate_configuration_descriptor(&descriptor, 0, 0),
        USBD_STATUS_ERROR
    );
}

#[test]
fn status_constants_match_ddk_values() {
    assert_eq!(USBD_STATUS_SUCCESS, 0);
    assert_eq!(USBD_STATUS_ERROR as u32, 0x8000_0000);
}

#[test]
fn version_is_a_value_type() {
    let version = get_usbd_i_version();
    assert_eq!(version, Clone::clone(&version));
    assert!(format!("{version:?}").contains("768"));
}
