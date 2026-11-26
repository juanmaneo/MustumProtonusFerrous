//! Oracle-derived test vectors for the ported `msvcp140_codecvt_ids` data exports.
//!
//! Vectors were captured by compiling the verbatim `dlls/msvcp140_codecvt_ids/main.c`, copied to
//! `target/canon/msvcp140_codecvt_ids/main.c`, with `gcc -fsanitize=address,undefined -g` through
//! `harness.c`, which includes it after `<stddef.h>`. The harness printed 15 rows. ASan and UBSan
//! flagged none, so no vector is excluded. Rows 06-09 echo the mangled names from
//! `msvcp140_codecvt_ids.spec`, which the comments in `main.c` repeat.

use super::*;
use std::mem::{align_of, offset_of, size_of};

/// Rows 01-04: the `symbol`, `sizeof`, `alignof` and `init_id` of each variable.
const VARIABLE_ROWS: [(&str, usize, usize, usize); 4] = [
    ("codecvt_char16_id", 8, 8, 0),
    ("codecvt_char16_char8_id", 8, 8, 0),
    ("codecvt_char32_id", 8, 8, 0),
    ("codecvt_char32_char8_id", 8, 8, 0),
];

/// Rows 06-09: the spec's exported name and the C symbol it maps to.
const SPEC_ROWS: [(&str, &str); 4] = [
    (
        "?id@?$codecvt@_SDU_Mbstatet@@@std@@2V0locale@2@A",
        "codecvt_char16_id",
    ),
    (
        "?id@?$codecvt@_S_QU_Mbstatet@@@std@@2V0locale@2@A",
        "codecvt_char16_char8_id",
    ),
    (
        "?id@?$codecvt@_UDU_Mbstatet@@@std@@2V0locale@2@A",
        "codecvt_char32_id",
    ),
    (
        "?id@?$codecvt@_U_QU_Mbstatet@@@std@@2V0locale@2@A",
        "codecvt_char32_char8_id",
    ),
];

#[test]
fn oracle_variable_rows() {
    for (export, &(symbol, sizeof, alignof, init_id)) in EXPORTS.iter().zip(VARIABLE_ROWS.iter()) {
        assert_eq!(export.symbol, symbol);
        assert_eq!(size_of::<LocaleId>(), sizeof);
        assert_eq!(align_of::<LocaleId>(), alignof);
        assert_eq!(export.initial.id, init_id);
    }
}

#[test]
fn oracle_locale_id_row() {
    // row 05 locale_id sizeof=8 alignof=8 offsetof_id=0
    assert_eq!(size_of::<LocaleId>(), 8);
    assert_eq!(align_of::<LocaleId>(), 8);
    assert_eq!(offset_of!(LocaleId, id), 0);
}

#[test]
fn oracle_spec_rows() {
    for &(name, symbol) in SPEC_ROWS.iter() {
        let export = export_by_name(name).expect("spec export");
        assert_eq!(export.symbol, symbol);
    }
}

#[test]
fn oracle_distinct_rows() {
    // rows 10-15: every pair of variables has a different address, so each export is its own
    // storage. Six pairs out of four symbols.
    for (i, a) in EXPORTS.iter().enumerate() {
        for b in &EXPORTS[i + 1..] {
            assert_ne!(a.symbol, b.symbol);
            assert_ne!(a.name, b.name);
        }
    }
}
