//! Oracle-derived test vectors for the ported cdosys logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies of
//! `DllGetClassObject` (`dlls/cdosys/main.c`), `DllCanUnloadNow`,
//! `DllRegisterServer` and `DllUnregisterServer` (`libs/winecrt0/dll_canunload.c`,
//! `libs/winecrt0/dll_register.c`), and `__wine_register_resources` and
//! `__wine_unregister_resources` (`libs/winecrt0/register.c`), with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/cdosys/`.
//! The harness printed 16 rows; ASan and UBSan reported nothing, so no
//! vectors are excluded.
//!
//! The harness enumerates zero `WINE_REGISTRY` resources, which is the cdosys
//! case. Rows 13-16 show `enum_calls=1`, so the enumeration ran and visited nothing.
//! The Rust port encodes that outcome as `S_OK`.

use super::*;

const CLSID: [u8; 16] = [
    0x2a, 0, 0, 0, 0x34, 0x12, 0x78, 0x56, 1, 2, 3, 4, 5, 6, 7, 8,
];
const IUNKNOWN: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0xc0, 0, 0, 0, 0, 0, 0, 0x46];
const ZERO_GUID: [u8; 16] = [0; 16];

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 dgco_all_null: obj_untouched=1
    assert_eq!(dll_get_class_object(None, None, false) as u32, 0x8004_0111);
    // row 02 dgco_clsid_only
    assert_eq!(
        dll_get_class_object(Some(&CLSID), None, false) as u32,
        0x8004_0111
    );
    // row 03 dgco_riid_only
    assert_eq!(
        dll_get_class_object(None, Some(&CLSID), false) as u32,
        0x8004_0111
    );
    // row 04 dgco_obj_only
    assert_eq!(dll_get_class_object(None, None, true) as u32, 0x8004_0111);
    // row 05 dgco_clsid_riid
    assert_eq!(
        dll_get_class_object(Some(&CLSID), Some(&CLSID), false) as u32,
        0x8004_0111
    );
    // row 06 dgco_clsid_obj
    assert_eq!(
        dll_get_class_object(Some(&CLSID), None, true) as u32,
        0x8004_0111
    );
    // row 07 dgco_riid_obj
    assert_eq!(
        dll_get_class_object(None, Some(&CLSID), true) as u32,
        0x8004_0111
    );
    // row 08 dgco_zero_guids_obj
    assert_eq!(
        dll_get_class_object(Some(&ZERO_GUID), Some(&ZERO_GUID), true) as u32,
        0x8004_0111
    );
    // row 09 dgco_clsid_iunknown_obj
    assert_eq!(
        dll_get_class_object(Some(&CLSID), Some(&IUNKNOWN), true) as u32,
        0x8004_0111
    );
    // row 10 dgco_iunknown_iunknown_obj
    assert_eq!(
        dll_get_class_object(Some(&IUNKNOWN), Some(&IUNKNOWN), true) as u32,
        0x8004_0111
    );
}

#[test]
fn oracle_dll_can_unload_now_rows() {
    // row 11 dcun_first
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
    // row 12 dcun_after_dgco: stateless, so the earlier call changes nothing
    assert_eq!(
        dll_get_class_object(Some(&CLSID), Some(&CLSID), false) as u32,
        0x8004_0111
    );
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
}

#[test]
fn oracle_dll_register_server_rows() {
    // row 13 dreg_first: enum_calls=1, rc=0x00000000
    assert_eq!(dll_register_server() as u32, 0x0000_0000);
    // row 14 dreg_second: enum_calls=1, rc=0x00000000
    assert_eq!(dll_register_server() as u32, 0x0000_0000);
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // row 15 dunreg_first: enum_calls=1, rc=0x00000000
    assert_eq!(dll_unregister_server() as u32, 0x0000_0000);
    // row 16 dunreg_second: enum_calls=1, rc=0x00000000
    assert_eq!(dll_unregister_server() as u32, 0x0000_0000);
}
