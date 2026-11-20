//! Rust-only edge cases for the position table and the order check that the oracle rows do not
//! reach directly.

use crate::{channel_mask, speaker_from_position, SPEAKER_MAP};

#[test]
fn every_table_position_maps_to_its_speaker() {
    for (index, &speaker) in SPEAKER_MAP.iter().enumerate() {
        let position = i32::try_from(index).expect("small index");
        assert_eq!(
            speaker_from_position(position),
            speaker,
            "position {position}"
        );
        assert_eq!(channel_mask(&[position]), speaker, "position {position}");
    }
}

#[test]
fn positions_without_a_speaker_fail_the_layout() {
    for position in [9, 18, 19, -1, -3, 21, i32::MIN, i32::MAX] {
        assert_eq!(speaker_from_position(position), 0, "position {position}");
        assert_eq!(channel_mask(&[position]), 0, "position {position}");
    }
}

#[test]
fn a_missing_speaker_after_valid_channels_fails_the_layout() {
    assert_eq!(channel_mask(&[0, 9]), 0);
    assert_eq!(channel_mask(&[0, 1, 21]), 0);
}

#[test]
fn order_check_refuses_repeated_and_lower_speakers() {
    assert_eq!(channel_mask(&[0, 0]), 0);
    assert_eq!(channel_mask(&[1, 0]), 0);
    assert_eq!(channel_mask(&[2, 0]), 0);
    assert_eq!(channel_mask(&[0, 2, 1]), 0);
}

#[test]
fn order_check_allows_a_gap_above_the_mask() {
    assert_eq!(channel_mask(&[0, 8]), 0x101);
    assert_eq!(channel_mask(&[0, 1, 2, 8]), 0x107);
    assert_eq!(channel_mask(&[]), 0);
}
