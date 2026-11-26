//! Edge-case tests for the ported usp10 logic.
//!
//! These cover empty and short inputs, the branches of both reorder
//! recursions, the legacy `+`/`-` rule, and the table walks at their
//! boundaries. Expectations are labelled by origin. "C-checked" values came
//! from a second run of the same verbatim C build under `target/canon/usp10/`,
//! not from the 25 counted rows. "Hand-derived" values were worked out from
//! the C source. "Not oracle-derived" checks are Rust-only properties.

use super::*;
use crate::direction::bidi_class;

fn strength_of(ch: u16, legacy: bool) -> u16 {
    let mut out = [0u16];
    assert!(bidi_get_strengths(&[ch], 1, legacy, &mut out));
    out[0]
}

fn reorder_v2l(level: i32, f_reverse: bool, plevel: &[u8]) -> (usize, Vec<i32>) {
    let mut indices: Vec<i32> = (0..plevel.len()).map(|i| i as i32).collect();
    let consumed = bidi_reorder_v2l_level(level, &mut indices, plevel, plevel.len(), f_reverse);
    (consumed, indices)
}

fn reorder_l2v(level: i32, f_reverse: bool, plevel: &[u8]) -> (usize, Vec<i32>) {
    let mut indices: Vec<i32> = (0..plevel.len()).map(|i| i as i32).collect();
    let consumed = bidi_reorder_l2v_level(level, &mut indices, plevel, plevel.len(), f_reverse);
    (consumed, indices)
}

#[test]
fn reorder_v2l_empty_input_consumes_nothing() {
    // C-checked: `ROW v2l level=0 rev=1 lv=[] -> ret=0 idx=[]`.
    assert_eq!(reorder_v2l(0, true, &[]), (0, vec![]));
}

#[test]
fn reorder_v2l_only_touches_the_first_cch_indices() {
    // Hand-derived from the C: level 1 is odd, so the two consumed entries are
    // reversed and the third is left alone.
    let mut indices = [7, 8, 9];
    let consumed = bidi_reorder_v2l_level(1, &mut indices, &[1, 1], 2, false);
    assert_eq!(consumed, 2);
    assert_eq!(indices, [8, 7, 9]);
}

#[test]
fn reorder_v2l_nested_even_levels_reverse_twice() {
    // C-checked: `ROW v2l level=0 rev=0 lv=[2,2] -> ret=2 idx=[0,1]`.
    assert_eq!(reorder_v2l(0, false, &[2, 2]), (2, vec![0, 1]));
}

#[test]
fn reorder_v2l_flag_forces_reversal_on_even_level() {
    // C-checked: `ROW v2l level=0 rev=1 lv=[0,0,0] -> ret=3 idx=[2,1,0]`.
    assert_eq!(reorder_v2l(0, true, &[0, 0, 0]), (3, vec![2, 1, 0]));
}

#[test]
fn reorder_v2l_break_inside_a_nested_level() {
    // C-checked: `ROW v2l level=0 rev=0 lv=[1,0,1] -> ret=3 idx=[0,1,2]`.
    assert_eq!(reorder_v2l(0, false, &[1, 0, 1]), (3, vec![0, 1, 2]));
}

#[test]
fn reorder_l2v_without_higher_levels_skips_the_second_pass() {
    // C-checked: `ROW l2v level=0 rev=0 lv=[0,0,0] -> ret=3 idx=[0,1,2]`.
    assert_eq!(reorder_l2v(0, false, &[0, 0, 0]), (3, vec![0, 1, 2]));
}

#[test]
fn reorder_l2v_odd_level_reverses_without_the_flag() {
    // C-checked: `ROW l2v level=1 rev=0 lv=[1,1] -> ret=2 idx=[1,0]`.
    assert_eq!(reorder_l2v(1, false, &[1, 1]), (2, vec![1, 0]));
}

#[test]
fn reorder_l2v_nested_second_pass() {
    // C-checked: `ROW l2v level=0 rev=0 lv=[0,2,2,0] -> ret=4 idx=[0,1,2,3]`.
    assert_eq!(reorder_l2v(0, false, &[0, 2, 2, 0]), (4, vec![0, 1, 2, 3]));
}

#[test]
fn strengths_write_only_the_first_count_entries() {
    // Not oracle-derived: checks the `count` bound on the output slice.
    let mut out = [9u16, 9];
    assert!(bidi_get_strengths(&[0x0041, 0x0031], 1, false, &mut out));
    assert_eq!(out, [BIDI_STRONG, 9]);
}

#[test]
fn legacy_flag_only_neutralises_plus_and_minus() {
    // C-checked: `+` and `-` with legacy=1 (U+002B, U+002D).
    assert_eq!(strength_of(0x002B, true), BIDI_NEUTRAL);
    assert_eq!(strength_of(0x002D, true), BIDI_NEUTRAL);
    assert_eq!(strength_of(0x002D, false), BIDI_WEAK);
    // C-checked: other European separators and common separators keep their class under the flag.
    assert_eq!(strength_of(0x2212, true), BIDI_WEAK);
    assert_eq!(strength_of(0x002C, true), BIDI_WEAK);
}

#[test]
fn strengths_cover_every_class_group() {
    // C-checked (edge run, legacy=0 and legacy=1 agree for these code units).
    let cases: [(u16, u16); 16] = [
        (0x2029, BIDI_NEUTRAL), // B
        (0x0009, BIDI_NEUTRAL), // S
        (0x202A, BIDI_STRONG),  // LRE
        (0x202D, BIDI_STRONG),  // LRO
        (0x202E, BIDI_STRONG),  // RLO
        (0x0300, BIDI_NEUTRAL), // NSM, via the default arm
        (0x0627, BIDI_STRONG),  // AL
        (0x0021, BIDI_NEUTRAL), // ON
        (0x0660, BIDI_WEAK),    // AN
        (0x00AD, BIDI_WEAK),    // BN
        (0x0024, BIDI_WEAK),    // ET
        (0x0000, BIDI_WEAK),    // table gives BN
        (0xFFFF, BIDI_STRONG),  // table gives L
        (0x002C, BIDI_WEAK),    // CS
        (0x0031, BIDI_WEAK),    // EN
        (0x0020, BIDI_NEUTRAL), // WS
    ];
    for (ch, expected) in cases {
        assert_eq!(strength_of(ch, false), expected, "ch={ch:#06x}");
        assert_eq!(strength_of(ch, true), expected, "ch={ch:#06x} legacy");
    }
}

#[test]
fn mirror_char_maps_pairs_and_keeps_unmapped_units() {
    // C-checked: `ROW mirror ch=U+0000 -> U+0000`, `ROW mirror ch=U+FFFF -> U+FFFF`.
    assert_eq!(mirror_char(0x0000), 0x0000);
    assert_eq!(mirror_char(0xFFFF), 0xFFFF);
    // C-checked: the closing brackets and the other pairs.
    assert_eq!(mirror_char(0x0029), 0x0028);
    assert_eq!(mirror_char(0x003C), 0x003E);
    assert_eq!(mirror_char(0x00AB), 0x00BB);
    assert_eq!(mirror_char(0x0F3A), 0x0F3B);
}

#[test]
fn class_and_mirror_lookups_stay_in_bounds_for_every_code_unit() {
    // Not oracle-derived: the table walks must not index out of range. The C
    // edge run gives the largest class as 22 (PDI), so that is the bound used.
    for ch in 0..=u16::MAX {
        assert!(bidi_class(ch) <= 22, "ch={ch:#06x}");
        let _ = mirror_char(ch);
        assert!(strength_of(ch, true) <= BIDI_WEAK);
    }
}
