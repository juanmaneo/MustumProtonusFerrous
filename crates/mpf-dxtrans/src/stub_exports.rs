/// `EXCEPTION_WINE_STUB` from `winternl.h`: "stub entry point called".
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// Win64 mangled names of the `@ stub` DirectX exports, plus `DllEnumClassObjects`,
/// as written in `dxtrans.spec`.
pub const STUB_EXPORTS: [&str; 7] = [
    "DllEnumClassObjects",
    "?DXConstOverArray@@YAXPEAVDXPMSAMPLE@@AEBV1@K@Z",
    "?DXConstUnderArray@@YAXPEAVDXPMSAMPLE@@AEBV1@K@Z",
    "?DXDitherArray@@YAXPEBUDXDITHERDESC@@@Z",
    "?DXLinearInterpolateArray@@YAXPEBVDXBASESAMPLE@@PEAUDXLIMAPINFO@@PEAV1@K@Z",
    "?DXOverArray@@YAXPEAVDXPMSAMPLE@@PEBV1@K@Z",
    "?DXOverArrayMMX@@YAXPEAVDXPMSAMPLE@@PEBV1@K@Z",
];

/// The `RaiseException` call made by `__wine_spec_unimplemented_stub`.
/// The stub never returns: it loops on this call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`, modelled as
/// the exception it raises. Returns the exception the stub would raise (the
/// Rust model returns instead of looping forever).
pub fn unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}
