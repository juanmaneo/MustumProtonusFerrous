//! Edge checks for the cases the oracle rows do not reach: the bounds of the table reads, the
//! absent table, each exit of the gasp and VDMX scans, the ratio clauses of `find_vdmx_group`,
//! the wrapping of `descent`, and the code point boundaries.

use super::*;
use crate::oracle_checks::hex;

/// A VDMX group with records for 8 ppem (`yMax` 7, `yMin` -2) and 16 ppem (`yMax` 14, `yMin` -4),
/// startsz 8 and endsz 20.
const GROUP: &str = "0002081400080007fffe0010000efffc";

/// A VDMX table with one ratio record `ratio` and one group at offset 12, followed by `tail`.
fn vdmx_with_ratio(ratio: &str, tail: &str) -> Vec<u8> {
    [hex("000000010001"), hex(ratio), hex("000c"), hex(tail)].concat()
}

#[test]
fn absent_table_reads_nothing() {
    let absent = Table::absent();
    assert_eq!(absent.read_ensure(0, 0), None);
    assert_eq!(absent.be_word(0), 0);
    assert_eq!(absent.byte(0), 0);
}

#[test]
fn read_ensure_accepts_ranges_that_end_at_the_table_end() {
    let data = hex("0102030405");
    let table = Table::new(&data);
    assert_eq!(table.read_ensure(3, 2), Some(&[0x04, 0x05][..]));
    assert_eq!(table.read_ensure(5, 0).map(<[u8]>::len), Some(0));
}

#[test]
fn read_ensure_rejects_ranges_that_run_past_the_table() {
    let data = hex("0102030405");
    let table = Table::new(&data);
    assert_eq!(table.read_ensure(4, 2), None);
    assert_eq!(table.read_ensure(6, 0), None);
    assert_eq!(table.read_ensure(0, 6), None);
}

#[test]
fn be_word_reads_big_endian_and_gives_zero_past_the_end() {
    let data = hex("0102ff");
    let table = Table::new(&data);
    assert_eq!(table.be_word(0), 0x0102);
    assert_eq!(table.be_word(1), 0x02ff);
    assert_eq!(table.be_word(2), 0);
}

#[test]
fn byte_reads_one_byte_and_gives_zero_past_the_end() {
    let data = hex("01");
    let table = Table::new(&data);
    assert_eq!(table.byte(0), 0x01);
    assert_eq!(table.byte(1), 0);
}

#[test]
fn gasp_absent_table_has_no_flags() {
    assert_eq!(gasp_flags(Table::absent(), 12.0), 0);
}

#[test]
fn gasp_version_one_is_accepted() {
    let table = hex("0001000200080002ffff0003");
    assert_eq!(gasp_flags(Table::new(&table), 7.0), 2);
    assert_eq!(gasp_flags(Table::new(&table), 9.0), 3);
}

#[test]
fn gasp_without_ranges_has_no_flags() {
    assert_eq!(gasp_flags(Table::new(&hex("00000000")), 12.0), 0);
}

#[test]
fn gasp_table_shorter_than_its_ranges_has_no_flags() {
    assert_eq!(gasp_flags(Table::new(&hex("0000000200080001")), 6.0), 0);
}

#[test]
fn gasp_emsize_at_a_range_maximum_takes_that_range() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0000000200080001ffff0003")), 8.0),
        1
    );
}

#[test]
fn gasp_nan_emsize_scans_every_range() {
    assert_eq!(
        gasp_flags(Table::new(&hex("0000000200080001ffff0003")), f32::NAN),
        3
    );
}

#[test]
fn gasp_emsize_past_every_range_takes_the_last_flags() {
    assert_eq!(gasp_flags(Table::new(&hex("0000000100080005")), 20.0), 5);
}

#[test]
fn vdmx_absent_table_has_no_size() {
    assert_eq!(vdmx_size(Table::absent(), 8), None);
}

#[test]
fn vdmx_truncated_header_has_no_size() {
    assert_eq!(vdmx_size(Table::new(&hex("00000001000101010101")), 8), None);
}

#[test]
fn vdmx_ratio_without_char_set_is_skipped() {
    let table = vdmx_with_ratio("00010101", GROUP);
    assert_eq!(vdmx_size(Table::new(&table), 8), None);
}

