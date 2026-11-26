//! Edge-case tests for the ported `msvcp140_codecvt_ids` data exports.
//!
//! These cover the hit and miss branches of the lookup and the whole-name match rule. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn export_by_name_returns_each_export() {
    // Not oracle-derived: the lookup itself. Each mangled name maps back to its own table entry.
    for export in EXPORTS.iter() {
        assert_eq!(export_by_name(export.name), Some(export));
    }
}

#[test]
fn export_by_name_rejects_names_that_are_not_exact() {
    // Not oracle-derived: boundary cases for the whole-name match. The C symbol is not an export
    // name. A prefix, a suffix, a leading space or a case change of a mangled name is not an
    // export either.
    let name = EXPORTS[0].name;
    assert_eq!(export_by_name(""), None);
    assert_eq!(export_by_name("codecvt_char16_id"), None);
    assert_eq!(export_by_name(&name[..name.len() - 1]), None);
    assert_eq!(export_by_name(&format!("{name}A")), None);
    assert_eq!(export_by_name(&format!(" {name}")), None);
    assert_eq!(export_by_name(&name.to_ascii_lowercase()), None);
}
