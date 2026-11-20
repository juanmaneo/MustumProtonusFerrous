//! Checks the port against rows recorded from the verbatim Wine C code (`dlls/msctf/msctf.c`:
//! `generate_Cookie`, `get_Cookie_magic`, `get_Cookie_data`, `remove_Cookie` and
//! `enumerate_Cookie` over the global cookie table), built with ASan and UBSan, with no reports.
//! Each row is a name and the steps of one scenario, replayed on a fresh table. Wine returns 0 or
//! NULL for a missing cookie, which the rows read as `None`. Magics and payloads are nonzero, so
//! that reading is unambiguous.

use crate::CookieTable;

enum Step {
    Generate {
        magic: u32,
        data: usize,
        id: u32,
    },
    GetMagic {
        id: u32,
        magic: Option<u32>,
    },
    GetData {
        id: u32,
        data: Option<usize>,
    },
    Remove {
        id: u32,
        data: Option<usize>,
    },
    Enumerate {
        magic: u32,
        index: u32,
        id: u32,
        next: u32,
    },
}

use Step::*;

#[rustfmt::skip]
const ROWS: &[(&str, &[Step])] = &[
    ("first_id_is_one", &[
        Generate { magic: 0x10, data: 0x1001, id: 1 },
        GetData { id: 1, data: Some(0x1001) },
        GetMagic { id: 1, magic: Some(0x10) },
    ]),
    ("ids_count_up_from_one", &[
        Generate { magic: 0x10, data: 0x11, id: 1 },
        Generate { magic: 0x10, data: 0x12, id: 2 },
        Generate { magic: 0x20, data: 0x13, id: 3 },
    ]),
    ("magic_is_kept_per_slot", &[
        Generate { magic: 0x10, data: 0x21, id: 1 },
        Generate { magic: 0x20, data: 0x22, id: 2 },
        GetMagic { id: 1, magic: Some(0x10) },
        GetMagic { id: 2, magic: Some(0x20) },
    ]),
    ("unknown_id_has_no_entry", &[
        GetData { id: 1, data: None },
        GetMagic { id: 1, magic: None },
        Generate { magic: 0x10, data: 0x31, id: 1 },
        GetData { id: 2, data: None },
        GetMagic { id: 2, magic: None },
    ]),
    ("id_zero_is_never_valid", &[
        Generate { magic: 0x10, data: 0x41, id: 1 },
        GetMagic { id: 0, magic: None },
        GetData { id: 0, data: None },
        Remove { id: 0, data: None },
        GetData { id: 1, data: Some(0x41) },
    ]),
    ("remove_hands_back_data", &[
        Generate { magic: 0x10, data: 0x51, id: 1 },
        Remove { id: 1, data: Some(0x51) },
        GetData { id: 1, data: None },
        GetMagic { id: 1, magic: None },
        Remove { id: 1, data: None },
    ]),
    ("removed_slots_are_skipped_by_enum", &[
        Generate { magic: 0x10, data: 0x61, id: 1 },
        Generate { magic: 0x10, data: 0x62, id: 2 },
        Generate { magic: 0x10, data: 0x63, id: 3 },
        Remove { id: 2, data: Some(0x62) },
        Enumerate { magic: 0x10, index: 0, id: 1, next: 1 },
        Enumerate { magic: 0x10, index: 1, id: 3, next: 3 },
        Enumerate { magic: 0x10, index: 3, id: 0, next: 3 },
    ]),
    ("reuses_lowest_free_slot", &[
        Generate { magic: 0x10, data: 0x65, id: 1 },
        Generate { magic: 0x10, data: 0x66, id: 2 },
        Generate { magic: 0x10, data: 0x67, id: 3 },
        Remove { id: 2, data: Some(0x66) },
        Generate { magic: 0x30, data: 0x71, id: 2 },
        GetData { id: 2, data: Some(0x71) },
    ]),
    ("reuses_free_slots_in_ascending_order", &[
        Generate { magic: 0x10, data: 0x81, id: 1 },
        Generate { magic: 0x10, data: 0x82, id: 2 },
        Generate { magic: 0x10, data: 0x83, id: 3 },
        Generate { magic: 0x10, data: 0x84, id: 4 },
        Remove { id: 3, data: Some(0x83) },
        Remove { id: 1, data: Some(0x81) },
        Generate { magic: 0x10, data: 0x85, id: 1 },
        Generate { magic: 0x10, data: 0x86, id: 3 },
        Generate { magic: 0x10, data: 0x87, id: 5 },
    ]),
    ("reuse_keeps_other_entries", &[
        Generate { magic: 0x10, data: 0x91, id: 1 },
        Generate { magic: 0x20, data: 0x92, id: 2 },
        Remove { id: 1, data: Some(0x91) },
        Generate { magic: 0x30, data: 0x93, id: 1 },
        GetData { id: 1, data: Some(0x93) },
        GetData { id: 2, data: Some(0x92) },
        GetMagic { id: 2, magic: Some(0x20) },
    ]),
    ("enum_lists_matches_in_order", &[
        Generate { magic: 0x10, data: 0xa1, id: 1 },
        Generate { magic: 0x20, data: 0xa2, id: 2 },
        Generate { magic: 0x10, data: 0xa3, id: 3 },
        Enumerate { magic: 0x10, index: 0, id: 1, next: 1 },
        Enumerate { magic: 0x10, index: 1, id: 3, next: 3 },
        Enumerate { magic: 0x10, index: 3, id: 0, next: 3 },
    ]),
    ("enum_without_match_keeps_cursor", &[
        Generate { magic: 0x10, data: 0xb1, id: 1 },
        Enumerate { magic: 0x20, index: 0, id: 0, next: 0 },
        Enumerate { magic: 0x20, index: 1, id: 0, next: 1 },
    ]),
    ("enum_past_the_end", &[
        Generate { magic: 0x10, data: 0xc1, id: 1 },
        Enumerate { magic: 0x10, index: 1, id: 0, next: 1 },
        Enumerate { magic: 0x10, index: 5, id: 0, next: 5 },
    ]),
    ("enum_after_reuse", &[
        Generate { magic: 0x10, data: 0xd1, id: 1 },
        Generate { magic: 0x20, data: 0xd2, id: 2 },
        Remove { id: 1, data: Some(0xd1) },
        Generate { magic: 0x20, data: 0xd3, id: 1 },
        Enumerate { magic: 0x20, index: 0, id: 1, next: 1 },
        Enumerate { magic: 0x20, index: 1, id: 2, next: 2 },
    ]),
    ("enum_from_the_middle", &[
        Generate { magic: 0x10, data: 0xe1, id: 1 },
        Generate { magic: 0x10, data: 0xe2, id: 2 },
        Enumerate { magic: 0x10, index: 1, id: 2, next: 2 },
    ]),
    ("enum_after_every_match_removed", &[
        Generate { magic: 0x10, data: 0xf1, id: 1 },
        Remove { id: 1, data: Some(0xf1) },
        Enumerate { magic: 0x10, index: 0, id: 0, next: 0 },
    ]),
    ("two_magics_interleaved_across_reuse", &[
        Generate { magic: 0x10, data: 0x101, id: 1 },
        Generate { magic: 0x20, data: 0x102, id: 2 },
        Generate { magic: 0x10, data: 0x103, id: 3 },
        Remove { id: 1, data: Some(0x101) },
        Generate { magic: 0x20, data: 0x104, id: 1 },
        Enumerate { magic: 0x10, index: 0, id: 3, next: 3 },
        Enumerate { magic: 0x20, index: 0, id: 1, next: 1 },
        Enumerate { magic: 0x20, index: 1, id: 2, next: 2 },
    ]),
    ("table_grows_past_ten", &[
        Generate { magic: 0x10, data: 0x201, id: 1 },
        Generate { magic: 0x10, data: 0x202, id: 2 },
        Generate { magic: 0x10, data: 0x203, id: 3 },
        Generate { magic: 0x10, data: 0x204, id: 4 },
        Generate { magic: 0x10, data: 0x205, id: 5 },
        Generate { magic: 0x10, data: 0x206, id: 6 },
        Generate { magic: 0x10, data: 0x207, id: 7 },
        Generate { magic: 0x10, data: 0x208, id: 8 },
        Generate { magic: 0x10, data: 0x209, id: 9 },
        Generate { magic: 0x10, data: 0x20a, id: 10 },
        Generate { magic: 0x10, data: 0x20b, id: 11 },
        Generate { magic: 0x10, data: 0x20c, id: 12 },
        GetData { id: 12, data: Some(0x20c) },
        GetMagic { id: 11, magic: Some(0x10) },
    ]),
    ("grows_into_holes_after_regrow", &[
        Generate { magic: 0x10, data: 0x301, id: 1 },
        Generate { magic: 0x10, data: 0x302, id: 2 },
        Generate { magic: 0x10, data: 0x303, id: 3 },
        Generate { magic: 0x10, data: 0x304, id: 4 },
        Generate { magic: 0x10, data: 0x305, id: 5 },
        Generate { magic: 0x10, data: 0x306, id: 6 },
        Generate { magic: 0x10, data: 0x307, id: 7 },
        Generate { magic: 0x10, data: 0x308, id: 8 },
        Generate { magic: 0x10, data: 0x309, id: 9 },
        Generate { magic: 0x10, data: 0x30a, id: 10 },
        Generate { magic: 0x10, data: 0x30b, id: 11 },
        Remove { id: 4, data: Some(0x304) },
        Remove { id: 11, data: Some(0x30b) },
        Generate { magic: 0x20, data: 0x311, id: 4 },
        Generate { magic: 0x20, data: 0x312, id: 11 },
        Generate { magic: 0x20, data: 0x313, id: 12 },
    ]),
    ("grows_twice_and_enumerates", &[
        Generate { magic: 0x10, data: 0x401, id: 1 },
        Generate { magic: 0x10, data: 0x402, id: 2 },
        Generate { magic: 0x10, data: 0x403, id: 3 },
        Generate { magic: 0x10, data: 0x404, id: 4 },
        Generate { magic: 0x10, data: 0x405, id: 5 },
        Generate { magic: 0x10, data: 0x406, id: 6 },
        Generate { magic: 0x10, data: 0x407, id: 7 },
        Generate { magic: 0x10, data: 0x408, id: 8 },
        Generate { magic: 0x10, data: 0x409, id: 9 },
        Generate { magic: 0x10, data: 0x40a, id: 10 },
        Generate { magic: 0x10, data: 0x40b, id: 11 },
        Generate { magic: 0x10, data: 0x40c, id: 12 },
        Generate { magic: 0x10, data: 0x40d, id: 13 },
        Generate { magic: 0x10, data: 0x40e, id: 14 },
        Generate { magic: 0x10, data: 0x40f, id: 15 },
        Generate { magic: 0x10, data: 0x410, id: 16 },
        Generate { magic: 0x10, data: 0x411, id: 17 },
        Generate { magic: 0x10, data: 0x412, id: 18 },
        Generate { magic: 0x10, data: 0x413, id: 19 },
        Generate { magic: 0x10, data: 0x414, id: 20 },
        Generate { magic: 0x10, data: 0x415, id: 21 },
        Generate { magic: 0x10, data: 0x416, id: 22 },
        Generate { magic: 0x10, data: 0x417, id: 23 },
        Generate { magic: 0x10, data: 0x418, id: 24 },
        Generate { magic: 0x10, data: 0x419, id: 25 },
        GetData { id: 25, data: Some(0x419) },
        Enumerate { magic: 0x10, index: 20, id: 21, next: 21 },
    ]),
    ("payload_is_opaque", &[
        Generate { magic: 0x10, data: 0xdeadbeef, id: 1 },
        GetData { id: 1, data: Some(0xdeadbeef) },
    ]),
];

#[test]
fn oracle_rows_match_wine() {
    for (name, steps) in ROWS {
        let mut table = CookieTable::new();
        for step in steps.iter() {
            match step {
                Generate { magic, data, id } => {
                    assert_eq!(table.generate(*magic, *data), *id, "{name}");
                }
                GetMagic { id, magic } => assert_eq!(table.get_magic(*id), *magic, "{name}"),
                GetData { id, data } => assert_eq!(table.get_data(*id).copied(), *data, "{name}"),
                Remove { id, data } => assert_eq!(table.remove(*id), *data, "{name}"),
                Enumerate {
                    magic,
                    index,
                    id,
                    next,
                } => {
                    let mut cursor = *index;
                    assert_eq!(table.enumerate(*magic, &mut cursor), *id, "{name}");
                    assert_eq!(cursor, *next, "{name}");
                }
            }
        }
    }
}
