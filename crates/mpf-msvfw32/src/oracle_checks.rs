//! Replays of the oracle rows. The oracle compiles the verbatim `get_size_image`, `num_colours`,
//! `compare_fourcc` and `fourcc_to_string` from `dlls/msvfw32`, and the C-locale `_strnicmp` loop
//! from `dlls/msvcrt/string.c`, with `-fsanitize=address,undefined`. The inputs are in
//! `target/canon/msvfw32/vectors.txt`, and the IDs O1 to O25 match that file. The expected values
//! are the sanitized binary's output. No row trips a sanitizer.

use super::*;

#[test]
fn oracle_size_image_rows() {
    let rows: [(&str, i32, i32, u16, u32); 10] = [
        ("O1", 4, 4, 8, 16),
        ("O2", 5, 3, 8, 24),
        ("O3", 3, 2, 24, 24),
        ("O4", 640, -480, 24, 921_600),
        ("O5", 7, 1, 1, 4),
        ("O6", -4, 4, 8, 2_147_483_632),
        ("O7", 1, 1, 4, 4),
        ("O8", 10, 0, 8, 0),
        ("O9", 0, 10, 8, 0),
        ("O10", 100_000, 2, 8, 200_000),
    ];
    for (id, width, height, depth, want) in rows {
        assert_eq!(get_size_image(width, height, depth), want, "{id}");
    }
}

#[test]
fn oracle_num_colours_rows() {
    let rows: [(&str, u32, u16, i32); 5] = [
        ("O11", 0, 8, 256),
        ("O12", 0, 4, 16),
        ("O13", 0, 24, 0),
        ("O14", 16, 24, 16),
        ("O15", 0, 0, 1),
    ];
    for (id, clr_used, bit_count, want) in rows {
        assert_eq!(num_colours(clr_used, bit_count), want, "{id}");
    }
}

#[test]
fn oracle_fourcc_rows() {
    let rows: [(&str, u32, u32, bool); 10] = [
        ("O16", 0x4D41_5243, 0x6D61_7263, true),
        ("O17", 0x4D41_5243, 0x4356_534D, false),
        ("O18", 0x40, 0x60, false),
        ("O19", 0x5958_0041, 0x575A_0061, true),
        ("O20", 0x5958_0041, 0x5958_0161, false),
        ("O21", 0, 0, true),
        ("O22", 0x4342_4100, 0x4442_4100, true),
        ("O23", 0x4443_4241, 0x4543_4241, false),
        ("O24", 0x0000_00C1, 0x0000_00E1, false),
        ("O25", 0x4D41_4857, 0x6D61_6877, true),
    ];
    for (id, a, b, want) in rows {
        assert_eq!(fourcc_eq_ignore_case(a, b), want, "{id}");
    }
}
