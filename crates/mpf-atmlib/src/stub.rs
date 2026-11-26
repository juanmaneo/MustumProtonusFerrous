//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The record that `__wine_spec_unimplemented_stub` raises. The C function loops on the
/// raise and never returns, so the port returns the record instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException<'a> {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    /// `ExceptionInformation[0]`: the spec file name, supplied by the caller.
    pub module: &'a str,
    /// `ExceptionInformation[1]`: the export name.
    pub function: &'a str,
}

/// `__wine_spec_unimplemented_stub(module, function)`.
pub fn unimplemented_stub<'a>(module: &'a str, function: &'a str) -> StubException<'a> {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}
