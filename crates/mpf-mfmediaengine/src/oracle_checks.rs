//! Oracle-derived test vectors for the ported mfmediaengine logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies under
//! `target/canon/mfmediaengine/` (`mftime.c`, `gcd.c`, `array_reserve.c`, `time_range_types.c`,
//! `time_range_methods.c`, `dll_get_class_object.c`, `classfactory.c`, `factory_qi.c`,
//! `factory_methods.c` and `unimplemented_stub.c`) with `gcc -std=gnu11 -fsanitize=address,undefined -g`,
//! and running `harness.c`. The harness printed 25 rows, and none was excluded because ASan and
//! UBSan reported nothing.
//!
//! The `IMFMediaTimeRange` rows read `hr` as the HRESULT of each call. The Rust `add_range`,
//! `clear` and `length` cannot fail, so they assert no return value.

use super::com::*;
use super::*;

/// `IID_IMFMediaTimeRange` from `include/mfmediaengine.idl`. Only these vectors use it.
const IID_IMF_MEDIA_TIME_RANGE: Guid = Guid {
    data1: 0xdb71_a2fc,
    data2: 0x078a,
    data3: 0x414e,
    data4: [0x9d, 0xf9, 0x8c, 0x25, 0x31, 0xb0, 0xaa, 0x6c],
};

fn range(start: f64, end: f64) -> Range {
    Range { start, end }
}

#[test]
fn oracle_mftime_rows() {
    // row 01 mftime_to_seconds(0) -> 0
    assert_eq!(mftime_to_seconds(0), 0.0);
    // row 02 mftime_to_seconds(1) -> 9.9999999999999995e-08, which is the f64 nearest 1e-7
    assert_eq!(mftime_to_seconds(1), 1e-7);
}

#[test]
fn oracle_gcd_rows() {
    // row 03 get_gcd(12,18) -> 6
    assert_eq!(get_gcd(12, 18), 6);
    // row 04 get_gcd(0,0) -> 0
    assert_eq!(get_gcd(0, 0), 0);
}

#[test]
fn oracle_array_reserve_rows() {
    // row 05 mf_array_reserve(cap=0,count=1,size=16) -> ret=1 cap=4
    assert_eq!(array_reserve_capacity(0, 1, 16), Some(4));
    // row 06 mf_array_reserve(cap=4,count=5,size=16) -> ret=1 cap=8
    assert_eq!(array_reserve_capacity(4, 5, 16), Some(8));
    // row 07 mf_array_reserve(cap=8,count=3,size=16) -> ret=1 cap=8
    assert_eq!(array_reserve_capacity(8, 3, 16), Some(8));
}

