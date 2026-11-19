//! Edge checks for the inputs the oracle rows do not reach: strings that end at a NUL in the middle,
//! the `wcstol` stand-in with its signs and saturation, the table name budget, every MIME class, and
//! the codec round trips. The C-defined values were also run through `target/canon/msi/harness`. The
//! cases that shift past `int` range are Rust-only, because the C has undefined behaviour there.

use super::*;

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

const HEX_GUID: &str = "0123456789ABCDEF0123456789ABCDEF";

#[test]
fn unsquash_reads_the_string_up_to_its_nul() {
    let mut text = units(HEX_GUID);
    text.push(0);
    text.extend(units("junk"));
    assert_eq!(unsquash_guid(&text), unsquash_guid(&units(HEX_GUID)));
    assert_eq!(
        unsquash_guid(&units("0123456789ABCDEF0123456789ABCDE\0F")),
        None
    );
}

#[test]
fn unsquash_needs_exactly_32_units() {
    assert_eq!(unsquash_guid(&units(&HEX_GUID[1..])), None);
    assert_eq!(unsquash_guid(&units(&format!("{HEX_GUID}0"))), None);
    assert_eq!(unsquash_guid(&[]), None);
}

#[test]
fn unsquash_reverses_the_digit_pairs_of_each_group() {
    assert_eq!(
        unsquash_guid(&units("abcdefabcdefabcdefabcdefabcdefab")),
        Some(units("{bafedcba-fedc-dcba-feba-dcfebadcfeba}"))
    );
}

#[test]
fn decode_rejects_units_outside_the_digit_set() {
    for bad in [0x0080, 0x00ff, 0xffff] {
        let mut text = units("!!!!!!!!!!!!!!!!!!!!");
        text[3] = bad;
        assert_eq!(decode_base85_guid(&text), None);
    }
}

#[test]
fn decode_reads_the_terminator_past_a_short_string() {
    assert_eq!(decode_base85_guid(&[]), None);
    assert_eq!(decode_base85_guid(&units("!!!!")), None);
}

#[test]
fn decode_stops_at_a_nul_inside_the_twenty_units() {
    let mut text = units("!!!!!!!!!!!!!!!!!!!!");
    text[7] = 0;
    assert_eq!(decode_base85_guid(&text), None);
}

#[test]
fn every_digit_decodes_to_its_position_in_the_digit_set() {
    for (value, &digit) in TABLE_ENC85.iter().enumerate() {
        let mut text = vec![u16::from(digit)];
        text.extend([u16::from(b'!'); 19]);
        assert_eq!(decode_base85_guid(&text), Some([value as u32, 0, 0, 0]));
    }
}

#[test]
fn digit_table_is_the_inverse_of_the_digit_set() {
    for (value, &digit) in TABLE_ENC85.iter().enumerate() {
        assert_eq!(usize::from(TABLE_DEC85[usize::from(digit)]), value);
    }
    assert_eq!(
        TABLE_DEC85.iter().filter(|&&v| v != 0xff).count(),
        TABLE_ENC85.len()
    );
}

#[test]
fn encode_then_decode_returns_every_word() {
    for word in [0, 1, 84, 85, 7224, 0x1234_5678, 0x8000_0000, u32::MAX] {
        let words = [word, !word, word.rotate_left(7), word ^ 0x5a5a_5a5a];
        assert_eq!(decode_base85_guid(&encode_base85_guid(&words)), Some(words));
    }
}

#[test]
fn encode_maps_all_ones_to_the_top_digit_of_each_word() {
    assert_eq!(
        encode_base85_guid(&[u32::MAX; 4]).to_vec(),
        units(&"!0_?{".repeat(4))
    );
}

#[test]
fn version_without_dots_has_only_a_major_field() {
    assert_eq!(version_str_to_dword(&units("7")), 0x0700_0000);
    assert_eq!(version_str_to_dword(&units("1.2.3.4")), 0x0102_0003);
    assert_eq!(version_str_to_dword(&units("1.2")), 0x0102_0000);
}

#[test]
fn version_keeps_only_the_low_byte_of_major_and_minor() {
    assert_eq!(version_str_to_dword(&units("300.300.70000")), 0x2c2c_1170);
}

#[test]
fn version_saturates_an_oversized_field_at_the_32_bit_range() {
    assert_eq!(version_str_to_dword(&units("99999999999999")), 0xff00_0000);
    assert_eq!(version_str_to_dword(&units("-99999999999999")), 0);
}

#[test]
fn version_skips_blanks_and_reads_signs_like_wcstol() {
    assert_eq!(
        version_str_to_dword(&units(" \t\n\x0b\x0c\r+7.-1.12")),
        0x07ff_000c
    );
    assert_eq!(version_str_to_dword(&units("-")), 0);
    assert_eq!(version_str_to_dword(&units("+")), 0);
    assert_eq!(version_str_to_dword(&units("")), 0);
}

#[test]
fn version_stops_at_a_nul() {
    assert_eq!(version_str_to_dword(&units("\x007.1")), 0);
}

