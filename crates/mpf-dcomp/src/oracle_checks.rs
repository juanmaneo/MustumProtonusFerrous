//! Oracle-derived test vectors for the ported dcomp logic.
//!
//! Rows were captured by compiling the verbatim Wine C body of
//! `dlls/dcomp/device.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/dcomp/` (raw output in `target/canon/dcomp/rows.txt`). The harness
//! runs each export with four argument patterns and prints the `HRESULT` and whether
//! the out pointer was set. The sanitizers flagged no rows, so none are excluded.
//! The last three rows are the `winerror.h` and `winnt.h` constants the harness prints.

use super::*;

const OUT_SENTINEL: usize = 0x1234;

#[test]
fn oracle_create_device_rows() {
    let obj = Some(0x1000usize);
    let iid = [0u8; 16];

    // DCompositionCreateDevice variant=1 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device(None, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice variant=2 hr=0x80004001 out=set
    let mut out = Some(OUT_SENTINEL);
    assert_eq!(
        dcomposition_create_device(obj, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, Some(OUT_SENTINEL));

    // DCompositionCreateDevice variant=3 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device(None, Some(&iid), Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice variant=4 hr=0x80004001 out=null
    assert_eq!(dcomposition_create_device(obj, Some(&iid), None), E_NOTIMPL);
}

#[test]
fn oracle_create_device2_rows() {
    let obj = Some(0x1000usize);
    let iid = [0u8; 16];

    // DCompositionCreateDevice2 variant=1 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device2(None, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice2 variant=2 hr=0x80004001 out=set
    let mut out = Some(OUT_SENTINEL);
    assert_eq!(
        dcomposition_create_device2(obj, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, Some(OUT_SENTINEL));

    // DCompositionCreateDevice2 variant=3 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device2(None, Some(&iid), Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice2 variant=4 hr=0x80004001 out=null
    assert_eq!(
        dcomposition_create_device2(obj, Some(&iid), None),
        E_NOTIMPL
    );
}

#[test]
fn oracle_create_device3_rows() {
    let obj = Some(0x1000usize);
    let iid = [0u8; 16];

    // DCompositionCreateDevice3 variant=1 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device3(None, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice3 variant=2 hr=0x80004001 out=set
    let mut out = Some(OUT_SENTINEL);
    assert_eq!(
        dcomposition_create_device3(obj, None, Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, Some(OUT_SENTINEL));

    // DCompositionCreateDevice3 variant=3 hr=0x80004001 out=null
    let mut out = None;
    assert_eq!(
        dcomposition_create_device3(None, Some(&iid), Some(&mut out)),
        E_NOTIMPL
    );
    assert_eq!(out, None);

    // DCompositionCreateDevice3 variant=4 hr=0x80004001 out=null
    assert_eq!(
        dcomposition_create_device3(obj, Some(&iid), None),
        E_NOTIMPL
    );
}

#[test]
fn oracle_constant_rows() {
    // E_NOTIMPL value=0x80004001
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    // EXCEPTION_WINE_STUB=0x80000100 EXCEPTION_NONCONTINUABLE=0x01
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}
