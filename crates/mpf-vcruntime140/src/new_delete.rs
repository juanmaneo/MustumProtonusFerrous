/// `sizeof(void *)` of this build. `operator new` never passes a smaller alignment to `_aligned_malloc`.
pub const POINTER_SIZE: usize = std::mem::size_of::<*const ()>();

/// `std::bad_alloc`, thrown by the scalar `operator new` when `_callnewh` declines to retry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadAlloc;

/// `operator new` allocates one byte for a zero size.
pub fn normalize_size(size: usize) -> usize {
    size.max(1)
}

/// An alignment below `sizeof(void *)` becomes `sizeof(void *)`. The C code does not check for a
/// power of two, and the port does not either.
pub fn normalize_alignment(alignment: usize) -> usize {
    alignment.max(POINTER_SIZE)
}

/// `operator new(size_t)`. It calls `malloc` until one succeeds. After each failure it calls `callnewh`,
/// and throws [`BadAlloc`] once that declines to retry. `operator new[]` delegates to it.
pub fn operator_new<P>(
    size: usize,
    mut malloc: impl FnMut(usize) -> Option<P>,
    mut callnewh: impl FnMut(usize) -> bool,
) -> Result<P, BadAlloc> {
    let size = normalize_size(size);
    loop {
        if let Some(p) = malloc(size) {
            return Ok(p);
        }
        if !callnewh(size) {
            return Err(BadAlloc);
        }
    }
}

/// `operator new(size_t, const std::nothrow_t&)`: the scalar operator with a failure caught as `None`.
pub fn operator_new_nothrow<P>(
    size: usize,
    malloc: impl FnMut(usize) -> Option<P>,
    callnewh: impl FnMut(usize) -> bool,
) -> Option<P> {
    operator_new(size, malloc, callnewh).ok()
}

/// `operator new(size_t, std::align_val_t)`. It calls `_aligned_malloc` with the normalized alignment
/// until one succeeds, with the same `callnewh` rules as [`operator_new`].
pub fn operator_new_aligned<P>(
    size: usize,
    alignment: usize,
    mut aligned_malloc: impl FnMut(usize, usize) -> Option<P>,
    callnewh: impl FnMut(usize) -> bool,
) -> Result<P, BadAlloc> {
    let alignment = normalize_alignment(alignment);
    operator_new(size, |s| aligned_malloc(s, alignment), callnewh)
}

/// `operator new(size_t, std::align_val_t, const std::nothrow_t&)`.
pub fn operator_new_aligned_nothrow<P>(
    size: usize,
    alignment: usize,
    aligned_malloc: impl FnMut(usize, usize) -> Option<P>,
    callnewh: impl FnMut(usize) -> bool,
) -> Option<P> {
    operator_new_aligned(size, alignment, aligned_malloc, callnewh).ok()
}

/// `operator delete(void *)`: frees a non-null pointer. Every unaligned delete overload delegates to it.
pub fn operator_delete<P>(ptr: Option<P>, free: impl FnOnce(P)) {
    if let Some(p) = ptr {
        free(p);
    }
}

/// `operator delete(void *, std::align_val_t)`: frees a non-null pointer with `_aligned_free`. Every
/// aligned delete overload delegates to it.
pub fn operator_delete_aligned<P>(ptr: Option<P>, aligned_free: impl FnOnce(P)) {
    operator_delete(ptr, aligned_free);
}
