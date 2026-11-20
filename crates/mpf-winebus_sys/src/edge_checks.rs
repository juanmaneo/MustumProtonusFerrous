use super::*;
use crate::effect::{FF_PERIODIC, FF_SINE, FF_SPRING};

fn layout_at(bit_size: u32) -> HidDeviceLayout {
    HidDeviceLayout {
        bit_size,
        ..Default::default()
    }
}

/// The direction table of `hatswitch_decompose`, written as the C `if` chain.
fn reference_decompose(value: u8, index: u32) -> (i32, i32) {
    let value = if index % 2 == 1 {
        value >> 4
    } else {
        value & 0x0f
    };
    let (mut x, mut y) = (0, 0);
    if value == 8 || value == 1 || value == 2 {
        y = -1;
    }
    if value == 6 || value == 5 || value == 4 {
        y = 1;
    }
    if value == 8 || value == 7 || value == 6 {
        x = -1;
    }
    if value == 2 || value == 3 || value == 4 {
        x = 1;
    }
    (x, y)
}

#[test]
fn default_layout_is_empty() {
    let layout = HidDeviceLayout::default();
    assert_eq!(layout.bit_size, 0);
    assert_eq!(layout.abs_axis_count + layout.rel_axis_count, 0);
    assert_eq!(layout.button_count, 0);
    assert_eq!(layout.hatswitch_count, 0);
}

#[test]
fn axis_size_boundaries() {
    assert_eq!(determine_axis_size(0, 255), 8);
    assert_eq!(determine_axis_size(-128, 127), 8);
    assert_eq!(determine_axis_size(0, 256), 16);
    assert_eq!(determine_axis_size(-129, 0), 16);
    assert_eq!(determine_axis_size(-128, 128), 16);
    assert_eq!(determine_axis_size(-32768, 32767), 16);
    assert_eq!(determine_axis_size(0, 65536), 32);
    assert_eq!(determine_axis_size(-32769, 0), 32);
}

#[test]
fn buttons_stop_at_the_report_limit() {
    let mut layout = layout_at(0x7_FFF8);
    assert!(layout.add_button_count(8));
    assert_eq!(layout.bit_size, REPORT_BIT_LIMIT);
    assert_eq!(layout.button_start, 0xFFFF);
    assert!(!layout.add_button_count(1));
    assert_eq!(layout.button_count, 8);
}

#[test]
fn unaligned_first_button_is_rejected() {
    let mut layout = layout_at(9);
    assert!(!layout.add_button_count(1));
    assert_eq!(layout.button_count, 0);
}

#[test]
fn hatswitch_rejects_unaligned_start_and_report_overflow() {
    assert!(!layout_at(9).add_hatswitch_count(1));

    let mut layout = layout_at(0x7_FFF8);
    assert!(layout.add_hatswitch_count(1));
    assert_eq!(layout.bit_size, REPORT_BIT_LIMIT);
    assert!(!layout.add_hatswitch_count(2));
    assert_eq!(layout.hatswitch_count, 1);
}

#[test]
fn relative_axes_follow_absolute_axes() {
    let mut layout = layout_at(8);
    assert!(layout.add_axis_count(false, 8, 1, 1, &[0x30]));
    assert!(layout.add_axis_count(true, 8, 1, 1, &[]));
    assert!(!layout.add_axis_count(false, 8, 1, 1, &[0x31]));
    assert_eq!(layout.abs_axis_count, 1);
    assert_eq!(layout.rel_axis_count, 1);
}

#[test]
fn absolute_axis_records_its_usage_page() {
    let mut layout = layout_at(8);
    assert!(layout.add_axis_count(false, 16, 1, 0x0c, &[0x30]));
    assert_eq!(
        layout.abs_axis_usages[0],
        UsageAndPage {
            usage: 0x30,
            usage_page: 0x0c,
        }
    );
    assert_eq!(layout.axis_sizes[0], 16);
    assert_eq!(layout.axis_byte_offsets[0], 1);
}

#[test]
fn axes_are_rejected_when_unaligned_or_past_the_report_limit() {
    assert!(!layout_at(9).add_axis_count(false, 8, 1, 1, &[0x30]));

    let mut layout = layout_at(0x7_FFF8);
    assert!(!layout.add_axis_count(false, 8, 2, 1, &[0x30, 0x31]));
    assert!(layout.add_axis_count(false, 8, 1, 1, &[0x30]));
    assert_eq!(layout.bit_size, REPORT_BIT_LIMIT);
}

