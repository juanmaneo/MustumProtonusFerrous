//! Edge-case tests for the ported softpub logic.
//!
//! These cover the constants, the lookup of names that are absent or differ in case, the forward
//! target of a stub, and the stub argument pass-through. Not every expectation is oracle-derived;
//! the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h, the same values the
    // stub harness uses.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passes the three fixed names. The code, flags and
    // parameter count do not depend on the names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn stub_helpers_name_stub_exports() {
    // Not oracle-derived: each helper must pass the name of a `stub` line in the export table, and
    // the module must be the DLL file name from the Makefile.
    for (name, record) in [
        (
            "SoftpubFreeDefUsageCallData",
            softpub_free_def_usage_call_data(),
        ),
        (
            "SoftpubLoadDefUsageCallData",
            softpub_load_def_usage_call_data(),
        ),
        ("AddPersonalTrustDBPages", add_personal_trust_db_pages()),
    ] {
        assert_eq!(
            find_export(name).map(|export| export.kind),
            Some(ExportKind::Stub)
        );
        assert_eq!(record.function, name);
        assert_eq!(record.module, "softpub.dll");
    }
}

#[test]
fn lookup_is_case_sensitive_and_exact() {
    // Not oracle-derived: names are matched exactly, as in the spec. A miss is `None` for both
    // lookups.
    assert_eq!(find_export("softpubcleanup"), None);
    assert_eq!(find_export("SoftpubCleanup "), None);
    assert_eq!(find_export(""), None);
    assert_eq!(forward_target("softpubcleanup"), None);
    assert_eq!(forward_target(""), None);
}

#[test]
fn stubs_have_no_forward_target() {
    // Not oracle-derived: a `stub` line has no forward target, so `forward_target` returns `None`
    // even though `find_export` finds the name.
    assert!(find_export("SoftpubFreeDefUsageCallData").is_some());
    assert_eq!(forward_target("SoftpubFreeDefUsageCallData"), None);
    assert_eq!(forward_target("AddPersonalTrustDBPages"), None);
}
