/// The `Count` field of `FAST_MUTEX` from `include/ddk/wdm.h`. The Windows structure also has
/// `Owner`, `Contention` and `Event`, which the try path does not read.
pub struct FastMutex {
    pub count: i32,
}

/// `InterlockedCompareExchange`: stores `exchange` when `dest` holds `comparand`, and returns the
/// value `dest` held before the call.
fn interlocked_compare_exchange(dest: &mut i32, exchange: i32, comparand: i32) -> i32 {
    let initial = *dest;
    if initial == comparand {
        *dest = exchange;
    }
    initial
}

/// `ExTryToAcquireFastMutex` from `dlls/hal/hal.c`. A free mutex has `Count` 1. Taking it sets
/// `Count` to 0, and the function reports whether the mutex was free.
pub fn ex_try_to_acquire_fast_mutex(mutex: &mut FastMutex) -> bool {
    interlocked_compare_exchange(&mut mutex.count, 0, 1) == 1
}
