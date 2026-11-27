//! Edge-case tests for the ported srvcli export table and stub records.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::collections::BTreeSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_names_are_unique_and_ordinals_follow_spec_order() {
    // Not oracle-derived: winebuild numbers the exports from 1 in spec order. The oracle tests
    // check each ordinal against its row.
    let names: BTreeSet<&str> = EXPORTS.iter().map(|export| export.name).collect();
    assert_eq!(names.len(), EXPORTS.len());
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1);
    }
}

#[test]
fn export_by_name_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: the loader matches export names exactly, so a different case, a
    // trailing space and the empty name find nothing.
    assert_eq!(
        export_by_name("NetFileEnum").map(|export| export.ordinal),
        Some(28)
    );
    assert_eq!(export_by_name("netfileenum"), None);
    assert_eq!(export_by_name("NetFileEnum "), None);
    assert_eq!(export_by_name(""), None);
}

#[test]
fn forward_target_splits_only_forwards() {
    // Not oracle-derived: the stub and the unknown-name branches of `forward_target` return None.
    assert_eq!(
        forward_target("NetShareDel"),
        Some(("netapi32", "NetShareDel"))
    );
    assert_eq!(forward_target("NetSessionGetInfo"), None);
    assert_eq!(forward_target("NoSuchExport"), None);
}

#[test]
fn stub_helpers_pass_their_names_through() {
    // Not oracle-derived: the record copies the module and function it is given, so a name that
    // is not in the table comes out unchanged.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "NoSuchExport"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "other.dll",
            function: "NoSuchExport",
        }
    );
}
