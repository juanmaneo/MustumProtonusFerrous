//! Port of the portable logic in Wine's `dlls/cldapi/main.c`, and of the stub
//! behaviour of the `@ stub` exports in `cldapi.spec`.
//!
//! Ported:
//! - `CfGetPlaceholderInfo` as [`cf_get_placeholder_info`]: a stub that returns
//!   `E_NOTIMPL` and never writes `buf` or `retlen`. The C body reads `h`, `info`
//!   and `len` only for its `FIXME` log, so the port does not take them.
//! - `CfGetPlatformInfo` as [`cf_get_platform_info`]: a stub that returns
//!   `E_NOTIMPL` and never writes `*info`. [`CfPlatformInfo`] mirrors the
//!   three-`DWORD` `CF_PLATFORM_INFO` layout.
//! - `E_NOTIMPL` as [`E_NOTIMPL`].
//! - The 49 `@ stub` exports, listed in [`STUB_EXPORTS`]. Their entry point calls
//!   `__wine_spec_unimplemented_stub` (`libs/winecrt0/stub.c`), which never returns.
//!   It raises `EXCEPTION_WINE_STUB` in a loop, with the spec file name and the
//!   export name as arguments: [`wine_spec_unimplemented_stub`] returns that raise.
//!
//! Skipped:
//! - The `FIXME` logging and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - The `RtlRaiseException` variant in `dlls/ntdll/exception.c`. It raises the same
//!   code, flags and arguments, but PE modules import `winecrt0` by default
//!   (`tools/makedep.c`, `get_default_imports`), so this crate follows that variant.
//! - Cloud Files behaviour. Wine implements none of it, and the stubs never return a value.

/// `E_NOTIMPL`, from `winerror.h`.
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
/// `EXCEPTION_WINE_STUB`, from `winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE`, from `winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;
/// The spec file name that winebuild passes to every stub (`-Wb,-F,cldapi.dll` in `tools/makedep.c`).
pub const MODULE_NAME: &str = "cldapi.dll";

/// The `CF_PLATFORM_INFO` layout from `include/cfapi.h`.
#[repr(C)]
pub struct CfPlatformInfo {
    pub build_number: u32,
    pub revision_number: u32,
    pub integration_number: u32,
}

/// The `@ stub` exports in `cldapi.spec`, in spec order.
pub const STUB_EXPORTS: [&str; 49] = [
    "CfAbortOperation",
    "CfCloseAppPolicy",
    "CfCloseHandle",
    "CfConnectSyncRoot",
    "CfConvertToPlaceholder",
    "CfCreatePlaceholders",
    "CfDehydratePlaceholder",
    "CfDehydratePlaceholderEx",
    "CfDisconnectSyncRoot",
    "CfEnumAppPolicy",
    "CfExecute",
    "CfGetCorrelationVector",
    "CfGetLastSyncStatus",
    "CfGetPlaceholderRangeInfo",
    "CfGetPlaceholderRangeInfoForHydration",
    "CfGetPlaceholderStateFromAttributeTag",
    "CfGetPlaceholderStateFromFileInfo",
    "CfGetPlaceholderStateFromFindData",
    "CfGetSyncRootInfoByHandle",
    "CfGetSyncRootInfoByPath",
    "CfGetTransferKey",
    "CfGetWin32HandleFromProtectedHandle",
    "CfHydratePlaceholder",
    "CfHydratePlaceholderWithAttributionPrivate",
    "CfLockProperties",
    "CfOpenAppPolicy",
    "CfOpenFileWithOplock",
    "CfOpenProgressEvent",
    "CfQueryProgress",
    "CfQuerySyncProviderStatus",
    "CfReferenceProtectedHandle",
    "CfRegisterSyncRoot",
    "CfReleaseProtectedHandle",
    "CfReleaseTransferKey",
    "CfReportProviderProgress",
    "CfReportProviderProgress2",
    "CfReportSyncStatus",
    "CfRetrieveProperties",
    "CfRevertPlaceholder",
    "CfSetAppPolicy",
    "CfSetCorrelationVector",
    "CfSetInSyncState",
    "CfSetPinState",
    "CfStoreProperties",
    "CfTestToggleSyncMsgThreadWaitEvent",
    "CfUnlockProperties",
    "CfUnregisterSyncRoot",
    "CfUpdatePlaceholder",
    "CfUpdateSyncProviderStatus",
];

/// `CfGetPlaceholderInfo`: returns `E_NOTIMPL` and leaves `buf` and `retlen` unchanged, null or not.
pub fn cf_get_placeholder_info(_buf: Option<&mut [u8]>, _retlen: Option<&mut u32>) -> i32 {
    E_NOTIMPL
}

/// `CfGetPlatformInfo`: returns `E_NOTIMPL` and leaves `*info` unchanged, null or not.
pub fn cf_get_platform_info(_info: Option<&mut CfPlatformInfo>) -> i32 {
    E_NOTIMPL
}

/// The arguments of the `RaiseException` call that a stub makes.
pub struct StubRaise<'a> {
    pub code: u32,
    pub flags: u32,
    pub module: &'a str,
    pub function: &'a str,
}

/// `__wine_spec_unimplemented_stub`: returns the raise that the C loop repeats forever.
/// Wine never returns from a stub, so there is no return value to model.
pub fn wine_spec_unimplemented_stub<'a>(module: &'a str, function: &'a str) -> StubRaise<'a> {
    StubRaise {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        module,
        function,
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
