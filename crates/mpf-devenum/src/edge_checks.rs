use crate::array::reserve_capacity;
use crate::guid::{format_guid, guid_prefix, parse_guid, Guid};
use crate::moniker::{format_display_name, DeviceKind, Moniker};
use crate::{parse_display_name, wide};

const GUID_TEXT: &str = "{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}";
const GUID_TEXT_B: &str = "{0A0B0C0D-1122-3344-5566-778899AABBCC}";

const GUID_A: Guid = Guid {
    data1: 0x1A2B_3C4D,
    data2: 0x5E6F,
    data3: 0x7081,
    data4: [0x92, 0xA3, 0xB4, 0xC5, 0xD6, 0xE7, 0xF8, 0x09],
};

const GUID_B: Guid = Guid {
    data1: 0x0A0B_0C0D,
    data2: 0x1122,
    data3: 0x3344,
    data4: [0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC],
};

fn display(moniker: &Moniker<'_>) -> String {
    String::from_utf16(&format_display_name(moniker)).expect("ASCII display name")
}

#[test]
fn array_reserve_keeps_capacity_when_the_count_fits() {
    assert_eq!(reserve_capacity(8, 8, 4), Some(8));
    assert_eq!(reserve_capacity(0, 0, 0), Some(0));
    assert_eq!(reserve_capacity(4, 2, 0), Some(4));
}

#[test]
fn array_reserve_rejects_zero_element_size_only_when_growing() {
    assert_eq!(reserve_capacity(4, 5, 0), None);
}

#[test]
fn array_reserve_grows_by_doubling_from_the_minimum() {
    assert_eq!(reserve_capacity(0, 1, 1), Some(8));
    assert_eq!(reserve_capacity(100, 101, 4), Some(200));
    assert_eq!(reserve_capacity(16, 33, 4), Some(64));
}

#[test]
fn array_reserve_stops_doubling_at_the_largest_allocation() {
    assert_eq!(reserve_capacity(0, u32::MAX, 1), Some(u32::MAX));
    assert_eq!(reserve_capacity(0, u32::MAX, 2), None);
    assert_eq!(reserve_capacity(8, 9, u32::MAX), None);
    // Wine's minimum of 8 is not capped by the element limit, so this matches the C result.
    assert_eq!(reserve_capacity(0, 1, u32::MAX), Some(8));
}

