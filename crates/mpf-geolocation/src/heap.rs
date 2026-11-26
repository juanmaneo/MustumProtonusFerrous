//! Handle model for geolocator allocations, their weak reference control blocks, and the
//! interface pointers that `QueryInterface` hands out.

/// An interface pointer returned by `QueryInterface` or `GetWeakReference`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iface {
    /// `IGeolocator` (also `IInspectable`, `IAgileObject`, `IUnknown`) of the geolocator at this index.
    Geolocator(usize),
    /// `IWeakReferenceSource` of the geolocator at this index.
    WeakReferenceSource(usize),
    /// `IWeakReference` of the control block at this index.
    WeakReference(usize),
    /// `IActivationFactory` of the static factory.
    ActivationFactory,
}

/// Outcome of each `calloc` in `factory_ActivateInstance`, in call order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alloc {
    /// Whether the `struct geolocator` allocation succeeds.
    pub object: bool,
    /// Whether the `struct weak_reference` control block allocation succeeds.
    pub record: bool,
}

/// One geolocator. `ref_strong` and `ref_weak` live in the control block; `object_alive` and
/// `record_alive` stand in for the `free` calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instance {
    pub ref_strong: u32,
    pub ref_weak: u32,
    pub object_alive: bool,
    pub record_alive: bool,
}

/// Arena of geolocator allocations. Freed slots stay in place, so indices remain stable.
#[derive(Debug, Default)]
pub struct Heap {
    instances: Vec<Instance>,
    calloc_calls: u32,
}

impl Heap {
    /// `calloc`: counts the call and reports whether the injected allocation succeeds.
    pub fn calloc(&mut self, ok: bool) -> bool {
        self.calloc_calls += 1;
        ok
    }

    /// Number of `calloc` calls made so far.
    pub fn calloc_calls(&self) -> u32 {
        self.calloc_calls
    }

    /// Adds the object allocation. Its control block is attached later by `weak_reference_source_init`.
    pub fn push_object(&mut self) -> usize {
        self.instances.push(Instance {
            ref_strong: 0,
            ref_weak: 0,
            object_alive: true,
            record_alive: false,
        });
        self.instances.len() - 1
    }

    /// `free(impl)` on the geolocator object.
    pub fn free_object(&mut self, idx: usize) {
        self.instance_mut(idx).object_alive = false;
    }

    pub fn instance(&self, idx: usize) -> &Instance {
        &self.instances[idx]
    }

    pub fn instance_mut(&mut self, idx: usize) -> &mut Instance {
        &mut self.instances[idx]
    }

    /// Allocations still held (object and record each count once), so a leak is a non-zero count.
    pub fn live_allocations(&self) -> usize {
        self.instances
            .iter()
            .map(|inst| usize::from(inst.object_alive) + usize::from(inst.record_alive))
            .sum()
    }
}
