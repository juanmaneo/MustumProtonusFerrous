//! Oracle-derived test vectors for the ported geolocation logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/geolocation/main.c`
//! and `dlls/geolocation/weakref.c`, plus the `libs/winecrt0/dll_canunload.c` default, with
//! minimal Windows and COM shims. The build used `gcc -fsanitize=address,undefined -g` under
//! `target/canon/geolocation/`. All 24 printed rows (R01-R24) ran clean: ASan and UBSan flagged
//! none, so no vectors are excluded.
//!
//! Each row starts from a fresh `Heap` and `Factory`, so a new geolocator is always index 0.
//! Stub members take no output parameters, so the harness's out values stay untouched by
//! construction.

use super::*;

const OK: Hresult = 0x0000_0000;
const E_NOTIMPL_ROW: Hresult = 0x8000_4001;

fn alloc_ok() -> Alloc {
    Alloc {
        object: true,
        record: true,
    }
}

/// Activates a geolocator on a fresh heap, which always lands in slot 0.
fn activate(heap: &mut Heap) {
    assert_eq!(
        factory_activate_instance(heap, alloc_ok()),
        (OK, Some(Iface::Geolocator(0)))
    );
}

/// The harness's `weak_of_instance`: QueryInterface(IWeakReferenceSource), GetWeakReference,
/// then release the source. Afterwards the object has one strong and two weak references.
fn take_weak(heap: &mut Heap) {
    assert_eq!(
        geolocator_query_interface(heap, 0, Iid::IWeakReferenceSource),
        (OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(
        weak_reference_source_get_weak_reference(heap, 0),
        (OK, Some(Iface::WeakReference(0)))
    );
    weak_reference_source_release(heap, 0);
}

#[test]
fn oracle_dll_exports() {
    // R01 DllCanUnloadNow hr=00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
    // R02 DllGetClassObject hr=80040111 out=0
    assert_eq!(dll_get_class_object(), 0x8004_0111);

    // R03-R06 share one static factory, whose count returns to 1 after each row.
    let mut factory = Factory::default();
    // R03 DllGetActivationFactory(Geolocator) hr=00000000 factory=1 ref=2 released=1
    assert_eq!(
        dll_get_activation_factory(&mut factory, Some("Windows.Devices.Geolocation.Geolocator")),
        (OK, Some(Iface::ActivationFactory))
    );
    assert_eq!(factory.ref_count, 2);
    assert_eq!(factory_release(&mut factory), 1);
    // R04 DllGetActivationFactory(Geocoder) hr=80040111 factory=0 ref=1
    assert_eq!(
        dll_get_activation_factory(&mut factory, Some("Windows.Devices.Geolocation.Geocoder")),
        (0x8004_0111, None)
    );
    assert_eq!(factory.ref_count, 1);
    // R05 DllGetActivationFactory(NULL) hr=80040111 factory=0 ref=1
    assert_eq!(
        dll_get_activation_factory(&mut factory, None),
        (0x8004_0111, None)
    );
    assert_eq!(factory.ref_count, 1);
    // R06 DllGetActivationFactory(lowercase) hr=80040111 factory=0 ref=1
    assert_eq!(
        dll_get_activation_factory(&mut factory, Some("windows.devices.geolocation.geolocator")),
        (0x8004_0111, None)
    );
    assert_eq!(factory.ref_count, 1);
}

#[test]
fn oracle_factory_and_activation() {
    let mut factory = Factory::default();
    // R07 factory QI(IUnknown) hr=00000000 out=1 ref=2 released=1
    assert_eq!(
        factory_query_interface(&mut factory, Iid::IUnknown),
        (OK, Some(Iface::ActivationFactory))
    );
    assert_eq!(factory.ref_count, 2);
    assert_eq!(factory_release(&mut factory), 1);
    // R08 factory QI(IGeolocator) hr=80004002 out=0 ref=1
    assert_eq!(
        factory_query_interface(&mut factory, Iid::IGeolocator),
        (0x8000_4002, None)
    );
    assert_eq!(factory.ref_count, 1);

    // R09 factory ActivateInstance hr=00000000 inst=1 strong=1 weak=1 released=0
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(heap.instance(0).ref_strong, 1);
    assert_eq!(heap.instance(0).ref_weak, 1);
    assert_eq!(geolocator_release(&mut heap, 0), 0);

    // R10 ActivateInstance(object alloc fails) hr=8007000e inst=0 allocs=1
    let mut heap = Heap::default();
    let object_fails = Alloc {
        object: false,
        record: true,
    };
    assert_eq!(
        factory_activate_instance(&mut heap, object_fails),
        (0x8007_000e, None)
    );
    assert_eq!(heap.calloc_calls(), 1);

    // R11 ActivateInstance(record alloc fails) hr=8007000e inst=0 allocs=2
    let mut heap = Heap::default();
    let record_fails = Alloc {
        object: true,
        record: false,
    };
    assert_eq!(
        factory_activate_instance(&mut heap, record_fails),
        (0x8007_000e, None)
    );
    assert_eq!(heap.calloc_calls(), 2);
    // The object is freed when the record fails. The oracle's LeakSanitizer run reported nothing.
    assert_eq!(heap.live_allocations(), 0);
}

#[test]
fn oracle_geolocator_query_interface() {
    // R12 geolocator QI chain: each pointer is the same object, strong 2..5, then 1 after the
    // four releases.
    let mut heap = Heap::default();
    activate(&mut heap);
    for (iid, strong) in [
        (Iid::IUnknown, 2),
        (Iid::IInspectable, 3),
        (Iid::IAgileObject, 4),
        (Iid::IGeolocator, 5),
    ] {
        assert_eq!(
            geolocator_query_interface(&mut heap, 0, iid),
            (OK, Some(Iface::Geolocator(0)))
        );
        assert_eq!(heap.instance(0).ref_strong, strong);
    }
    for _ in 0..4 {
        geolocator_release(&mut heap, 0);
    }
    assert_eq!(heap.instance(0).ref_strong, 1);

    // R13 geolocator QI(IWeakReferenceSource) hr=00000000 src=1 strong=2 released=1
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReferenceSource),
        (OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(heap.instance(0).ref_strong, 2);
    assert_eq!(weak_reference_source_release(&mut heap, 0), 1);

    // R14 geolocator QI(IWeakReference) hr=80004002 out=0; QI(unrelated) hr=80004002 out=0 strong=1
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReference),
        (0x8000_4002, None)
    );
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::Other),
        (0x8000_4002, None)
    );
    assert_eq!(heap.instance(0).ref_strong, 1);
}

