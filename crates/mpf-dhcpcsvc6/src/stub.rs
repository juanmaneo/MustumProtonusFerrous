/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = dhcpcsvc6.dll` in `dlls/dhcpcsvc6/Makefile.in`.
const DHCPCSVC6_DLL_NAME: &str = "dhcpcsvc6.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `Dhcpv6AcquireParameters`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_acquire_parameters() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6AcquireParameters")
}

/// `Dhcpv6CApiCleanup`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_c_api_cleanup() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6CApiCleanup")
}

/// `Dhcpv6CApiInitialize`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_c_api_initialize() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6CApiInitialize")
}

/// `Dhcpv6CancelOperation`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_cancel_operation() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6CancelOperation")
}

/// `Dhcpv6EnableDhcp`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_enable_dhcp() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6EnableDhcp")
}

/// `Dhcpv6EnableTracing`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_enable_tracing() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6EnableTracing")
}

/// `Dhcpv6FreeLeaseInfo`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_free_lease_info() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6FreeLeaseInfo")
}

/// `Dhcpv6FreeLeaseInfoArray`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_free_lease_info_array() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6FreeLeaseInfoArray")
}

/// `Dhcpv6GetTraceArray`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_get_trace_array() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6GetTraceArray")
}

/// `Dhcpv6GetUserClasses`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_get_user_classes() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6GetUserClasses")
}

/// `Dhcpv6IsEnabled`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_is_enabled() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6IsEnabled")
}

/// `Dhcpv6QueryLeaseInfo`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_query_lease_info() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6QueryLeaseInfo")
}

/// `Dhcpv6QueryLeaseInfoArray`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_query_lease_info_array() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6QueryLeaseInfoArray")
}

/// `Dhcpv6ReleaseParameters`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_release_parameters() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6ReleaseParameters")
}

/// `Dhcpv6ReleasePrefix`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_release_prefix() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6ReleasePrefix")
}

/// `Dhcpv6ReleasePrefixEx`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_release_prefix_ex() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6ReleasePrefixEx")
}

/// `Dhcpv6RenewPrefix`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_renew_prefix() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6RenewPrefix")
}

/// `Dhcpv6RenewPrefixEx`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_renew_prefix_ex() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6RenewPrefixEx")
}

/// `Dhcpv6RequestParams`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_request_params() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6RequestParams")
}

/// `Dhcpv6RequestPrefix`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_request_prefix() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6RequestPrefix")
}

/// `Dhcpv6RequestPrefixEx`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_request_prefix_ex() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6RequestPrefixEx")
}

/// `Dhcpv6SetUserClass`, exported as `@ stub` in `dlls/dhcpcsvc6/dhcpcsvc6.spec`.
pub fn dhcpv6_set_user_class() -> StubException {
    wine_spec_unimplemented_stub(DHCPCSVC6_DLL_NAME, "Dhcpv6SetUserClass")
}