#[test]
fn oracle_time_range_rows() {
    // row 08 AddRange(1,2) AddRange(5,6) -> hr=0x0 len=2 ranges=[1,2][5,6]
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(5.0, 6.0);
    assert_eq!(t.length(), 2);
    assert_eq!(t.ranges(), &[range(1.0, 2.0), range(5.0, 6.0)]);

    // row 09 AddRange(1,2) AddRange(1.5,1.7) -> hr=0x0 len=1 ranges=[1,2]
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(1.5, 1.7);
    assert_eq!(t.length(), 1);
    assert_eq!(t.ranges(), &[range(1.0, 2.0)]);

    // row 10 AddRange(1,2) AddRange(0,3) -> hr=0x0 len=1 ranges=[0,3]
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(0.0, 3.0);
    assert_eq!(t.length(), 1);
    assert_eq!(t.ranges(), &[range(0.0, 3.0)]);

    // row 11 AddRange(1,2) AddRange(2,4) -> hr=0x0 len=1 ranges=[1,4]
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(2.0, 4.0);
    assert_eq!(t.length(), 1);
    assert_eq!(t.ranges(), &[range(1.0, 4.0)]);

    // row 12 AddRange(1,2) AddRange(5,10) AddRange(3,7) -> hr=0x0 len=2 ranges=[1,2][3,10]
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.add_range(5.0, 10.0);
    t.add_range(3.0, 7.0);
    assert_eq!(t.length(), 2);
    assert_eq!(t.ranges(), &[range(1.0, 2.0), range(3.0, 10.0)]);

    // row 13 AddRange(1,2) Clear -> hr=0x0 len=0
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    t.clear();
    assert_eq!(t.length(), 0);

    // row 14 AddRange(1,2) GetStart(1) -> hr=0x80070057 start=-1 (the C out-parameter is not written)
    let mut t = TimeRange::default();
    t.add_range(1.0, 2.0);
    assert_eq!(t.start(1), Err(E_INVALIDARG));
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 15 DllGetClassObject(CLSID_MFMediaEngineClassFactory,IID_IClassFactory) -> hr=0x0 obj=set
    assert_eq!(
        dll_get_class_object(&CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY, &IID_ICLASSFACTORY),
        Ok(ClassFactory)
    );

    // row 16 DllGetClassObject(CLSID_MFMediaEngineClassFactory,IID_IMFMediaEngineClassFactory) -> hr=0x80004002 obj=NULL
    assert_eq!(
        dll_get_class_object(
            &CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY,
            &IID_IMF_MEDIA_ENGINE_CLASS_FACTORY
        ),
        Err(E_NOINTERFACE)
    );

    // row 17 DllGetClassObject(other CLSID,IID_IClassFactory) -> hr=0x80040111 obj=NULL
    // The row passes IID_IUnknown as the class id, which is not CLSID_MFMediaEngineClassFactory.
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_class_factory_rows() {
    // row 18 classfactory.CreateInstance(outer=non-NULL,IID_IMFMediaEngineClassFactory) -> hr=0x80040110 obj=NULL
    assert_eq!(
        ClassFactory.create_instance(true, &IID_IMF_MEDIA_ENGINE_CLASS_FACTORY),
        Err(CLASS_E_NOAGGREGATION)
    );

    // row 19 classfactory.CreateInstance(outer=NULL,IID_IMFMediaEngineClassFactory) -> hr=0x0 obj=set
    assert_eq!(
        ClassFactory.create_instance(false, &IID_IMF_MEDIA_ENGINE_CLASS_FACTORY),
        Ok(MediaEngineClassFactory)
    );

    // row 20 factory.QueryInterface(IID_IMFMediaTimeRange) -> hr=0x80004002 obj=NULL
    assert_eq!(
        MediaEngineClassFactory.query_interface(&IID_IMF_MEDIA_TIME_RANGE),
        Err(E_NOINTERFACE)
    );

    // row 21 classfactory.LockServer(TRUE) -> hr=0x0
    assert_eq!(ClassFactory.lock_server(true), S_OK);
}

#[test]
fn oracle_media_engine_factory_rows() {
    // row 22 factory.IsTypeSupported(video/mp4,NULL) -> hr=0x0 is_supported=0
    // The C arguments are never read, so the port takes none.
    assert_eq!(MediaEngineClassFactory.is_type_supported(), Ok(false));

    // row 23 factory.CreateMediaKeys(com.example,NULL) -> hr=0x80004001
    assert_eq!(MediaEngineClassFactory.create_media_keys(), E_NOTIMPL);

    // row 24 factory.CreateMediaSourceExtension(flags=0,attr=NULL) -> hr=0x80004001 mse=NULL
    assert_eq!(
        MediaEngineClassFactory.create_media_source_extension(),
        E_NOTIMPL
    );
}

#[test]
fn oracle_dll_get_activation_factory_row() {
    // row 25 DllGetActivationFactory -> raise code=0x80000100 flags=0x1 nargs=2 p0=mfmediaengine.dll p1=DllGetActivationFactory
    assert_eq!(
        dll_get_activation_factory(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "mfmediaengine.dll",
            function: "DllGetActivationFactory",
        }
    );
}
