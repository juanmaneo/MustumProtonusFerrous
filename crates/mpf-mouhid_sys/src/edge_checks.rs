//! Edge-case tests for the ported mouhid.sys logic.
//!
//! These cover boundary values and every branch of the conversions, `MulDiv` and the contact
//! state machine. Expectations that are not oracle-derived are marked with a comment: they were
//! worked out by hand from the Wine C source, or they pin the x86 behaviour chosen for the
//! undefined behaviour in the C.

use super::*;

const PAGE_GENERIC: u16 = 0x01;
const GENERIC_X: u16 = 0x30;
const GENERIC_Y: u16 = 0x31;
const PAGE_DIGITIZER: u16 = 0x0d;
const DIGITIZER_TIP_SWITCH: u16 = 0x42;
const DIGITIZER_CONTACT_ID: u16 = 0x51;
const DIGITIZER_CONTACT_COUNT: u16 = 0x54;

type Fields = Vec<(u16, u16, u16, u32)>;

/// `ReportReader` over `(page, collection, usage, value)` tuples.
struct Report<'a>(&'a [(u16, u16, u16, u32)]);

impl ReportReader for Report<'_> {
    fn usage_value(&self, page: u16, collection: u16, usage: u16) -> Option<u32> {
        self.0
            .iter()
            .find(|&&(p, c, u, _)| p == page && c == collection && u == usage)
            .map(|&(_, _, _, value)| value)
    }

    fn usages(&self, page: u16, collection: u16, usage: u16) -> Option<u32> {
        self.usage_value(page, collection, usage)
            .map(|value| u32::from(value != 0))
    }
}

fn value_caps(bit_size: u16, logical_min: i32, logical_max: i32) -> ValueCaps {
    ValueCaps {
        bit_size,
        logical_min,
        logical_max,
    }
}

fn contact_caps(link_collection: u16) -> ContactCaps {
    ContactCaps {
        link_collection,
        x: value_caps(16, 0, 4095),
        y: value_caps(16, 0, 4095),
    }
}

fn count(value: u32) -> Fields {
    vec![(PAGE_DIGITIZER, 0, DIGITIZER_CONTACT_COUNT, value)]
}

/// The fields of a touching contact in `collection`, at raw position `(x, y)`.
fn touch(collection: u16, id: u32, x: u32, y: u32) -> Fields {
    vec![
        (PAGE_DIGITIZER, collection, DIGITIZER_TIP_SWITCH, 1),
        (PAGE_DIGITIZER, collection, DIGITIZER_CONTACT_ID, id),
        (PAGE_GENERIC, collection, GENERIC_X, x),
        (PAGE_GENERIC, collection, GENERIC_Y, y),
    ]
}

/// The tip switch released in `collection`.
fn lift(collection: u16) -> Fields {
    vec![(PAGE_DIGITIZER, collection, DIGITIZER_TIP_SWITCH, 0)]
}

fn run(tracker: &mut ContactTracker, caps: &[ContactCaps], fields: &Fields) -> Vec<PointerEvent> {
    let mut events = Vec::new();
    tracker.process_report(&Report(fields), caps, &mut events);
    events
}

#[test]
fn sign_extend_passes_values_through_without_a_sign_bit() {
    // Not oracle-derived: BitSize 1 gives sign == 1, which returns the value unchanged.
    assert_eq!(sign_extend(1, &value_caps(1, -1, 1)), 1);
    assert_eq!(sign_extend(0xffff_ffff, &value_caps(1, -1, 1)), -1);
    // Not oracle-derived: an unsigned logical range never sign-extends.
    assert_eq!(sign_extend(0x80, &value_caps(8, 0, 255)), 0x80);
    assert_eq!(sign_extend(0xffff, &value_caps(16, 0, 65535)), 0xffff);
}

#[test]
fn sign_extend_wraps_signed_values() {
    // Not oracle-derived: hand-computed from `value - ((value & sign) << 1)`.
    assert_eq!(sign_extend(0x7f, &value_caps(8, -128, 127)), 127);
    assert_eq!(sign_extend(0x80, &value_caps(8, -128, 127)), -128);
    assert_eq!(sign_extend(0x7fff, &value_caps(16, -32768, 32767)), 32767);
    assert_eq!(sign_extend(0x8000, &value_caps(16, -32768, 32767)), -32768);
    // Not oracle-derived: for BitSize 32 the sign bit is the top bit, so the result is i32::MIN.
    assert_eq!(sign_extend(0x8000_0000, &value_caps(32, -1, 1)), i32::MIN);
}

