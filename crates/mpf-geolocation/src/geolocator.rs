//! Port of the `IGeolocator` object in `main.c`: QueryInterface and reference counting. Its
//! members are in `stub.rs`.

use crate::heap::{Heap, Iface};
use crate::hresult::{Hresult, E_NOINTERFACE, S_OK};
use crate::iid::{inspectable_base, Iid};
use crate::weakref::{weak_reference_strong_add_ref, weak_reference_strong_release};

/// `geolocator_QueryInterface`. Every pointer it returns is AddRef'd through the object's strong count.
pub fn geolocator_query_interface(
    heap: &mut Heap,
    idx: usize,
    iid: Iid,
) -> (Hresult, Option<Iface>) {
    let iface = if inspectable_base(iid) || iid == Iid::IGeolocator {
        Iface::Geolocator(idx)
    } else if iid == Iid::IWeakReferenceSource {
        Iface::WeakReferenceSource(idx)
    } else {
        return (E_NOINTERFACE, None);
    };
    geolocator_add_ref(heap, idx);
    (S_OK, Some(iface))
}

/// `geolocator_AddRef`.
pub fn geolocator_add_ref(heap: &mut Heap, idx: usize) -> u32 {
    weak_reference_strong_add_ref(heap, idx)
}

/// `geolocator_Release`. The object is freed when its strong count reaches zero.
pub fn geolocator_release(heap: &mut Heap, idx: usize) -> u32 {
    let ref_count = weak_reference_strong_release(heap, idx);
    if ref_count == 0 {
        heap.free_object(idx);
    }
    ref_count
}
