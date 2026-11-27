//! Oracle-derived vectors for the ported `mshtml.tlb` enum constants.
//!
//! The rows were captured by copying the twelve `typedef enum` blocks of `include/mshtml.idl`
//! verbatim into `target/canon/mshtml.tlb/mshtml_enums.h`, compiling `harness.c` with
//! `gcc -fsanitize=address,undefined -g`, and running it. The harness prints the value gcc
//! evaluates for each of the 288 enumerators. ASan and UBSan reported nothing, so no vector was
//! excluded. The rows below are a selection of that printed output, chosen for the first and
//! last members of each table, the explicit values, and the `_Max` sentinels.

use super::*;

/// `(enum table, member name, value)` rows printed by the oracle harness.
const ORACLE_ROWS: &[(&[Member], &str, i32)] = &[
    (POINTER_GRAVITY, "POINTER_GRAVITY_Left", 0),
    (POINTER_GRAVITY, "POINTER_GRAVITY_Max", 2147483647),
    (MARKUP_CONTEXT_TYPE, "CONTEXT_TYPE_EnterScope", 2),
    (ELEMENT_ADJACENCY, "ELEM_ADJ_AfterEnd", 3),
    (MOVEUNIT_ACTION, "MOVEUNIT_PREVCHAR", 0),
    (MOVEUNIT_ACTION, "MOVEUNIT_NEXTBLOCK", 19),
    (MOVEUNIT_ACTION, "MOVEUNIT_ACTION_Max", 2147483647),
    (SELECTION_TYPE, "SELECTION_TYPE_Control", 3),
    (ELEMENT_TAG_ID, "TAGID_NULL", 0),
    (ELEMENT_TAG_ID, "TAGID_A", 2),
    (ELEMENT_TAG_ID, "TAGID_LAST_PREDEFINED", 10000),
    (ELEMENT_TAG_ID, "ELEMENT_TAG_ID_Max", 2147483647),
    (COORD_SYSTEM, "COORD_SYSTEM_FRAME", 4),
    (DISPLAY_MOVEUNIT, "DISPLAY_MOVEUNIT_PreviousLine", 1),
    (DISPLAY_MOVEUNIT, "DISPLAY_MOVEUNIT_NextLine", 2),
    (DISPLAY_GRAVITY, "DISPLAY_GRAVITY_NextLine", 2),
    (CARET_DIRECTION, "CARET_DIRECTION_FORWARD", 3),
    (DomConstructor, "DomConstructorObject", 0),
    (DomConstructor, "DomConstructorXMLHttpRequest", 106),
    (DomConstructor, "DomConstructorMax", 107),
    (DomConstructor, "DomConstructor_Max", 2147483647),
    (DOM_EVENT_PHASE, "DEP_CAPTURING_PHASE", 1),
    (DOM_EVENT_PHASE, "DEP_AT_TARGET", 2),
    (DOM_EVENT_PHASE, "DEP_BUBBLING_PHASE", 3),
    (DOM_EVENT_PHASE, "DOM_EVENT_PHASE_Max", 2147483647),
];

#[test]
fn oracle_enum_rows() {
    for (table, name, value) in ORACLE_ROWS {
        assert_eq!(member_value(table, name), Some(*value), "{name}");
    }
}
