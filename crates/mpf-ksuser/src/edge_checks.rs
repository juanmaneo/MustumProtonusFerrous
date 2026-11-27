//! Edge-case tests for the ported ksuser stub logic.
//!
//! These cover the constants, the export table and its lookup, and the stub record for names the
//! harness did not print. Not every expectation is oracle-derived; the ones that aren't say so in
//! a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the four lines of dlls/ksuser/ksuser.spec, in order.
    let names = EXPORTS.map(|(name, _)| name);
    assert_eq!(
        names,
        [
            "KsCreateAllocator",
            "KsCreateClock",
            "KsCreatePin",
            "KsCreateTopologyNode",
        ]
    );
}

#[test]
fn every_export_maps_to_its_own_stub() {
    // Not oracle-derived: the pairing of each name with its helper. The records themselves are
    // checked against the harness in oracle_checks.rs.
    for (name, helper) in EXPORTS {
        assert_eq!(stub_for_export(name), Some(helper()));
        assert_eq!(helper().module, "ksuser.dll");
        assert_eq!(helper().function, name);
    }
}

#[test]
fn stub_for_export_rejects_names_outside_the_spec() {
    // Not oracle-derived: names that ksuser.spec does not export. Names are case-sensitive, and
    // padding or a missing name gives no record.
    for name in ["kscreateclock", "KsCreateClock ", "KsCreateFilter", ""] {
        assert_eq!(stub_for_export(name), None, "name={name:?}");
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the code, flags and parameter count do not depend on the names. The
    // oracle covers the module names "ksuser.dll", "fontsub.dll" and "", but not "other.dll".
    let other = wine_spec_unimplemented_stub("other.dll", "KsCreateTopologyNodeX");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "KsCreateTopologyNodeX");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);
}
