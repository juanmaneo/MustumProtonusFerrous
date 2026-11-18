//! Checks the public API on descriptors chosen for their edges: truncated
//! items, the value list limit, the stacks, and each kind of item.

use super::*;
use std::fmt::Debug;

fn parsed(desc: &[u8]) -> PreparsedData {
    parse_descriptor(desc).expect("descriptor parses")
}

fn check_traits<T: Clone + Debug + PartialEq>(value: &T) {
    assert_eq!(&value.clone(), value);
    assert!(!format!("{value:?}").is_empty());
}

fn input_after_usages(usages: usize) -> Vec<u8> {
    let mut desc = vec![0x75, 0x01, 0x95, 0x01, 0x09, 0x01, 0x81, 0x02];
    desc.extend([0x09, 0x02].repeat(usages));
    desc.extend([0x81, 0x02]);
    desc
}

#[test]
fn truncated_items_fail_the_parse() {
    assert!(parse_descriptor(&[0x92, 0x01]).is_none());
    assert!(parse_descriptor(&[0x91]).is_none());
    let output = parsed(&[0x92, 0x01, 0x00]).output;
    assert_eq!((output.count, output.byte_length), (1, 1));
    assert!(output.values.is_empty());
}

#[test]
fn top_level_bounds_follow_the_outer_collections() {
    assert_eq!(
        top_level_collections(&[0xA1, 0x01, 0xC0, 0x92, 0x01]),
        vec![0, 3]
    );
    assert_eq!(top_level_collections(&[]), vec![0]);
    assert_eq!(top_level_collections(&[0xC0, 0xA1, 0x01, 0xC0]), vec![0]);
}

#[test]
fn usage_list_holds_255_usages() {
    assert!(parse_descriptor(&[0x09, 0x01].repeat(255)).is_some());
    assert!(parse_descriptor(&[0x09, 0x01].repeat(256)).is_none());
}

#[test]
fn value_list_fails_past_its_grown_size() {
    assert!(parse_descriptor(&input_after_usages(49)).is_none());
    assert_eq!(parsed(&input_after_usages(30)).input.values.len(), 31);
}

#[test]
fn unbalanced_items_are_rejected() {
    assert!(parse_descriptor(&[0xC0]).is_none());
    assert!(parse_descriptor(&[0xB4]).is_none());
    assert!(parse_descriptor(&[0xA8, 0x01]).is_none());
    assert!(parse_descriptor(&[0xA1, 0x01, 0xC0]).is_some());
}

#[test]
fn push_and_pop_restore_the_globals() {
    let desc = [
        0x05, 0x01, 0xA4, 0x05, 0x09, 0xB4, 0xA4, 0xB4, 0x09, 0x30, 0x75, 0x08, 0x95, 0x01, 0x81,
        0x02,
    ];
    let value = parsed(&desc).input.values[0];
    assert_eq!((value.usage_page, value.usage_min), (1, 0x30));
}

#[test]
fn signed_globals_keep_their_sign() {
    let desc = [
        0x16, 0x00, 0x80, 0x27, 0xFF, 0xFF, 0xFF, 0x7F, 0x35, 0x81, 0x45, 0xFF, 0x55, 0xFE, 0x65,
        0x01, 0x75, 0x08, 0x95, 0x01, 0x09, 0x30, 0x81, 0x02,
    ];
    let value = parsed(&desc).input.values[0];
    assert_eq!(value.logical_min, -32768);
    assert_eq!(value.logical_max, 2147483647);
    assert_eq!((value.physical_min, value.physical_max), (-127, -1));
    assert_eq!((value.units_exp, value.units), (-2, 1));
}

#[test]
fn local_designators_and_strings() {
    let desc = [
        0x75, 0x01, 0x95, 0x01, 0x39, 0x02, 0x49, 0x01, 0x59, 0x04, 0x79, 0x05, 0x89, 0x06, 0x99,
        0x07, 0x81, 0x02,
    ];
    let value = parsed(&desc).input.values[0];
    assert_eq!((value.designator_min, value.designator_max), (1, 4));
    assert_eq!((value.string_min, value.string_max), (6, 7));
    assert_eq!(value.flags & 0xC0, 0xC0);
}

#[test]
fn usage_ranges_and_their_resets() {
    let desc = [
        0x05, 0x09, 0x19, 0x01, 0x29, 0x03, 0x75, 0x01, 0x95, 0x03, 0x81, 0x02, 0x29, 0x05, 0x19,
        0x04, 0x81, 0x02, 0x19, 0x01, 0x29, 0x03, 0x09, 0x09, 0x81, 0x02,
    ];
    let values = parsed(&desc).input.values;
    let ranges: Vec<(u16, u16)> = values
        .iter()
        .map(|value| (value.usage_min, value.usage_max))
        .collect();
    assert_eq!(ranges, vec![(1, 3), (4, 5), (9, 9)]);
    assert!(values.iter().all(|value| value.usage_page == 9));
}

#[test]
fn four_byte_usage_sets_the_page() {
    let desc = [
        0x0B, 0x01, 0x00, 0x07, 0x00, 0x75, 0x01, 0x95, 0x01, 0x81, 0x02,
    ];
    let value = parsed(&desc).input.values[0];
    assert_eq!((value.usage_page, value.usage_min), (7, 1));
}

