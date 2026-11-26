//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`, which every `@ stub` export of
//! `cng.sys.spec` calls (winebuild emits the call, see `tools/winebuild/import.c`). A stub never
//! returns a value. It raises `EXCEPTION_WINE_STUB` non-continuably with two parameters, the module
//! name and the function name, and loops on the raise.

use crate::exports::{find_export, ExportKind};

/// `EXCEPTION_WINE_STUB` from `winternl.h`: "stub entry point called".
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// Module name winebuild passes to the stub: the spec file's basename with `.spec` removed
/// (`set_dll_file_name` in `tools/winebuild/main.c`). Inferred, not stated in the spec file.
pub const WINE_SPEC_FILE_NAME: &str = "cng.sys";

/// The `RaiseException` record that `__wine_spec_unimplemented_stub` builds. The C stub never
/// returns, so this value stands in for the raise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Calling export `function`. Returns the exception the stub raises when `function` is an `@ stub`
/// export. Returns `None` for forwarded exports, whose code runs in `bcrypt.dll`, and for names
/// that `cng.sys` does not export.
pub fn unimplemented_stub(function: &str) -> Option<StubException> {
    let export = find_export(function)?;
    match export.kind {
        ExportKind::Stub => Some(StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: WINE_SPEC_FILE_NAME,
            function: export.name,
        }),
        ExportKind::Forward(_) => None,
    }
}
