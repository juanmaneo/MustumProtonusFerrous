//! Oracle-derived test vectors for the ported qedit logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies in `dlls/qedit/main.c` (lines
//! 39 to 274) and the `DllCanUnloadNow` default in `libs/winecrt0/dll_canunload.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/qedit/`. The harness added minimal
//! COM type shims (`shim.h`) and recorded the calls to `CoCreateInstance`, `CoTaskMemAlloc` and
//! the `IFilterMapper2` methods. The 30 printed rows are A1 to A9, B1 to B7, C1 to C6,
//! DllCanUnloadNow, E1 to E4 and F1 to F3. ASan and UBSan flagged none of them, so none were
//! excluded.

use super::*;
use crate::com::failed;
use std::cell::{Cell, RefCell};

pub(super) const IID_IFILTERMAPPER2: Guid = Guid {
    data1: 0xb79b_b0b0,
    data2: 0x33c1,
    data3: 0x11d1,
    data4: [0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75],
};

#[derive(Debug, PartialEq)]
pub(super) enum Call {
    Register(Guid, String, RegFilter2),
    Unregister(Guid),
}

/// The `IFilterMapper2` that a `DllRegisterServer` run uses. Dropping it counts as the
/// `IFilterMapper2_Release` call in the C code.
pub(super) struct Recorder<'a> {
    calls: &'a RefCell<Vec<Call>>,
    released: &'a Cell<u32>,
}

impl FilterMapper for Recorder<'_> {
    fn register_filter(&mut self, clsid: &Guid, name: &str, filter: &RegFilter2) {
        self.calls
            .borrow_mut()
            .push(Call::Register(*clsid, name.to_string(), *filter));
    }

    fn unregister_filter(&mut self, clsid: &Guid) {
        self.calls.borrow_mut().push(Call::Unregister(*clsid));
    }
}

impl Drop for Recorder<'_> {
    fn drop(&mut self) {
        self.released.set(self.released.get() + 1);
    }
}

/// The two `RegisterFilter` calls of `DllRegisterServer`, in C order.
pub(super) fn registered_calls() -> Vec<Call> {
    vec![
        Call::Register(
            CLSID_SAMPLEGRABBER,
            "SampleGrabber".to_string(),
            SAMPLE_GRABBER_FILTER,
        ),
        Call::Register(
            CLSID_NULLRENDERER,
            "Null Renderer".to_string(),
            NULL_RENDERER_FILTER,
        ),
    ]
}

/// Runs `DllRegisterServer` (`register`) or `DllUnregisterServer` with the given resources
/// HRESULT and `CoCreateInstance` HRESULT. Returns the HRESULT, the recorded calls, the number of
/// mapper releases and the number of `CoCreateInstance` calls.
pub(super) fn run_registration(
    register: bool,
    resources: Hresult,
    cocreate: Hresult,
) -> (Hresult, Vec<Call>, u32, u32) {
    let calls = RefCell::new(Vec::new());
    let released = Cell::new(0);
    let created = Cell::new(0);
    let create_mapper = || {
        created.set(created.get() + 1);
        if failed(cocreate) {
            return Err(cocreate);
        }
        Ok(Recorder {
            calls: &calls,
            released: &released,
        })
    };
    let hr = if register {
        dll_register_server(resources, create_mapper)
    } else {
        dll_unregister_server(resources, create_mapper)
    };
    (hr, calls.into_inner(), released.get(), created.get())
}

