//! Oracle rows from the verbatim Wine C in `dlls/dplayx/dplayx_messages.c`, built with
//! `gcc -fsanitize=address,undefined -g` as `target/canon/dplayx/oracle.c`. Each `expected`
//! string is the harness output: `OK|offset|flags|id|version|short|long|playerData|spData|
//! playerIds|parentId|shortcutIds` or `ERR|offset`, with `-` for an absent field and names
//! as hex code units. Row `S17` (shortcut ids at an odd offset) is excluded: UBSan reports
//! the misaligned `DPID` load in the C.

use crate::{read_packed_player, read_super_packed_player};

enum Layout {
    Packed,
    SuperPacked,
}

struct Row {
    id: &'static str,
    layout: Layout,
    hex: &'static str,
    offset: usize,
    max_size: usize,
    expected: &'static str,
}

const ROWS: &[Row] = &[
    Row {
        id: "P01",
        layout: Layout::Packed,
        hex: "300000001200000056340000000000000000000000000000000000000000000077000000300000000b00000099000000",
        offset: 0,
        max_size: 48,
        expected: "OK|48|18|13398|11|-|-|-|-|-|153|-",
    },
    Row {
        id: "P02",
        layout: Layout::Packed,
        hex: "300000001200000056340000040000000600000002000000040000000200000077000000300000000b00000099000000410000006f006b000000aabbcc1122334433221188776655",
        offset: 0,
        max_size: 72,
        expected: "OK|72|18|13398|11|0041|006f006b|cc112233|aabb|287454020,1432778632|153|-",
    },
    Row {
        id: "P03",
        layout: Layout::Packed,
        hex: "300000001200000056340000030000000000000000000000000000000000000077000000300000000b000000990000000000ff",
        offset: 0,
        max_size: 51,
        expected: "OK|51|18|13398|11||-|-|-|-|153|-",
    },
    Row {
        id: "P04",
        layout: Layout::Packed,
        hex: "300000001200000056340000040000000000000000000000000000000000000077000000300000000b0000009900000041004200",
        offset: 0,
        max_size: 52,
        expected: "ERR|0",
    },
    Row {
        id: "P05",
        layout: Layout::Packed,
        hex: "300000001200000056340000000000000000000000000000000000000000000077000000300000000b000000990000",
        offset: 0,
        max_size: 47,
        expected: "ERR|0",
    },
    Row {
        id: "P07",
        layout: Layout::Packed,
        hex: "300000001200000056340000040000000000000000000000000000000000000077000000300000000b00000099000000410000",
        offset: 0,
        max_size: 50,
        expected: "ERR|0",
    },
    Row {
        id: "P08",
        layout: Layout::Packed,
        hex: "300000001200000056340000000000000400000000000000000000000000000077000000300000000b000000990000005a000000",
        offset: 0,
        max_size: 52,
        expected: "OK|52|18|13398|11|-|005a|-|-|-|153|-",
    },
    Row {
        id: "P09",
        layout: Layout::Packed,
        hex: "deadbeef300000001200000056340000000000000000000002000000000000000000000077000000300000000b00000099000000aabb",
        offset: 4,
        max_size: 54,
        expected: "OK|54|18|13398|11|-|-|-|aabb|-|153|-",
    },
    Row {
        id: "P10",
        layout: Layout::Packed,
        hex: "300000001200000056340000000000000000000000000000000000000300000077000000300000000b000000990000000100000002000000",
        offset: 0,
        max_size: 56,
        expected: "ERR|0",
    },
    Row {
        id: "P11",
        layout: Layout::Packed,
        hex: "300000001200000056340000000000000000000000000000000000000200000077000000300000000b000000990000000700000008000000",
        offset: 0,
        max_size: 56,
        expected: "OK|56|18|13398|11|-|-|-|-|7,8|153|-",
    },
    Row {
        id: "S01",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000000000000d000000",
        offset: 0,
        max_size: 20,
        expected: "OK|20|33|17767|13|-|-|-|-|-|0|-",
    },
    Row {
        id: "S02",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000010000000d00000041000000",
        offset: 0,
        max_size: 24,
        expected: "OK|24|33|17767|13|0041|-|-|-|-|0|-",
    },
    Row {
        id: "S04",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000030000000d0000004300000044000000",
        offset: 0,
        max_size: 28,
        expected: "OK|28|33|17767|13|0043|0044|-|-|-|0|-",
    },
    Row {
        id: "S05",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000140000000d00000002aabb01cc",
        offset: 0,
        max_size: 25,
        expected: "OK|25|33|17767|13|-|-|aabb|cc|-|0|-",
    },
    Row {
        id: "S06",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000280000000d0000000200aabb0000",
        offset: 0,
        max_size: 26,
        expected: "OK|26|33|17767|13|-|-|aabb|-|-|0|-",
    },
    Row {
        id: "S07",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000fc0700000d00000000000000000000000100000078563412bc9a0000010000000df0ad0b",
        offset: 0,
        max_size: 48,
        expected: "OK|48|33|17767|13|-|-|-|-|305419896|39612|195948557",
    },
    Row {
        id: "S08",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000a80000000d0000000200aabb0000010010325476",
        offset: 0,
        max_size: 32,
        expected: "OK|32|33|17767|13|-|-|aabb|-|1985229328|0|-",
    },
    Row {
        id: "S11",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000000000000d0000",
        offset: 0,
        max_size: 19,
        expected: "ERR|0",
    },
    Row {
        id: "S12",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000000100000d000000",
        offset: 0,
        max_size: 20,
        expected: "ERR|0",
    },
    Row {
        id: "S14",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000100000000d00000003aabb",
        offset: 0,
        max_size: 23,
        expected: "ERR|0",
    },
    Row {
        id: "S15",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000010000000d000000420043000000",
        offset: 0,
        max_size: 26,
        expected: "OK|26|33|17767|13|00420043|-|-|-|-|0|-",
    },
    Row {
        id: "S16",
        layout: Layout::SuperPacked,
        hex: "140000002100000067450000000000000d0000000500aabbccddee",
        offset: 0,
        max_size: 27,
        expected: "OK|20|33|17767|13|-|-|-|-|-|0|-",
    },
];

fn decode_hex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn describe_name(name: &Option<Vec<u16>>) -> String {
    match name {
        None => "-".to_string(),
        Some(units) => units.iter().map(|unit| format!("{unit:04x}")).collect(),
    }
}

fn describe_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_string()
    } else {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

fn describe_ids(ids: &[u32]) -> String {
    if ids.is_empty() {
        "-".to_string()
    } else {
        ids.iter().map(u32::to_string).collect::<Vec<_>>().join(",")
    }
}

fn render(row: &Row) -> String {
    let data = decode_hex(row.hex);
    let mut offset = row.offset;
    let parsed = match row.layout {
        Layout::Packed => read_packed_player(&data, &mut offset, row.max_size),
        Layout::SuperPacked => read_super_packed_player(&data, &mut offset, row.max_size),
    };
    match parsed {
        Ok(info) => format!(
            "OK|{offset}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            info.flags,
            info.id,
            info.version_or_system_player_id,
            describe_name(&info.short_name),
            describe_name(&info.long_name),
            describe_bytes(&info.player_data),
            describe_bytes(&info.sp_data),
            describe_ids(&info.player_ids),
            info.parent_id,
            describe_ids(&info.shortcut_ids),
        ),
        Err(_) => format!("ERR|{}", row.offset),
    }
}

#[test]
fn oracle_rows_match_the_verbatim_c() {
    for row in ROWS {
        assert_eq!(render(row), row.expected, "oracle row {}", row.id);
    }
}
