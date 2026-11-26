/// `EXCEPTION_WINE_STUB` from `include/winternl.h`, raised by `__wine_spec_unimplemented_stub`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`, set on the record a stub raises.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// Module name passed to `__wine_spec_unimplemented_stub`: the spec file name without `.spec`.
pub const MODULE_NAME: &str = "connect";

/// Exports declared `stub` in `dlls/connect/connect.spec`.
pub const STUB_EXPORTS: [&str; 13] = [
    "AddConnectionOptionListEntries",
    "CreateVPNConnection",
    "DllGetClassObject",
    "GetInternetConnected",
    "GetNetworkConnected",
    "GetVPNConnected",
    "HrIsInternetConnected",
    "HrIsInternetConnectedGUID",
    "IsInternetConnectedGUID",
    "IsUniqueConnectionName",
    "RegisterPageWithPage",
    "UnregisterPage",
    "UnregisterPagesLink",
];

/// The exception a stub export raises. The generated stub never returns,
/// so this value replaces a return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StubFault {
    pub code: u32,
    pub flags: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// Fault raised by `function` if it is a `stub` export, or `None` otherwise.
/// Names are matched exactly.
pub fn stub_fault(function: &str) -> Option<StubFault> {
    STUB_EXPORTS
        .iter()
        .find(|&&name| name == function)
        .map(|&name| StubFault {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            module: MODULE_NAME,
            function: name,
        })
}
