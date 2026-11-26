//! Oracle-derived test vectors for `DllGetClassObject`.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `DllGetClassObject`
//! (`dlls/msisys.ocx/msisys.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/msisys.ocx/`, and running the harness. All 18 printed rows ran clean under
//! ASan and UBSan, so none are excluded.

use super::*;

/// (row, clsid, iid, ppv, hr) as printed by the harness. `hr` is the returned HRESULT in
/// unsigned hex. `ppv` is NULL in the harness for odd-numbered rows and a non-NULL slot for
/// even-numbered rows, and the slot is never written.
const ROWS: [(u8, &str, &str, &str, u32); 18] = [
    (1, "NULL", "NULL", "NULL", 0x8004_0111),
    (2, "NULL", "NULL", "&slot(0x1234)", 0x8004_0111),
    (3, "NULL", "IUnknown", "NULL", 0x8004_0111),
    (4, "NULL", "IUnknown", "&slot(0x1234)", 0x8004_0111),
    (5, "NULL", "IClassFactory", "NULL", 0x8004_0111),
    (6, "NULL", "IClassFactory", "&slot(0x1234)", 0x8004_0111),
    (7, "A", "NULL", "NULL", 0x8004_0111),
    (8, "A", "NULL", "&slot(0x1234)", 0x8004_0111),
    (9, "A", "IUnknown", "NULL", 0x8004_0111),
    (10, "A", "IUnknown", "&slot(0x1234)", 0x8004_0111),
    (11, "A", "IClassFactory", "NULL", 0x8004_0111),
    (12, "A", "IClassFactory", "&slot(0x1234)", 0x8004_0111),
    (13, "ZERO", "NULL", "NULL", 0x8004_0111),
    (14, "ZERO", "NULL", "&slot(0x1234)", 0x8004_0111),
    (15, "ZERO", "IUnknown", "NULL", 0x8004_0111),
    (16, "ZERO", "IUnknown", "&slot(0x1234)", 0x8004_0111),
    (17, "ZERO", "IClassFactory", "NULL", 0x8004_0111),
    (18, "ZERO", "IClassFactory", "&slot(0x1234)", 0x8004_0111),
];

#[test]
fn oracle_dll_get_class_object_rows() {
    for (row, clsid, iid, ppv, hr) in ROWS {
        // The port takes no arguments. The C body ignores clsid, iid and ppv, so the same
        // HRESULT comes back for every input row.
        assert_eq!(
            dll_get_class_object() as u32,
            hr,
            "row {row:02} clsid={clsid} iid={iid} ppv={ppv}"
        );
    }
}
