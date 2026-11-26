//! Oracle-derived test vectors for the ported winnls32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WINNLS32GetEnableStatus`
//! and `WINNLS32EnableIME` (`dlls/winnls32/winnls.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/winnls32/`, and running the harness. The 16 printed rows: 3 for
//! `WINNLS32GetEnableStatus`, 8 for `WINNLS32EnableIME`, and 5 `@ stub` rows. ASan and UBSan
//! reported nothing, so no vector is excluded.

use super::*;

#[test]
fn oracle_get_enable_status_rows() {
    // row 01 WINNLS32GetEnableStatus hwnd=0x0 flag=0 -> ret=0
    assert_eq!(winnls32_get_enable_status(0), 0);
    // row 02 WINNLS32GetEnableStatus hwnd=0x1234 flag=0 -> ret=0
    assert_eq!(winnls32_get_enable_status(0x1234), 0);
    // row 03 WINNLS32GetEnableStatus hwnd=0xffffffffffffffff flag=0 -> ret=0
    assert_eq!(winnls32_get_enable_status(usize::MAX), 0);
}

#[test]
fn oracle_enable_ime_rows() {
    // (row, hwnd, fEnable, ret)
    let rows: [(usize, Hwnd, Bool, Bool); 8] = [
        // row 04 WINNLS32EnableIME hwnd=0x0 flag=0 -> ret=1
        (4, 0, 0, 1),
        // row 05 WINNLS32EnableIME hwnd=0x0 flag=1 -> ret=0
        (5, 0, 1, 0),
        // row 06 WINNLS32EnableIME hwnd=0x1234 flag=0 -> ret=1
        (6, 0x1234, 0, 1),
        // row 07 WINNLS32EnableIME hwnd=0x1234 flag=1 -> ret=0
        (7, 0x1234, 1, 0),
        // row 08 WINNLS32EnableIME hwnd=0x0 flag=2 -> ret=0
        (8, 0, 2, 0),
        // row 09 WINNLS32EnableIME hwnd=0x0 flag=-1 -> ret=0
        (9, 0, -1, 0),
        // row 10 WINNLS32EnableIME hwnd=0x0 flag=-2147483648 -> ret=0
        (10, 0, i32::MIN, 0),
        // row 11 WINNLS32EnableIME hwnd=0x0 flag=2147483647 -> ret=0
        (11, 0, i32::MAX, 0),
    ];
    for (row, hwnd, enable, ret) in rows {
        assert_eq!(winnls32_enable_ime(hwnd, enable), ret, "row {row:02}");
    }
}

#[test]
fn oracle_stub_rows() {
    // row 12 stub WINNLS32GetIMEHotKey -> code=0x80000100 flags=0x1 nparams=2 module=winnls32.dll function=WINNLS32GetIMEHotKey
    assert_eq!(
        winnls32_get_ime_hot_key(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winnls32.dll",
            function: "WINNLS32GetIMEHotKey",
        }
    );
    // row 13 stub IMP32GetIME -> code=0x80000100 flags=0x1 nparams=2 module=winnls32.dll function=IMP32GetIME
    assert_eq!(
        imp32_get_ime(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winnls32.dll",
            function: "IMP32GetIME",
        }
    );
    // row 14 stub IMP32QueryIME -> code=0x80000100 flags=0x1 nparams=2 module=winnls32.dll function=IMP32QueryIME
    assert_eq!(
        imp32_query_ime(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winnls32.dll",
            function: "IMP32QueryIME",
        }
    );
    // row 15 stub IMP32SetIME -> code=0x80000100 flags=0x1 nparams=2 module=winnls32.dll function=IMP32SetIME
    assert_eq!(
        imp32_set_ime(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winnls32.dll",
            function: "IMP32SetIME",
        }
    );
    // row 16 stub IME32SendIMEMessageEx -> code=0x80000100 flags=0x1 nparams=2 module=winnls32.dll function=IME32SendIMEMessageEx
    assert_eq!(
        ime32_send_ime_message_ex(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "winnls32.dll",
            function: "IME32SendIMEMessageEx",
        }
    );
}
