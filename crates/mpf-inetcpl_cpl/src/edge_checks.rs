//! Edge cases of the level selection that the oracle rows do not reach.

use super::*;

#[test]
fn every_template_maps_to_its_position() {
    for (position, &template) in URL_TEMPLATES.iter().enumerate() {
        assert_eq!(index_from_urltemplate(template), position as u32);
    }
}

#[test]
fn values_between_templates_are_unsupported() {
    for value in [0x1, 0x10499, 0x10501, 0x11001, 0x12001, 0x1ffff, 0x20000] {
        assert_eq!(index_from_urltemplate(value), u32::MAX, "value {value:#x}");
    }
}

#[test]
fn trackbar_index_overrides_unsupported_level() {
    assert_eq!(current_index(2, 0x99), 2);
    assert_eq!(current_index(u32::MAX, URLTEMPLATE_HIGH), u32::MAX);
}

#[test]
fn unsupported_level_without_trackbar_index_wraps() {
    assert_eq!(current_index(0, 0x99), u32::MAX);
    assert_eq!(trackbar_pos(current_index(0, 0x99)), 6);
}

#[test]
fn trackbar_pos_wraps_past_the_last_position() {
    assert_eq!(trackbar_pos(NUM_TRACKBAR_POS), 0);
    assert_eq!(trackbar_pos(NUM_TRACKBAR_POS + 1), u32::MAX);
}
