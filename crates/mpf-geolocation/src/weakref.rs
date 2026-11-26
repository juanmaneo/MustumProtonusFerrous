//! Port of `weakref.c`: the weak reference control block shared by a geolocator and its weak
//! references.
//!
//! The control block is a separate allocation, so the object can be freed while weak references
//! survive. `ref_strong` counts the object's references. `ref_weak` counts the weak references
//! plus one owned by the source.

use crate::geolocator::{geolocator_add_ref, geolocator_query_interface, geolocator_release};
use crate::heap::{Heap, Iface};
use crate::hresult::{Hresult, E_NOINTERFACE, E_OUTOFMEMORY, S_OK};
use crate::iid::Iid;

/// `weak_reference_source_init`, via `weak_reference_create`: one strong and one weak reference.
pub fn weak_reference_source_init(heap: &mut Heap, idx: usize, alloc_ok: bool) -> Hresult {
    if !heap.calloc(alloc_ok) {
        return E_OUTOFMEMORY;
    }
    let inst = heap.instance_mut(idx);
    inst.record_alive = true;
    inst.ref_strong = 1;
    inst.ref_weak = 1;
    S_OK
}

/// `weak_reference_strong_add_ref`.
pub fn weak_reference_strong_add_ref(heap: &mut Heap, idx: usize) -> u32 {
    let inst = heap.instance_mut(idx);
    inst.ref_strong += 1;
    inst.ref_strong
}

/// `weak_reference_strong_release`. Dropping the last strong reference also releases the weak
/// reference that the source owns.
pub fn weak_reference_strong_release(heap: &mut Heap, idx: usize) -> u32 {
    let inst = heap.instance_mut(idx);
    inst.ref_strong -= 1;
    let ref_count = inst.ref_strong;
    if ref_count == 0 {
        weak_reference_release(heap, idx);
    }
    ref_count
}

/// `weak_reference_QueryInterface`: accepts IUnknown and IWeakReference.
pub fn weak_reference_query_interface(
    heap: &mut Heap,
    idx: usize,
    iid: Iid,
) -> (Hresult, Option<Iface>) {
    if !matches!(iid, Iid::IUnknown | Iid::IWeakReference) {
        return (E_NOINTERFACE, None);
    }
    weak_reference_add_ref(heap, idx);
    (S_OK, Some(Iface::WeakReference(idx)))
}

/// `weak_reference_AddRef`.
pub fn weak_reference_add_ref(heap: &mut Heap, idx: usize) -> u32 {
    let inst = heap.instance_mut(idx);
    inst.ref_weak += 1;
    inst.ref_weak
}

/// `weak_reference_Release`. The control block is freed with its last weak reference.
pub fn weak_reference_release(heap: &mut Heap, idx: usize) -> u32 {
    let inst = heap.instance_mut(idx);
    inst.ref_weak -= 1;
    let ref_count = inst.ref_weak;
    if ref_count == 0 {
        inst.record_alive = false;
    }
    ref_count
}

/// `weak_reference_Resolve`. Once the strong count is zero the object is gone, so the result is
/// `S_OK` with no pointer. Otherwise the strong count is held across the QueryInterface on the
/// object. The C compare-exchange loop succeeds on its first attempt in this single-threaded model.
pub fn weak_reference_resolve(heap: &mut Heap, idx: usize, iid: Iid) -> (Hresult, Option<Iface>) {
    let inst = heap.instance_mut(idx);
    if inst.ref_strong == 0 {
        return (S_OK, None);
    }
    inst.ref_strong += 1;
    let result = geolocator_query_interface(heap, idx, iid);
    geolocator_release(heap, idx);
    result
}

/// `weak_reference_source_GetWeakReference`.
pub fn weak_reference_source_get_weak_reference(
    heap: &mut Heap,
    idx: usize,
) -> (Hresult, Option<Iface>) {
    weak_reference_add_ref(heap, idx);
    (S_OK, Some(Iface::WeakReference(idx)))
}

/// `weak_reference_source_QueryInterface`: forwards to the object.
pub fn weak_reference_source_query_interface(
    heap: &mut Heap,
    idx: usize,
    iid: Iid,
) -> (Hresult, Option<Iface>) {
    geolocator_query_interface(heap, idx, iid)
}

/// `weak_reference_source_AddRef`: forwards to the object.
pub fn weak_reference_source_add_ref(heap: &mut Heap, idx: usize) -> u32 {
    geolocator_add_ref(heap, idx)
}

/// `weak_reference_source_Release`: forwards to the object.
pub fn weak_reference_source_release(heap: &mut Heap, idx: usize) -> u32 {
    geolocator_release(heap, idx)
}
