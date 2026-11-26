/// One `struct shared_mutex_elem` on `shared_mutex_list`. Its `shared_mutex` holds only an
/// SRWLOCK, which is OS state and is not modelled.
struct Element {
    ptr: usize,
    refs: i32,
}

/// The `shared_mutex_list` of `dlls/msvcp140_atomic_wait/main.c`, without its critical section.
/// Each instance pointer is in the list at most once, so the list order does not change a result.
#[derive(Default)]
pub struct SharedMutexList {
    elements: Vec<Element>,
}

impl SharedMutexList {
    /// The reference count of the element for `ptr`, or `None` when the list has no such element.
    pub fn ref_count(&self, ptr: usize) -> Option<i32> {
        self.elements.iter().find(|e| e.ptr == ptr).map(|e| e.refs)
    }

    /// The number of elements in the list.
    pub fn live_count(&self) -> usize {
        self.elements.len()
    }
}

/// `__std_acquire_shared_mutex_for_instance`. The C function returns `&sme->mutex`, the SRWLOCK
/// in the element for `ptr`. The port returns `ptr`, which identifies that element while it is in
/// the list.
pub fn std_acquire_shared_mutex_for_instance(list: &mut SharedMutexList, ptr: usize) -> usize {
    match list.elements.iter_mut().find(|e| e.ptr == ptr) {
        Some(sme) => sme.refs += 1,
        None => list.elements.push(Element { ptr, refs: 1 }),
    }
    ptr
}

/// `__std_release_shared_mutex_for_instance`. Drops one reference, and removes the element when
/// the last reference goes. An unknown `ptr` does nothing.
pub fn std_release_shared_mutex_for_instance(list: &mut SharedMutexList, ptr: usize) {
    let Some(index) = list.elements.iter().position(|e| e.ptr == ptr) else {
        return;
    };
    let sme = &mut list.elements[index];
    sme.refs -= 1;
    if sme.refs == 0 {
        list.elements.remove(index);
    }
}
