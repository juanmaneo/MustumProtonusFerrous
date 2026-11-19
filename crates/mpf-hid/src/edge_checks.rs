//! Edge cases of the port. Most rows are spec strings that the Wine C harness ran, so
//! their expected values come from C. The direct tests cover what C cannot answer: the
//! undefined-behaviour cases, the divergences listed in `lib.rs`, and the derived traits.

use super::*;
use crate::oracle_checks::replay;
use crate::value::{mul_div, sign_extend};

/// Edge specs recorded from the verbatim Wine C code: `(spec, harness output)`.
#[rustfmt::skip]
const EDGE_ROWS: &[(&str, &str)] = &[
    // get_usages_feature
    ("op=GetUsages rt=2 feat=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 r=000500 il=3 ol=3 fl=3", "st=00110000 u=1,3"),
    // get_usage_value_output
    ("op=GetUsageValue rt=1 out=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 r=007f00 il=3 ol=3 fl=3", "st=00110000 v=127"),
    // set_usage_value_output
    ("op=SetUsageValue rt=1 out=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 v=85 r=000000 il=3 ol=3 fl=3", "st=00110000 r=005500"),
    // set_data_output
    ("op=SetData rt=1 out=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=0:1 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000100"),
    // set_usages_feature
    ("op=SetUsages rt=2 feat=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=1,3 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000500"),
    // get_data_feature
    ("op=GetData rt=2 feat=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 r=000500 il=3 ol=3 fl=3", "st=00110000 d=0:1,2:1"),
    // get_specific_button_caps_feature
    ("op=GetSpecificButtonCaps rt=2 feat=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7", "st=00110000 caps=7:0:1:2:1:3"),
    // initialize_report_feature
    ("op=InitializeReportForID rt=2 feat=1:48:48:5:1:0:8:1:2:8:0:0:1:-127:127:0:0 rid=5 r=000000 il=3 ol=3 fl=3", "st=00110000 r=050000"),
    // max_usage_list_length_output
    ("op=MaxUsageListLength rt=1 out=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7", "n=3"),
    // max_data_list_length_output
    ("op=MaxDataListLength rt=1 out=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0", "n=3"),
    // get_usage_value_pg0
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=0 us=48 r=007f00 il=3 ol=3 fl=3", "st=c0110004"),
    // get_usage_value_collection_mismatch
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 co=2 us=48 r=007f00 il=3 ol=3 fl=3", "st=c0110004"),
    // get_usage_value_length_mismatch
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 il=5 r=007f00 ol=3 fl=3", "st=c0110003"),
    // get_usage_value_empty_report
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 r= il=0 ol=0 fl=0", "st=c0110003"),
    // get_usage_value_button_not_found
    ("op=GetUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,1:49:50:0:2:0:1:2:2:4:0:1:1:0:1:0:0 pg=1 us=49 r=007f03 il=3 ol=3 fl=3", "st=c0110004"),
    // get_scaled_identity
    ("op=GetScaledUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 r=007f00 il=3 ol=3 fl=3", "st=00110000 v=127"),
    // get_specific_value_caps_any_page
    ("op=GetSpecificValueCaps in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=0", "st=00110000 caps=1:0:1:2:48:48"),
    // get_specific_value_caps_usage_mismatch
    ("op=GetSpecificValueCaps in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=49", "st=c0110004"),
    // get_specific_button_caps_range
    ("op=GetSpecificButtonCaps in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7", "st=00110000 caps=7:0:1:0:4:7"),
    // get_specific_value_caps_overflow
    ("op=GetSpecificValueCaps in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 cap=1", "st=c0110007"),
    // max_usage_list_length_any_page
    ("op=MaxUsageListLength in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=0", "n=3"),
    // max_usage_list_length_no_match
    ("op=MaxUsageListLength in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=7", "n=0"),
    // max_data_list_length_unused_cap
    ("op=MaxDataListLength in=0:0:0:0:1:0:8:1:2:0:0:0:1:0:255:0:0,7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0", "n=3"),
    // get_usage_value_array_not_range
    ("op=GetUsageValueArray in=1:48:50:0:1:0:8:2:0:16:0:0:1:0:255:0:0 pg=1 us=48 vb=0000 r=00abcd il=3 ol=3 fl=3", "st=c011000b"),
    // get_usage_value_array_rc1
    ("op=GetUsageValueArray in=1:48:48:0:1:0:8:1:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=00 r=00ab il=2 ol=2 fl=2", "st=c011000b"),
    // get_usage_value_array_short
    ("op=GetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=00 r=00abcd il=3 ol=3 fl=3", "st=c0110007"),
    // get_usage_value_array_long
    ("op=GetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=ffffffff r=00abcd il=3 ol=3 fl=3", "st=00110000 vb=abcd0000"),
    // set_usage_value_array_short
    ("op=SetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=00 r=000000 il=3 ol=3 fl=3", "st=c0110007 r=000000"),
    // set_usage_value_array_long
    ("op=SetUsageValueArray in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 pg=1 us=48 vb=abcdef r=000000 il=3 ol=3 fl=3", "st=00110000 r=00abcd"),
    // get_scaled_out_of_range
    ("op=GetScaledUsageValue in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:100:0:100 pg=1 co=1 us=49 r=0000c8 il=3 ol=3 fl=3", "st=c0110005"),
    // get_scaled_bad_log
    ("op=GetScaledUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:127:-127:0:0 pg=1 us=48 r=007f00 il=3 ol=3 fl=3", "st=c0110006"),
    // get_scaled_bad_phys
    ("op=GetScaledUsageValue in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:100:0 pg=1 co=1 us=49 r=000080 il=3 ol=3 fl=3", "st=c0110006"),
    // set_scaled_identity
    ("op=SetScaledUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 pg=1 us=48 v=85 r=000000 il=3 ol=3 fl=3", "st=00110000 r=005500"),
    // set_scaled_bad_log
    ("op=SetScaledUsageValue in=1:48:48:0:1:0:8:1:2:8:0:0:1:127:-127:0:0 pg=1 us=48 v=5 r=000000 il=3 ol=3 fl=3", "st=c0110006 r=000000"),
    // set_scaled_bad_phys
    ("op=SetScaledUsageValue in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:100:0 pg=1 co=1 us=49 v=50 r=000000 il=3 ol=3 fl=3", "st=c0110006 r=000000"),
    // get_usages_collection_mismatch
    ("op=GetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 co=2 r=000500 il=3 ol=3 fl=3", "st=c0110004"),
    // get_usages_mixed_page
    ("op=GetUsages in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,1:49:50:0:2:0:1:2:2:4:0:1:1:0:1:0:0 pg=1 r=007f03 il=3 ol=3 fl=3", "st=00110000 u=49,50"),
    // get_usages_report_id_match
    ("op=GetUsages in=7:1:3:5:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 r=050500 il=3 ol=3 fl=3", "st=00110000 u=1,3"),
    // get_usages_report_id_mismatch
    ("op=GetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 r=050500 il=3 ol=3 fl=3", "st=c011000a"),
    // get_usages_chained
    ("op=GetUsages in=7:4:4:0:1:0:8:1:0:5:0:0:1:0:255:0:0,7:5:5:0:2:0:8:1:0:4:1:1:1:0:255:0:0 pg=7 r=000201 il=3 ol=3 fl=3", "st=00110000 u=4,5"),
    // get_usages_ex_output_too_small
    ("op=GetUsagesEx in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 r=000500 cap=1 il=3 ol=3 fl=3", "st=c0110007"),
    // set_usages_unknown_usage
    ("op=SetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=9 r=000000 il=3 ol=3 fl=3", "st=c0110004 r=000000"),
    // set_usages_array_index
    ("op=SetUsages in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7 ul=5 r=0000000000 il=5 ol=5 fl=5", "st=00110000 r=0002000000"),
    // set_usages_array_full
    ("op=SetUsages in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7 ul=4 r=0001020304 il=5 ol=5 fl=5", "st=c0110007 r=0001020304"),
    // unset_usages_not_set
    ("op=UnsetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=1 r=000000 il=3 ol=3 fl=3", "st=c011000f r=000000"),
    // unset_usages_unknown_usage
    ("op=UnsetUsages in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 pg=7 ul=9 r=000700 il=3 ol=3 fl=3", "st=c0110004 r=000700"),
    // unset_usages_array_empty
    ("op=UnsetUsages in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 pg=7 ul=5 r=0000000000 il=5 ol=5 fl=5", "st=c011000f r=0000000000"),
    // get_data_mixed
    ("op=GetData in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0,1:49:50:0:2:0:1:2:2:4:0:1:1:0:1:0:0 r=007f03 il=3 ol=3 fl=3", "st=00110000 d=0:127,0:1,1:1"),
    // get_data_chained
    ("op=GetData in=7:4:4:0:1:0:8:1:0:5:0:0:1:0:255:0:0,7:5:5:0:2:0:8:1:0:4:1:1:1:0:255:0:0 r=000201 il=3 ol=3 fl=3", "st=00110000 d=0:1,1:1"),
    // get_data_bit_size_zero
    ("op=GetData in=1:48:48:0:1:0:0:1:2:8:0:0:1:0:255:0:0 r=007f00 il=3 ol=3 fl=3", "st=00110000 d="),
    // get_data_value_count_not_one
    ("op=GetData in=1:48:48:0:1:0:8:2:2:8:0:0:1:0:255:0:0 r=007f00 il=3 ol=3 fl=3", "st=00110000 d="),
    // get_data_cap_zero_button
    ("op=GetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 r=000500 cap=0 il=3 ol=3 fl=3", "st=c0110007"),
    // get_data_value_no_room
    ("op=GetData in=1:48:48:0:1:0:8:1:2:8:0:0:1:-127:127:0:0 r=007f00 cap=0 il=3 ol=3 fl=3", "st=c0110007"),
    // set_data_button_release
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=0:0 r=000100 il=3 ol=3 fl=3", "st=00110000 r=000000"),
    // set_data_button_release_not_set
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=0:0 r=000000 il=3 ol=3 fl=3", "st=c011000f r=000000"),
    // set_data_second_item_fails
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=0:1,1:0 r=000000 il=3 ol=3 fl=3", "st=c011000f r=000100"),
    // set_data_not_found
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl=9:1 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // set_data_index_below_range
    ("op=SetData in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:0:100 dl=0:5 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // set_data_index_above_range
    ("op=SetData in=1:49:49:0:2:0:8:1:2:8:1:1:1:0:255:0:100 dl=2:5 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // set_data_no_items
    ("op=SetData in=7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 dl= r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // set_data_bit_size_zero
    ("op=SetData in=1:48:48:0:1:0:0:1:2:8:0:0:1:0:255:0:0 dl=0:55 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // set_data_value_count_not_one
    ("op=SetData in=1:48:48:0:1:0:8:2:2:8:0:0:1:0:255:0:0 dl=0:5 r=000000 il=3 ol=3 fl=3", "st=00110000 r=000000"),
    // set_data_is_value_array
    ("op=SetData in=1:48:48:0:1:0:8:2:0:8:0:0:1:0:255:0:0 dl=0:1 r=000000 il=3 ol=3 fl=3", "st=c011000c r=000000"),
    // set_data_array_press
    ("op=SetData in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 dl=0:1 r=0000000000 il=5 ol=5 fl=5", "st=00110000 r=0001000000"),
    // set_data_array_raw_zero_mismatch
    ("op=SetData in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 dl=0:0 r=0002000000 il=5 ol=5 fl=5", "st=c011000f r=0002000000"),
    // set_data_array_no_slot
    ("op=SetData in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 dl=0:5 r=0001020304 il=5 ol=5 fl=5", "st=c0110007 r=0001020304"),
    // set_data_array_release_shift
    ("op=SetData in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 dl=0:0 r=000102030405 il=6 ol=6 fl=6", "st=00110000 r=000203040005"),
    // set_data_array_release_absent
    ("op=SetData in=7:4:7:0:1:0:8:4:0:28:0:3:1:0:255:0:0 dl=0:0 r=0002030402 il=5 ol=5 fl=5", "st=c011000f r=0002030402"),
    // init_report_not_exist
    ("op=InitializeReportForID in=1:48:48:5:1:0:8:1:2:8:0:0:1:-127:127:0:0 rid=9 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // init_report_length_mismatch
    ("op=InitializeReportForID in=1:48:48:5:1:0:8:1:2:8:0:0:1:-127:127:0:0 rid=5 il=5 r=000000 ol=3 fl=3", "st=c0110003 r=000000"),
    // init_report_empty
    ("op=InitializeReportForID in=1:48:48:5:1:0:8:1:2:8:0:0:1:-127:127:0:0 rid=5 r= il=0 ol=0 fl=0", "st=c0110003 r="),
    // init_report_unused_cap
    ("op=InitializeReportForID in=0:0:0:0:1:0:8:1:2:0:0:0:1:0:255:0:0 rid=0 r=000000 il=3 ol=3 fl=3", "st=c0110010 r=000000"),
    // get_caps_range_wrap
    ("op=GetCaps in=1:48:50:0:1:0:8:2:0:16:5:3:1:0:255:0:0 il=3 ppg=1 pus=48", "caps=48:1:3:0:0:0:0:1:65535:0:0:0:0:0:0"),
    // get_caps_unused_cap
    ("op=GetCaps in=0:0:0:0:1:0:8:1:2:0:0:0:1:0:255:0:0,7:1:3:0:1:0:1:3:2:28:0:2:1:0:1:0:0 il=3 ppg=7 pus=1", "caps=1:7:3:0:0:0:1:0:3:0:0:0:0:0:0"),
    // copy_bits_zero_count
    ("op=CopyBits dst=00 src=ff sh=0 cnt=0", "dst=00"),
    // copy_bits_shift_left
    ("op=CopyBits dst=00000000 src=ff sh=2 cnt=5", "dst=7c000000"),
    // copy_bits_src_offset
    ("op=CopyBits dst=0000 src=8001 sh=-7 cnt=2", "dst=0300"),
];

/// A scalar value field of one usage on page 1, in collection 1.
fn scalar(usage: u16, start_byte: u16, bit_size: u16) -> ValueCaps {
    ValueCaps {
        usage_page: 1,
        usage_min: usage,
        usage_max: usage,
        start_byte,
        bit_size,
        report_count: 1,
        bit_field: 2,
        flags: 8,
        link_collection: 1,
        ..ValueCaps::default()
    }
}

/// Preparsed data with input caps only.
fn input(byte_length: u16, values: Vec<ValueCaps>) -> PreparsedData {
    PreparsedData {
        input: ReportCaps {
            byte_length,
            values,
        },
        ..PreparsedData::default()
    }
}

fn invalid_length<T>() -> Result<T, Status> {
    Err(HIDP_STATUS_INVALID_REPORT_LENGTH)
}

#[test]
fn public_types_derive_their_traits() {
    let rt = ReportType::Feature;
    assert_eq!(Clone::clone(&rt), ReportType::Feature);
    assert_ne!(rt, ReportType::Input);
    assert_eq!(format!("{rt:?}"), "Feature");

    let cap = ValueCaps::default();
    assert_eq!(Clone::clone(&cap), cap);
    assert_ne!(cap, scalar(48, 1, 8));
    assert!(format!("{cap:?}").starts_with("ValueCaps {"));

    let report = ReportCaps::default();
    assert_eq!(report.clone(), report);
    assert!(format!("{report:?}").starts_with("ReportCaps {"));

    let device = PreparsedData::default();
    assert_eq!(device.clone(), device);
    assert_ne!(device, input(1, vec![scalar(48, 0, 8)]));
    assert!(format!("{device:?}").starts_with("PreparsedData {"));

    let data = HidpData::default();
    assert_eq!(Clone::clone(&data), data);
    assert_ne!(
        data,
        HidpData {
            data_index: 1,
            raw_value: 0
        }
    );
    assert!(format!("{data:?}").starts_with("HidpData {"));

    let usage = UsageAndPage::default();
    assert_eq!(Clone::clone(&usage), usage);
    assert_ne!(
        usage,
        UsageAndPage {
            usage_page: 1,
            usage: 0
        }
    );
    assert!(format!("{usage:?}").starts_with("UsageAndPage {"));

    let caps = Caps::default();
    assert_eq!(Clone::clone(&caps), caps);
    assert_ne!(
        caps,
        Caps {
            usage: 1,
            ..Caps::default()
        }
    );
    assert!(format!("{caps:?}").starts_with("Caps {"));
}

#[test]
fn value_caps_flags_and_bit_position() {
    let mut cap = ValueCaps::default();
    assert!(!cap.is_used());
    assert!(cap.is_array());
    assert!(!cap.is_button() && !cap.is_range() && !cap.has_more());
    cap.usage_max = 3;
    assert!(cap.is_used());
    cap.usage_max = 0;
    cap.usage_min = 2;
    assert!(cap.is_used());
    cap.bit_field = 2;
    assert!(!cap.is_array());
    cap.flags = 4 | 16 | 1;
    assert!(cap.is_button() && cap.is_range() && cap.has_more());
    cap.start_byte = 2;
    cap.start_bit = 3;
    assert_eq!(cap.first_bit(), 19);
}

#[test]
fn mul_div_rounds_to_nearest_and_guards_zero_and_overflow() {
    assert_eq!(mul_div(2, 1, 3), 1);
    assert_eq!(mul_div(-2, 1, 3), -1);
    assert_eq!(mul_div(3, 2, 4), 2);
    assert_eq!(mul_div(-3, -2, 4), 2);
    assert_eq!(mul_div(-3, 2, 4), -2);
    assert_eq!(mul_div(3, -2, 4), -2);
    assert_eq!(mul_div(3, 2, -4), -2);
    assert_eq!(mul_div(-3, 2, -4), 2);
    assert_eq!(mul_div(1, 1, 0), -1);
    assert_eq!(mul_div(i32::MAX, i32::MAX, 1), -1);
    assert_eq!(mul_div(i32::MAX, -i32::MAX, 1), -1);
}

#[test]
fn sign_extend_reads_negative_fields_only_with_a_negative_minimum() {
    let signed8 = ValueCaps {
        bit_size: 8,
        logical_min: -127,
        ..ValueCaps::default()
    };
    assert_eq!(sign_extend(0x7f, &signed8), 127);
    assert_eq!(sign_extend(0xff, &signed8), -1);
    let unsigned8 = ValueCaps {
        bit_size: 8,
        logical_min: 0,
        ..ValueCaps::default()
    };
    assert_eq!(sign_extend(0xff, &unsigned8), 255);
    let one_bit = ValueCaps {
        bit_size: 1,
        logical_min: -1,
        ..ValueCaps::default()
    };
    assert_eq!(sign_extend(1, &one_bit), 1);
    let no_bits = ValueCaps {
        bit_size: 0,
        logical_min: -127,
        ..ValueCaps::default()
    };
    assert_eq!(sign_extend(0xff, &no_bits), 255);
}

#[test]
fn get_scaled_sign_extends_a_negative_field() {
    // Not a harness row: C's LONG is 64 bits on the LP64 host, so it returns 4294967295.
    let cap = ValueCaps {
        logical_min: -127,
        logical_max: 127,
        ..scalar(48, 1, 8)
    };
    let p = input(3, vec![cap]);
    assert_eq!(
        get_scaled_usage_value(&p, ReportType::Input, 1, 0, 48, &[0, 0xff, 0]),
        Ok(-1)
    );
}

#[test]
fn set_scaled_with_a_single_physical_value_is_bad_log() {
    // C divides by zero here (phy_range is 0), so this is not a harness row.
    let cap = ValueCaps {
        logical_max: 255,
        physical_min: 5,
        physical_max: 5,
        ..scalar(48, 0, 8)
    };
    let p = input(1, vec![cap]);
    assert_eq!(
        set_scaled_usage_value(&p, ReportType::Input, 1, 0, 48, 3, &mut [0]),
        Err(HIDP_STATUS_BAD_LOG_PHY_VALUES)
    );
}

#[test]
fn fields_wider_than_32_bits_are_buffer_too_small() {
    // C reads a 40-bit scalar into a 64-bit ULONG on LP64, so the harness cannot show this.
    let p = input(6, vec![scalar(48, 1, 40)]);
    let mut report = [0u8; 6];
    assert_eq!(
        get_usage_value(&p, ReportType::Input, 1, 0, 48, &report),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
    assert_eq!(
        get_scaled_usage_value(&p, ReportType::Input, 1, 0, 48, &report),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
    assert_eq!(
        set_usage_value(&p, ReportType::Input, 1, 0, 48, 1, &mut report),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
    assert_eq!(
        set_scaled_usage_value(&p, ReportType::Input, 1, 0, 48, 1, &mut report),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
    let mut out = [HidpData::default(); 2];
    assert_eq!(
        get_data(&p, ReportType::Input, &report, &mut out),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
}

#[test]
fn set_data_scalar_past_bit_32_is_buffer_too_small() {
    // C has no check here and writes past RawValue, so the harness cannot show this.
    let cap = ValueCaps {
        start_bit: 30,
        ..scalar(48, 0, 8)
    };
    let p = input(5, vec![cap]);
    let mut report = [0u8; 5];
    let items = [HidpData {
        data_index: 0,
        raw_value: 0x55,
    }];
    assert_eq!(
        set_data(&p, ReportType::Input, &items, &mut report),
        Err(HIDP_STATUS_BUFFER_TOO_SMALL)
    );
    assert_eq!(report, [0u8; 5]);
}

#[test]
fn inverted_usage_range_lists_nothing() {
    // C wraps the range around; the port treats it as empty.
    let cap = ValueCaps {
        usage_min: 50,
        usage_max: 48,
        flags: 4,
        ..scalar(50, 1, 1)
    };
    let p = input(2, vec![cap]);
    let mut out = [0u16; 4];
    assert_eq!(
        get_usages(&p, ReportType::Input, 1, 0, &[0, 0xff], &mut out),
        Ok(0)
    );
}

#[test]
fn last_chained_cap_without_a_successor_is_read_alone() {
    // C walks past the caps array here; the port stops at the last cap.
    let cap = ValueCaps {
        usage_page: 7,
        usage_min: 4,
        usage_max: 4,
        start_byte: 1,
        bit_size: 8,
        report_count: 1,
        flags: 5,
        link_collection: 1,
        ..ValueCaps::default()
    };
    let p = input(2, vec![cap]);
    let mut out = [0u16; 2];
    assert_eq!(
        get_usages(&p, ReportType::Input, 7, 0, &[0, 1], &mut out),
        Ok(1)
    );
    assert_eq!(out[0], 4);
}

#[test]
fn value_and_button_caps_list_their_kind() {
    let button = ValueCaps {
        flags: 4,
        ..scalar(1, 1, 1)
    };
    let p = input(3, vec![scalar(48, 1, 8), button]);
    let mut out = [ValueCaps::default(); 4];
    assert_eq!(get_value_caps(&p, ReportType::Input, &mut out), Ok(1));
    assert_eq!(out[0].usage_min, 48);
    assert_eq!(get_button_caps(&p, ReportType::Input, &mut out), Ok(1));
    assert_eq!(out[0].usage_min, 1);
}

#[test]
fn empty_report_is_invalid_for_every_report_op() {
    let p = input(3, vec![scalar(48, 1, 8)]);
    let rt = ReportType::Input;
    let mut none: [u8; 0] = [];
    let mut value = [0u8; 1];
    let mut usages = [0u16; 1];
    let mut usage_pairs = [UsageAndPage::default(); 1];
    let mut data = [HidpData::default(); 1];
    let items = [HidpData::default()];
    assert_eq!(get_usage_value(&p, rt, 1, 0, 48, &none), invalid_length());
    assert_eq!(
        get_usage_value_array(&p, rt, 1, 0, 48, &mut value, &none),
        invalid_length()
    );
    assert_eq!(
        get_scaled_usage_value(&p, rt, 1, 0, 48, &none),
        invalid_length()
    );
    assert_eq!(
        set_usage_value(&p, rt, 1, 0, 48, 1, &mut none),
        invalid_length()
    );
    assert_eq!(
        set_usage_value_array(&p, rt, 1, 0, 48, &value, &mut none),
        invalid_length()
    );
    assert_eq!(
        set_scaled_usage_value(&p, rt, 1, 0, 48, 1, &mut none),
        invalid_length()
    );
    assert_eq!(
        get_usages(&p, rt, 1, 0, &none, &mut usages),
        invalid_length()
    );
    assert_eq!(
        get_usages_ex(&p, rt, 0, &none, &mut usage_pairs),
        invalid_length()
    );
    assert_eq!(set_usages(&p, rt, 1, 0, &[48], &mut none), invalid_length());
    assert_eq!(
        unset_usages(&p, rt, 1, 0, &[48], &mut none),
        invalid_length()
    );
    assert_eq!(get_data(&p, rt, &none, &mut data), invalid_length());
    assert_eq!(set_data(&p, rt, &items, &mut none), invalid_length());
    assert_eq!(
        initialize_report_for_id(&p, rt, 1, &mut none),
        invalid_length()
    );
}

#[test]
fn edge_rows_match_wine() {
    for (spec, expected) in EDGE_ROWS {
        assert_eq!(replay(spec), *expected, "row {spec}");
    }
}
