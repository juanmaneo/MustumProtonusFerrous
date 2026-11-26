//! Oracle-derived test vectors for the ported amsi logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies from `dlls/amsi/main.c`
//! (lines 27-65) and `libs/winecrt0/stub.c` (lines 34-41) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/amsi/` and running the
//! harness. It printed 20 rows with no sanitizer reports. One extra vector, `AmsiInitialize`
//! with a NULL out pointer, is flagged by UBSan and ASan (SEGV) and is excluded.

use super::*;

#[test]
fn oracle_initialize_and_open_session_rows() {
    // 01 AmsiInitialize("amsi-test", ctx_in=NULL) -> hr=0x00000000 out=0xdeadbeef
    let mut ctx = 0x0;
    assert_eq!(amsi_initialize(&mut ctx), 0);
    assert_eq!(ctx, 0xdead_beef);
    // 02 AmsiInitialize(NULL, ctx_in=0x1234) -> hr=0x00000000 out=0xdeadbeef
    let mut ctx = 0x1234;
    assert_eq!(amsi_initialize(&mut ctx), 0);
    assert_eq!(ctx, 0xdead_beef);
    // 03 AmsiInitialize("", ctx_in=0xdeadbeef) -> hr=0x00000000 out=0xdeadbeef
    let mut ctx = 0xdead_beef;
    assert_eq!(amsi_initialize(&mut ctx), 0);
    assert_eq!(ctx, 0xdead_beef);
    // 04 AmsiOpenSession(ctx=NULL, sess_in=NULL) -> hr=0x00000000 out=0xdeadbeef
    let mut sess = 0x0;
    assert_eq!(amsi_open_session(&mut sess), 0);
    assert_eq!(sess, 0xdead_beef);
    // 05 AmsiOpenSession(ctx=0xdeadbeef, sess_in=0x8000) -> hr=0x00000000 out=0xdeadbeef
    let mut sess = 0x8000;
    assert_eq!(amsi_open_session(&mut sess), 0);
    assert_eq!(sess, 0xdead_beef);
    // 06 AmsiOpenSession(ctx=0x1, sess_in=NULL) -> hr=0x00000000 out=0xdeadbeef
    let mut sess = 0x0;
    assert_eq!(amsi_open_session(&mut sess), 0);
    assert_eq!(sess, 0xdead_beef);
}

#[test]
fn oracle_close_session_and_uninitialize_rows() {
    // 07 AmsiCloseSession(NULL, NULL) -> void
    amsi_close_session();
    // 08 AmsiCloseSession(0xdeadbeef, 0xdeadbeef) -> void
    amsi_close_session();
    // 15 AmsiUninitialize(NULL) -> void
    amsi_uninitialize();
    // 16 AmsiUninitialize(0xdeadbeef) -> void
    amsi_uninitialize();
}

#[test]
fn oracle_scan_buffer_rows() {
    // 09 AmsiScanBuffer(ctx=0xdeadbeef, "EICAR", 5, "eicar", sess=0xdeadbeef, res_in=0) -> hr=0x00000000 out=0x1
    let mut res = 0;
    assert_eq!(amsi_scan_buffer(&mut res), 0);
    assert_eq!(res, 0x1);
    // 10 AmsiScanBuffer(NULL, NULL, 0, NULL, NULL, res_in=0x8000) -> hr=0x00000000 out=0x1
    let mut res = 0x8000;
    assert_eq!(amsi_scan_buffer(&mut res), 0);
    assert_eq!(res, 0x1);
    // 11 AmsiScanBuffer(ctx=0xdeadbeef, "EICAR", 0xffffffff, "x", sess=0x1, res_in=0x4000) -> hr=0x00000000 out=0x1
    let mut res = 0x4000;
    assert_eq!(amsi_scan_buffer(&mut res), 0);
    assert_eq!(res, 0x1);
}

#[test]
fn oracle_scan_string_rows() {
    // 12 AmsiScanString(ctx=0xdeadbeef, "Invoke-Expression", "ps", sess=0xdeadbeef, res_in=0) -> hr=0x00000000 out=0x1
    let mut res = 0;
    assert_eq!(amsi_scan_string(&mut res), 0);
    assert_eq!(res, 0x1);
    // 13 AmsiScanString(NULL, NULL, NULL, NULL, res_in=0xffffffff) -> hr=0x00000000 out=0x1
    let mut res = 0xffff_ffff;
    assert_eq!(amsi_scan_string(&mut res), 0);
    assert_eq!(res, 0x1);
    // 14 AmsiScanString(NULL, "", "", sess=0x0, res_in=0x8000) -> hr=0x00000000 out=0x1
    let mut res = 0x8000;
    assert_eq!(amsi_scan_string(&mut res), 0);
    assert_eq!(res, 0x1);
}

#[test]
fn oracle_stub_rows() {
    // 17 AmsiUacInitialize() -> raise code=0x80000100 flags=0x01 nargs=2 module="amsi.dll" function="AmsiUacInitialize"
    // 18 AmsiUacScan() -> raise code=0x80000100 flags=0x01 nargs=2 module="amsi.dll" function="AmsiUacScan"
    // 19 AmsiUacUninitialize() -> raise code=0x80000100 flags=0x01 nargs=2 module="amsi.dll" function="AmsiUacUninitialize"
    // 20 DllGetClassObject() -> raise code=0x80000100 flags=0x01 nargs=2 module="amsi.dll" function="DllGetClassObject"
    for function in [
        "AmsiUacInitialize",
        "AmsiUacScan",
        "AmsiUacUninitialize",
        "DllGetClassObject",
    ] {
        let e = wine_spec_unimplemented_stub("amsi.dll", function);
        assert_eq!(e.code, 0x8000_0100);
        assert_eq!(e.flags, 0x01);
        assert_eq!(e.module, "amsi.dll");
        assert_eq!(e.function, function);
    }
}
