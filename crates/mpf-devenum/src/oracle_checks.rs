//! Twenty-four rows captured from the ASan/UBSan Wine C oracle under
//! `target/canon/devenum/oracle.c`. The oracle compiles `parsedisplayname.c`, `mediacatenum.c`
//! and `combase.c` routines verbatim. Only the COM, registry and allocator boundary is shimmed,
//! and `realloc` only returns a static block so the capacity arithmetic can be observed.

use crate::array::reserve_capacity;
use crate::guid::{format_guid, parse_guid, Guid};
use crate::moniker::{format_display_name, DeviceKind, Moniker};
use crate::{parse_display_name, wide};

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

fn text(units: &[u16]) -> String {
    String::from_utf16(units).expect("oracle text is ASCII")
}

fn capacity_text(capacity: Option<u32>) -> String {
    capacity.map_or_else(|| "FALSE".to_string(), |value| value.to_string())
}

fn guid_row(input: &str) -> String {
    parse_guid(&wide(input)).map_or_else(|| "FAIL".to_string(), |guid| text(&format_guid(&guid)))
}

fn kind_name(kind: DeviceKind) -> &'static str {
    match kind {
        DeviceKind::Filter => "filter",
        DeviceKind::Codec => "codec",
    }
}

fn parse_row(input: &str) -> String {
    match parse_display_name(&wide(input)) {
        None => "MK_E_SYNTAX".to_string(),
        Some(Moniker::Dmo { clsid, class }) => format!(
            "dmo|{}|{}",
            text(&format_guid(&clsid)),
            text(&format_guid(&class))
        ),
        Some(Moniker::Device { kind, class, name }) => {
            let class = class.map_or_else(|| "-".to_string(), |guid| text(&format_guid(&guid)));
            format!("{}|{class}|{}", kind_name(kind), text(name))
        }
    }
}

fn format_row(moniker: &Moniker<'_>) -> String {
    text(&format_display_name(moniker))
}

fn run(row: &str) -> String {
    match row {
        "array_fits" => capacity_text(reserve_capacity(8, 8, 4)),
        "array_grow_from_zero" => capacity_text(reserve_capacity(0, 1, 4)),
        "array_double" => capacity_text(reserve_capacity(8, 9, 4)),
        "array_jump" => capacity_text(reserve_capacity(8, 100, 4)),
        "array_over_max" => capacity_text(reserve_capacity(0, 0x4000_0001, 4)),
        "array_at_max" => capacity_text(reserve_capacity(0, 0x3FFF_FFFF, 4)),
        "array_zero_size_fits" => capacity_text(reserve_capacity(4, 2, 0)),
        "guid_upper" => guid_row("{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}"),
        "guid_lower" => guid_row("{0a0b0c0d-1122-3344-5566-778899aabbcc}"),
        "guid_no_brace" => guid_row("1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809"),
        "guid_bad_dash" => guid_row("{1A2B3C4D-5E6F-7081-92A3xB4C5D6E7F809}"),
        "guid_bad_hex" => guid_row("{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F80G}"),
        "guid_too_short" => guid_row("{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F80}"),
        "parse_filter_class" => {
            parse_row("@device:sw:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}\\Foo Bar")
        }
        "parse_codec_class" => parse_row("@device:cm:{0a0b0c0d-1122-3344-5566-778899aabbcc}\\Codec"),
        "parse_filter_plain" => parse_row("@device:sw:Plain Filter"),
        "parse_codec_short_guid" => parse_row("@device:cm:{1A2B"),
        "parse_dmo" => parse_row(
            "@device:dmo:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}{0a0b0c0d-1122-3344-5566-778899aabbcc}",
        ),
        "parse_dmo_no_class" => parse_row("@device:dmo:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}"),
        "parse_unknown_type" => parse_row("@device:xx:Foo"),
        "parse_filter_bad_guid" => {
            parse_row("@device:sw:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F80G}\\Foo")
        }
        "format_filter_class" => format_row(&Moniker::Device {
            kind: DeviceKind::Filter,
            class: Some(GUID_A),
            name: &wide("Foo"),
        }),
        "format_codec_plain" => format_row(&Moniker::Device {
            kind: DeviceKind::Codec,
            class: None,
            name: &wide("Bar"),
        }),
        "format_dmo" => format_row(&Moniker::Dmo {
            clsid: GUID_A,
            class: GUID_B,
        }),
        _ => String::new(),
    }
}

#[test]
fn oracle_rows_match_the_port() {
    let rows = [
        ("array_fits", "8"),
        ("array_grow_from_zero", "8"),
        ("array_double", "16"),
        ("array_jump", "128"),
        ("array_over_max", "FALSE"),
        ("array_at_max", "1073741823"),
        ("array_zero_size_fits", "4"),
        ("guid_upper", "{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}"),
        ("guid_lower", "{0A0B0C0D-1122-3344-5566-778899AABBCC}"),
        ("guid_no_brace", "FAIL"),
        ("guid_bad_dash", "FAIL"),
        ("guid_bad_hex", "FAIL"),
        ("guid_too_short", "FAIL"),
        (
            "parse_filter_class",
            "filter|{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}|Foo Bar",
        ),
        (
            "parse_codec_class",
            "codec|{0A0B0C0D-1122-3344-5566-778899AABBCC}|Codec",
        ),
        ("parse_filter_plain", "filter|-|Plain Filter"),
        ("parse_codec_short_guid", "codec|-|{1A2B"),
        (
            "parse_dmo",
            "dmo|{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}|{0A0B0C0D-1122-3344-5566-778899AABBCC}",
        ),
        ("parse_dmo_no_class", "MK_E_SYNTAX"),
        ("parse_unknown_type", "MK_E_SYNTAX"),
        (
            "parse_filter_bad_guid",
            "filter|-|{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F80G}\\Foo",
        ),
        (
            "format_filter_class",
            "@device:sw:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}\\Foo",
        ),
        ("format_codec_plain", "@device:cm:Bar"),
        (
            "format_dmo",
            "@device:dmo:{1A2B3C4D-5E6F-7081-92A3-B4C5D6E7F809}{0A0B0C0D-1122-3344-5566-778899AABBCC}",
        ),
    ];
    for (row, expected) in rows {
        assert_eq!(run(row), expected, "oracle row {row}");
    }
}

#[test]
fn unknown_oracle_row_has_an_empty_representation() {
    assert_eq!(run("not-a-row"), "");
}
