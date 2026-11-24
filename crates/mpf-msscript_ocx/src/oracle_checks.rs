//! Rows taken from the verbatim-Wine oracle at `target/canon/msscript.ocx/oracle.c`. Each comment
//! gives the oracle output line that a row was derived from.

use super::{
    enumerate::{self, Next, Skip},
    error_fill::PosFill,
    module_table, procedure, timeout, S_FALSE, S_OK,
};

const OUT_OF_RANGE: u32 = 0x800A_0009;
const INVALID_PROPERTY_VALUE: u32 = 0x800A_017C;

/// Models `fill_error_pos`: the position lookup runs only for a present object, and only on the first call.
fn fill_with(fill: &mut PosFill, has_object: bool, position: Option<(u32, i32)>) -> u32 {
    let mut calls = 0;
    fill.fill(|| {
        if !has_object {
            return None;
        }
        calls += 1;
        position
    });
    calls
}

#[test]
fn is_power_of_2_rows() {
    // oracle: is_power_of_2 x=0 -> 1, x=3 -> 0, x=4 -> 1
    assert!(module_table::needs_growth(0));
    assert!(!module_table::needs_growth(3));
    assert!(module_table::needs_growth(4));
}

#[test]
fn module_index_rows() {
    // oracle: module_index index=0/1/4/-1 count=3 -> hr 0x800a0009, 0x0, 0x800a0009, 0x800a0009
    assert_eq!(module_table::module_index(0, 3), Err(OUT_OF_RANGE));
    assert_eq!(module_table::module_index(1, 3), Ok(0));
    assert_eq!(module_table::module_index(4, 3), Err(OUT_OF_RANGE));
    assert_eq!(module_table::module_index(-1, 3), Err(OUT_OF_RANGE));
}

#[test]
fn module_next_rows() {
    // oracle: module_next total pos celt -> fetched pos hr; refs always equals fetched
    let rows = [
        ((5, 0, 2), (2, 2, S_OK)),
        ((5, 3, 4), (2, 5, S_FALSE)),
        ((5, 5, 1), (0, 5, S_FALSE)),
        ((3, 1, 0), (0, 1, S_OK)),
    ];
    for ((total, pos, celt), (fetched, new_pos, hr)) in rows {
        assert_eq!(
            enumerate::next(total, pos, celt),
            Next {
                fetched,
                pos: new_pos,
                hr
            }
        );
    }
}

#[test]
fn module_skip_rows() {
    // oracle: module_skip total pos celt -> pos hr
    let rows = [
        ((5, 0, 2), (2, S_OK)),
        ((5, 3, 4), (5, S_FALSE)),
        ((5, 5, 0), (5, S_OK)),
    ];
    for ((total, pos, celt), (new_pos, hr)) in rows {
        assert_eq!(enumerate::skip(total, pos, celt), Skip { pos: new_pos, hr });
    }
}

#[test]
fn procedure_skip_row() {
    // oracle: proc_skip count=7 pos=2 celt=5 -> pos=7 hr=S_OK (exact fit, no shortfall)
    assert_eq!(enumerate::skip(7, 2, 5), Skip { pos: 7, hr: S_OK });
}

#[test]
fn put_timeout_rows() {
    // oracle: put_timeout t -> hr timeout, starting from a zeroed control
    let rows = [
        (-2, Err(INVALID_PROPERTY_VALUE), 0),
        (-1, Ok(()), -1),
        (10000, Ok(()), 10000),
    ];
    for (input, expected, stored_after) in rows {
        let mut stored = 0;
        assert_eq!(timeout::set_timeout(&mut stored, input), expected);
        assert_eq!(stored, stored_after);
    }
}

#[test]
fn has_return_value_rows() {
    // oracle: has_return vt=24 -> out=0 (VARIANT_FALSE); vt=3 -> out=-1 (VARIANT_TRUE)
    assert!(!procedure::has_return_value(procedure::VT_VOID));
    assert!(procedure::has_return_value(3));
}

#[test]
fn fill_pos_rows() {
    // oracle: fill_pos filled object hr line col -> line column calls
    // filled=0 object=0: no lookup, values stay zero.
    let mut fill = PosFill::default();
    assert_eq!(fill_with(&mut fill, false, Some((7, 3))), 0);
    assert_eq!((fill.line(), fill.column()), (0, 0));

    // filled=0 object=1 hr=S_OK: one lookup, values latched.
    let mut fill = PosFill::default();
    assert_eq!(fill_with(&mut fill, true, Some((7, 3))), 1);
    assert_eq!((fill.line(), fill.column()), (7, 3));

    // filled=0 object=1 hr=E_FAIL: one lookup, values stay zero.
    let mut fill = PosFill::default();
    assert_eq!(fill_with(&mut fill, true, None), 1);
    assert_eq!((fill.line(), fill.column()), (0, 0));

    // filled=1 (already latched by a failed fill) object=1 hr=S_OK: no lookup, values stay zero.
    let mut fill = PosFill::default();
    fill_with(&mut fill, true, None);
    assert_eq!(fill_with(&mut fill, true, Some((7, 3))), 0);
    assert_eq!((fill.line(), fill.column()), (0, 0));
}
