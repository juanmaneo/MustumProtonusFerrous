//! Oracle-derived test vectors for the ported thumbcache logic.
//!
//! Rows were captured by compiling the verbatim Wine C body of `dlls/thumbcache/thumbcache.c`
//! (lines 31 to 209, `DllGetClassObject` included) with the Windows shims in
//! `target/canon/thumbcache/shim.h`, under `gcc -fsanitize=address,undefined -g`, and running the
//! harness in `target/canon/thumbcache/harness.c`. All 23 printed rows are clean: ASan and UBSan
//! flagged none, so no row is excluded.
//!
//! The `obj` column of each row maps onto the return value: `Some` for `cf` and `obj`, and `None`
//! for `NULL` and `untouched`. For `cf` results the factory itself is the object, so only the
//! `hr` is asserted.

use super::*;

const SHARED_BITMAP_CLSID: Guid = Guid {
    data1: 0x4db2_6476,
    data2: 0x6787,
    data3: 0x4046,
    data4: [0xb8, 0x36, 0xe8, 0x41, 0x2a, 0x9e, 0x8a, 0x27],
};

const ZERO_GUID: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 DllGetClassObject(Local,IClassFactory) hr=00000000 obj=cf
    assert_eq!(
        dll_get_class_object(&CLSID_LOCAL_THUMBNAIL_CACHE, &IID_ICLASSFACTORY),
        (S_OK, Some(ClassFactory))
    );

    // row 02 DllGetClassObject(Local,IUnknown) hr=00000000 obj=cf
    assert_eq!(
        dll_get_class_object(&CLSID_LOCAL_THUMBNAIL_CACHE, &IID_IUNKNOWN),
        (S_OK, Some(ClassFactory))
    );

    // row 03 DllGetClassObject(Local,IThumbnailCache) hr=80004002 obj=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_LOCAL_THUMBNAIL_CACHE, &IID_ITHUMBNAILCACHE),
        (E_NOINTERFACE, None)
    );

    // row 04 DllGetClassObject(SharedBitmap,IClassFactory) hr=80040111 obj=untouched
    assert_eq!(
        dll_get_class_object(&SHARED_BITMAP_CLSID, &IID_ICLASSFACTORY),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );

    // row 05 DllGetClassObject(zero,IUnknown) hr=80040111 obj=untouched
    assert_eq!(
        dll_get_class_object(&ZERO_GUID, &IID_IUNKNOWN),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );

    // row 06 DllGetClassObject(Local,zero) hr=80004002 obj=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_LOCAL_THUMBNAIL_CACHE, &ZERO_GUID),
        (E_NOINTERFACE, None)
    );
}

#[test]
fn oracle_class_factory_rows() {
    let cf = ClassFactory;

    // row 07 cf.QueryInterface(IUnknown) hr=00000000 obj=cf
    assert_eq!(cf.query_interface(&IID_IUNKNOWN), S_OK);

    // row 08 cf.QueryInterface(IClassFactory) hr=00000000 obj=cf
    assert_eq!(cf.query_interface(&IID_ICLASSFACTORY), S_OK);

    // row 09 cf.QueryInterface(IThumbnailCache) hr=80004002 obj=NULL
    assert_eq!(cf.query_interface(&IID_ITHUMBNAILCACHE), E_NOINTERFACE);

    // row 10 cf.AddRef=2 cf.Release=1
    assert_eq!((cf.add_ref(), cf.release()), (2, 1));

    // row 11 cf.LockServer(1)=00000000 cf.LockServer(0)=00000000
    assert_eq!((cf.lock_server(true), cf.lock_server(false)), (S_OK, S_OK));

    // row 12 cf.CreateInstance(NULL,IThumbnailCache) hr=00000000 obj=obj release=0
    let (hr, obj) = cf.create_instance(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    assert_eq!(hr, S_OK);
    assert_eq!(obj.map(|mut tc| tc.release()), Some(0));

    // row 13 cf.CreateInstance(outer,IUnknown) hr=00000000 obj=obj release=0
    let (hr, obj) = cf.create_instance(&IID_IUNKNOWN, &|| Some(ThumbCache::default()));
    assert_eq!(hr, S_OK);
    assert_eq!(obj.map(|mut tc| tc.release()), Some(0));

    // row 14 cf.CreateInstance(NULL,IClassFactory) hr=80004002 obj=NULL
    assert_eq!(
        cf.create_instance(&IID_ICLASSFACTORY, &|| Some(ThumbCache::default())),
        (E_NOINTERFACE, None)
    );

    // row 15 cf.CreateInstance(alloc fails) hr=8007000e obj=NULL
    assert_eq!(
        cf.create_instance(&IID_ITHUMBNAILCACHE, &|| None),
        (E_OUTOFMEMORY, None)
    );
}

#[test]
fn oracle_thumb_cache_rows() {
    // row 16 thumbcache_create(IThumbnailCache) hr=00000000 obj=obj addref=2 release=1 release=0
    let (hr, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    assert_eq!(hr, S_OK);
    let mut tc = obj.expect("S_OK carries the object");
    assert_eq!((tc.add_ref(), tc.release(), tc.release()), (2, 1, 0));

    // row 17 tc.QueryInterface(IUnknown) hr=00000000 same=1 release=1 release=0
    let (_, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    let mut tc = obj.expect("S_OK carries the object");
    assert_eq!(tc.query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!((tc.release(), tc.release()), (1, 0));

    // row 18 tc.QueryInterface(IClassFactory) hr=80004002 obj=NULL release=0
    let (_, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    let mut tc = obj.expect("S_OK carries the object");
    assert_eq!(tc.query_interface(&IID_ICLASSFACTORY), E_NOINTERFACE);
    assert_eq!(tc.release(), 0);

    // row 19 tc.GetThumbnail hr=80004001 release=0
    let (_, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    let mut tc = obj.expect("S_OK carries the object");
    assert_eq!(tc.get_thumbnail(), E_NOTIMPL);
    assert_eq!(tc.release(), 0);

    // row 20 tc.GetThumbnailByID hr=80004001 release=0
    let (_, obj) = thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| Some(ThumbCache::default()));
    let mut tc = obj.expect("S_OK carries the object");
    assert_eq!(tc.get_thumbnail_by_id(), E_NOTIMPL);
    assert_eq!(tc.release(), 0);

    // row 21 thumbcache_create(IUnknown) hr=00000000 obj=obj release=0
    let (hr, obj) = thumb_cache_create(&IID_IUNKNOWN, &|| Some(ThumbCache::default()));
    assert_eq!(hr, S_OK);
    assert_eq!(obj.map(|mut tc| tc.release()), Some(0));

    // row 22 thumbcache_create(zero) hr=80004002 obj=NULL
    assert_eq!(
        thumb_cache_create(&ZERO_GUID, &|| Some(ThumbCache::default())),
        (E_NOINTERFACE, None)
    );

    // row 23 thumbcache_create(alloc fails) hr=8007000e obj=untouched
    assert_eq!(
        thumb_cache_create(&IID_ITHUMBNAILCACHE, &|| None),
        (E_OUTOFMEMORY, None)
    );
}
