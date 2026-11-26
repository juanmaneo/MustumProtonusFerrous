//! Edge-case tests for the ported thumbcache logic.
//!
//! These cover the constants, the `QueryInterface` check, the reference-count wrap, the allocator
//! contract, the CLSID match, and the export table. The oracle rows cover the Wine behaviour; the
//! expectations here that are not oracle-derived say so in a comment.

use super::*;
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/thumbcache.idl,
    // include/unknwn.idl and dlls/thumbcache/thumbcache_reg.idl.
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL, 0x8000_4001_u32 as i32);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as i32);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000e_u32 as i32);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111_u32 as i32);
    assert_eq!(IID_IUNKNOWN.data1, 0);
    assert_eq!(IID_IUNKNOWN.data4, [0xc0, 0, 0, 0, 0, 0, 0, 0x46]);
    assert_eq!(IID_ICLASSFACTORY.data1, 1);
    assert_eq!(IID_ICLASSFACTORY.data4, [0xc0, 0, 0, 0, 0, 0, 0, 0x46]);
    assert_eq!(IID_ITHUMBNAILCACHE.data1, 0xf676_c15d);
    assert_eq!(IID_ITHUMBNAILCACHE.data2, 0x596a);
    assert_eq!(IID_ITHUMBNAILCACHE.data3, 0x4ce2);
    assert_eq!(
        IID_ITHUMBNAILCACHE.data4,
        [0x82, 0x34, 0x33, 0x99, 0x6f, 0x44, 0x5d, 0xb1]
    );
    assert_eq!(CLSID_LOCAL_THUMBNAIL_CACHE.data1, 0x50ef_4544);
    assert_eq!(CLSID_LOCAL_THUMBNAIL_CACHE.data2, 0xac9f);
    assert_eq!(CLSID_LOCAL_THUMBNAIL_CACHE.data3, 0x4a8e);
    assert_eq!(
        CLSID_LOCAL_THUMBNAIL_CACHE.data4,
        [0xb2, 0x1b, 0x8a, 0x26, 0x18, 0x0d, 0xb1, 0x3f]
    );
}

#[test]
fn query_interface_hr_accepts_only_listed_iids() {
    // Not oracle-derived: the generic check, with an empty list and with two entries.
    assert_eq!(query_interface_hr(&IID_IUNKNOWN, &[]), E_NOINTERFACE);
    assert_eq!(
        query_interface_hr(&IID_IUNKNOWN, &[IID_ICLASSFACTORY, IID_IUNKNOWN]),
        S_OK
    );
    assert_eq!(
        query_interface_hr(&IID_ITHUMBNAILCACHE, &[IID_IUNKNOWN]),
        E_NOINTERFACE
    );
}

#[test]
fn reference_count_wraps_like_interlocked_ops() {
    // Not oracle-derived: InterlockedIncrement and InterlockedDecrement wrap on LONG overflow,
    // and the port keeps that. The oracle never drives the count past 2.
    let mut tc = ThumbCache {
        ref_count: i32::MAX,
    };
    assert_eq!(tc.add_ref(), 0x8000_0000);

    let mut tc = ThumbCache::default();
    assert_eq!(tc.release(), u32::MAX);
}

#[test]
fn create_calls_allocator_once_and_sets_count_to_one() {
    // Not oracle-derived: the allocator runs once on success and once on failure, and a
    // successful create hands back an object whose count is 1.
    let calls = Cell::new(0);
    let alloc = || {
        calls.set(calls.get() + 1);
        Some(ThumbCache::default())
    };

    let (hr, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &alloc);
    assert_eq!(hr, S_OK);
    assert_eq!(obj.map(|tc| tc.ref_count), Some(1));

    let (hr, obj) = thumb_cache_create(&IID_ICLASSFACTORY, &alloc);
    assert_eq!((hr, obj), (E_NOINTERFACE, None));
    assert_eq!(calls.get(), 2);
}

#[test]
fn class_id_must_match_every_field() {
    // Not oracle-derived: a CLSID that differs from the served one only in its last byte is refused.
    let mut near = CLSID_LOCAL_THUMBNAIL_CACHE;
    near.data4[7] ^= 1;
    assert_eq!(
        dll_get_class_object(&near, &IID_IUNKNOWN),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: names and order from dlls/thumbcache/thumbcache.spec. Names are
    // case-sensitive, and names the spec does not list are unknown.
    let names: Vec<&str> = EXPORTS.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer"
        ]
    );
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Ported));
    assert_eq!(
        export_kind("DllRegisterServer"),
        Some(ExportKind::NotInSource)
    );
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind("MergeFontPackage"), None);
}
