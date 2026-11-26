//! Oracle-derived test vectors for the ported dxtrans logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies of
//! `DllGetClassObject` (`dlls/dxtrans/main.c`) and `__wine_spec_unimplemented_stub`
//! (`libs/winecrt0/stub.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/dxtrans/`. The harness printed 15 rows; ASan and UBSan reported
//! nothing, so no vectors are excluded.

use super::*;

const CLSID: [u8; 16] = [
    0x2a, 0x00, 0x00, 0x00, 0x34, 0x12, 0x78, 0x56, 1, 2, 3, 4, 5, 6, 7, 8,
];
const ZERO_GUID: [u8; 16] = [0; 16];

fn stub_row(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        module: "dxtrans.dll",
        function,
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 dgco_all_null
    assert_eq!(dll_get_class_object(None, None, false), 0x8004_0111);
    // row 02 dgco_clsid_only
    assert_eq!(dll_get_class_object(Some(&CLSID), None, false), 0x8004_0111);
    // row 03 dgco_riid_only
    assert_eq!(dll_get_class_object(None, Some(&CLSID), false), 0x8004_0111);
    // row 04 dgco_out_only
    assert_eq!(dll_get_class_object(None, None, true), 0x8004_0111);
    // row 05 dgco_clsid_riid
    assert_eq!(
        dll_get_class_object(Some(&CLSID), Some(&CLSID), false),
        0x8004_0111
    );
    // row 06 dgco_all_set
    assert_eq!(
        dll_get_class_object(Some(&CLSID), Some(&ZERO_GUID), true),
        0x8004_0111
    );
    // row 07 dgco_zero_guids
    assert_eq!(
        dll_get_class_object(Some(&ZERO_GUID), Some(&ZERO_GUID), true),
        0x8004_0111
    );
    // row 08 dgco_clsid_out
    assert_eq!(dll_get_class_object(Some(&CLSID), None, true), 0x8004_0111);
}

#[test]
fn oracle_unimplemented_stub_rows() {
    // row 09 stub_DllEnumClassObjects
    assert_eq!(
        unimplemented_stub("dxtrans.dll", "DllEnumClassObjects"),
        stub_row("DllEnumClassObjects")
    );
    // rows 10-15 stub_?DX*
    for (row, name) in STUB_EXPORTS.iter().enumerate().skip(1) {
        assert_eq!(
            unimplemented_stub("dxtrans.dll", name),
            stub_row(name),
            "row {:02}",
            row + 9
        );
    }
}
