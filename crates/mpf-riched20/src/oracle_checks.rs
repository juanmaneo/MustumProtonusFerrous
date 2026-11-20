//! Rows produced by compiling Wine's verbatim `RTFCharToHex` with ASan and UBSan.

use crate::rtf_char_to_hex;

const ORACLE_ROWS: &[(u8, i32)] = &[
    (48, 0),
    (49, 1),
    (50, 2),
    (51, 3),
    (52, 4),
    (53, 5),
    (54, 6),
    (55, 7),
    (56, 8),
    (57, 9),
    (65, 10),
    (66, 11),
    (67, 12),
    (68, 13),
    (69, 14),
    (70, 15),
    (97, 10),
    (98, 11),
    (99, 12),
    (100, 13),
    (101, 14),
    (102, 15),
    (45, -42),
    (47, -40),
    (58, -29),
];

#[test]
fn matches_wine_oracle_rows() {
    assert_eq!(ORACLE_ROWS.len(), 25);
    for &(byte, expected) in ORACLE_ROWS {
        assert_eq!(rtf_char_to_hex(byte), expected, "byte {byte}");
    }
}
