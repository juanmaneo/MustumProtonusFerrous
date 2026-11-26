/// `EXCEPTION_WINE_STUB` from `include/winternl.h`, raised by `__wine_spec_unimplemented_stub`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// Module name passed to `__wine_spec_unimplemented_stub`.
pub const MODULE_NAME: &str = "dciman32";

/// Exports declared `stub` in `dlls/dciman32/dciman32.spec`.
pub const STUB_EXPORTS: [&str; 17] = [
    "DCIBeginAccess",
    "DCICreateOffscreen",
    "DCICreateOverlay",
    "DCIDestroy",
    "DCIDraw",
    "DCIEndAccess",
    "DCIEnum",
    "DCISetClipList",
    "DCISetDestination",
    "DCISetSrcDestClip",
    "GetDCRegionData",
    "GetWindowRegionData",
    "WinWatchClose",
    "WinWatchDidStatusChange",
    "WinWatchGetClipList",
    "WinWatchNotify",
    "WinWatchOpen",
];

/// The exception a stub export raises. The generated stub never returns,
/// so this value replaces a return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StubFault {
    pub code: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Fault raised by `function` if it is a `stub` export, or `None` otherwise.
/// Names are matched exactly, as `GetProcAddress` does.
pub fn stub_fault(function: &str) -> Option<StubFault> {
    STUB_EXPORTS
        .iter()
        .find(|&&name| name == function)
        .map(|&name| StubFault {
            code: EXCEPTION_WINE_STUB,
            module: MODULE_NAME,
            function: name,
        })
}
