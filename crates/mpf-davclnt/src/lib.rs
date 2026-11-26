//! Port of the portable logic in Wine's `dlls/davclnt/main.c`.
//!
//! `davclnt.dll` defines two functions, `DavRegisterAuthCallback` and
//! `DavUnregisterAuthCallback`, and 19 `stub` exports in `davclnt.spec`. It
//! defines no `DllMain` or `DllCanUnloadNow`, so the defaults from
//! `libs/winecrt0/dll_main.c` and `libs/winecrt0/dll_canunload.c` are linked in.
//! The spec stubs jump to `__wine_spec_unimplemented_stub` from `libs/winecrt0/stub.c`.
//!
//! Ported:
//! - `DavRegisterAuthCallback` (`main.c`): [`dav_register_auth_callback`], which
//!   ignores both arguments and returns [`DAV_REGISTER_AUTH_CALLBACK_HANDLE`].
//! - `DavUnregisterAuthCallback` (`main.c`): [`dav_unregister_auth_callback`], a no-op.
//! - Default `DllMain` (`libs/winecrt0/dll_main.c`): [`dll_main`], which returns
//!   TRUE for every reason and disables thread library calls on attach.
//! - Default `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`): [`dll_can_unload_now`].
//! - Stub model (modelled, not ported): [`unimplemented_stub`] and [`export_stub`].
//!   The stub never returns. It raises `EXCEPTION_WINE_STUB` with the module and
//!   function names as arguments, so the port returns that exception. One export
//!   per entry of [`STUB_EXPORTS`]: the five `Dav*` functions, `DllGetClassObject`
//!   and the 13 `NP*` network provider entries.
//! - Constants from `davclnt.h`: the `DAV_AUTHN_SCHEME_*` bits and [`AuthNextStep`].
//!
//! Skipped:
//! - The `FIXME` traces in `main.c`: logging, no return value.
//! - The callback is never invoked, so the callback typedefs and the argument
//!   structs (`DAV_CALLBACK_AUTH_BLOB`, `DAV_CALLBACK_AUTH_UNP`, `DAV_CALLBACK_CRED`)
//!   have no behaviour to port.
//! - `DavGetHTTPFromUNCPath` and `DavGetUNCFromHTTPPath`: declared in `davclnt.h`,
//!   but absent from `davclnt.spec` and undefined in `main.c`.
//! - `DisableThreadLibraryCalls` itself: an OS call. Only the decision to call it
//!   is modelled. The `inst` and `reserved` arguments of `DllMain` are unused.
//! - `RaiseException` and SEH: the port returns the exception instead of raising it.

/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;
/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;
/// `S_FALSE` from `winerror.h`.
pub const S_FALSE: i32 = 1;
/// Module name passed to the stub: the DLL's file name, from `MODULE` in `Makefile.in`.
pub const MODULE_FILE_NAME: &str = "davclnt.dll";

/// `OPAQUE_HANDLE` from `davclnt.h`, which is a `DWORD`.
pub type OpaqueHandle = u32;
/// The handle `DavRegisterAuthCallback` returns (`0xdeadbeef` in `main.c`).
pub const DAV_REGISTER_AUTH_CALLBACK_HANDLE: OpaqueHandle = 0xdead_beef;

/// `DAV_AUTHN_SCHEME_*` from `davclnt.h`.
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 0x0000_0001;
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 0x0000_0002;
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 0x0000_0004;
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 0x0000_0008;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 0x0000_0010;
pub const DAV_AUTHN_SCHEME_CERT: u32 = 0x0001_0000;
pub const DAV_AUTHN_SCHEME_FBA: u32 = 0x0010_0000;

/// `AUTHNEXTSTEP` from `davclnt.h`: what the auth callback asks WebDAV to do next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AuthNextStep {
    DefaultBehavior = 0,
    RetryRequest = 1,
    CancelRequest = 2,
}

/// The `stub` exports of `davclnt.spec`, in spec order.
pub const STUB_EXPORTS: [&str; 19] = [
    "DavCancelConnectionsToServer",
    "DavFreeUsedDiskSpace",
    "DavGetDiskSpaceUsage",
    "DavGetTheLockOwnerOfTheFile",
    "DavInvalidateCache",
    "DllGetClassObject",
    "NPAddConnection3",
    "NPAddConnection",
    "NPCancelConnection",
    "NPCloseEnum",
    "NPEnumResource",
    "NPFormatNetworkName",
    "NPGetCaps",
    "NPGetConnection",
    "NPGetResourceInformation",
    "NPGetResourceParent",
    "NPGetUniversalName",
    "NPGetUser",
    "NPOpenEnum",
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

/// The spec-file `stub` export `function` of `davclnt.dll`.
pub fn export_stub(function: &str) -> StubException<'_> {
    unimplemented_stub(MODULE_FILE_NAME, function)
}

/// `DavRegisterAuthCallback`. The C body only traces with `FIXME` and returns
/// `0xdeadbeef`. `callback` is a `PFNDAVAUTHCALLBACK` pointer (0 for NULL) and is never read.
pub fn dav_register_auth_callback(_callback: usize, _version: u32) -> OpaqueHandle {
    DAV_REGISTER_AUTH_CALLBACK_HANDLE
}

/// `DavUnregisterAuthCallback`. The C body only traces with `FIXME`.
pub fn dav_unregister_auth_callback(_handle: OpaqueHandle) {}

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

/// `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`: always `S_FALSE`.
pub fn dll_can_unload_now() -> i32 {
    S_FALSE
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