#[test]
fn sign_extend_bit_size_zero_masks_the_shift_count() {
    // Not oracle-derived: BitSize 0 is undefined in C (excluded from the oracle). The port
    // takes the shift count modulo 32, so the sign bit is 0x8000_0000.
    assert_eq!(
        sign_extend(0x8000_0001, &value_caps(0, -1, 1)),
        i32::MIN + 1
    );
    assert_eq!(sign_extend(1, &value_caps(0, -1, 1)), 1);
}

#[test]
fn scale_value_maps_range_ends_onto_min_and_max() {
    // Not oracle-derived: the ends of the logical range map to the ends of min..=max.
    let range = value_caps(16, 0, 4095);
    assert_eq!(scale_value(0, &range, 0, 65535), 0);
    assert_eq!(scale_value(4095, &range, 0, 65535), 65535);
    assert_eq!(scale_value(0, &range, 10, 20), 10);
    assert_eq!(scale_value(4095, &range, 10, 20), 20);
    // Not oracle-derived: a signed range with a negative min and max.
    let signed = value_caps(8, -128, 127);
    assert_eq!(scale_value(0x80, &signed, -1000, 1000), -1000);
    assert_eq!(scale_value(0x7f, &signed, -1000, 1000), 1000);
}

#[test]
fn scale_value_outside_the_logical_range_is_zero() {
    // Not oracle-derived: values below logical_min or above logical_max scale to 0.
    assert_eq!(scale_value(4096, &value_caps(16, 0, 4095), 0, 65535), 0);
    assert_eq!(scale_value(50, &value_caps(8, 100, 200), 0, 65535), 0);
    assert_eq!(scale_value(201, &value_caps(8, 100, 200), 0, 65535), 0);
    // Not oracle-derived: a sign-extended value below a signed logical_min.
    assert_eq!(scale_value(0x80, &value_caps(8, -10, 10), 0, 65535), 0);
}

#[test]
fn mul_div_rounds_each_sign_combination_half_away_from_zero() {
    // Not oracle-derived: hand-computed from the Wine MulDiv rounding rule.
    assert_eq!(mul_div(-5, -3, 2), 8);
    assert_eq!(mul_div(5, -3, 2), -8);
    assert_eq!(mul_div(-3, 0, 2), 0);
    assert_eq!(mul_div(0, -3, 2), 0);
    assert_eq!(mul_div(0, 0, 2), 0);
    // Not oracle-derived: a negative divisor is negated before rounding.
    assert_eq!(mul_div(-5, 3, -2), 8);
}

#[test]
fn mul_div_result_range_boundaries() {
    // Not oracle-derived: the result must lie in -2147483647..=2147483647.
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
    assert_eq!(mul_div(i32::MAX, 2, 2), i32::MAX);
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
    // Not oracle-derived: negating INT_MIN is undefined in C. The port wraps it, so the
    // divisor stays negative and the quotient truncates to 0.
    assert_eq!(mul_div(1, 1, i32::MIN), 0);
}

#[test]
fn process_report_without_contact_count_keeps_contacts() {
    // Not oracle-derived: hand-written. The oracle covers the same path with other values.
    let caps = [contact_caps(1)];
    let mut tracker = ContactTracker::default();
    let events = run(
        &mut tracker,
        &caps,
        &[count(1), touch(1, 3, 4095, 0)].concat(),
    );
    assert_eq!(events.len(), 1);

    let events = run(&mut tracker, &caps, &touch(1, 3, 0, 4095));
    assert!(events.is_empty());
    assert_eq!(
        tracker.contacts(),
        &[Contact {
            id: 3,
            x: 65535,
            y: 0
        }]
    );
}

