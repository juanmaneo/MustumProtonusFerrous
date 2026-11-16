//! Process basics, mirroring `dlls/kernel32/process.c` / `dlls/kernelbase/`.

use core::ffi::c_void;

/// `(HANDLE)-1`, the pseudo-handle returned by GetCurrentProcess.
pub const CURRENT_PROCESS_PSEUDO: *mut c_void = -1isize as *mut c_void;

/// `(HANDLE)-2`, the pseudo-handle returned by GetCurrentThread.
pub const CURRENT_THREAD_PSEUDO: *mut c_void = -2isize as *mut c_void;

/// GetCurrentProcessId.
pub fn get_current_process_id() -> u32 {
    std::process::id()
}

/// GetCurrentProcess (always succeeds, returns the pseudo-handle).
pub fn get_current_process() -> *mut c_void {
    CURRENT_PROCESS_PSEUDO
}

/// GetCurrentThread (pseudo-handle).
pub fn get_current_thread() -> *mut c_void {
    CURRENT_THREAD_PSEUDO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_and_thread_pseudo_handles() {
        assert_eq!(get_current_process_id(), std::process::id());
        assert_eq!(get_current_process() as isize, -1);
        assert_eq!(get_current_thread() as isize, -2);
        assert_eq!(CURRENT_PROCESS_PSEUDO as isize, -1);
        assert_eq!(CURRENT_THREAD_PSEUDO as isize, -2);
    }
}
