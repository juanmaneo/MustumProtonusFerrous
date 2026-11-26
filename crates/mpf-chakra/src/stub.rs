/// `EXCEPTION_WINE_STUB`, from `winternl.h`: raised when a stub export is called.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE`, from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The exception that `__wine_spec_unimplemented_stub` raises. The C call never returns, so there is
/// no return value to model. `module` and `function` are the two raise arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubRaise<'a> {
    pub code: u32,
    pub flags: u32,
    pub module: &'a str,
    pub function: &'a str,
}

/// `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`: a non-continuable `EXCEPTION_WINE_STUB`
/// carrying the module and export names.
pub fn unimplemented_stub<'a>(module: &'a str, function: &'a str) -> StubRaise<'a> {
    StubRaise {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}
