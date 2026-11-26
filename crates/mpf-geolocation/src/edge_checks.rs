//! Boundary and branch tests for the ported geolocation logic.
//!
//! These cover cases the 24 oracle rows leave out. A test whose expectations are not
//! oracle-derived says so in a comment.

use super::*;

const ALLOC_OK: Alloc = Alloc {
    object: true,
    record: true,
};

#[test]
fn failed_boundaries() {
    // Not oracle-derived: FAILED(hr) tests the sign bit, so the boundary sits at 0x8000_0000.
    assert!(!failed(0x7fff_ffff));
    assert!(failed(0x8000_0000));
    assert!(!failed(S_OK));
    assert!(!failed(S_FALSE));
    assert!(failed(E_NOTIMPL));
}

#[test]
fn inspectable_base_accepts_three_iids() {
    // Not oracle-derived: the accepted set is read from the `inspectable_base` definition.
    assert!(inspectable_base(Iid::IUnknown));
    assert!(inspectable_base(Iid::IInspectable));
    assert!(inspectable_base(Iid::IAgileObject));
    assert!(!inspectable_base(Iid::IGeolocator));
    assert!(!inspectable_base(Iid::IWeakReference));
    assert!(!inspectable_base(Iid::IWeakReferenceSource));
    assert!(!inspectable_base(Iid::IActivationFactory));
    assert!(!inspectable_base(Iid::Other));
}

#[test]
fn factory_query_interface_accepts_base_and_rejects_others() {
    // Not oracle-derived: these IIDs are read from the `factory_QueryInterface` body.
    let mut factory = Factory::default();
    assert_eq!(
        factory_query_interface(&mut factory, Iid::IInspectable),
        (S_OK, Some(Iface::ActivationFactory))
    );
    assert_eq!(factory.ref_count, 2);
    assert_eq!(
        factory_query_interface(&mut factory, Iid::IAgileObject),
        (S_OK, Some(Iface::ActivationFactory))
    );
    assert_eq!(factory.ref_count, 3);
    assert_eq!(
        factory_query_interface(&mut factory, Iid::IWeakReferenceSource),
        (E_NOINTERFACE, None)
    );
    assert_eq!(factory.ref_count, 3);
    assert_eq!(
        factory_query_interface(&mut factory, Iid::Other),
        (E_NOINTERFACE, None)
    );
    assert_eq!(factory.ref_count, 3);
}

#[test]
fn weak_reference_source_forwards_to_object() {
    // Not oracle-derived: the forwarding is read from the `weakref.c` bodies.
    let mut heap = Heap::default();
    assert_eq!(
        factory_activate_instance(&mut heap, ALLOC_OK),
        (S_OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReferenceSource),
        (S_OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(heap.instance(0).ref_strong, 2);
    assert_eq!(
        weak_reference_source_query_interface(&mut heap, 0, Iid::IUnknown),
        (S_OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(heap.instance(0).ref_strong, 3);
    assert_eq!(
        weak_reference_source_query_interface(&mut heap, 0, Iid::Other),
        (E_NOINTERFACE, None)
    );
    assert_eq!(heap.instance(0).ref_strong, 3);
    assert_eq!(weak_reference_source_add_ref(&mut heap, 0), 4);
    assert_eq!(weak_reference_source_release(&mut heap, 0), 3);
    assert_eq!(heap.instance(0).ref_weak, 1);
}

#[test]
fn allocations_outlive_the_object_until_last_weak_release() {
    // Not oracle-derived as a whole: the counts follow from the `weakref.c` bodies. Some of the
    // individual steps also appear in the oracle rows.
    let mut heap = Heap::default();
    assert_eq!(
        factory_activate_instance(&mut heap, ALLOC_OK),
        (S_OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(
        geolocator_query_interface(&mut heap, 0, Iid::IWeakReferenceSource),
        (S_OK, Some(Iface::WeakReferenceSource(0)))
    );
    assert_eq!(
        weak_reference_source_get_weak_reference(&mut heap, 0),
        (S_OK, Some(Iface::WeakReference(0)))
    );
    assert_eq!(weak_reference_source_release(&mut heap, 0), 1);
    assert_eq!(geolocator_add_ref(&mut heap, 0), 2);
    assert_eq!(geolocator_release(&mut heap, 0), 1);
    assert!(heap.instance(0).object_alive);
    assert!(heap.instance(0).record_alive);
    assert_eq!(weak_reference_add_ref(&mut heap, 0), 3);
    assert_eq!(weak_reference_release(&mut heap, 0), 2);
    assert_eq!(weak_reference_release(&mut heap, 0), 1);
    assert_eq!(heap.live_allocations(), 2);
    assert_eq!(geolocator_release(&mut heap, 0), 0);
    assert_eq!(
        *heap.instance(0),
        Instance {
            ref_strong: 0,
            ref_weak: 0,
            object_alive: false,
            record_alive: false,
        }
    );
    assert_eq!(heap.live_allocations(), 0);
}

#[test]
fn second_activation_takes_slot_one() {
    // Not oracle-derived: every oracle row starts from a fresh heap, so only slot 0 is seen.
    let mut heap = Heap::default();
    assert_eq!(
        factory_activate_instance(&mut heap, ALLOC_OK),
        (S_OK, Some(Iface::Geolocator(0)))
    );
    assert_eq!(
        factory_activate_instance(&mut heap, ALLOC_OK),
        (S_OK, Some(Iface::Geolocator(1)))
    );
    assert_eq!(heap.calloc_calls(), 4);
    assert_eq!(heap.live_allocations(), 4);
    assert_eq!(geolocator_release(&mut heap, 1), 0);
    assert_eq!(heap.live_allocations(), 2);
    assert_eq!(
        *heap.instance(0),
        Instance {
            ref_strong: 1,
            ref_weak: 1,
            object_alive: true,
            record_alive: true,
        }
    );
}

#[test]
fn activation_factory_class_name_stops_at_first_nul() {
    // Not oracle-derived: `wcscmp` stops at the first NUL, so bytes after it are ignored.
    // Expectations are read from the `DllGetActivationFactory` body.
    let mut factory = Factory::default();
    assert_eq!(
        dll_get_activation_factory(
            &mut factory,
            Some("Windows.Devices.Geolocation.Geolocator\0trailing")
        ),
        (S_OK, Some(Iface::ActivationFactory))
    );
    assert_eq!(factory.ref_count, 2);
}

#[test]
fn activation_factory_rejects_names_truncated_by_nul() {
    // Not oracle-derived: an empty prefix or a truncated name matches nothing, so the factory's
    // reference count stays put.
    let mut factory = Factory::default();
    assert_eq!(
        dll_get_activation_factory(&mut factory, Some("")),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );
    assert_eq!(
        dll_get_activation_factory(
            &mut factory,
            Some("\0Windows.Devices.Geolocation.Geolocator")
        ),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );
    assert_eq!(
        dll_get_activation_factory(
            &mut factory,
            Some("Windows.Devices.Geolocation\0.Geolocator")
        ),
        (CLASS_E_CLASSNOTAVAILABLE, None)
    );
    assert_eq!(factory.ref_count, 1);
}
