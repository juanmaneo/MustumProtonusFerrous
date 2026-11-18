use super::*;

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn entry(report_id: u8, collection_number: u8, lengths: (u16, u16, u16)) -> ReportIds {
    ReportIds {
        report_id,
        collection_number,
        input_length: lengths.0,
        output_length: lengths.1,
        feature_length: lengths.2,
    }
}

#[test]
fn new_queue_is_empty_at_default_length() {
    let mut queue = ReportQueue::<u8>::new();
    assert_eq!(queue.length(), DEFAULT_LENGTH);
    assert_eq!((queue.read_index(), queue.write_index()), (0, 0));
    assert_eq!(queue.pop(), None);
}

#[test]
fn default_queue_matches_new() {
    assert_eq!(ReportQueue::<u8>::default(), ReportQueue::<u8>::new());
}

#[test]
fn pop_on_empty_queue_keeps_indices() {
    let mut queue = ReportQueue::<u8>::new();
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.pop(), None);
    assert_eq!((queue.read_index(), queue.write_index()), (0, 0));
}

#[test]
fn push_then_pop_is_first_in_first_out() {
    let mut queue = ReportQueue::new();
    queue.resize(4).unwrap();
    queue.push(1u8);
    queue.push(2);
    queue.push(3);
    assert_eq!(queue.write_index(), 3);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.read_index(), 1);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!((queue.read_index(), queue.write_index()), (3, 3));
    assert_eq!(queue.pop(), None);
    assert_eq!((queue.read_index(), queue.write_index()), (3, 3));
}

#[test]
fn write_index_wraps_to_zero() {
    let mut queue = ReportQueue::new();
    queue.resize(2).unwrap();
    queue.push(1u8);
    assert_eq!(queue.write_index(), 1);
    queue.push(2);
    assert_eq!((queue.read_index(), queue.write_index()), (1, 0));
}

#[test]
fn read_index_wraps_to_zero_on_overflow() {
    let mut queue = ReportQueue::new();
    queue.resize(2).unwrap();
    for report in 1u8..=3 {
        queue.push(report);
    }
    assert_eq!((queue.read_index(), queue.write_index()), (0, 1));
}

#[test]
fn pop_wraps_read_index_to_zero() {
    let mut queue = ReportQueue::new();
    queue.resize(2).unwrap();
    queue.push(1u8);
    queue.push(2);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!((queue.read_index(), queue.write_index()), (0, 0));
}

#[test]
fn overflow_drops_the_oldest_report() {
    let mut queue = ReportQueue::new();
    queue.resize(2).unwrap();
    queue.push(1u8);
    queue.push(2);
    assert_eq!(queue.read_index(), 1);
    assert_eq!(queue.pop(), Some(2));
}

#[test]
fn drained_overflowed_ring_returns_stale_slot_like_c() {
    // hid_queue_pop_report only advances when the slot is not the write index,
    // so after an overflow the report left in the write slot comes out once
    // more, as the verbatim C does (harness rows q_wrap_length2 and q_overwrite_40).
    let mut queue = ReportQueue::new();
    queue.resize(2).unwrap();
    queue.push(1u8);
    queue.push(2);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), None);
}

#[test]
fn resize_rejects_lengths_outside_the_allowed_range() {
    let mut queue = ReportQueue::new();
    queue.push(9u8);
    for length in [0, 1, MAX_LENGTH + 1] {
        assert_eq!(queue.resize(length), Err(STATUS_INVALID_PARAMETER));
        assert_eq!(queue.length(), DEFAULT_LENGTH);
    }
    assert_eq!(queue.pop(), Some(9));
}

#[test]
fn resize_accepts_both_bounds() {
    let mut queue = ReportQueue::<u8>::new();
    assert_eq!(queue.resize(MIN_LENGTH), Ok(()));
    assert_eq!(queue.length(), MIN_LENGTH);
    assert_eq!(queue.resize(MAX_LENGTH), Ok(()));
    assert_eq!(queue.length(), MAX_LENGTH);
}

#[test]
fn resize_to_the_current_length_keeps_reports() {
    let mut queue = ReportQueue::new();
    queue.push(7u8);
    assert_eq!(queue.resize(DEFAULT_LENGTH), Ok(()));
    assert_eq!(queue.pop(), Some(7));
}

#[test]
fn resize_to_a_new_length_drops_reports_and_resets_indices() {
    let mut queue = ReportQueue::new();
    queue.push(7u8);
    queue.push(8);
    assert_eq!(queue.resize(8), Ok(()));
    assert_eq!(
        (queue.read_index(), queue.write_index(), queue.length()),
        (0, 0, 8)
    );
    assert_eq!(queue.pop(), None);
}

