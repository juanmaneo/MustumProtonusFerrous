use core::ffi::c_void;

/// `NP_Initialize` from `dlls/npmshtml/main.c`. The C body ignores `arg` and returns 0.
pub fn np_initialize(_arg: *mut c_void) -> i16 {
    0
}

/// `NP_Shutdown` from `dlls/npmshtml/main.c`, which takes no arguments and returns 0.
pub fn np_shutdown() -> i16 {
    0
}
