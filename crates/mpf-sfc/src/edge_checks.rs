//! Edge-case tests for the `sfc.spec` export table.
//!
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

#[test]
fn ordinals_are_one_to_sixteen_each_used_once() {
    // Not oracle-derived: a property of the table. winebuild rejects a redefined ordinal, and the
    // oracle rows show ordinals 1 to 16.
    let mut ordinals: Vec<u16> = EXPORTS.iter().map(|e| e.ordinal).collect();
    ordinals.sort_unstable();
    assert_eq!(ordinals, (1..=16).collect::<Vec<u16>>());
}

#[test]
fn nameless_entries_are_the_first_nine_ordinals() {
    // Not oracle-derived: restates the oracle rows (`@` in the spec) as a property.
    let nameless: Vec<u16> = EXPORTS
        .iter()
        .filter(|e| e.name.is_none())
        .map(|e| e.ordinal)
        .collect();
    assert_eq!(nameless, (1..=9).collect::<Vec<u16>>());
}

#[test]
fn export_by_name_is_case_sensitive_and_skips_nameless_entries() {
    // Not oracle-derived: lookup semantics. Names are compared exactly, as GetProcAddress does.
    assert_eq!(export_by_name("SfpVerifyFile"), Some(EXPORTS[15]));
    assert_eq!(export_by_name("SRSetRestorePointA"), Some(EXPORTS[10]));
    assert_eq!(export_by_name("sfpverifyfile"), None);
    // A forward target is not an export name.
    assert_eq!(export_by_name("SfcInitProt"), None);
    // Nameless entries have no name, so the empty name matches nothing.
    assert_eq!(export_by_name(""), None);
}

#[test]
fn export_by_ordinal_finds_every_entry_and_nothing_else() {
    // Not oracle-derived: lookup semantics around the range 1 to 16.
    for export in EXPORTS {
        assert_eq!(export_by_ordinal(export.ordinal), Some(export));
    }
    assert_eq!(export_by_ordinal(0), None);
    assert_eq!(export_by_ordinal(17), None);
    assert_eq!(export_by_ordinal(u16::MAX), None);
}

#[test]
fn forward_target_splits_at_the_last_dot() {
    // Not oracle-derived: `find_forwarded_export` in dlls/ntdll/loader.c uses strrchr, so the
    // module ends at the last dot. The oracle forwards have one dot each.
    let with_link = |link_name: &'static str| Export {
        line: 0,
        ordinal: 0,
        name: None,
        link_name,
    };
    assert_eq!(EXPORTS[0].forward_target(), Some(("sfc_os", "SfcInitProt")));
    assert_eq!(with_link("a.b.c").forward_target(), Some(("a.b", "c")));
    assert_eq!(with_link(".f").forward_target(), Some(("", "f")));
    assert_eq!(with_link("m.").forward_target(), Some(("m", "")));
    assert_eq!(with_link("NoDot").forward_target(), None);
}

#[test]
fn name_table_is_sorted_and_holds_only_named_entries() {
    // Not oracle-derived: restates winebuild's strcmp sort as a property. The oracle hint rows
    // check the same order.
    let table = name_table();
    assert_eq!(table.len(), 7);
    assert!(table.iter().all(|e| e.name.is_some()));
    assert!(table.windows(2).all(|w| w[0].name < w[1].name));
}