#[test]
fn report_zero_fills_past_the_payload() {
    let report = HidReport::new(&[1, 2], 4).unwrap();
    assert_eq!(report.buffer(), [1u8, 2, 0, 0]);
}

#[test]
fn report_with_exact_payload_length() {
    let report = HidReport::new(&[1, 2], 2).unwrap();
    assert_eq!(report.buffer(), [1u8, 2]);
}

#[test]
fn report_rejects_a_payload_longer_than_the_report() {
    assert_eq!(HidReport::new(&[1, 2, 3], 2), None);
}

#[test]
fn empty_report_and_empty_payload() {
    assert_eq!(HidReport::new(&[], 0).unwrap().buffer(), [0u8; 0]);
    assert_eq!(HidReport::new(&[], 3).unwrap().buffer(), [0u8, 0, 0]);
}

#[test]
fn find_report_on_empty_description_is_none() {
    assert_eq!(
        find_report_with_type_and_id(&[], 0, ReportType::Input, 1, true),
        None
    );
}

#[test]
fn find_report_returns_the_first_match() {
    let reports = [entry(1, 1, (8, 0, 0)), entry(1, 1, (4, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 1, false),
        Some(0)
    );
}

#[test]
fn find_report_skips_entries_of_the_wrong_length_for_the_type() {
    let reports = [entry(1, 1, (8, 0, 0)), entry(1, 1, (0, 0, 4))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 1, false),
        Some(0)
    );
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Output, 1, false),
        None
    );
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Feature, 1, false),
        Some(1)
    );
}

#[test]
fn find_report_collection_zero_matches_any_collection() {
    let reports = [entry(0, 3, (8, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 0, false),
        Some(0)
    );
}

#[test]
fn find_report_collection_must_match_when_given() {
    let reports = [entry(0, 2, (8, 0, 0)), entry(0, 1, (8, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 1, ReportType::Input, 0, false),
        Some(1)
    );
    assert_eq!(
        find_report_with_type_and_id(&reports, 4, ReportType::Input, 0, false),
        None
    );
}

#[test]
fn find_report_id_zero_matches_any_id() {
    let reports = [entry(0, 1, (8, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 9, false),
        Some(0)
    );
}

#[test]
fn find_report_id_must_match_when_not_any_id() {
    let reports = [entry(1, 1, (8, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 2, false),
        None
    );
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 1, false),
        Some(0)
    );
}

#[test]
fn find_report_any_id_ignores_the_id() {
    let reports = [entry(1, 1, (8, 0, 0))];
    assert_eq!(
        find_report_with_type_and_id(&reports, 0, ReportType::Input, 2, true),
        Some(0)
    );
}

#[test]
fn every_table_entry_is_found_with_its_product() {
    for (id, product) in crate::strings::DEVICE_STRINGS {
        assert_eq!(id.len(), 17, "match id {id}");
        let device_id = utf16(&format!("HID\\{id}"));
        assert_eq!(
            find_device_string(&device_id, HID_STRING_ID_IPRODUCT),
            Some(*product)
        );
    }
}

#[test]
fn only_the_product_index_is_answered() {
    let device_id = utf16("HID\\VID_045E&PID_028E");
    assert_eq!(find_device_string(&device_id, 0), None);
    assert_eq!(find_device_string(&device_id, 14), None);
    assert_eq!(find_device_string(&device_id, 16), None);
}

#[test]
fn device_id_without_a_backslash_is_none() {
    assert_eq!(find_device_string(&utf16("HIDVID_045E&PID_028E"), 15), None);
}

#[test]
fn match_id_shorter_than_the_table_id_is_none() {
    assert_eq!(
        find_device_string(&utf16("HID\\VID_045E&PID_028"), 15),
        None
    );
}

#[test]
fn only_the_segment_after_the_last_backslash_is_matched() {
    assert_eq!(
        find_device_string(&utf16("USB\\VID_045E&PID_028E\\7&1"), 15),
        None
    );
    assert_eq!(
        find_device_string(&utf16("USB\\7&1\\VID_045E&PID_028E"), 15),
        Some("Controller (XBOX 360 For Windows)")
    );
}

#[test]
fn match_is_ascii_case_insensitive() {
    assert_eq!(
        find_device_string(&utf16("hid\\vid_045e&pid_028e"), 15),
        Some("Controller (XBOX 360 For Windows)")
    );
}

#[test]
fn non_ascii_letters_are_not_folded() {
    assert_eq!(
        find_device_string(&utf16("HID\\V\u{130}D_045E&PID_028E"), 15),
        None
    );
}

#[test]
fn unknown_product_id_is_none() {
    assert_eq!(
        find_device_string(&utf16("HID\\VID_045E&PID_9999"), 15),
        None
    );
}