/// Runs `CreateInstance` on a fresh factory for `clsid`. The class's create step returns
/// `create_hr`, and the `QueryInterface` on the new object returns `qi_hr`. Returns the HRESULT,
/// the class whose create step ran, and the number of releases of the new object.
fn create(
    clsid: &Guid,
    has_outer: bool,
    iid: &Guid,
    create_hr: Hresult,
    qi_hr: Hresult,
) -> (Hresult, Option<ClassId>, u32) {
    let factory = dll_get_class_object(clsid, &IID_ICLASSFACTORY, true).unwrap();
    let created = Cell::new(None);
    let punk_releases = Cell::new(0);
    let hr = factory.create_instance(has_outer, iid, |class| {
        created.set(Some(class));
        if failed(create_hr) {
            return create_hr;
        }
        punk_releases.set(punk_releases.get() + 1);
        qi_hr
    });
    (hr, created.get(), punk_releases.get())
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // A1 DllGetClassObject(AMTimeline, IClassFactory, alloc=ok) -> S_OK, ref=1, create=timeline
    assert_eq!(
        dll_get_class_object(&CLSID_AMTIMELINE, &IID_ICLASSFACTORY, true),
        Ok(ClassFactory::new(ClassId::AmTimeline))
    );
    // A2 DllGetClassObject(MediaDet, IUnknown, alloc=ok) -> S_OK, ref=1, create=mediadet
    assert_eq!(
        dll_get_class_object(&CLSID_MEDIADET, &IID_IUNKNOWN, true),
        Ok(ClassFactory::new(ClassId::MediaDet))
    );
    // A3 DllGetClassObject(NullRenderer, IClassFactory, alloc=ok) -> S_OK, create=nullrenderer
    assert_eq!(
        dll_get_class_object(&CLSID_NULLRENDERER, &IID_ICLASSFACTORY, true),
        Ok(ClassFactory::new(ClassId::NullRenderer))
    );
    // A4 DllGetClassObject(SampleGrabber, IClassFactory, alloc=ok) -> S_OK, create=samplegrabber
    assert_eq!(
        dll_get_class_object(&CLSID_SAMPLEGRABBER, &IID_ICLASSFACTORY, true),
        Ok(ClassFactory::new(ClassId::SampleGrabber))
    );
    // A5 DllGetClassObject(SampleGrabber, IFilterMapper2, alloc=ok) -> E_NOINTERFACE, obj untouched
    assert_eq!(
        dll_get_class_object(&CLSID_SAMPLEGRABBER, &IID_IFILTERMAPPER2, true),
        Err(E_NOINTERFACE)
    );
    // A6 DllGetClassObject(unknown, IClassFactory, alloc=ok) -> CLASS_E_CLASSNOTAVAILABLE
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &IID_ICLASSFACTORY, true),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    // A7 DllGetClassObject(unknown, IFilterMapper2, alloc=ok) -> E_NOINTERFACE (IID is checked first)
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &IID_IFILTERMAPPER2, true),
        Err(E_NOINTERFACE)
    );
    // A8 DllGetClassObject(SampleGrabber, IClassFactory, alloc=fail) -> E_OUTOFMEMORY, allocs=0
    assert_eq!(
        dll_get_class_object(&CLSID_SAMPLEGRABBER, &IID_ICLASSFACTORY, false),
        Err(E_OUTOFMEMORY)
    );
    // A9 DllGetClassObject(unknown, IClassFactory, alloc=fail) -> CLASS_E_CLASSNOTAVAILABLE
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &IID_ICLASSFACTORY, false),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_factory_method_rows() {
    let fresh = || dll_get_class_object(&CLSID_SAMPLEGRABBER, &IID_ICLASSFACTORY, true).unwrap();

    // B1 QueryInterface(IUnknown) -> S_OK, ppobj=factory, ref=2
    let mut f = fresh();
    assert_eq!(f.query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!(f.ref_count(), 2);
    // B2 QueryInterface(IClassFactory) -> S_OK, ppobj=factory, ref=2
    let mut f = fresh();
    assert_eq!(f.query_interface(&IID_ICLASSFACTORY), S_OK);
    assert_eq!(f.ref_count(), 2);
    // B3 QueryInterface(IFilterMapper2) -> E_NOINTERFACE, ppobj=NULL, ref=1
    let mut f = fresh();
    assert_eq!(f.query_interface(&IID_IFILTERMAPPER2), E_NOINTERFACE);
    assert_eq!(f.ref_count(), 1);
    // B4 AddRef -> ret=2
    let mut f = fresh();
    assert_eq!(f.add_ref(), 2);
    // B5 Release from 1 -> ret=0, and the C code frees the object
    let mut f = fresh();
    assert_eq!(f.release(), 0);
    // B6 LockServer(TRUE) -> S_OK
    assert_eq!(fresh().lock_server(true), S_OK);
    // B7 LockServer(FALSE) -> S_OK
    assert_eq!(fresh().lock_server(false), S_OK);
}

#[test]
fn oracle_create_instance_rows() {
    // C1 CreateInstance(AMTimeline, outer=0, IClassFactory), create=S_OK, qi=S_OK -> S_OK,
    // created=AMTimeline, punk_releases=1
    assert_eq!(
        create(&CLSID_AMTIMELINE, false, &IID_ICLASSFACTORY, S_OK, S_OK),
        (S_OK, Some(ClassId::AmTimeline), 1)
    );
    // C2 CreateInstance(MediaDet, outer=1, IUnknown), create=S_OK, qi=S_OK -> S_OK,
    // created=MediaDet, punk_releases=1
    assert_eq!(
        create(&CLSID_MEDIADET, true, &IID_IUNKNOWN, S_OK, S_OK),
        (S_OK, Some(ClassId::MediaDet), 1)
    );
    // C3 CreateInstance(NullRenderer, outer=1, IClassFactory) -> E_NOINTERFACE, created=none
    assert_eq!(
        create(&CLSID_NULLRENDERER, true, &IID_ICLASSFACTORY, S_OK, S_OK),
        (E_NOINTERFACE, None, 0)
    );
    // C4 CreateInstance(SampleGrabber, outer=0, IClassFactory), create=E_OUTOFMEMORY -> E_OUTOFMEMORY,
    // created=SampleGrabber, punk_releases=0
    assert_eq!(
        create(
            &CLSID_SAMPLEGRABBER,
            false,
            &IID_ICLASSFACTORY,
            E_OUTOFMEMORY,
            S_OK
        ),
        (E_OUTOFMEMORY, Some(ClassId::SampleGrabber), 0)
    );
    // C5 CreateInstance(NullRenderer, outer=0, IUnknown), qi=E_NOINTERFACE -> E_NOINTERFACE,
    // created=NullRenderer, punk_releases=1
    assert_eq!(
        create(
            &CLSID_NULLRENDERER,
            false,
            &IID_IUNKNOWN,
            S_OK,
            E_NOINTERFACE
        ),
        (E_NOINTERFACE, Some(ClassId::NullRenderer), 1)
    );
    // C6 CreateInstance(SampleGrabber, outer=0, IUnknown) -> S_OK, created=SampleGrabber,
    // punk_releases=1
    assert_eq!(
        create(&CLSID_SAMPLEGRABBER, false, &IID_IUNKNOWN, S_OK, S_OK),
        (S_OK, Some(ClassId::SampleGrabber), 1)
    );
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // DllCanUnloadNow -> S_FALSE
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn oracle_registration_rows() {
    // E1 DllRegisterServer, __wine_register_resources=E_FAIL -> E_FAIL, CoCreateInstance not called
    assert_eq!(run_registration(true, E_FAIL, S_OK), (E_FAIL, vec![], 0, 0));
    // E2 DllRegisterServer, CoCreateInstance=REGDB_E_CLASSNOTREG -> REGDB_E_CLASSNOTREG,
    // cocreate_calls=1, no filter calls
    assert_eq!(
        run_registration(true, S_OK, REGDB_E_CLASSNOTREG),
        (REGDB_E_CLASSNOTREG, vec![], 0, 1)
    );
    // E3 DllRegisterServer, CoCreateInstance=S_FALSE -> S_OK, both filters registered,
    // mapper_releases=1
    assert_eq!(
        run_registration(true, S_OK, S_FALSE),
        (S_OK, registered_calls(), 1, 1)
    );
    // E4 DllRegisterServer, all S_OK -> S_OK, both filters registered, mapper_releases=1
    assert_eq!(
        run_registration(true, S_OK, S_OK),
        (S_OK, registered_calls(), 1, 1)
    );
    // F1 DllUnregisterServer, __wine_unregister_resources=E_FAIL -> E_FAIL, cocreate_calls=0
    assert_eq!(
        run_registration(false, E_FAIL, S_OK),
        (E_FAIL, vec![], 0, 0)
    );
    // F2 DllUnregisterServer, CoCreateInstance=REGDB_E_CLASSNOTREG -> REGDB_E_CLASSNOTREG,
    // cocreate_calls=1, mapper_releases=0
    assert_eq!(
        run_registration(false, S_OK, REGDB_E_CLASSNOTREG),
        (REGDB_E_CLASSNOTREG, vec![], 0, 1)
    );
    // F3 DllUnregisterServer, all S_OK -> S_OK, both filters unregistered, mapper_releases=1
    assert_eq!(
        run_registration(false, S_OK, S_OK),
        (
            S_OK,
            vec![
                Call::Unregister(CLSID_SAMPLEGRABBER),
                Call::Unregister(CLSID_NULLRENDERER)
            ],
            1,
            1
        )
    );
}
