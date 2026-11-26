//! Edge-case tests for the ported olepro32 logic.
//!
//! These cover the forward table's shape, the lookup boundaries and the `HRESULT` shim. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(std::mem::size_of::<HRESULT>(), 4);
}

#[test]
fn forward_table_is_seven_consecutive_ordinals_to_oleaut32() {
    // Not oracle-derived: the shape is read from spec lines 248 to 254. The oracle checks each row.
    assert_eq!(FORWARDS.len(), 7);
    for (i, f) in FORWARDS.iter().enumerate() {
        assert_eq!(usize::from(f.ordinal), 248 + i);
        assert_eq!(f.target_dll, "oleaut32");
        assert_eq!(f.target_name, f.name);
        assert_eq!(f.stack_bytes % 4, 0);
    }
}

#[test]
fn forward_for_finds_every_table_entry() {
    // Not oracle-derived: a round trip over the table.
    for f in FORWARDS.iter() {
        assert_eq!(forward_for(f.name), Some(*f));
    }
}

#[test]
fn forward_for_matches_the_whole_name_only() {
    // Not oracle-derived: boundaries around the oracle's lookups. Trailing, leading, prefix,
    // suffix, case and dotted-target variants are not forwards.
    assert_eq!(forward_for("OleIconToCursor "), None);
    assert_eq!(forward_for(" OleIconToCursor"), None);
    assert_eq!(forward_for("OleIconToCurso"), None);
    assert_eq!(forward_for("OleIconToCursor2"), None);
    assert_eq!(forward_for("oleiconTocursor"), None);
    assert_eq!(forward_for("oleaut32.OleIconToCursor"), None);
}

#[test]
fn forward_is_copy() {
    // Not oracle-derived: the value semantics that `forward_for` relies on.
    let first = FORWARDS[0];
    let copy = first;
    assert_eq!(copy, first);
}