#[test]
fn guid_parses_every_hex_digit_in_either_case() {
    let guid = parse_guid(&wide("{01234567-89ab-cdef-0123-456789ABCDEF}")).expect("valid GUID");
    assert_eq!(
        guid,
        Guid {
            data1: 0x0123_4567,
            data2: 0x89AB,
            data3: 0xCDEF,
            data4: [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
        }
    );
    assert_eq!(
        parse_guid(&wide("{01234567-89AB-CDEF-0123-456789abcdef}")),
        Some(guid)
    );
}

#[test]
fn guid_rejects_any_wrong_character_at_every_position() {
    for index in 0..GUID_TEXT.len() {
        let mut text = wide(GUID_TEXT);
        text[index] = u16::from(b'#');
        assert_eq!(parse_guid(&text), None, "position {index}");
    }
}

#[test]
fn guid_rejects_characters_just_outside_the_hex_ranges() {
    for bad in *b"/:G`g@" {
        let mut text = wide(GUID_TEXT);
        text[1] = u16::from(bad);
        assert_eq!(parse_guid(&text), None, "character {}", char::from(bad));
    }
}

#[test]
fn guid_rejects_wrong_lengths_and_empty_input() {
    assert_eq!(parse_guid(&wide(&GUID_TEXT[..37])), None);
    assert_eq!(parse_guid(&wide(&format!("{GUID_TEXT}x"))), None);
    assert_eq!(parse_guid(&[]), None);
}

#[test]
fn guid_prefix_reads_only_the_first_guid_window() {
    let with_tail = wide(&format!("{GUID_TEXT}\\Foo"));
    assert_eq!(guid_prefix(&with_tail), parse_guid(&wide(GUID_TEXT)));
    let mut nul_inside = wide(GUID_TEXT);
    nul_inside[10] = 0;
    assert_eq!(guid_prefix(&nul_inside), None);
    assert_eq!(guid_prefix(&[]), None);
}

#[test]
fn guid_text_is_upper_case_and_zero_padded() {
    assert_eq!(
        String::from_utf16(&format_guid(&Guid::default())).unwrap(),
        "{00000000-0000-0000-0000-000000000000}"
    );
    let guid = Guid {
        data1: 1,
        data2: 2,
        data3: 3,
        data4: [4, 5, 6, 7, 8, 9, 10, 255],
    };
    assert_eq!(
        String::from_utf16(&format_guid(&guid)).unwrap(),
        "{00000001-0002-0003-0405-060708090AFF}"
    );
}

#[test]
fn display_name_formats_a_filter_without_a_class() {
    let name = wide("Plain");
    let filter = Moniker::Device {
        kind: DeviceKind::Filter,
        class: None,
        name: &name,
    };
    assert_eq!(display(&filter), "@device:sw:Plain");
}

#[test]
fn parse_display_name_rejects_missing_colon_and_empty_input() {
    assert_eq!(parse_display_name(&[]), None);
    assert_eq!(parse_display_name(&wide("noColonHere")), None);
}

#[test]
fn parse_display_name_rejects_truncated_type_prefixes() {
    assert_eq!(parse_display_name(&wide("@device:s")), None);
    assert_eq!(parse_display_name(&wide("@device:dm")), None);
}

#[test]
fn parse_display_name_reads_empty_device_names() {
    let text = wide("@device:sw:");
    assert_eq!(
        parse_display_name(&text),
        Some(Moniker::Device {
            kind: DeviceKind::Filter,
            class: None,
            name: &[],
        })
    );
}

#[test]
fn parse_display_name_keeps_an_empty_name_when_the_guid_fills_the_window() {
    // Wine advances past the GUID and its separator, which reads beyond this string. The port
    // returns an empty name instead.
    let text = wide(&format!("@device:cm:{GUID_TEXT}"));
    assert_eq!(
        parse_display_name(&text),
        Some(Moniker::Device {
            kind: DeviceKind::Codec,
            class: Some(GUID_A),
            name: &[],
        })
    );
}

#[test]
fn parse_display_name_keeps_the_text_of_an_invalid_guid_as_the_name() {
    let text = wide("@device:sw:{1A2B}\\Foo");
    assert_eq!(
        parse_display_name(&text),
        Some(Moniker::Device {
            kind: DeviceKind::Filter,
            class: None,
            name: &text[11..],
        })
    );
}

#[test]
fn parse_display_name_requires_two_guids_for_a_dmo() {
    let missing_class = wide(&format!("@device:dmo:{GUID_TEXT}"));
    assert_eq!(parse_display_name(&missing_class), None);
    let short_class = wide(&format!("@device:dmo:{GUID_TEXT}{{1A2B}}"));
    assert_eq!(parse_display_name(&short_class), None);
    let bad_clsid = wide(&format!("@device:dmo:{{1A2B}}{GUID_TEXT_B}"));
    assert_eq!(parse_display_name(&bad_clsid), None);
}

#[test]
fn parse_display_name_ignores_text_after_a_dmo_class() {
    let text = wide(&format!("@device:dmo:{GUID_TEXT}{GUID_TEXT_B}junk"));
    assert_eq!(
        parse_display_name(&text),
        Some(Moniker::Dmo {
            clsid: GUID_A,
            class: GUID_B,
        })
    );
}

#[test]
fn display_names_parse_back_to_the_same_moniker() {
    let name = wide("Foo Bar");
    let filter = Moniker::Device {
        kind: DeviceKind::Filter,
        class: Some(GUID_A),
        name: &name,
    };
    let text = wide(&display(&filter));
    assert_eq!(parse_display_name(&text), Some(filter));

    let codec = Moniker::Device {
        kind: DeviceKind::Codec,
        class: None,
        name: &[],
    };
    let text = wide(&display(&codec));
    assert_eq!(parse_display_name(&text), Some(codec));

    let dmo = Moniker::Dmo {
        clsid: GUID_A,
        class: GUID_B,
    };
    let text = wide(&display(&dmo));
    assert_eq!(parse_display_name(&text), Some(dmo));
}
