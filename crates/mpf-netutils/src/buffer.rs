/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// `NERR_Success` from `include/lmerr.h`.
pub const NERR_SUCCESS: u32 = 0;

/// The process-heap calls that the buffer functions make. `Block` is the `LPVOID` that a call
/// takes or returns. `None` is NULL.
pub trait ProcessHeap {
    type Block: Copy;

    /// `HeapAlloc(GetProcessHeap(), 0, size)`. A failed call returns `None`.
    fn alloc(&mut self, size: usize) -> Option<Self::Block>;
    /// `HeapReAlloc(GetProcessHeap(), 0, block, size)`. A failed call returns `None` and leaves
    /// `block` allocated.
    fn realloc(&mut self, block: Self::Block, size: usize) -> Option<Self::Block>;
    /// `HeapFree(GetProcessHeap(), 0, block)`. The NULL block is passed through unchanged, so the
    /// heap decides its result. Wine's `RtlFreeHeap` returns TRUE for NULL.
    fn free(&mut self, block: Option<Self::Block>) -> bool;
    /// `HeapSize(GetProcessHeap(), 0, block)`. A failed call returns `usize::MAX`, which is
    /// `(SIZE_T)-1`.
    fn size(&mut self, block: Self::Block) -> usize;
    /// `GetLastError()`.
    fn last_error(&self) -> u32;
}

/// `NetApiBufferAllocate`. A NULL `buffer` is `ERROR_INVALID_PARAMETER`. Otherwise `*buffer`
/// receives the block, or NULL when the allocation fails, and the last error is returned.
pub fn net_api_buffer_allocate<H: ProcessHeap>(
    heap: &mut H,
    byte_count: u32,
    buffer: Option<&mut Option<H::Block>>,
) -> u32 {
    let Some(buffer) = buffer else {
        return ERROR_INVALID_PARAMETER;
    };
    *buffer = heap.alloc(byte_count as usize);
    if buffer.is_some() {
        NERR_SUCCESS
    } else {
        heap.last_error()
    }
}

/// `NetApiBufferFree`. The heap's result is ignored, so the return value is always `NERR_Success`.
pub fn net_api_buffer_free<H: ProcessHeap>(heap: &mut H, buffer: Option<H::Block>) -> u32 {
    heap.free(buffer);
    NERR_SUCCESS
}

/// `NetApiBufferReallocate`. A non-zero size resizes `old_buffer`, or allocates when it is NULL.
/// A zero size frees `old_buffer` and stores NULL, but only if the free succeeds. A failed
/// resize stores NULL in `*new_buffer` and leaves the old block allocated, as the C code does.
pub fn net_api_buffer_reallocate<H: ProcessHeap>(
    heap: &mut H,
    old_buffer: Option<H::Block>,
    new_byte_count: u32,
    new_buffer: &mut Option<H::Block>,
) -> u32 {
    if new_byte_count != 0 {
        *new_buffer = match old_buffer {
            Some(old) => heap.realloc(old, new_byte_count as usize),
            None => heap.alloc(new_byte_count as usize),
        };
        if new_buffer.is_some() {
            NERR_SUCCESS
        } else {
            heap.last_error()
        }
    } else {
        if !heap.free(old_buffer) {
            return heap.last_error();
        }
        *new_buffer = None;
        NERR_SUCCESS
    }
}

/// `NetApiBufferSize`. A NULL `buffer` is `ERROR_INVALID_PARAMETER` and leaves `*byte_count`
/// untouched. The C code keeps the low DWORD of `HeapSize`. A failed `HeapSize` gives
/// `0xFFFFFFFF` after that truncation, and the port reports 0 for it.
pub fn net_api_buffer_size<H: ProcessHeap>(
    heap: &mut H,
    buffer: Option<H::Block>,
    byte_count: &mut u32,
) -> u32 {
    let Some(buffer) = buffer else {
        return ERROR_INVALID_PARAMETER;
    };
    let size = heap.size(buffer) as u32;
    *byte_count = if size != u32::MAX { size } else { 0 };
    NERR_SUCCESS
}
