//! Oracle-derived test vectors for the ported usp10 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/gdi32/uniscribe/{bidi,direction,mirror}.c`, `usp10.c`,
//! `usp10_internal.h` and `include/usp10.h` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/usp10/`. The
//! harness printed 25 rows. ASan and UBSan reported nothing, so none are
//! excluded. Each test quotes its `ROW` line from that run.

use super::*;
use crate::bidi::{EN, ES, ET, L, PDF, R, WS};
use crate::direction::bidi_class;

/// Runs `BIDI_ReorderV2lLevel` over the whole of `plevel`, starting from the identity order.
fn reorder_v2l(level: i32, f_reverse: bool, plevel: &[u8]) -> (usize, Vec<i32>) {
    let mut indices: Vec<i32> = (0..plevel.len()).map(|i| i as i32).collect();
    let consumed = bidi_reorder_v2l_level(level, &mut indices, plevel, plevel.len(), f_reverse);
    (consumed, indices)
}

/// Runs `BIDI_ReorderL2vLevel` over the whole of `plevel`, starting from the identity order.
fn reorder_l2v(level: i32, f_reverse: bool, plevel: &[u8]) -> (usize, Vec<i32>) {
    let mut indices: Vec<i32> = (0..plevel.len()).map(|i| i as i32).collect();
    let consumed = bidi_reorder_l2v_level(level, &mut indices, plevel, plevel.len(), f_reverse);
    (consumed, indices)
}

/// `BIDI_GetStrengths` on one code unit, with `legacy` as `fLegacyBidiClass`.
fn strength_of(ch: u16, legacy: bool) -> u16 {
    let mut out = [0u16];
    assert!(bidi_get_strengths(&[ch], 1, legacy, &mut out));
    out[0]
}

#[test]
fn oracle_reorder_v2l_rows() {
    // ROW v2l level=0 rev=0 lv=[0,0,0] -> ret=3 idx=[0,1,2]
    assert_eq!(reorder_v2l(0, false, &[0, 0, 0]), (3, vec![0, 1, 2]));
    // ROW v2l level=0 rev=0 lv=[1,1,1] -> ret=3 idx=[2,1,0]
    assert_eq!(reorder_v2l(0, false, &[1, 1, 1]), (3, vec![2, 1, 0]));
    // ROW v2l level=0 rev=0 lv=[0,1,1,0] -> ret=4 idx=[0,2,1,3]
    assert_eq!(reorder_v2l(0, false, &[0, 1, 1, 0]), (4, vec![0, 2, 1, 3]));
    // ROW v2l level=0 rev=0 lv=[1,2,2,1] -> ret=4 idx=[3,1,2,0]
    assert_eq!(reorder_v2l(0, false, &[1, 2, 2, 1]), (4, vec![3, 1, 2, 0]));
    // ROW v2l level=0 rev=0 lv=[2,2,1,3] -> ret=4 idx=[3,2,0,1]
    assert_eq!(reorder_v2l(0, false, &[2, 2, 1, 3]), (4, vec![3, 2, 0, 1]));
    // ROW v2l level=1 rev=0 lv=[1,1,0,1] -> ret=2 idx=[1,0,2,3]
    assert_eq!(reorder_v2l(1, false, &[1, 1, 0, 1]), (2, vec![1, 0, 2, 3]));
    // ROW v2l level=0 rev=1 lv=[0,0,2] -> ret=3 idx=[2,1,0]
    assert_eq!(reorder_v2l(0, true, &[0, 0, 2]), (3, vec![2, 1, 0]));
    // ROW v2l level=0 rev=0 lv=[1,2,3,2,1] -> ret=5 idx=[4,1,2,3,0]
    assert_eq!(
        reorder_v2l(0, false, &[1, 2, 3, 2, 1]),
        (5, vec![4, 1, 2, 3, 0])
    );
    // ROW v2l level=0 rev=0 lv=[] -> ret=0 idx=[]
    assert_eq!(reorder_v2l(0, false, &[]), (0, vec![]));
}

#[test]
fn oracle_reorder_l2v_rows() {
    // ROW l2v level=1 rev=0 lv=[1,1,1] -> ret=3 idx=[2,1,0]
    assert_eq!(reorder_l2v(1, false, &[1, 1, 1]), (3, vec![2, 1, 0]));
    // ROW l2v level=0 rev=0 lv=[2,2,1,1] -> ret=4 idx=[2,3,1,0]
    assert_eq!(reorder_l2v(0, false, &[2, 2, 1, 1]), (4, vec![2, 3, 1, 0]));
    // ROW l2v level=0 rev=0 lv=[0,1,1,0] -> ret=4 idx=[0,2,1,3]
    assert_eq!(reorder_l2v(0, false, &[0, 1, 1, 0]), (4, vec![0, 2, 1, 3]));
}

#[test]
fn oracle_strength_rows() {
    // ROW strength ch=U+0041 legacy=0 -> class=1 ok=1 strength=1
    assert_eq!(bidi_class(0x0041), L);
    assert_eq!(strength_of(0x0041, false), BIDI_STRONG);
    // ROW strength ch=U+0031 legacy=0 -> class=4 ok=1 strength=2
    assert_eq!(bidi_class(0x0031), EN);
    assert_eq!(strength_of(0x0031, false), BIDI_WEAK);
    // ROW strength ch=U+0020 legacy=0 -> class=12 ok=1 strength=0
    assert_eq!(bidi_class(0x0020), WS);
    assert_eq!(strength_of(0x0020, false), BIDI_NEUTRAL);
    // ROW strength ch=U+002B legacy=0 -> class=8 ok=1 strength=2
    assert_eq!(bidi_class(0x002B), ES);
    assert_eq!(strength_of(0x002B, false), BIDI_WEAK);
    // ROW strength ch=U+002B legacy=1 -> class=0 ok=1 strength=0
    assert_eq!(strength_of(0x002B, true), BIDI_NEUTRAL);
    // ROW strength ch=U+05D0 legacy=0 -> class=2 ok=1 strength=1
    assert_eq!(bidi_class(0x05D0), R);
    assert_eq!(strength_of(0x05D0, false), BIDI_STRONG);
    // ROW strength ch=U+0300 legacy=0 -> class=6 ok=1 strength=0 (class 6 is NSM)
    assert_eq!(bidi_class(0x0300), 6);
    assert_eq!(strength_of(0x0300, false), BIDI_NEUTRAL);
    // ROW strength ch=U+202C legacy=0 -> class=18 ok=1 strength=2
    assert_eq!(bidi_class(0x202C), PDF);
    assert_eq!(strength_of(0x202C, false), BIDI_WEAK);
    // ROW strength ch=U+0024 legacy=0 -> class=9 ok=1 strength=2
    assert_eq!(bidi_class(0x0024), ET);
    assert_eq!(strength_of(0x0024, false), BIDI_WEAK);
}

#[test]
fn oracle_mirror_rows() {
    // ROW mirror ch=U+0028 -> U+0029
    assert_eq!(mirror_char(0x0028), 0x0029);
    // ROW mirror ch=U+005B -> U+005D
    assert_eq!(mirror_char(0x005B), 0x005D);
    // ROW mirror ch=U+2039 -> U+203A
    assert_eq!(mirror_char(0x2039), 0x203A);
    // ROW mirror ch=U+0041 -> U+0041
    assert_eq!(mirror_char(0x0041), 0x0041);
}
