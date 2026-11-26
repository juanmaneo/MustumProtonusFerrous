//! Edge-case tests for the ported dhcpcsvc6 stub records.
//!
//! These cover the constants, the export table against the spec, the lookup of names the spec
//! does not export, and the argument pass-through of the stub record. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the names are copied from dlls/dhcpcsvc6/dhcpcsvc6.spec, which has 22
    // `@ stub` lines and no other kind of entry.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "Dhcpv6AcquireParameters",
            "Dhcpv6CApiCleanup",
            "Dhcpv6CApiInitialize",
            "Dhcpv6CancelOperation",
            "Dhcpv6EnableDhcp",
            "Dhcpv6EnableTracing",
            "Dhcpv6FreeLeaseInfo",
            "Dhcpv6FreeLeaseInfoArray",
            "Dhcpv6GetTraceArray",
            "Dhcpv6GetUserClasses",
            "Dhcpv6IsEnabled",
            "Dhcpv6QueryLeaseInfo",
            "Dhcpv6QueryLeaseInfoArray",
            "Dhcpv6ReleaseParameters",
            "Dhcpv6ReleasePrefix",
            "Dhcpv6ReleasePrefixEx",
            "Dhcpv6RenewPrefix",
            "Dhcpv6RenewPrefixEx",
            "Dhcpv6RequestParams",
            "Dhcpv6RequestPrefix",
            "Dhcpv6RequestPrefixEx",
            "Dhcpv6SetUserClass",
        ]
    );
}

#[test]
fn every_helper_raises_for_its_own_export_name() {
    for (name, record) in EXPORTS {
        assert_eq!(
            record(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "dhcpcsvc6.dll",
                function: name,
            }
        );
    }
}

#[test]
fn unknown_names_have_no_stub_record() {
    // Not oracle-derived: the harness only ran names the spec exports. Names are case-sensitive,
    // so the lowercase and trailing-space forms are misses, as they are in the spec.
    for name in [
        "",
        "dhcpv6acquireparameters",
        "Dhcpv6CApiCleanup ",
        "Dhcpv6Unknown",
        "1",
    ] {
        assert_eq!(export_stub(name), None);
    }
}

#[test]
fn stub_record_passes_names_through_unchanged() {
    // Not oracle-derived: the harness only passed export names. This checks that the port keeps
    // any module and function strings as given, as the C pointers do.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "",
        }
    );
}
