//! Oracle-derived test vectors for the ported npmshtml logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `NP_Initialize` and
//! `NP_Shutdown` (`dlls/npmshtml/main.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/npmshtml/`, and running the harness. The 18 printed rows ran clean under ASan
//! and UBSan, so none are excluded. The rows run in the harness order, so the state sequence
//! matches. Rows 07 and 08 print `stack` and `heap` instead of an address, because the address
//! differs per run; the port returns the same value for any pointer.

use super::*;
use core::ffi::c_void;

#[test]
fn oracle_np_initialize_and_shutdown_rows() {
    let mut stack_value = 42i32;
    let mut heap = vec![0u8; 16];

    // row 01 fn=NP_Initialize arg=NULL ret=0
    assert_eq!(np_initialize(core::ptr::null_mut()), 0);
    // row 02 fn=NP_Initialize arg=0x1 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x1)), 0);
    // row 03 fn=NP_Initialize arg=0x10 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x10)), 0);
    // row 04 fn=NP_Initialize arg=0x7fffffff ret=0
    assert_eq!(
        np_initialize(core::ptr::without_provenance_mut(0x7fff_ffff)),
        0
    );
    // row 05 fn=NP_Initialize arg=0xdeadbeef ret=0
    assert_eq!(
        np_initialize(core::ptr::without_provenance_mut(0xdead_beef)),
        0
    );
    // row 06 fn=NP_Initialize arg=0xffffffffffffffff ret=0
    assert_eq!(
        np_initialize(core::ptr::without_provenance_mut(usize::MAX)),
        0
    );
    // row 07 fn=NP_Initialize arg=stack ret=0
    assert_eq!(
        np_initialize((&mut stack_value as *mut i32).cast::<c_void>()),
        0
    );
    // row 08 fn=NP_Initialize arg=heap ret=0
    assert_eq!(np_initialize(heap.as_mut_ptr().cast::<c_void>()), 0);
    // row 09 fn=NP_Shutdown ret=0
    assert_eq!(np_shutdown(), 0);
    // row 10 fn=NP_Initialize arg=NULL ret=0
    assert_eq!(np_initialize(core::ptr::null_mut()), 0);
    // row 11 fn=NP_Shutdown ret=0
    assert_eq!(np_shutdown(), 0);
    // row 12 fn=NP_Shutdown ret=0
    assert_eq!(np_shutdown(), 0);
    // row 13 fn=NP_Initialize arg=0x1 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x1)), 0);
    // row 14 fn=NP_Initialize arg=0x1 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x1)), 0);
    // row 15 fn=NP_Shutdown ret=0
    assert_eq!(np_shutdown(), 0);
    // row 16 fn=NP_Initialize arg=0x2 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x2)), 0);
    // row 17 fn=NP_Initialize arg=0x3 ret=0
    assert_eq!(np_initialize(core::ptr::without_provenance_mut(0x3)), 0);
    // row 18 fn=NP_Shutdown ret=0
    assert_eq!(np_shutdown(), 0);
}
