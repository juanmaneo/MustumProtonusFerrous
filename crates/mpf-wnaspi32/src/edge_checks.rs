//! Safety and boundary cases for malformed controller counts and status packing.

use crate::support_info;

#[test]
fn zero_count_reports_no_adapters_status() {
    let result = support_info(0);
    assert_eq!(result, 0x0000_e800);
    assert_eq!((result >> 8) & 0xFF, 0xE8);
    assert_eq!(result & 0xFF, 0x00);
}

#[test]
fn nonzero_count_reports_completed_status() {
    let result = support_info(1);
    assert_eq!((result >> 8) & 0xFF, 0x01);
    assert_eq!(result & 0xFF, 0x01);
}

#[test]
fn count_bits_are_preserved_in_low_byte() {
    let result = support_info(0xAB);
    assert_eq!(result & 0xFF, 0xAB);
    assert_eq!((result >> 8) & 0xFF, 0x01);
}

#[test]
fn count_above_255_overlaps_status_byte() {
    let result = support_info(0x1AB);
    assert_eq!(result, 0x0000_01AB);
    assert_eq!((result >> 8) & 0xFF, 0x01);
    assert_eq!(result & 0xFF, 0xAB);
}

#[test]
fn high_bits_of_count_are_preserved() {
    let result = support_info(0x1234_5678);
    assert_eq!(result, 0x1234_5778);
}

#[test]
fn max_u32_is_idempotent() {
    assert_eq!(support_info(u32::MAX), u32::MAX);
}
