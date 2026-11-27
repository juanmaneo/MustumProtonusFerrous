/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = ksuser.dll` in `dlls/ksuser/Makefile.in`.
const KSUSER_DLL_NAME: &str = "ksuser.dll";

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

/// `KsCreateAllocator`, exported as `@ stub` in `dlls/ksuser/ksuser.spec`.
pub fn ks_create_allocator() -> StubException {
    wine_spec_unimplemented_stub(KSUSER_DLL_NAME, "KsCreateAllocator")
}

/// `KsCreateClock`, exported as `@ stub` in `dlls/ksuser/ksuser.spec`.
pub fn ks_create_clock() -> StubException {
    wine_spec_unimplemented_stub(KSUSER_DLL_NAME, "KsCreateClock")
}

/// `KsCreatePin`, exported as `@ stub` in `dlls/ksuser/ksuser.spec`.
pub fn ks_create_pin() -> StubException {
    wine_spec_unimplemented_stub(KSUSER_DLL_NAME, "KsCreatePin")
}

/// `KsCreateTopologyNode`, exported as `@ stub` in `dlls/ksuser/ksuser.spec`.
pub fn ks_create_topology_node() -> StubException {
    wine_spec_unimplemented_stub(KSUSER_DLL_NAME, "KsCreateTopologyNode")
}
