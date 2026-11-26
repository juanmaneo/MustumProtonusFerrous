use crate::stub;
use crate::StubException;

/// The helper that raises the record for one `@ stub` export.
pub type StubFn = fn() -> StubException;

/// The `dlls/dhcpcsvc6/dhcpcsvc6.spec` export table, in spec order. Every export is `@ stub`,
/// so each entry pairs the export name with the helper that raises its record.
pub const EXPORTS: [(&str, StubFn); 22] = [
    ("Dhcpv6AcquireParameters", stub::dhcpv6_acquire_parameters),
    ("Dhcpv6CApiCleanup", stub::dhcpv6_c_api_cleanup),
    ("Dhcpv6CApiInitialize", stub::dhcpv6_c_api_initialize),
    ("Dhcpv6CancelOperation", stub::dhcpv6_cancel_operation),
    ("Dhcpv6EnableDhcp", stub::dhcpv6_enable_dhcp),
    ("Dhcpv6EnableTracing", stub::dhcpv6_enable_tracing),
    ("Dhcpv6FreeLeaseInfo", stub::dhcpv6_free_lease_info),
    (
        "Dhcpv6FreeLeaseInfoArray",
        stub::dhcpv6_free_lease_info_array,
    ),
    ("Dhcpv6GetTraceArray", stub::dhcpv6_get_trace_array),
    ("Dhcpv6GetUserClasses", stub::dhcpv6_get_user_classes),
    ("Dhcpv6IsEnabled", stub::dhcpv6_is_enabled),
    ("Dhcpv6QueryLeaseInfo", stub::dhcpv6_query_lease_info),
    (
        "Dhcpv6QueryLeaseInfoArray",
        stub::dhcpv6_query_lease_info_array,
    ),
    ("Dhcpv6ReleaseParameters", stub::dhcpv6_release_parameters),
    ("Dhcpv6ReleasePrefix", stub::dhcpv6_release_prefix),
    ("Dhcpv6ReleasePrefixEx", stub::dhcpv6_release_prefix_ex),
    ("Dhcpv6RenewPrefix", stub::dhcpv6_renew_prefix),
    ("Dhcpv6RenewPrefixEx", stub::dhcpv6_renew_prefix_ex),
    ("Dhcpv6RequestParams", stub::dhcpv6_request_params),
    ("Dhcpv6RequestPrefix", stub::dhcpv6_request_prefix),
    ("Dhcpv6RequestPrefixEx", stub::dhcpv6_request_prefix_ex),
    ("Dhcpv6SetUserClass", stub::dhcpv6_set_user_class),
];

/// The `@ stub` record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, record)| record())
}
