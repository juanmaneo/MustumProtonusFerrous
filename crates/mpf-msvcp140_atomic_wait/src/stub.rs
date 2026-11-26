/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msvcp140_atomic_wait.dll` in `dlls/msvcp140_atomic_wait/Makefile.in`.
const MSVCP140_ATOMIC_WAIT_DLL_NAME: &str = "msvcp140_atomic_wait.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `__std_atomic_compare_exchange_128`, exported as `@ stub`.
pub fn std_atomic_compare_exchange_128() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP140_ATOMIC_WAIT_DLL_NAME,
        "__std_atomic_compare_exchange_128",
    )
}

/// `__std_atomic_get_mutex`, exported as `@ stub`.
pub fn std_atomic_get_mutex() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_atomic_get_mutex")
}

/// `__std_atomic_has_cmpxchg16b`, exported as `@ stub`.
pub fn std_atomic_has_cmpxchg16b() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_atomic_has_cmpxchg16b")
}

/// `__std_atomic_notify_all_indirect`, exported as `@ stub`.
pub fn std_atomic_notify_all_indirect() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP140_ATOMIC_WAIT_DLL_NAME,
        "__std_atomic_notify_all_indirect",
    )
}

/// `__std_atomic_notify_one_indirect`, exported as `@ stub`.
pub fn std_atomic_notify_one_indirect() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP140_ATOMIC_WAIT_DLL_NAME,
        "__std_atomic_notify_one_indirect",
    )
}

/// `__std_atomic_set_api_level`, exported as `@ stub`.
pub fn std_atomic_set_api_level() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_atomic_set_api_level")
}

/// `__std_atomic_wait_get_deadline`, exported as `@ stub`.
pub fn std_atomic_wait_get_deadline() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP140_ATOMIC_WAIT_DLL_NAME,
        "__std_atomic_wait_get_deadline",
    )
}

/// `__std_atomic_wait_get_remaining_timeout`, exported as `@ stub`.
pub fn std_atomic_wait_get_remaining_timeout() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP140_ATOMIC_WAIT_DLL_NAME,
        "__std_atomic_wait_get_remaining_timeout",
    )
}

/// `__std_atomic_wait_indirect`, exported as `@ stub`.
pub fn std_atomic_wait_indirect() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_atomic_wait_indirect")
}

/// `__std_tzdb_delete_sys_info`, exported as `@ stub`.
pub fn std_tzdb_delete_sys_info() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_tzdb_delete_sys_info")
}

/// `__std_tzdb_get_sys_info`, exported as `@ stub`.
pub fn std_tzdb_get_sys_info() -> StubException {
    wine_spec_unimplemented_stub(MSVCP140_ATOMIC_WAIT_DLL_NAME, "__std_tzdb_get_sys_info")
}
