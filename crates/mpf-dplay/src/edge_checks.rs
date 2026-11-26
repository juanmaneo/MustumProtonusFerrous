//! Edge-case tests for the ported dplay logic.
//!
//! These cover the export lookup, the data-only rule at the dot boundary, and the version
//! constants. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the two lines of dlls/dplay/dplay.spec, in order. Rows 01 and 02 check
    // the same values through winebuild.
    assert_eq!(
        EXPORTS.map(|e| e.name),
        ["DirectPlayCreate", "DirectPlayEnumerate"]
    );
    assert_eq!(EXPORTS.map(|e| e.nb_args), [3, 2]);
    assert_eq!(
        EXPORTS.map(|e| e.forward),
        ["dplayx.DirectPlayCreate", "dplayx.DirectPlayEnumerate"]
    );
}

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: the lookup of the port. A name matches only when it equals the spec name.
    assert_eq!(export("DirectPlayCreate").map(|e| e.ordinal), Some(1));
    assert_eq!(export("DirectPlayEnumerate").map(|e| e.ordinal), Some(2));
    assert_eq!(export("directplaycreate"), None);
    assert_eq!(export("DirectPlayCreate "), None);
    assert_eq!(export(""), None);
    assert_eq!(export("DirectPlayUnknown"), None);
}

#[test]
fn data_only_treats_any_dot_as_a_forward() {
    // Not oracle-derived: the port follows the `strchr(link_name, '.')` test in
    // tools/winebuild/parser.c. A dot at either end counts, and one name without a dot rejects
    // the whole module.
    assert!(data_only_accepts(&["."]));
    assert!(data_only_accepts(&[".x"]));
    assert!(data_only_accepts(&["x."]));
    assert!(data_only_accepts(&["dplayx.A", "dplayx.B"]));
    assert!(!data_only_accepts(&["dplayx.A", "B"]));
    assert!(!data_only_accepts(&["dplayx"]));
}

#[test]
fn version_info_matches_makefile() {
    // Not oracle-derived: the VER_* values of dlls/dplay/Makefile.in. The oracle does not run
    // wrc, so the version resource itself is not checked.
    assert_eq!(
        version_info(),
        VersionInfo {
            file_description: "Wine DirectPlay",
            product_version: [5, 3, 0, 900],
            ole_self_register: true,
        }
    );
}
