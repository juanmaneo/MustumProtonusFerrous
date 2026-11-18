//! Replays the Wine oracle (`target/canon/inetcpl.cpl/oracle.c`) against this crate.
//! The rows are the verbatim output of that driver, which compiles lines 59-86 of
//! `dlls/inetcpl.cpl/security.c` and copies the expressions of lines 104 and 117,
//! built with `gcc -fsanitize=address,undefined`. The run reported no sanitizer findings.

use super::*;

const INDEX_ROWS: &[(u32, u32)] = &[
    (0x00000000, 0),
    (0x00000001, 4294967295),
    (0x0000ffff, 4294967295),
    (0x00010000, 1),
    (0x00010001, 4294967295),
    (0x00010500, 2),
    (0x00011000, 3),
    (0x00011500, 4),
    (0x00012000, 5),
    (0x00012001, 4294967295),
    (0x00020000, 4294967295),
    (0xffffffff, 4294967295),
];

const SELECT_ROWS: &[(u32, u32, u32)] = &[
    (0, 0x00010000, 1),
    (3, 0x00011000, 3),
    (0, 0x00000099, 4294967295),
    (4, 0x00012000, 4),
];

const TRACKBAR_ROWS: &[(u32, u32)] = &[(0, 5), (1, 4), (2, 3), (5, 0), (4294967295, 6)];

#[test]
fn index_from_urltemplate_matches_oracle() {
    for &(value, expected) in INDEX_ROWS {
        assert_eq!(index_from_urltemplate(value), expected, "value {value:#x}");
    }
}

#[test]
fn current_index_matches_oracle() {
    for &(tb_index, level, expected) in SELECT_ROWS {
        assert_eq!(
            current_index(tb_index, level),
            expected,
            "tb {tb_index} level {level:#x}"
        );
    }
}

#[test]
fn trackbar_pos_matches_oracle() {
    for &(current, expected) in TRACKBAR_ROWS {
        assert_eq!(trackbar_pos(current), expected, "current {current}");
    }
}
