/// `__std_bulk_submit_threadpool_work`: calls `SubmitThreadpoolWork(work)` once for each of the
/// `count` submissions. `submit` stands for the OS call.
pub fn std_bulk_submit_threadpool_work<W: Copy>(work: W, count: usize, mut submit: impl FnMut(W)) {
    for _ in 0..count {
        submit(work);
    }
}
