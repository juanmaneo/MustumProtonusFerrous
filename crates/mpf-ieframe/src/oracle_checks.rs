//! Oracle rows from Wine's verbatim C (`target/canon/ieframe/oracle.c`, built with
//! `gcc -fsanitize=address,undefined -g`). The harness holds lines 714 and 718 to 720 of
//! `dlls/ieframe/intshcut.c` unchanged, and prints the mode that `PropertySetStorage_Open` computes
//! for each input. Each row is an input `grfMode` and the value the C code printed. The sanitizers
//! reported nothing for these inputs, so no vector is excluded.

use super::*;

/// `(grfMode, mode the C code passes on)`.
const ROWS: &[(u32, u32)] = &[
    (0x0000_0000, 0x0000_0010),
    (0x0000_0001, 0x0000_0012),
    (0x0000_0002, 0x0000_0012),
    (0x0000_0003, 0x0000_0013),
    (0x0000_0004, 0x0000_0014),
    (0x0000_0010, 0x0000_0010),
    (0x0000_0011, 0x0000_0012),
    (0x0000_0012, 0x0000_0012),
    (0x0000_0013, 0x0000_0013),
    (0x0000_0020, 0x0000_0030),
    (0x0000_0021, 0x0000_0032),
    (0x0000_00f1, 0x0000_00f2),
    (0x0000_00ff, 0x0000_00ff),
    (0x0000_001f, 0x0000_001f),
    (0x8000_0001, 0x8000_0012),
    (0x7fff_ffff, 0x7fff_ffff),
    (0xffff_ffff, 0xffff_ffff),
    (0xffff_fff1, 0xffff_fff2),
    (0xffff_fff2, 0xffff_fff2),
    (0x1122_3341, 0x1122_3352),
    (0x0000_0011, 0x0000_0012),
];

#[test]
fn oracle_rows_match_wine() {
    for &(grf_mode, mode) in ROWS {
        assert_eq!(open_mode(grf_mode), mode, "grfMode {grf_mode:#010x}");
    }
}
