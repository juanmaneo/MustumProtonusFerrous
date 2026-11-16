//! Exception handling boundary: the SEH <-> Rust panic bridge.
//!
//! Design (mirrors how `dlls/ntdll/exception.c` and `signal_x86_64.c` frame
//! Windows exceptions): on the Win32 side every thread has an
//! EXCEPTION_REGISTRATION_RECORD chain; on the MPF side the equivalent is a
//! `catch_unwind` boundary. `guard` runs a closure and converts a Rust panic
//! into an EXCEPTION_RECORD-shaped error code, so a panicking "DLL entry
//! point" can never unwind across the FFI edge into the loader.

use std::panic::{catch_unwind, AssertUnwindSafe};

pub const EXCEPTION_ACCESS_VIOLATION: u32 = 0xC000_0005;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: u32 = 0xC000_001D;
/// Marker code for a Rust panic crossing the boundary
/// (STATUS_FAIL_FAST_EXCEPTION territory; we keep it distinct).
pub const EXCEPTION_RUST_PANIC: u32 = 0xE000_0001;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exception {
    pub code: u32,
    pub address: usize,
}

/// Runs `f`, converting a panic into `Err(EXCEPTION_RUST_PANIC)` instead of
/// unwinding. This is the MPF equivalent of a top-level SEH handler
/// (`__try/__except` around the whole module call in ntdll).
pub fn guard<F, R>(f: F) -> Result<R, Exception>
where
    F: FnOnce() -> R,
{
    catch_unwind(AssertUnwindSafe(f)).map_err(|_| Exception {
        code: EXCEPTION_RUST_PANIC,
        address: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static HOOK_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn guard_passes_values_through() {
        assert_eq!(guard(|| 42), Ok(42));
        let r: Result<String, Exception> = guard(|| "ok".to_string());
        assert_eq!(r.unwrap(), "ok");
    }

    #[test]
    fn guard_catches_panics() {
        let _guard = HOOK_LOCK.lock().unwrap();
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {})); // keep test output clean
        let r = guard(|| -> i32 { panic!("boom") });
        std::panic::set_hook(prev);
        assert_eq!(
            r,
            Err(Exception {
                code: EXCEPTION_RUST_PANIC,
                address: 0
            })
        );
    }

    #[test]
    fn guard_catches_abort_style_panics_from_closures() {
        let _guard = HOOK_LOCK.lock().unwrap();
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let r = guard(|| {
            let v: Vec<i32> = Vec::new();
            v[0] // index out of bounds -> panic
        });
        std::panic::set_hook(prev);
        assert!(r.is_err());
    }
}
