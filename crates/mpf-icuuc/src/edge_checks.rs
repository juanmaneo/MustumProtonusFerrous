//! Edge-case tests for the `icuuc.spec` export table.
//!
//! The lookups are exact and case-sensitive, and the table keeps the spec's shape. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: near misses of `u_cleanup`, which the spec exports with no arguments.
    assert_eq!(find_export("u_cleanup").map(|export| export.args), Some(""));
    for name in ["U_CLEANUP", "u_cleanu", "u_cleanup ", "icu.u_cleanup", ""] {
        assert_eq!(find_export(name), None, "{name:?}");
    }
}

#[test]
fn names_outside_this_spec_do_not_forward() {
    // Not oracle-derived: `CreateFontPackage` is exported by fontsub.spec, not by icuuc.spec.
    assert_eq!(forward_target("CreateFontPackage"), None);
    assert_eq!(forward_target(""), None);
}

#[test]
fn first_and_last_entries_are_the_first_and_last_spec_lines() {
    // Oracle-derived: rows 1 and 542 of the C harness output. The length is the spec's `@` count.
    assert_eq!(EXPORTS[0].name, "UCNV_FROM_U_CALLBACK_ESCAPE");
    assert_eq!(EXPORTS[541].name, "utrace_vformat");
    assert_eq!(EXPORTS.len(), 542);
}

#[test]
fn varargs_is_the_only_utrace_format_line() {
    // Not oracle-derived: `grep -c '^@ varargs' icuuc.spec` is 1, and that line is utrace_format.
    let varargs: Vec<&Export> = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Varargs)
        .collect();
    assert_eq!(varargs.len(), 1);
    assert_eq!(varargs[0].name, "utrace_format");
    assert_eq!(varargs[0].args, "str long long str");
}

#[test]
fn zero_argument_exports_have_empty_argument_lists() {
    // Not oracle-derived: `grep -c '^@ .*()' icuuc.spec` is 13.
    let empty = EXPORTS
        .iter()
        .filter(|export| export.args.is_empty())
        .count();
    assert_eq!(empty, 13);
}

#[test]
fn names_are_unique_c_identifiers_and_find_themselves() {
    // Not oracle-derived: a duplicate name would make find_export ambiguous, and a name outside
    // [A-Za-z0-9_] would not be a C identifier. Both are checked for every entry.
    let mut seen = HashSet::new();
    for export in EXPORTS.iter() {
        assert!(seen.insert(export.name), "{} appears twice", export.name);
        assert!(
            export
                .name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "{} is not an identifier",
            export.name
        );
        assert_eq!(find_export(export.name), Some(export));
    }
    assert_eq!(seen.len(), EXPORTS.len());
}

#[test]
fn every_export_forwards_to_icu_under_its_own_name() {
    // Oracle-derived: the C harness exits unless each target is icu.<name>. This pins it for the
    // whole table through forward_target.
    assert_eq!(FORWARD_MODULE, "icu");
    for export in EXPORTS.iter() {
        assert_eq!(
            forward_target(export.name),
            Some((FORWARD_MODULE, export.name))
        );
    }
}
