//! Boundary and branch tests for the objsel port.
//!
//! Expectations marked "Not oracle-derived" come from the Wine source, the headers or the IDL, not from
//! a harness row. Each one names the source it follows.

use super::*;
use crate::guid::is_equal_guid;
use std::cell::Cell;

/// Writes a GUID the way the headers do: lower-case hex, in the `DEFINE_GUID` field layout.
fn guid_text(g: &Guid) -> String {
    let d = g.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        g.data1, g.data2, g.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// An allocator that succeeds or returns NULL, as `HeapAlloc` does.
fn heap(ok: bool) -> impl FnOnce() -> bool {
    move || ok
}

/// A picker with the given count and `destroyed` clear.
fn picker(refs: i32) -> ObjectPicker {
    ObjectPicker {
        refs,
        destroyed: false,
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: HRESULT values from include/winerror.h.
    assert_eq!(S_OK as u32, 0x0000_0000);
    assert_eq!(S_FALSE as u32, 0x0000_0001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_POINTER as u32, 0x8000_4003);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
    assert_eq!(CLASS_E_NOAGGREGATION as u32, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);

    // Not oracle-derived: GUID text from include/objsel.h (CLSID and IID of IDsObjectPicker) and
    // include/unknwn.idl (IUnknown and IClassFactory).
    assert_eq!(
        guid_text(&CLSID_DS_OBJECT_PICKER),
        "17d6ccd8-3b7b-11d2-b9e0-00c04fd8dbf7"
    );
    assert_eq!(
        guid_text(&IID_DS_OBJECT_PICKER),
        "0c87e64e-3b7a-11d2-b9e0-00c04fd8dbf7"
    );
    assert_eq!(
        guid_text(&IID_UNKNOWN),
        "00000000-0000-0000-c000-000000000046"
    );
    assert_eq!(
        guid_text(&IID_CLASS_FACTORY),
        "00000001-0000-0000-c000-000000000046"
    );
}

#[test]
fn failed_is_a_sign_test() {
    // Not oracle-derived: FAILED in include/winerror.h is `hr < 0`, so the boundary is the sign bit.
    assert!(!failed(S_OK));
    assert!(!failed(S_FALSE));
    assert!(!failed(i32::MAX));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
    assert!(failed(E_NOINTERFACE));
    assert!(failed(E_FAIL));
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: the harness compares GUIDs with memcmp over sizeof(GUID), so all sixteen bytes
    // count. Each case below changes one field and must not match.
    let base = IID_DS_OBJECT_PICKER;
    let mut last_byte = base.data4;
    last_byte[7] ^= 1;
    assert!(is_equal_guid(&base, &IID_DS_OBJECT_PICKER));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: base.data1 ^ 1,
            ..base
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data2: base.data2 ^ 1,
            ..base
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data3: base.data3 ^ 1,
            ..base
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data4: last_byte,
            ..base
        }
    ));
    assert!(!is_equal_guid(&base, &IID_UNKNOWN));
}

#[test]
fn picker_release_destroys_only_at_zero() {
    // Not oracle-derived: OBJSEL_IDsObjectPicker_Release frees only when `ref == 0`. Rows 18 and 19 cover
    // 2 -> 1 and 1 -> 0. A count already below zero steps further down and is not destroyed.
    let mut p = picker(0);
    assert_eq!(p.release(), u32::MAX);
    assert_eq!(p.refs, -1);
    assert!(!p.destroyed);
}

#[test]
fn picker_counts_wrap_at_the_32_bit_limits() {
    // Not oracle-derived: the port wraps the LONG count as InterlockedIncrement and InterlockedDecrement
    // do. Row 20 covers the increment side, INT32_MAX -> INT32_MIN. This covers the decrement side.
    let mut p = picker(i32::MIN);
    assert_eq!(p.release(), i32::MAX as u32);
    assert_eq!(p.refs, i32::MAX);
    assert!(!p.destroyed);
}

#[test]
fn object_picker_create_failure_keeps_existing_out() {
    // Not oracle-derived: row 13 starts from NULL. The C writes *ppvObj only on success, so a non-NULL
    // *ppvObj survives an allocation failure.
    let mut out = Some(picker(7));
    assert_eq!(object_picker_create(&mut out, heap(false)), E_OUTOFMEMORY);
    assert_eq!(out, Some(picker(7)));
}

#[test]
fn create_instance_checks_out_before_outer_and_iid() {
    // Not oracle-derived: row 12 covers a NULL out with outer set and IDsObjectPicker. The C checks out
    // first, so a NULL out gives E_POINTER whether or not outer is set and whatever the IID is.
    let factory = ClassFactory::default();
    assert_eq!(
        factory.create_instance(true, &IID_UNKNOWN, None, heap(true)),
        E_POINTER
    );
    assert_eq!(
        factory.create_instance(false, &IID_UNKNOWN, None, heap(true)),
        E_POINTER
    );
}

#[test]
fn lock_server_false_drops_a_reference() {
    // Not oracle-derived: row 08 covers LockServer(TRUE). The C calls IClassFactory_Release for FALSE.
    let mut factory = ClassFactory { refs: 1 };
    assert_eq!(factory.lock_server(false), S_OK);
    assert_eq!(factory.refs, 0);
}

#[test]
fn dll_register_and_unregister_call_their_resource_function_once() {
    // Not oracle-derived: the S_OK case of DllRegisterServer, and a call count that shows each export
    // calls its resource function exactly once and returns that result unchanged.
    let calls = Cell::new(0);
    assert_eq!(
        dll_register_server(|| {
            calls.set(calls.get() + 1);
            S_OK
        }),
        S_OK
    );
    assert_eq!(calls.get(), 1);
    assert_eq!(
        dll_unregister_server(|| {
            calls.set(calls.get() + 1);
            E_FAIL
        }),
        E_FAIL
    );
    assert_eq!(calls.get(), 2);
}
