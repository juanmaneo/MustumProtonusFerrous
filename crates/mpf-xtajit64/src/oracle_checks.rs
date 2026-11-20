//! Oracle rows for the xtajit64 port, checked against the output of Wine's
//! `cpu.c`, compiled verbatim.
//!
//! The expected lines are copied from the C oracle's run in
//! `target/canon/xtajit64/ok.txt`, which is not committed. Do not edit them by
//! hand.

use crate::is_processor_feature_present;

const ORACLE_ROWS: &[(&str, u32, bool)] = &[
    // feature, expected, from oracle
    ("0", 0, true),
    ("1", 1, true),
    ("2", 2, true),
    ("3", 3, true),
    ("4", 4, true),
    ("5", 5, true),
    ("6", 6, true),
    ("7", 7, true),
    ("8", 8, true),
    ("9", 9, true),
    ("10", 10, true),
    ("11", 11, true),
    ("12", 12, true),
    ("13", 13, false),
    ("14", 14, false),
    ("15", 15, false),
    ("31", 31, false),
    ("32", 32, false),
    ("63", 63, false),
    ("64", 64, false),
    ("65", 65, false),
    ("100", 100, false),
    ("4294967295", u32::MAX, false),
];

#[test]
fn oracle_rows_match_the_c_driver() {
    for (feature_str, feature, expected) in ORACLE_ROWS {
        let result = is_processor_feature_present(*feature);
        assert_eq!(
            result, *expected,
            "feature {} produced {} but oracle produced {}",
            feature_str, result, expected
        );
    }
}