#[test]
fn parse_packs_four_fields_and_ignores_the_rest() {
    assert_eq!(
        parse_version_string(&units("1.2.3.4.5.6")),
        (0x0001_0002, 0x0003_0004)
    );
    assert_eq!(
        parse_version_string(&units("1.2.3")),
        (0x0001_0002, 0x0003_0000)
    );
    assert_eq!(parse_version_string(&units("7")), (0x0007_0000, 0));
    assert_eq!(parse_version_string(&units("")), (0, 0));
    assert_eq!(parse_version_string(&units("1.2\0.3.4")), (0x0001_0002, 0));
}

#[test]
fn parse_lets_a_second_field_past_16_bits_overlap_the_major_half() {
    assert_eq!(parse_version_string(&units("1.70000")), (0x0001_1170, 0));
}

/// Rust-only: the C shifts `40000` and `-1` left by 16 in `int`, which UBSan flags. The crate
/// shifts a `u32`, so the result is the wrapped DWORD.
#[test]
fn parse_wraps_a_major_field_past_the_int_shift() {
    assert_eq!(parse_version_string(&units("40000.1")), (0x9c40_0001, 0));
    assert_eq!(parse_version_string(&units("-1.0")), (0xffff_0000, 0));
}

#[test]
fn compare_file_versions_orders_ms_before_ls() {
    assert_eq!(compare_file_versions(0x0002_0000, 0, &units("1.9")), 1);
    assert_eq!(compare_file_versions(0x0001_0000, 0, &units("2.0")), -1);
    assert_eq!(
        compare_file_versions(0x0001_0002, 0x0003_0005, &units("1.2.3.4")),
        1
    );
    assert_eq!(
        compare_file_versions(0x0001_0002, 0x0003_0003, &units("1.2.3.4")),
        -1
    );
    assert_eq!(compare_file_versions(0, 0, &units("")), 0);
}

#[test]
fn compare_font_versions_looks_only_at_the_major_pair() {
    assert_eq!(compare_font_versions(&units("1.2.9.9"), &units("1.3")), -1);
    assert_eq!(compare_font_versions(&units("1.3"), &units("1.2.9.9")), 1);
    assert_eq!(
        compare_font_versions(&units("1.2.0.0"), &units("1.2.9.9")),
        0
    );
}

#[test]
fn encode_maps_every_mime_class_and_round_trips() {
    let name = "09AZaz._-";
    let encoded = encode_streamname(false, &units(name)).expect("no budget");
    assert_eq!(decode_streamname(&encoded), units(name));
}

#[test]
fn encode_keeps_a_mime_unit_alone_when_the_next_unit_is_plain() {
    assert_eq!(
        encode_streamname(false, &units("A-")),
        Some(vec![0x480a, 0x002d])
    );
    assert_eq!(
        encode_streamname(false, &units("-A")),
        Some(vec![0x002d, 0x480a])
    );
}

#[test]
fn encode_table_name_has_thirty_units_after_the_marker() {
    let fits = encode_streamname(true, &units(&"ab".repeat(29)));
    assert_eq!(fits.map(|v| v.len()), Some(30));
    assert_eq!(encode_streamname(true, &units(&"ab".repeat(30))), None);
    assert_eq!(encode_streamname(true, &units(&"-".repeat(30))), None);
}

#[test]
fn encode_name_without_the_table_marker_has_no_budget() {
    let long = "-".repeat(100);
    assert_eq!(
        encode_streamname(false, &units(&long)).map(|v| v.len()),
        Some(100)
    );
    assert_eq!(encode_streamname(false, &[]), Some(vec![]));
    assert_eq!(encode_streamname(true, &[]), Some(vec![TABLE_MARKER]));
}

#[test]
fn encode_stops_at_a_nul() {
    assert_eq!(encode_streamname(false, &units("A\0B")), Some(vec![0x480a]));
}

#[test]
fn decode_expands_single_and_pair_units() {
    assert_eq!(decode_streamname(&[0x4800, 0x483f]), units("0_"));
    assert_eq!(decode_streamname(&[0x3800, 0x47ff]), units("00__"));
    assert_eq!(decode_streamname(&[0x4800, 0, 0x4800]), units("0"));
}

#[test]
fn decode_of_a_leading_nul_is_empty() {
    assert!(decode_streamname(&[0]).is_empty());
    assert!(decode_streamname(&[0, 0x0041]).is_empty());
}

#[test]
fn decode_passes_units_outside_the_mime_range_through() {
    let raw = vec![0x0041, 0x37ff, 0x4840, 0xffff];
    assert_eq!(decode_streamname(&raw), raw);
}

#[test]
fn decode_inverts_encode_for_names_without_the_table_marker() {
    for name in ["Foo-Bar.1", "Name", "a_b.C-9", "x", ""] {
        let encoded = encode_streamname(false, &units(name)).expect("no budget");
        assert_eq!(decode_streamname(&encoded), units(name));
    }
}