#[test]
fn tip_switch_zero_or_missing_releases_the_contact() {
    // Not oracle-derived: hand-written. A zero tip and a missing tip both skip the collection.
    let caps = [contact_caps(1)];
    let mut tracker = ContactTracker::default();
    run(&mut tracker, &caps, &[count(1), touch(1, 3, 0, 0)].concat());
    let events = run(&mut tracker, &caps, &[count(1), lift(1)].concat());
    assert_eq!(
        events,
        [PointerEvent {
            kind: PointerKind::Up,
            id: 3,
            flags: 0x4000,
            x: 0,
            y: 0,
        }]
    );
    assert!(tracker.contacts().is_empty());

    run(&mut tracker, &caps, &[count(1), touch(1, 3, 0, 0)].concat());
    let events = run(&mut tracker, &caps, &count(1));
    assert_eq!(
        events,
        [PointerEvent {
            kind: PointerKind::Up,
            id: 3,
            flags: 0x4000,
            x: 0,
            y: 0,
        }]
    );
    assert!(tracker.contacts().is_empty());
}

#[test]
fn missing_contact_field_releases_the_contact() {
    // Not oracle-derived: hand-written. Each of the four per-contact fields is removed in turn.
    let caps = [contact_caps(1)];
    for missing in 0..4 {
        let mut tracker = ContactTracker::default();
        run(&mut tracker, &caps, &[count(1), touch(1, 3, 0, 0)].concat());
        let mut fields = touch(1, 3, 0, 0);
        fields.remove(missing);
        let events = run(&mut tracker, &caps, &[count(1), fields].concat());
        assert_eq!(
            events,
            [PointerEvent {
                kind: PointerKind::Up,
                id: 3,
                flags: 0x4000,
                x: 0,
                y: 0,
            }],
            "missing field index {missing}"
        );
        assert!(tracker.contacts().is_empty());
    }
}

#[test]
fn update_moves_a_matching_contact_and_releases_the_rest() {
    // Not oracle-derived: hand-written. The matching contact is not the first in the old list.
    let caps = [contact_caps(1), contact_caps(2)];
    let mut tracker = ContactTracker::default();
    run(
        &mut tracker,
        &caps,
        &[count(2), touch(1, 1, 0, 0), touch(2, 2, 4095, 4095)].concat(),
    );
    let events = run(&mut tracker, &caps, &[count(1), touch(2, 2, 0, 0)].concat());
    assert_eq!(
        events,
        [
            PointerEvent {
                kind: PointerKind::Update,
                id: 2,
                flags: 0x4006,
                x: 0,
                y: 0,
            },
            PointerEvent {
                kind: PointerKind::Up,
                id: 1,
                flags: 0x4000,
                x: 0,
                y: 0,
            },
        ]
    );
    assert_eq!(tracker.contacts(), &[Contact { id: 2, x: 0, y: 0 }]);
}

#[test]
fn empty_caps_releases_every_contact_in_list_order() {
    // Not oracle-derived: hand-written. The same release order as the oracle removal scenario.
    let caps = [contact_caps(1), contact_caps(2)];
    let mut tracker = ContactTracker::default();
    run(
        &mut tracker,
        &caps,
        &[count(2), touch(1, 1, 0, 0), touch(2, 2, 4095, 4095)].concat(),
    );
    let events = run(&mut tracker, &[], &count(0));
    assert_eq!(
        events,
        [
            PointerEvent {
                kind: PointerKind::Up,
                id: 1,
                flags: 0x4000,
                x: 0,
                y: 0,
            },
            PointerEvent {
                kind: PointerKind::Up,
                id: 2,
                flags: 0x4000,
                x: 65535,
                y: 65535,
            },
        ]
    );
    assert!(tracker.contacts().is_empty());
}

#[test]
fn out_of_range_axis_reports_zero_instead_of_dropping_the_contact() {
    // Not oracle-derived: hand-written. A raw value outside the logical range scales to 0.
    let caps = [contact_caps(1)];
    let mut tracker = ContactTracker::default();
    let events = run(
        &mut tracker,
        &caps,
        &[count(1), touch(1, 5, 5000, 4095)].concat(),
    );
    assert_eq!(
        events,
        [PointerEvent {
            kind: PointerKind::Down,
            id: 5,
            flags: 0x4007,
            x: 0,
            y: 65535,
        }]
    );
}

#[test]
fn release_all_on_an_empty_tracker_sends_nothing() {
    // Not oracle-derived: hand-written.
    let mut tracker = ContactTracker::default();
    let mut events = Vec::new();
    tracker.release_all(&mut events);
    assert!(events.is_empty());
    assert!(tracker.contacts().is_empty());
}
