//! Edge-case tests for the `icuin.spec` export table.
//!
//! The spec has no code to run, so these tests cover the shape of the table and the boundaries
//! of the lookups. Expectations that the oracle does not print say so in a comment.

use super::*;

#[test]
fn table_names_are_unique() {
    // Not oracle-derived: a duplicate name would hide the later entry from find_export. The
    // oracle rows are compared one by one, so they would not catch a duplicate on their own.
    for (i, export) in EXPORTS.iter().enumerate() {
        assert!(
            EXPORTS[i + 1..]
                .iter()
                .all(|other| other.name != export.name),
            "{} is exported twice",
            export.name
        );
    }
}

#[test]
fn lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: near misses of real names (prefix, suffix, case, trailing space).
    assert!(find_export("ucal_open").is_some());
    assert_eq!(find_export("ucal_ope"), None);
    assert_eq!(find_export("ucal_opens"), None);
    assert_eq!(find_export("UCAL_OPEN"), None);
    assert_eq!(find_export("ucal_open "), None);
}

#[test]
fn table_bounds_are_the_first_and_last_spec_lines() {
    // Not oracle-derived: pins the first and last lines of icuin.spec as boundary cases. The
    // oracle rows check the same entries.
    assert_eq!(
        EXPORTS[0],
        Export {
            name: "u_formatMessage",
            kind: ExportKind::Varargs,
            args: "str ptr long ptr long ptr",
        }
    );
    assert_eq!(EXPORTS[EXPORTS.len() - 1].name, "utrans_unregisterID");
    assert_eq!(
        forward("utrans_unregisterID"),
        Some(Forward {
            module: "icu",
            function: "utrans_unregisterID",
        })
    );
}

#[test]
fn forward_of_unexported_name_is_none() {
    // Not oracle-derived: names the spec does not export, including the forward target written
    // out as a name and the empty name.
    assert_eq!(forward("icu.ucal_open"), None);
    assert_eq!(forward(""), None);
    assert_eq!(forward("ucal_open2"), None);
}
