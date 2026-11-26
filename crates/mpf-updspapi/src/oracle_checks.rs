//! Oracle-derived test vectors for the ported updspapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `UpdSpSetDynamicStringA`
//! (`dlls/updspapi/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/updspapi/`. The harness printed
//! 16 rows: 12 for the C function and 4 for the `@ stub` exports. ASan and UBSan reported nothing,
//! so no vector is excluded.

use super::*;

fn stub(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: "updspapi.dll",
        function,
    }
}

#[test]
fn oracle_upd_sp_set_dynamic_string_a_rows() {
    // row 01 fn=UpdSpSetDynamicStringA hinf=0x0 str1=NULL str2=NULL -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x0, None, None), 1);
    // row 02 fn=UpdSpSetDynamicStringA hinf=0x1 str1=NULL str2=NULL -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x1, None, None), 1);
    // row 03 fn=UpdSpSetDynamicStringA hinf=0x1234 str1="" str2="" -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x1234, Some(b""), Some(b"")), 1);
    // row 04 fn=UpdSpSetDynamicStringA hinf=0x1234 str1="A" str2="B" -> ret=1
    assert_eq!(
        upd_sp_set_dynamic_string_a(0x1234, Some(b"A"), Some(b"B")),
        1
    );
    // row 05 fn=UpdSpSetDynamicStringA hinf=0x0 str1=NULL str2="x" -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x0, None, Some(b"x")), 1);
    // row 06 fn=UpdSpSetDynamicStringA hinf=0x0 str1="x" str2=NULL -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x0, Some(b"x"), None), 1);
    // row 07 fn=UpdSpSetDynamicStringA hinf=0xffffffffffffffff str1="long string with spaces" str2="tail" -> ret=1
    assert_eq!(
        upd_sp_set_dynamic_string_a(usize::MAX, Some(b"long string with spaces"), Some(b"tail")),
        1
    );
    // row 08 fn=UpdSpSetDynamicStringA hinf=0xdeadbeef str1="%s %p %n" str2="%d" -> ret=1
    assert_eq!(
        upd_sp_set_dynamic_string_a(0xdead_beef, Some(b"%s %p %n"), Some(b"%d")),
        1
    );
    // row 09 fn=UpdSpSetDynamicStringA hinf=0x7fff00000000 str1="\xff\x80" str2="\x01" -> ret=1
    assert_eq!(
        upd_sp_set_dynamic_string_a(0x7fff_0000_0000, Some(b"\xff\x80"), Some(b"\x01")),
        1
    );
    // row 10 fn=UpdSpSetDynamicStringA hinf=0x1 str1="" str2=NULL -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x1, Some(b""), None), 1);
    // row 11 fn=UpdSpSetDynamicStringA hinf=0x10 str1=NULL str2="" -> ret=1
    assert_eq!(upd_sp_set_dynamic_string_a(0x10, None, Some(b"")), 1);
    // row 12 fn=UpdSpSetDynamicStringA hinf=0x20 str1="same" str2="same" -> ret=1
    assert_eq!(
        upd_sp_set_dynamic_string_a(0x20, Some(b"same"), Some(b"same")),
        1
    );
}

#[test]
fn oracle_stub_rows() {
    // row 13 stub UpdSpSetDynamicStringExA -> raise code=0x80000100 flags=0x1 nargs=2 p0="updspapi.dll" p1="UpdSpSetDynamicStringExA" raised=1
    assert_eq!(
        upd_sp_set_dynamic_string_ex_a(),
        stub("UpdSpSetDynamicStringExA")
    );
    // row 14 stub UpdSpSetDynamicStringExW -> raise code=0x80000100 flags=0x1 nargs=2 p0="updspapi.dll" p1="UpdSpSetDynamicStringExW" raised=1
    assert_eq!(
        upd_sp_set_dynamic_string_ex_w(),
        stub("UpdSpSetDynamicStringExW")
    );
    // row 15 stub UpdSpSetDynamicStringW -> raise code=0x80000100 flags=0x1 nargs=2 p0="updspapi.dll" p1="UpdSpSetDynamicStringW" raised=1
    assert_eq!(
        upd_sp_set_dynamic_string_w(),
        stub("UpdSpSetDynamicStringW")
    );
    // row 16 stub UpdSpStringTableEnum -> raise code=0x80000100 flags=0x1 nargs=2 p0="updspapi.dll" p1="UpdSpStringTableEnum" raised=1
    assert_eq!(upd_sp_string_table_enum(), stub("UpdSpStringTableEnum"));
}
