//! Safety and boundary cases for malformed format byte slices and masks outside Wine's mapping.

use crate::{formats_equal, static_mask_to_channels, WAVEFORMATEX_HEADER_SIZE};

fn header_with_extension(size: u16) -> Vec<u8> {
    let mut header = vec![0; WAVEFORMATEX_HEADER_SIZE];
    header[16..18].copy_from_slice(&size.to_le_bytes());
    header
}

#[test]
fn truncated_first_header_is_unequal() {
    assert!(!formats_equal(&[0; 17], &[0; 18]));
}

#[test]
fn truncated_second_header_is_unequal() {
    assert!(!formats_equal(&[0; 18], &[0; 17]));
}

#[test]
fn first_truncated_extension_is_unequal() {
    let first = header_with_extension(2);
    let second = [header_with_extension(2), vec![1, 2]].concat();
    assert!(!formats_equal(&first, &second));
}

#[test]
fn second_truncated_extension_is_unequal() {
    let first = [header_with_extension(2), vec![1, 2]].concat();
    let second = header_with_extension(2);
    assert!(!formats_equal(&first, &second));
}

#[test]
fn format_with_zero_extension_accepts_extra_trailing_data() {
    let header = header_with_extension(0);
    assert!(formats_equal(
        &header,
        &[header.as_slice(), &[0xaa]].concat()
    ));
}

#[test]
fn back_center_is_mapped_after_all_prior_channels() {
    let mapped = static_mask_to_channels(0x0002_0000);
    assert_eq!(mapped.channel_count, 1);
    assert_eq!(mapped.speaker_mask, 0x0000_0100);
    assert_eq!(mapped.channel_indices[12], 0);
    assert!(mapped.channel_indices[..12]
        .iter()
        .all(|&index| index == u32::MAX));
}

#[test]
fn multiple_objects_keep_wine_mapping_order() {
    let mapped = static_mask_to_channels(0x0000_0102);
    assert_eq!(mapped.channel_count, 2);
    assert_eq!(mapped.speaker_mask, 0x0000_0021);
    assert_eq!(mapped.channel_indices[0], 0);
    assert_eq!(mapped.channel_indices[7], 1);
}
