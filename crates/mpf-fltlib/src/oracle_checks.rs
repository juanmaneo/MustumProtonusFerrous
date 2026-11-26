//! Oracle-derived test vectors for the ported fltlib logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/fltlib/fltlib.c`
//! and `include/winerror.h` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/fltlib/` and running the harness. The 22 printed rows were all clean under
//! ASan and UBSan, so no vector is excluded. The C stubs read their arguments only for the
//! trace call, so the connect and find rows vary inputs that the Rust stubs do not take.

use super::*;

fn name() -> Vec<u16> {
    "MyFilter".encode_utf16().collect()
}

#[test]
fn oracle_hresult_from_win32_rows() {
    // hresult_from_win32 <x> => <hr>
    let rows: [(u32, i32); 11] = [
        (0, 0),
        (1, -2147024895),
        (2, -2147024894),
        (18, -2147024878),
        (50, -2147024846),
        (87, -2147024809),
        (259, -2147024637),
        (2147483647, -2146959361),
        (2147483648, -2147483648),
        (4294967295, -1),
        (65795, -2147024637),
    ];
    for (x, want) in rows {
        assert_eq!(hresult_from_win32(x), want, "input {x}");
    }
}

#[test]
fn oracle_connect_communication_port_rows() {
    // connect name=set opts=0 ctx=0 => hr=-2147467263 port=-1
    // connect name=null opts=7 ctx=16 => hr=-2147467263 port=-1
    // connect name=empty opts=max ctx=max => hr=-2147467263 port=-1
    for _ in 0..3 {
        assert_eq!(filter_connect_communication_port(), (-2147467263, -1));
    }
}

#[test]
fn oracle_find_first_rows() {
    // findfirst class=0 buf=null size=0 => hr=-2147024637
    // findfirst class=1 buf=set size=4 => hr=-2147024637
    assert_eq!(filter_find_first(), -2147024637);
    assert_eq!(filter_find_first(), -2147024637);
}

#[test]
fn oracle_find_close_rows() {
    // findclose handle=null => hr=0
    // findclose handle=0x1234 => hr=0
    assert_eq!(filter_find_close(), 0);
    assert_eq!(filter_find_close(), 0);
}

#[test]
fn oracle_load_rows() {
    // load name=null => hr=-2147024846
    // load name=set => hr=-2147024846
    assert_eq!(filter_load(), -2147024846);
    assert_eq!(filter_load(), -2147024846);
}

#[test]
fn oracle_unload_rows() {
    // unload name=null => hr=-2147024809
    // unload name=set => hr=0
    assert_eq!(filter_unload(None), -2147024809);
    assert_eq!(filter_unload(Some(&name())), 0);
}
