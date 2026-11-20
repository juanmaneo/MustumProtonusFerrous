//! Twenty rows emitted by the ASan/UBSan-compiled Wine C oracle in `target/canon/mmdevapi/oracle.c`.

use crate::{formats_equal, static_mask_to_channels, StaticChannelMap};

fn assert_mapping(mask: u32, count: u16, speaker_mask: u32, indices: [u32; 13]) {
    assert_eq!(
        static_mask_to_channels(mask),
        StaticChannelMap {
            channel_count: count,
            speaker_mask,
            channel_indices: indices,
        }
    );
}

#[test]
fn oracle_o01_empty_static_mask() {
    assert_mapping(0, 0, 0, [u32::MAX; 13]);
}

#[test]
fn oracle_o02_all_mapped_objects() {
    assert_mapping(
        0x0002_1ffe,
        13,
        0x0002_d73f,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    );
}

#[test]
fn oracle_o03_front_left() {
    assert_mapping(
        0x0000_0002,
        1,
        0x0000_0001,
        [
            0,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[test]
fn oracle_o04_front_stereo() {
    assert_mapping(
        0x0000_0006,
        2,
        0x0000_0003,
        [
            0,
            1,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[test]
fn oracle_o05_side_left_and_back_left() {
    assert_mapping(
        0x0000_00a0,
        2,
        0x0000_0210,
        [
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            0,
            u32::MAX,
            1,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[test]
fn oracle_o06_top_back_left() {
    assert_mapping(
        0x0001_0800,
        1,
        0x0000_8000,
        [
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            0,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[test]
fn oracle_o07_dynamic_object_is_ignored() {
    assert_mapping(0x0000_0001, 0, 0, [u32::MAX; 13]);
}

#[test]
fn oracle_o08_bottom_objects_are_ignored() {
    assert_mapping(0x0001_e000, 0, 0, [u32::MAX; 13]);
}

#[test]
fn oracle_o09_unknown_speaker_bit_is_ignored() {
    assert_mapping(0x8000_0000, 0, 0, [u32::MAX; 13]);
}

#[test]
fn oracle_o10_ignored_bits_do_not_change_mapped_output() {
    assert_mapping(
        0x8003_ffff,
        13,
        0x0002_d73f,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    );
}

fn base_format() -> [u8; 18] {
    let mut bytes = [0; 18];
    bytes[0..2].copy_from_slice(&3u16.to_le_bytes());
    bytes[2..4].copy_from_slice(&2u16.to_le_bytes());
    bytes[4..8].copy_from_slice(&48_000u32.to_le_bytes());
    bytes[8..12].copy_from_slice(&384_000u32.to_le_bytes());
    bytes[12..14].copy_from_slice(&8u16.to_le_bytes());
    bytes[14..16].copy_from_slice(&32u16.to_le_bytes());
    bytes
}

#[test]
fn oracle_o11_equal_base_format() {
    let bytes = base_format();
    assert!(formats_equal(&bytes, &bytes));
}

#[test]
fn oracle_o12_format_tag_mismatch() {
    let first = base_format();
    let mut second = first;
    second[0] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o13_channel_count_mismatch() {
    let first = base_format();
    let mut second = first;
    second[2] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o14_sample_rate_mismatch() {
    let first = base_format();
    let mut second = first;
    second[4] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o15_average_byte_rate_mismatch() {
    let first = base_format();
    let mut second = first;
    second[8] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o16_block_alignment_mismatch() {
    let first = base_format();
    let mut second = first;
    second[12] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o17_bits_per_sample_mismatch() {
    let first = base_format();
    let mut second = first;
    second[14] += 1;
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o18_extension_size_mismatch() {
    let mut first = base_format().to_vec();
    let mut second = first.clone();
    first[16] = 1;
    second[16] = 2;
    first.extend_from_slice(&[0, 0]);
    second.extend_from_slice(&[0, 0]);
    assert!(!formats_equal(&first, &second));
}

#[test]
fn oracle_o19_extension_match_ignores_trailing_bytes() {
    let mut first = base_format().to_vec();
    let mut second = first.clone();
    first[16] = 2;
    second[16] = 2;
    first.extend_from_slice(&[0x12, 0x34]);
    second.extend_from_slice(&[0x12, 0x34, 0x99]);
    assert!(formats_equal(&first, &second));
}

#[test]
fn oracle_o20_extension_mismatch() {
    let mut first = base_format().to_vec();
    let mut second = first.clone();
    first[16] = 2;
    second[16] = 2;
    first.extend_from_slice(&[0x12, 0x34]);
    second.extend_from_slice(&[0x12, 0x35]);
    assert!(!formats_equal(&first, &second));
}
