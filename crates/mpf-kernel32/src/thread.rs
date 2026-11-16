//! Thread basics, mirroring `dlls/kernel32/thread.c`.

use core::ffi::c_void;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

/// WAIT_OBJECT_0
pub const WAIT_OBJECT_0: u32 = 0;
/// WAIT_TIMEOUT
pub const WAIT_TIMEOUT: u32 = 0x102;
/// WAIT_FAILED
pub const WAIT_FAILED: u32 = 0xFFFFFFFF;

thread_local! {
    static THREAD_ID: Cell<u32> = const { Cell::new(0) };
}

static NEXT_THREAD_ID: AtomicU32 = AtomicU32::new(1);

/// GetCurrentThreadId: stable per-thread ids starting at 1, like Windows.
pub fn get_current_thread_id() -> u32 {
    THREAD_ID.with(|t| {
        let v = t.get();
        if v != 0 {
            v
        } else {
            let n = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
            t.set(n);
            n
        }
    })
}

/// Win32 thread start routine signature.
pub type ThreadStart = extern "system" fn(*mut c_void) -> u32;

struct ThreadTable {
    next: u64,
    threads: HashMap<u64, std::thread::JoinHandle<u32>>,
}

static THREADS: Mutex<Option<ThreadTable>> = Mutex::new(None);

fn with_table<R>(f: impl FnOnce(&mut ThreadTable) -> R) -> R {
    let mut guard = THREADS.lock().unwrap();
    f(guard.get_or_insert(ThreadTable {
        next: 1,
        threads: HashMap::new(),
    }))
}

/// CreateThread: spawns a std thread running `start(param)`; returns a
/// handle id usable with `wait_for_thread`. The parameter crosses the
/// thread boundary as a usize, like a Win32 LPARAM.
pub fn create_thread(start: ThreadStart, param: *mut c_void) -> u64 {
    let addr = param as usize;
    let join = std::thread::spawn(move || start(addr as *mut c_void));
    with_table(|t| {
        let id = t.next;
        t.next += 1;
        t.threads.insert(id, join);
        id
    })
}

/// WaitForSingleObject on a thread handle: joins it (we ignore the timeout,
/// like an INFINITE wait) and returns WAIT_OBJECT_0; WAIT_FAILED if the
/// handle is unknown.
pub fn wait_for_thread(handle: u64, _timeout_ms: u32) -> u32 {
    with_table(|t| match t.threads.remove(&handle) {
        Some(join) => {
            let _ = join.join();
            WAIT_OBJECT_0
        }
        None => WAIT_FAILED,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicU32;

    extern "system" fn sets_value(p: *mut c_void) -> u32 {
        let shared = unsafe { &*(p as *const AtomicU32) };
        shared.store(42, Ordering::Relaxed);
        7
    }

    #[test]
    fn thread_ids_are_stable_and_unique() {
        let a = get_current_thread_id();
        let b = get_current_thread_id();
        assert_eq!(a, b);
        assert!(a > 0);
        let other = std::thread::spawn(get_current_thread_id).join().unwrap();
        assert_ne!(a, other);
    }

    #[test]
    fn create_and_wait_thread() {
        let shared = Arc::new(AtomicU32::new(0));
        let ptr = Arc::into_raw(shared.clone()) as *mut c_void;
        let handle = create_thread(sets_value, ptr);
        assert_eq!(wait_for_thread(handle, 0), WAIT_OBJECT_0);
        assert_eq!(shared.load(Ordering::Relaxed), 42);
        unsafe { drop(Arc::from_raw(ptr)) };
    }

    #[test]
    fn wait_on_unknown_handle_fails() {
        assert_eq!(wait_for_thread(u64::MAX, 0), WAIT_FAILED);
    }
}