#[test]
fn vdmx_ratios_outside_the_device_have_no_group() {
    for ratio in ["01020101", "01010201", "01010100", "01000001"] {
        let table = vdmx_with_ratio(ratio, GROUP);
        assert_eq!(vdmx_size(Table::new(&table), 8), None, "ratio {ratio}");
    }
}

#[test]
fn vdmx_group_beyond_the_table_has_no_size() {
    let table = [
        hex("000000010001"),
        hex("01010101"),
        hex("0014"),
        hex("0000000000000000"),
    ]
    .concat();
    assert_eq!(vdmx_size(Table::new(&table), 8), None);
}

#[test]
fn vdmx_group_without_records_has_no_size() {
    let table = vdmx_with_ratio("01010101", "0000081400000000");
    assert_eq!(vdmx_size(Table::new(&table), 8), None);
}

#[test]
fn vdmx_multiple_ratios_use_the_first_matching_group() {
    let table = [
        hex("000000010002"),
        hex("01020101"),
        hex("01010101"),
        hex("00000012"),
        hex("0001081400080007fffe"),
    ]
    .concat();
    assert_eq!(vdmx_size(Table::new(&table), 8), Some((7, 2)));
}

#[test]
fn vdmx_emsize_below_startsz_has_no_size() {
    let table = vdmx_with_ratio("01010101", GROUP);
    assert_eq!(vdmx_size(Table::new(&table), 7), None);
}

#[test]
fn vdmx_emsize_past_every_record_has_no_size() {
    let table = vdmx_with_ratio("01010101", GROUP);
    assert_eq!(vdmx_size(Table::new(&table), 17), None);
}

#[test]
fn vdmx_ascent_and_descent_keep_their_bit_patterns() {
    let positive_ymin = vdmx_with_ratio("01010101", "00010814000800070005");
    assert_eq!(vdmx_size(Table::new(&positive_ymin), 8), Some((7, 65531)));

    let negative_ymax = vdmx_with_ratio("01010101", "000108140008fff68000");
    assert_eq!(
        vdmx_size(Table::new(&negative_ymax), 8),
        Some((65526, 32768))
    );
}

#[test]
fn container_ignores_bytes_after_the_signature() {
    assert_eq!(
        container_type(&hex("774f4646ffffffff")),
        ContainerType::Woff
    );
}

#[test]
fn container_empty_data_is_unknown() {
    assert_eq!(container_type(&[]), ContainerType::Unknown);
}

#[test]
fn container_other_wof_version_is_unknown() {
    assert_eq!(container_type(&hex("774f4631")), ContainerType::Unknown);
}

#[test]
fn zero_width_range_edges() {
    for cp in [
        0x34e, 0x350, 0x061b, 0x061d, 0x180a, 0x180f, 0x200a, 0x2010, 0xfefe, 0xff00,
    ] {
        assert!(!is_zero_width(cp), "{cp:#x}");
    }
    for cp in [0x34f, 0x061c, 0x180b, 0x180e, 0x200b, 0x200f, 0xfeff] {
        assert!(is_zero_width(cp), "{cp:#x}");
    }
}

#[test]
fn default_ignorable_range_edges() {
    for cp in [
        0x7f, 0x80, 0xac, 0xae, 0x17b3, 0x17b6, 0x2070, 0x202f, 0x1d172, 0x1d17b, 0xe1000, 0xfe10,
        0xfff9,
    ] {
        assert!(!is_default_ignorable(cp), "{cp:#x}");
    }
    for cp in [
        0x17b4, 0x17b5, 0x180b, 0x200b, 0x202a, 0x202e, 0x2060, 0x206f, 0xfe00, 0xfe0f, 0xfeff,
        0xfff0, 0xfff8, 0x1d173, 0x1d17a, 0xe0000, 0xe0fff,
    ] {
        assert!(is_default_ignorable(cp), "{cp:#x}");
    }
}

#[test]
fn whitespace_range_edges() {
    for cp in [
        0x08, 0x0e, 0x84, 0x86, 0x9f, 0x167f, 0x1681, 0x1fff, 0x2027, 0x202a, 0x2030, 0x2060,
        0x3001,
    ] {
        assert!(!is_whitespace(cp), "{cp:#x}");
    }
    for cp in [
        0x0a, 0x0c, 0x0d, 0x20, 0x85, 0xa0, 0x1680, 0x2000, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f,
        0x3000,
    ] {
        assert!(is_whitespace(cp), "{cp:#x}");
    }
}