#[test]
fn unknown_axis_size_writes_nothing() {
    let mut layout = layout_at(8);
    assert!(layout.add_axis_count(false, 8, 1, 1, &[0x30]));
    layout.axis_sizes[0] = 0;
    let mut report = [0u8; 2];
    assert!(!layout.set_abs_axis(&mut report, 0, 5));
    assert_eq!(report, [0, 0]);
}

#[test]
fn writers_reject_indices_past_their_counts() {
    let mut layout = layout_at(8);
    assert!(layout.add_axis_count(false, 8, 1, 1, &[0x30]));
    assert!(layout.add_axis_count(true, 16, 1, 1, &[]));
    assert!(layout.add_hatswitch_count(1));
    assert!(layout.add_button_count(2));
    let mut report = [0u8; 6];

    assert!(!layout.set_abs_axis(&mut report, 1, 1));
    assert!(!layout.set_rel_axis(&mut report, 1, 1));
    assert!(!layout.set_button(&mut report, 2, true));
    assert!(!layout.set_hatswitch_x(&mut report, 2, 1));
    assert!(!layout.set_hatswitch_y(&mut report, 2, 1));
    assert!(!layout.move_hatswitch(&mut report, 2, 1, 1));
    assert_eq!(report, [0; 6]);

    assert!(layout.set_hatswitch_x(&mut report, 1, 1));
    assert_eq!(report[4], 0x30);
    assert!(layout.set_button(&mut report, 1, true));
    assert_eq!(report[5], 0x02);
}

#[test]
fn hatswitch_setters_keep_the_other_direction() {
    let mut layout = layout_at(8);
    assert!(layout.add_hatswitch_count(2));
    let mut report = [0u8; 3];
    assert!(layout.set_hatswitch_y(&mut report, 0, -1));
    assert!(layout.set_hatswitch_x(&mut report, 0, 1));
    assert_eq!(report[1], 0x02);
    assert!(layout.set_hatswitch_y(&mut report, 1, 1));
    assert_eq!(report[1], 0x52);
}

#[test]
fn sync_restores_a_dropped_report_once() {
    let mut report = [1u8, 2];
    let mut last = [3u8, 4];
    let mut dropped = false;
    drop_report(&mut dropped);
    assert!(!sync_report(&mut report, &mut last, &mut dropped));
    assert_eq!(report, [3, 4]);
    assert!(!dropped);
    assert!(sync_report(&mut report, &mut last, &mut dropped));
}

#[test]
fn decompose_matches_the_c_direction_table_for_every_byte() {
    for index in [0, 1] {
        for value in 0..=255u8 {
            assert_eq!(
                hatswitch_decompose(value, index),
                reference_decompose(value, index)
            );
        }
    }
}

#[test]
fn compose_keeps_the_other_nibble_and_round_trips() {
    for index in [0, 1] {
        for x in -2..=2 {
            for y in -2..=2 {
                let mut value = 0xab;
                hatswitch_compose(x, y, &mut value, index);
                assert_eq!(hatswitch_decompose(value, index), (x.signum(), y.signum()));
                let other_nibble = if index % 2 == 1 { 0x0f } else { 0xf0 };
                assert_eq!(value & other_nibble, 0xab & other_nibble);
            }
        }
    }
    let mut value = 0xab;
    hatswitch_compose(0, 0, &mut value, 1);
    assert_eq!(value, 0x0b);
    hatswitch_compose(0, 0, &mut value, 0);
    assert_eq!(value, 0x00);
}

#[test]
fn effect_usages_map_to_periodic_and_plain_types() {
    assert_eq!(
        ff_effect_type_from_usage(0x31),
        Some(FfEffectType {
            effect_type: FF_PERIODIC,
            waveform: Some(FF_SINE),
        })
    );
    assert_eq!(
        ff_effect_type_from_usage(0x40),
        Some(FfEffectType {
            effect_type: FF_SPRING,
            waveform: None,
        })
    );
    assert_eq!(ff_effect_type_from_usage(0x35), None);
}
