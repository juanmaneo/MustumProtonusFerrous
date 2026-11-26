//! Oracle-derived test vectors for the dmusic32 stub exports.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/dmusic32/dmusic32_main.c` with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/dmusic32/` and running the harness. The 20 printed rows
//! (12 for `CreateCDirectMusicEmulatePort`, 8 for `EnumLegacyDevices`) all
//! returned `S_OK` (0). ASan and UBSan reported nothing, so 0 vectors were excluded.
//!
//! The printed pointer addresses change between runs and the return value does
//! not depend on them, so each argument is recorded as its class: `NULL` (0), a
//! heap pointer (`HEAP`), the value `1`, or all-ones (`MAX`).

use super::*;

const NULL: usize = 0;
const HEAP: usize = 0x7afe_ee0f_0030;
const ONE: usize = 1;
const MAX: usize = usize::MAX;

#[test]
fn oracle_create_cdirectmusic_emulate_port_rows() {
    let rows: [(usize, usize, usize); 12] = [
        (NULL, HEAP, HEAP + 0x10),
        (HEAP, ONE, NULL),
        (HEAP + 0x10, NULL, ONE),
        (ONE, HEAP + 0x10, HEAP),
        (MAX, MAX, MAX),
        (NULL, HEAP, HEAP + 0x10),
        (HEAP, ONE, NULL),
        (HEAP + 0x10, NULL, ONE),
        (ONE, HEAP + 0x10, HEAP),
        (MAX, MAX, MAX),
        (NULL, HEAP, HEAP + 0x10),
        (HEAP, ONE, NULL),
    ];
    for (p1, p2, p3) in rows {
        assert_eq!(create_cdirectmusic_emulate_port(p1, p2, p3), S_OK);
    }
}

#[test]
fn oracle_enum_legacy_devices_rows() {
    let rows: [(usize, usize); 8] = [
        (NULL, ONE),
        (HEAP, MAX),
        (HEAP + 0x10, NULL),
        (ONE, HEAP),
        (MAX, HEAP + 0x10),
        (NULL, ONE),
        (HEAP, MAX),
        (HEAP + 0x10, NULL),
    ];
    for (p1, p2) in rows {
        assert_eq!(enum_legacy_devices(p1, p2), S_OK);
    }
}
