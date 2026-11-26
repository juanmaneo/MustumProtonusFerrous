//! Port of the portable logic in Wine's `dlls/acledit/main.c`.
//!
//! `acledit.dll` defines one function, `FMExtensionProcW`, and six `stub`
//! exports in `acledit.spec`. It defines no `DllMain`, so the default body from
//! `libs/winecrt0/dll_main.c` is linked in. The spec stubs jump to
//! `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`.
//!
//! Ported:
//! - `FMExtensionProcW` (`main.c`): [`fm_extension_proc_w`], which always returns 0.
//! - Default `DllMain` (`libs/winecrt0/dll_main.c`): [`dll_main`], which returns
//!   TRUE for every reason and disables thread library calls on attach.
//! - Stub model (modelled, not ported): [`unimplemented_stub`] and [`export_stub`].
//!   The stub never returns. It raises `EXCEPTION_WINE_STUB` with the module and
//!   function names as arguments, so the port returns that exception. One export
//!   per entry of [`STUB_EXPORTS`].
//!
//! Skipped:
//! - The `FIXME` trace in `FMExtensionProcW`: logging, no return value.
//! - `DisableThreadLibraryCalls` itself: an OS call. Only the decision to call it
//!   is modelled. The `inst` and `reserved` arguments of `DllMain` are unused.
//! - `RaiseException` and SEH: the port returns the exception instead of raising it.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;
/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;
/// Module name passed to the stub. `winebuild` turns `acledit.spec` into `acledit`
/// and appends `.dll`, because the name has no dot.
pub const MODULE_FILE_NAME: &str = "acledit.dll";

/// The `stub` exports of `acledit.spec`, in ordinal order (1, 2, 3, 6, 7, 8).
pub const STUB_EXPORTS: [&str; 6] = [
    "EditAuditInfo",
    "EditOwnerInfo",
    "EditPermissionInfo",
    "SedDiscretionaryAclEditor",
    "SedSystemAclEditor",
    "SedTakeOwnership",
];

/// The exception raised by `__wine_spec_unimplemented_stub`: code, flags, and the
/// two `ExceptionInformation` strings (module name and function name).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException<'a> {
    pub code: u32,
    pub flags: u32,
    pub module: &'a str,
    pub function: &'a str,
}

/// `__wine_spec_unimplemented_stub`. The C version loops on `RaiseException` and
/// never returns, so this returns the exception it raises.
pub fn unimplemented_stub<'a>(module: &'a str, function: &'a str) -> StubException<'a> {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}

/// The spec-file `stub` export `function` of `acledit.dll`.
pub fn export_stub(function: &str) -> StubException<'_> {
    unimplemented_stub(MODULE_FILE_NAME, function)
}

/// Observable effects of one call to the default `DllMain`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DllMainOutcome {
    /// The `BOOL` return value.
    pub ret: bool,
    /// Whether `DisableThreadLibraryCalls` is called.
    pub disable_thread_library_calls: bool,
}

/// Default `DllMain`: returns TRUE for every reason and calls
/// `DisableThreadLibraryCalls` only on `DLL_PROCESS_ATTACH`.
pub fn dll_main(reason: u32) -> DllMainOutcome {
    DllMainOutcome {
        ret: true,
        disable_thread_library_calls: reason == DLL_PROCESS_ATTACH,
    }
}

/// `FMExtensionProcW`. The C body only traces with `FIXME` and returns 0.
pub fn fm_extension_proc_w(_hwnd: usize, _event: usize, _lparam: isize) -> i32 {
    0
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
