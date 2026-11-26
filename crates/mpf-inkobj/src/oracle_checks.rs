//! Oracle-derived test vectors for the ported inkobj logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `DllGetClassObject`
//! (`dlls/inkobj/inkobj.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/inkobj/`, and running the harness. The 17 printed rows were all clean: ASan and
//! UBSan reported nothing, so no vector is excluded. Each row also passes a `ppv` out-pointer,
//! which is NULL or a sentinel. The C body leaves it unchanged, and the port does not model it.

use super::*;

const ZERO: Guid = [0; 16];
const ONES: Guid = [0xff; 16];
const CLSID: Guid = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
const IUNKNOWN: Guid = [0, 0, 0, 0, 0, 0, 0, 0, 0xc0, 0, 0, 0, 0, 0, 0, 0x46];

/// `(rclsid, riid, return value)` for each harness row, in row order.
const ROWS: [(Option<&Guid>, Option<&Guid>, Hresult); 17] = [
    // row 01 rclsid=NULL riid=NULL ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (None, None, CLASS_E_CLASSNOTAVAILABLE),
    // row 02 rclsid=NULL riid=NULL ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (None, None, CLASS_E_CLASSNOTAVAILABLE),
    // row 03 rclsid=ZERO riid=ZERO ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (Some(&ZERO), Some(&ZERO), CLASS_E_CLASSNOTAVAILABLE),
    // row 04 rclsid=ZERO riid=ZERO ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&ZERO), Some(&ZERO), CLASS_E_CLASSNOTAVAILABLE),
    // row 05 rclsid=ZERO riid=NULL ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&ZERO), None, CLASS_E_CLASSNOTAVAILABLE),
    // row 06 rclsid=NULL riid=IUNKNOWN ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (None, Some(&IUNKNOWN), CLASS_E_CLASSNOTAVAILABLE),
    // row 07 rclsid=ZERO riid=IUNKNOWN ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (Some(&ZERO), Some(&IUNKNOWN), CLASS_E_CLASSNOTAVAILABLE),
    // row 08 rclsid=CLSID riid=NULL ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&CLSID), None, CLASS_E_CLASSNOTAVAILABLE),
    // row 09 rclsid=CLSID riid=IUNKNOWN ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&CLSID), Some(&IUNKNOWN), CLASS_E_CLASSNOTAVAILABLE),
    // row 10 rclsid=CLSID riid=IUNKNOWN ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (Some(&CLSID), Some(&IUNKNOWN), CLASS_E_CLASSNOTAVAILABLE),
    // row 11 rclsid=CLSID riid=ZERO ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&CLSID), Some(&ZERO), CLASS_E_CLASSNOTAVAILABLE),
    // row 12 rclsid=CLSID riid=CLSID ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&CLSID), Some(&CLSID), CLASS_E_CLASSNOTAVAILABLE),
    // row 13 rclsid=ONES riid=ONES ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&ONES), Some(&ONES), CLASS_E_CLASSNOTAVAILABLE),
    // row 14 rclsid=ONES riid=NULL ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (Some(&ONES), None, CLASS_E_CLASSNOTAVAILABLE),
    // row 15 rclsid=NULL riid=ZERO ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (None, Some(&ZERO), CLASS_E_CLASSNOTAVAILABLE),
    // row 16 rclsid=ONES riid=IUNKNOWN ppv=sentinel -> ret=0x80040111 ppv_after=sentinel
    (Some(&ONES), Some(&IUNKNOWN), CLASS_E_CLASSNOTAVAILABLE),
    // row 17 rclsid=CLSID riid=CLSID ppv=NULL -> ret=0x80040111 ppv_after=NULL
    (Some(&CLSID), Some(&CLSID), CLASS_E_CLASSNOTAVAILABLE),
];

#[test]
fn oracle_dll_get_class_object_rows() {
    for (index, (rclsid, riid, expected)) in ROWS.into_iter().enumerate() {
        assert_eq!(
            dll_get_class_object(rclsid, riid),
            expected,
            "row {:02}",
            index + 1
        );
    }
}
