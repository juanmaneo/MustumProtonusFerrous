//! Edge-case tests for the ported `mshtml.tlb` enum constants.
//!
//! These cover the found and missing branches of the two lookups, the first-match rule, the C
//! numbering of enumerators without `=`, the `_Max` sentinels and the member counts. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn member_value_finds_by_exact_name_and_misses_others() {
    // Oracle-derived: POINTER_GRAVITY_Max is oracle row 02 and TAGID_A is row 10. The misses
    // are not oracle-derived: they cover the missing branch of `member_value`.
    assert_eq!(
        member_value(POINTER_GRAVITY, "POINTER_GRAVITY_Max"),
        Some(2_147_483_647)
    );
    assert_eq!(member_value(ELEMENT_TAG_ID, "TAGID_A"), Some(2));
    assert_eq!(member_value(POINTER_GRAVITY, "POINTER_GRAVITY_Nope"), None);
    assert_eq!(member_value(POINTER_GRAVITY, "pointer_gravity_left"), None);
    assert_eq!(member_value(POINTER_GRAVITY, "POINTER_GRAVITY"), None);
    assert_eq!(member_value(&[], "POINTER_GRAVITY_Left"), None);
}

#[test]
fn member_name_returns_the_first_member_with_a_shared_value() {
    // Not oracle-derived: no table in include/mshtml.idl shares a value, so this uses a
    // synthetic table to test the first-match rule of `member_name`.
    const SHARED: &[Member] = &[
        Member {
            name: "FIRST",
            value: 7,
        },
        Member {
            name: "SECOND",
            value: 7,
        },
    ];
    assert_eq!(member_name(SHARED, 7), Some("FIRST"));
}

#[test]
fn member_name_finds_by_value_and_misses_others() {
    // Oracle-derived: DISPLAY_MOVEUNIT_NextLine is oracle row 15. The misses are not
    // oracle-derived: value 3 is not a member of POINTER_GRAVITY, which has 0, 1 and i32::MAX.
    assert_eq!(
        member_name(DISPLAY_MOVEUNIT, 2),
        Some("DISPLAY_MOVEUNIT_NextLine")
    );
    assert_eq!(member_name(POINTER_GRAVITY, 3), None);
    assert_eq!(member_name(POINTER_GRAVITY, -1), None);
    assert_eq!(member_name(&[], 0), None);
}

#[test]
fn implicit_enumerators_count_up_from_their_first_value() {
    // Not oracle-derived: C numbers enumerators without `=` one above the previous value, from
    // 0, or from the explicit `= 1` that starts DISPLAY_MOVEUNIT and DISPLAY_GRAVITY. The last
    // member of each table is its `_Max` sentinel and is skipped here.
    let tables = [
        (POINTER_GRAVITY, 0),
        (MARKUP_CONTEXT_TYPE, 0),
        (ELEMENT_ADJACENCY, 0),
        (MOVEUNIT_ACTION, 0),
        (SELECTION_TYPE, 0),
        (COORD_SYSTEM, 0),
        (DISPLAY_MOVEUNIT, 1),
        (DISPLAY_GRAVITY, 1),
        (CARET_DIRECTION, 0),
        (DomConstructor, 0),
    ];
    for (table, first) in tables {
        let implicit = &table[..table.len() - 1];
        for (index, member) in implicit.iter().enumerate() {
            assert_eq!(member.value, first + index as i32, "{}", member.name);
        }
    }
}

#[test]
fn every_table_ends_with_a_max_sentinel() {
    // Not oracle-derived: each `typedef enum` in include/mshtml.idl ends with a `_Max` member
    // set to 2147483647 (0x7fffffff), which the oracle printed for each table.
    let tables = [
        POINTER_GRAVITY,
        MARKUP_CONTEXT_TYPE,
        ELEMENT_ADJACENCY,
        MOVEUNIT_ACTION,
        SELECTION_TYPE,
        ELEMENT_TAG_ID,
        COORD_SYSTEM,
        DISPLAY_MOVEUNIT,
        DISPLAY_GRAVITY,
        CARET_DIRECTION,
        DomConstructor,
        DOM_EVENT_PHASE,
    ];
    for table in tables {
        let last = table.last().expect("every table has members");
        assert!(last.name.ends_with("_Max"), "{}", last.name);
        assert_eq!(last.value, i32::MAX, "{}", last.name);
    }
}

#[test]
fn member_counts_match_the_idl() {
    // Not oracle-derived: the member counts of the twelve `typedef enum` blocks in
    // include/mshtml.idl, counting the `_Max` sentinels too.
    let counts = [
        (POINTER_GRAVITY, 3),
        (MARKUP_CONTEXT_TYPE, 6),
        (ELEMENT_ADJACENCY, 5),
        (MOVEUNIT_ACTION, 21),
        (SELECTION_TYPE, 5),
        (ELEMENT_TAG_ID, 114),
        (COORD_SYSTEM, 6),
        (DISPLAY_MOVEUNIT, 7),
        (DISPLAY_GRAVITY, 3),
        (CARET_DIRECTION, 5),
        (DomConstructor, 109),
        (DOM_EVENT_PHASE, 4),
    ];
    for (table, count) in counts {
        assert_eq!(table.len(), count);
    }
}
