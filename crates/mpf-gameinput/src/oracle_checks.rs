//! Oracle-derived test vectors for the ported gameinput stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/gameinput/gameinput.c` with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/gameinput/` and running the harness. The harness printed
//! 18 rows (C01 to C06, D01 to D12). ASan and UBSan flagged none of them, so none are excluded.
//!
//! The C body never reads `clsid` or `riid` (only the `FIXME` log does), so the
//! Rust signature omits them. The D rows vary them to show that they make no difference.

use super::*;

#[test]
fn oracle_game_input_create_rows() {
    // (id, out is non-null, slot before, return value, slot after).
    // Rows C01 to C06 of target/canon/gameinput/out.txt.
    let rows: [(&str, bool, usize, u32, usize); 6] = [
        ("C01", false, 0, 0x8000_4001, 0),
        ("C02", true, 0, 0x8000_4001, 0),
        ("C03", true, 0x1234, 0x8000_4001, 0x1234),
        ("C04", true, usize::MAX, 0x8000_4001, usize::MAX),
        ("C05", true, 0x1234, 0x8000_4001, 0x1234),
        ("C06", false, 0x1234, 0x8000_4001, 0x1234),
    ];
    for (id, have_out, slot_before, ret, slot_after) in rows {
        let mut slot = slot_before;
        let hr = if have_out {
            game_input_create(Some(&mut slot))
        } else {
            game_input_create::<usize>(None)
        };
        assert_eq!(hr as u32, ret, "{id}");
        assert_eq!(slot, slot_after, "{id}");
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // (id, out is non-null, slot before, return value, slot after).
    // Rows D01 to D12 of target/canon/gameinput/out.txt. clsid and riid vary per row
    // in the C harness and are not part of the Rust signature.
    let rows: [(&str, bool, usize, u32, usize); 12] = [
        ("D01", false, 0, 0x8004_0111, 0),
        ("D02", true, 0, 0x8004_0111, 0),
        ("D03", true, 0x1234, 0x8004_0111, 0x1234),
        ("D04", false, 0, 0x8004_0111, 0),
        ("D05", true, 0, 0x8004_0111, 0),
        ("D06", true, 0x1234, 0x8004_0111, 0x1234),
        ("D07", true, 0, 0x8004_0111, 0),
        ("D08", true, 0x1234, 0x8004_0111, 0x1234),
        ("D09", true, 0x1234, 0x8004_0111, 0x1234),
        ("D10", true, 0x1234, 0x8004_0111, 0x1234),
        ("D11", false, 0x1234, 0x8004_0111, 0x1234),
        ("D12", true, usize::MAX, 0x8004_0111, usize::MAX),
    ];
    for (id, have_out, slot_before, ret, slot_after) in rows {
        let mut slot = slot_before;
        let hr = if have_out {
            dll_get_class_object(Some(&mut slot))
        } else {
            dll_get_class_object::<usize>(None)
        };
        assert_eq!(hr as u32, ret, "{id}");
        assert_eq!(slot, slot_after, "{id}");
    }
}