#[test]
fn null_and_constant_flags() {
    let null = parsed(&[0x75, 0x01, 0x95, 0x01, 0x81, 0x42]).input.values[0];
    assert_eq!(null.null_value, 1);
    let constant = parsed(&[0x75, 0x01, 0x95, 0x01, 0x09, 0x01, 0x81, 0x07])
        .input
        .values[0];
    assert_ne!(constant.flags & HID_VALUE_CAPS_IS_CONSTANT, 0);
    assert_eq!(constant.flags & HID_VALUE_CAPS_IS_ABSOLUTE, 0);
}

#[test]
fn zero_report_count_counts_empty_values() {
    let report = parsed(&[0x95, 0x00, 0x09, 0x01, 0x09, 0x01, 0x81, 0x02]).input;
    assert_eq!((report.count, report.byte_length), (2, 1));
    assert!(report.values.is_empty());
}

#[test]
fn array_with_two_usages_is_stored_in_reverse() {
    let input = parsed(&[0x75, 0x08, 0x95, 0x02, 0x09, 0x01, 0x09, 0x02, 0x81, 0x00]).input;
    assert_eq!(input.byte_length, 3);
    let (first, second) = (input.values[0], input.values[1]);
    assert_eq!((first.usage_min, second.usage_min), (2, 1));
    assert_ne!(first.flags & HID_VALUE_CAPS_ARRAY_HAS_MORE, 0);
    assert_eq!(second.flags & HID_VALUE_CAPS_ARRAY_HAS_MORE, 0);
    assert_ne!(second.flags & HID_VALUE_CAPS_IS_BUTTON, 0);
    assert_eq!((first.start_byte, first.start_bit), (1, 0));
}

#[test]
fn value_without_usage_has_the_full_index_range() {
    let value = parsed(&[0x75, 0x01, 0x95, 0x01, 0x81, 0x02]).input.values[0];
    assert_eq!((value.usage_min, value.usage_max), (0, 0));
    assert_eq!((value.data_index_min, value.data_index_max), (0, 0xFFFF));
}

#[test]
fn collection_tree_links_its_children() {
    let desc = [
        0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x09, 0x07, 0xA1, 0x00, 0xC0, 0x09, 0x08, 0xA1, 0x00,
        0xC0, 0xC0,
    ];
    let data = parsed(&desc);
    assert_eq!((data.usage_page, data.usage), (1, 6));
    let node =
        |usage, parent, children, collection_type, next_sibling, first_child| CollectionNode {
            usage_page: 1,
            usage,
            parent,
            number_of_children: children,
            next_sibling,
            first_child,
            collection_type,
        };
    assert_eq!(
        data.collections,
        vec![
            node(6, 0, 2, 1, 0, 2),
            node(7, 0, 0, 0, 0, 0),
            node(8, 0, 0, 0, 1, 0)
        ]
    );
}

#[test]
fn reserved_items_are_ignored() {
    assert!(parse_descriptor(&[0xFE, 0x02, 0x00, 0x05, 0x01]).is_some());
}

#[test]
fn device_description_of_empty_and_truncated_descriptors() {
    let empty = get_collection_description(&[]).unwrap();
    assert_eq!(empty.collections, vec![]);
    assert_eq!(empty.report_ids, vec![]);

    let truncated = get_collection_description(&[0xA1, 0x01, 0xC0, 0x92]).unwrap();
    assert_eq!(truncated.collections.len(), 1);
    assert!(truncated.report_ids.is_empty());
}

#[test]
fn device_description_lists_feature_reports() {
    let desc = [
        0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x85, 0x01, 0x75, 0x08, 0x95, 0x02, 0x09, 0x01, 0xB1,
        0x02, 0xC0,
    ];
    let device = get_collection_description(&desc).unwrap();
    let collection = &device.collections[0];
    assert_eq!(
        (
            collection.usage_page,
            collection.usage,
            collection.collection_number
        ),
        (1, 6, 1)
    );
    assert_eq!(collection.feature_length, 3);
    assert_eq!(
        device.report_ids,
        vec![ReportIdDesc {
            report_id: 1,
            collection_number: 1,
            input_length: 0,
            output_length: 0,
            feature_length: 3,
        }]
    );
}

#[test]
fn device_description_fails_on_an_unparsed_collection() {
    assert_eq!(
        get_collection_description(&[0xA1, 0x01, 0xA8, 0xC0]),
        Err(HIDP_STATUS_INTERNAL_ERROR)
    );
}

#[test]
fn public_types_support_their_derived_traits() {
    check_traits(&ValueCaps::default());
    let data = parsed(&[
        0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x09, 0x07, 0xC0, 0x75, 0x01, 0x95, 0x01, 0x81, 0x02,
    ]);
    check_traits(&data);
    check_traits(&data.collections[0]);
    check_traits(&data.input);
    let device = get_collection_description(&[
        0xA1, 0x01, 0x09, 0x01, 0x75, 0x01, 0x95, 0x01, 0x81, 0x02, 0xC0,
    ])
    .unwrap();
    check_traits(&device);
    check_traits(&device.collections[0]);
    check_traits(&device.report_ids[0]);
}
