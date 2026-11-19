//! Thread count rules of `D3DX10CreateThreadPump` from `async.c`.

use crate::{HResult, E_FAIL};

/// The I/O and processing thread counts that `D3DX10CreateThreadPump` uses. A count of 1024 or
/// more fails with `E_FAIL`. A zero I/O count becomes one thread, and a zero processing count
/// becomes `processor_count`, which stands in for `GetSystemInfo().dwNumberOfProcessors`.
pub fn thread_pump_thread_counts(
    io_threads: u32,
    proc_threads: u32,
    processor_count: u32,
) -> Result<(u32, u32), HResult> {
    if io_threads >= 1024 || proc_threads >= 1024 {
        return Err(E_FAIL);
    }

    let io_threads = io_threads.max(1);
    let proc_threads = if proc_threads == 0 {
        processor_count
    } else {
        proc_threads
    };
    Ok((io_threads, proc_threads))
}
