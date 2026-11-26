//! Oracle-derived test vectors for the ported dpnhpast logic.
//!
//! Captured by compiling the verbatim sources `dlls/dpnhpast/main.c`,
//! `libs/winecrt0/dll_canunload.c`, `dll_register.c` and `register.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/dpnhpast/`. The harness
//! printed 23 rows (`row=00` to `row=22`), each run in a forked child. ASan and UBSan
//! flagged none of them, so no vector is excluded. Expected results use the printed hex.

use super::ResourceStep::{NotLoaded, OutOfMemory, Registered, RegistrarFailed};
use super::*;
use std::ffi::c_void;

const IID: [u8; 16] = [
    0x78, 0x56, 0x34, 0x12, 0x34, 0x12, 0x78, 0x56, 1, 2, 3, 4, 5, 6, 7, 8,
];
const CLSID: [u8; 16] = [
    0xf0, 0xde, 0xbc, 0x9a, 0x11, 0x11, 0x22, 0x22, 9, 10, 11, 12, 13, 14, 15, 16,
];

/// An HRESULT written as the oracle prints it.
fn hr(bits: u32) -> Hresult {
    bits as i32
}

#[test]
fn oracle_direct_play_nat_help_create_rows() {
    // row=00 flags=0: pIID NULL, ppvInterface NULL
    assert_eq!(direct_play_nat_help_create(None, None), hr(0x8000_4001));
    // row=01 flags=1: pIID set, ppvInterface NULL
    assert_eq!(
        direct_play_nat_help_create(Some(&IID), None),
        hr(0x8000_4001)
    );
    // row=02 flags=2: ppvInterface set, the slot is left unchanged
    let sentinel = 0x1234usize as *mut c_void;
    let mut slot = sentinel;
    assert_eq!(
        direct_play_nat_help_create(None, Some(&mut slot)),
        hr(0x8000_4001)
    );
    assert_eq!(slot, sentinel);
    // row=03 flags=3: both set, the slot is left unchanged
    let mut slot = sentinel;
    assert_eq!(
        direct_play_nat_help_create(Some(&IID), Some(&mut slot)),
        hr(0x8000_4001)
    );
    assert_eq!(slot, sentinel);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row=04 flags=0: ppv NULL
    assert_eq!(dll_get_class_object(&CLSID, &IID, None), hr(0x8000_4005));
    // row=05 flags=2: ppv set, the slot is left unchanged
    let sentinel = 0x1234usize as *mut c_void;
    let mut slot = sentinel;
    assert_eq!(
        dll_get_class_object(&CLSID, &IID, Some(&mut slot)),
        hr(0x8000_4005)
    );
    assert_eq!(slot, sentinel);
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // row=06
    assert_eq!(dll_can_unload_now(), hr(0x0000_0001));
}

#[test]
fn oracle_dll_register_server_rows() {
    let rows: &[(&[ResourceStep], Hresult)] = &[
        // row=07 steps=-
        (&[], hr(0x0000_0000)),
        // row=08 steps=S
        (&[Registered(S_OK)], hr(0x0000_0000)),
        // row=09 steps=SS
        (&[Registered(S_OK), Registered(S_OK)], hr(0x0000_0000)),
        // row=10 steps=N
        (&[NotLoaded], hr(0x0000_0000)),
        // row=11 steps=NS
        (&[NotLoaded, Registered(S_OK)], hr(0x0000_0000)),
        // row=12 steps=O
        (&[OutOfMemory], hr(0x8007_000e)),
        // row=13 steps=F
        (&[Registered(E_FAIL)], hr(0x8000_4005)),
        // row=14 steps=SFS
        (
            &[Registered(S_OK), Registered(E_FAIL), Registered(S_OK)],
            hr(0x8000_4005),
        ),
        // row=15 steps=I
        (&[Registered(S_FALSE)], hr(0x0000_0001)),
        // row=16 steps=A
        (&[RegistrarFailed(E_NOINTERFACE)], hr(0x8000_4002)),
        // row=17 steps=SO
        (&[Registered(S_OK), OutOfMemory], hr(0x8007_000e)),
        // row=18 steps=AS
        (
            &[RegistrarFailed(E_NOINTERFACE), Registered(S_OK)],
            hr(0x8000_4002),
        ),
    ];
    for &(steps, want) in rows {
        assert_eq!(dll_register_server(steps), want, "steps {steps:?}");
    }
}

#[test]
fn oracle_dll_unregister_server_rows() {
    let rows: &[(&[ResourceStep], Hresult)] = &[
        // row=19 steps=-
        (&[], hr(0x0000_0000)),
        // row=20 steps=SF
        (&[Registered(S_OK), Registered(E_FAIL)], hr(0x8000_4005)),
        // row=21 steps=IS
        (&[Registered(S_FALSE), Registered(S_OK)], hr(0x0000_0000)),
        // row=22 steps=A
        (&[RegistrarFailed(E_NOINTERFACE)], hr(0x8000_4002)),
    ];
    for &(steps, want) in rows {
        assert_eq!(dll_unregister_server(steps), want, "steps {steps:?}");
    }
}