#[test]
fn oracle_weak_reference() {
    // R15 GetWeakReference hr=00000000 record=1 weak=2 released_weak=1 released_src=1
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReferenceSource),
        (OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(
        weak_reference_source_get_weak_reference(&mut heap, 0),
        (OK, Some(Iface::WeakReference(0)))
    );
    assert_eq!(heap.instance(0).ref_weak, 2);
    assert_eq!(weak_reference_release(&mut heap, 0), 1);
    assert_eq!(weak_reference_source_release(&mut heap, 0), 1);
    // Harness teardown: releasing the last strong reference frees the object and the record.
    assert_eq!(geolocator_release(&mut heap, 0), 0);
    assert_eq!(heap.live_allocations(), 0);

    // R16 weak QI weak=2 | IUnknown hr=00000000 same=1 weak=3 rel=2 | IWeakReference
    // hr=00000000 same=1 weak=3 rel=2 | IGeolocator hr=80004002 out=0 weak=2
    let mut heap = Heap::default();
    activate(&mut heap);
    take_weak(&mut heap);
    assert_eq!(heap.instance(0).ref_weak, 2);
    assert_eq!(
        weak_reference_query_interface(&mut heap, 0, Iid::IUnknown),
        (OK, Some(Iface::WeakReference(0)))
    );
    assert_eq!(heap.instance(0).ref_weak, 3);
    assert_eq!(weak_reference_release(&mut heap, 0), 2);
    assert_eq!(
        weak_reference_query_interface(&mut heap, 0, Iid::IWeakReference),
        (OK, Some(Iface::WeakReference(0)))
    );
    assert_eq!(heap.instance(0).ref_weak, 3);
    assert_eq!(weak_reference_release(&mut heap, 0), 2);
    assert_eq!(
        weak_reference_query_interface(&mut heap, 0, Iid::IGeolocator),
        (0x8000_4002, None)
    );
    assert_eq!(heap.instance(0).ref_weak, 2);
    // Harness teardown.
    weak_reference_release(&mut heap, 0);
    assert_eq!(geolocator_release(&mut heap, 0), 0);
    assert_eq!(heap.live_allocations(), 0);

    // R17 weak Resolve(IGeolocator) hr=00000000 same=1 strong=2 released=1
    let mut heap = Heap::default();
    activate(&mut heap);
    take_weak(&mut heap);
    assert_eq!(
        weak_reference_resolve(&mut heap, 0, Iid::IGeolocator),
        (OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(heap.instance(0).ref_strong, 2);
    assert_eq!(geolocator_release(&mut heap, 0), 1);

    // R18 weak Resolve(IWeakReference) hr=80004002 out=0 strong=1
    let mut heap = Heap::default();
    activate(&mut heap);
    take_weak(&mut heap);
    assert_eq!(
        weak_reference_resolve(&mut heap, 0, Iid::IWeakReference),
        (0x8000_4002, None)
    );
    assert_eq!(heap.instance(0).ref_strong, 1);
}

#[test]
fn oracle_release_and_source() {
    // R19 release last strong with external weak ret=0 weak=1 strong=0
    // resolve hr=00000000 out=0 weak_release=0
    let mut heap = Heap::default();
    activate(&mut heap);
    take_weak(&mut heap);
    assert_eq!(geolocator_release(&mut heap, 0), 0);
    assert_eq!(heap.instance(0).ref_weak, 1);
    assert_eq!(heap.instance(0).ref_strong, 0);
    assert_eq!(
        weak_reference_resolve(&mut heap, 0, Iid::IGeolocator),
        (OK, None)
    );
    assert_eq!(weak_reference_release(&mut heap, 0), 0);

    // R20 release last strong no external weak ret=0 weak_before=1
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(heap.instance(0).ref_weak, 1);
    assert_eq!(geolocator_release(&mut heap, 0), 0);

    // R21 source QI(IGeolocator) hr=00000000 same=1 strong=3 addref=4 release=3 out_release=2
    // src_release=1
    let mut heap = Heap::default();
    activate(&mut heap);
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReferenceSource),
        (OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(
        weak_reference_source_query_interface(&mut heap, 0, Iid::IGeolocator),
        (OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(heap.instance(0).ref_strong, 3);
    assert_eq!(weak_reference_source_add_ref(&mut heap, 0), 4);
    assert_eq!(weak_reference_source_release(&mut heap, 0), 3);
    assert_eq!(geolocator_release(&mut heap, 0), 2);
    assert_eq!(weak_reference_source_release(&mut heap, 0), 1);
    assert_eq!(geolocator_release(&mut heap, 0), 0);
}

#[test]
fn oracle_stub_members() {
    // R22 IInspectable stubs: geolocator hr=80004001 x3, factory hr=80004001 x3. Both objects
    // share these members, and the harness's count and trust values stay untouched.
    assert_eq!(stub_hresult(Member::GetIids), E_NOTIMPL_ROW);
    assert_eq!(stub_hresult(Member::GetRuntimeClassName), E_NOTIMPL_ROW);
    assert_eq!(stub_hresult(Member::GetTrustLevel), E_NOTIMPL_ROW);

    // R23 IGeolocator property stubs hr=80004001 x7; acc=9 dv=2.5 u=77 ps=3 stay untouched.
    for member in [
        Member::GetDesiredAccuracy,
        Member::SetDesiredAccuracy,
        Member::GetMovementThreshold,
        Member::SetMovementThreshold,
        Member::GetReportInterval,
        Member::SetReportInterval,
        Member::LocationStatus,
    ] {
        assert_eq!(stub_hresult(member), E_NOTIMPL_ROW);
    }

    // R24 IGeolocator async/event stubs: E_NOTIMPL except add_PositionChanged, which returns
    // S_OK. The harness's op=0 and tok=85 stay untouched.
    assert_eq!(stub_hresult(Member::GetGeopositionAsync), E_NOTIMPL_ROW);
    assert_eq!(
        stub_hresult(Member::GetGeopositionAsyncWithAgeAndTimeout),
        E_NOTIMPL_ROW
    );
    assert_eq!(stub_hresult(Member::AddPositionChanged), 0x0000_0000);
    assert_eq!(stub_hresult(Member::RemovePositionChanged), E_NOTIMPL_ROW);
    assert_eq!(stub_hresult(Member::AddStatusChanged), E_NOTIMPL_ROW);
    assert_eq!(stub_hresult(Member::RemoveStatusChanged), E_NOTIMPL_ROW);
}
