//! `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`, which every `@ stub`
//! export calls. The ntdll copy in `dlls/ntdll/exception.c` raises the same code and
//! flags through `RtlRaiseException`; this port models the winecrt0 copy.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The exception record raised by a stub.
///
/// The C function re-raises in an infinite loop and never returns, so the port returns
/// the record that the raise carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException<'a> {
    pub code: u32,
    pub flags: u32,
    /// `ExceptionInformation[0]`.
    pub module: &'a str,
    /// `ExceptionInformation[1]`.
    pub function: &'a str,
}

/// Models `__wine_spec_unimplemented_stub(module, function)`.
pub fn wine_spec_unimplemented_stub<'a>(module: &'a str, function: &'a str) -> StubException<'a> {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}
