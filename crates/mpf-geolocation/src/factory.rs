//! Port of the static activation factory in `main.c`: reference counting, QueryInterface, and
//! ActivateInstance.

use crate::heap::{Alloc, Heap, Iface};
use crate::hresult::{failed, Hresult, E_NOINTERFACE, E_OUTOFMEMORY, S_OK};
use crate::iid::{inspectable_base, Iid};
use crate::weakref::weak_reference_source_init;

/// `struct geolocator_statics`. The factory is static, so only its reference count is modelled.
#[derive(Debug)]
pub struct Factory {
    pub ref_count: u32,
}

impl Default for Factory {
    /// The static factory starts with one reference.
    fn default() -> Self {
        Self { ref_count: 1 }
    }
}

/// `factory_QueryInterface`: accepts IUnknown, IInspectable, IAgileObject, and IActivationFactory.
pub fn factory_query_interface(factory: &mut Factory, iid: Iid) -> (Hresult, Option<Iface>) {
    if !inspectable_base(iid) && iid != Iid::IActivationFactory {
        return (E_NOINTERFACE, None);
    }
    factory_add_ref(factory);
    (S_OK, Some(Iface::ActivationFactory))
}

/// `factory_AddRef`.
pub fn factory_add_ref(factory: &mut Factory) -> u32 {
    factory.ref_count += 1;
    factory.ref_count
}

/// `factory_Release`.
pub fn factory_release(factory: &mut Factory) -> u32 {
    factory.ref_count -= 1;
    factory.ref_count
}

/// `factory_ActivateInstance`. The object is allocated first, then its control block. If the
/// control block fails, the object is freed and the failure is returned.
pub fn factory_activate_instance(heap: &mut Heap, alloc: Alloc) -> (Hresult, Option<Iface>) {
    if !heap.calloc(alloc.object) {
        return (E_OUTOFMEMORY, None);
    }
    let idx = heap.push_object();
    let hr = weak_reference_source_init(heap, idx, alloc.record);
    if failed(hr) {
        heap.free_object(idx);
        return (hr, None);
    }
    (S_OK, Some(Iface::Geolocator(idx)))
}
