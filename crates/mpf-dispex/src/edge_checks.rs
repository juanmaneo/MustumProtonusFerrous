//! Edge-case tests for the ported dispex logic.
//!
//! These cover flag boundaries, the 16-bit truncation, size wraparound, and every
//! branch of the callee and fill-in checks. Expectations marked "not oracle-derived"
//! come from reading `usrmarshal.c`, not from a captured row.

use super::*;

#[test]
fn proxy_flags_low_nibble_and_null_markers() {
    assert_eq!(proxy_flags(0x0f, true, true), 0x0f);
    // Bit 4 is outside `wFlags & 0xf`.
    assert_eq!(proxy_flags(0x10, true, true), 0);
    assert_eq!(proxy_flags(0, false, true), NULL_RESULT);
    assert_eq!(proxy_flags(0, true, false), NULL_EXCEPINFO);
    assert_eq!(proxy_flags(0, false, false), NULL_RESULT | NULL_EXCEPINFO);
}

#[test]
fn split_without_byref_keeps_order_and_ignores_alloc() {
    // Not oracle-derived: with no by-ref args the C code allocates nothing,
    // so a failing allocator must not matter.
    let split = split_args(&[3, 2, 0], false).unwrap();
    assert_eq!(split.copy_vts, vec![3, 2, 0]);
    assert!(split.ref_vts.is_empty());
    assert!(split.ref_idx.is_empty());
    assert_eq!(split_args(&[], false).unwrap().copy_vts, Vec::<u16>::new());
}

#[test]
fn split_with_byref_and_alloc_failure_is_none() {
    assert_eq!(split_args(&[VT_BYREF | 3], false), None);
}

#[test]
fn split_records_byref_slots_in_order() {
    let split = split_args(&[VT_BYREF | 3, 2, VT_BYREF, 5], true).unwrap();
    assert_eq!(split.copy_vts, vec![VT_EMPTY, 2, VT_EMPTY, 5]);
    assert_eq!(split.ref_vts, vec![VT_BYREF | 3, VT_BYREF]);
    assert_eq!(split.ref_idx, vec![0, 2]);
}

#[test]
fn copy_block_size_formula_and_wrap() {
    // Not oracle-derived: direct evaluation of the C expression.
    assert_eq!(copy_block_size(0, 0, 16, 4), 0);
    assert_eq!(copy_block_size(1, 1, 16, 4), 16 + 20);
    // 2^28 args of 16 bytes is 2^32; the DWORD truncation gives 0.
    assert_eq!(copy_block_size(0x1000_0000, 0, 16, 4), 0);
    // One extra byref adds VARIANT + UINT per slot.
    assert_eq!(copy_block_size(0, 1, 24, 4), 28);
}

#[test]
fn stub_flags_16_bit_truncation() {
    // Not oracle-derived: `dwFlags & 0xffff` is in the C source, and this is the
    // only value that would show it other than the oracle rows.
    let sf = stub_flags(0x0001_8000);
    assert_eq!(sf.invoke_flags, 0x8000);
    assert!(!sf.result_null);
    assert!(!sf.excep_null);
}

#[test]
fn stub_flags_each_null_bit_alone() {
    let sf = stub_flags(NULL_RESULT);
    assert!(sf.result_null && !sf.excep_null);
    let sf = stub_flags(NULL_EXCEPINFO);
    assert!(!sf.result_null && sf.excep_null);
}

#[test]
fn seed_and_clear_round_trip_with_no_refs() {
    // Not oracle-derived: an empty ref list leaves the copy array untouched.
    let split = ArgSplit {
        copy_vts: vec![3, 2],
        ref_vts: vec![],
        ref_idx: vec![],
    };
    let mut vts = seed_ref_args(&split);
    assert_eq!(vts, vec![3, 2]);
    clear_ref_slots(&mut vts, &split.ref_idx);
    assert_eq!(vts, vec![3, 2]);
}

#[test]
fn callee_result_branches() {
    let before = [3u16, 2];
    let changed = [2u16, 2];
    // Success with a by-ref arg and a changed VT: BADCALLEE.
    assert_eq!(callee_result(S_OK, &before, &changed, 1), DISP_E_BADCALLEE);
    // Success with no by-ref args: no check, even if the arrays differ.
    assert_eq!(callee_result(S_OK, &before, &changed, 0), S_OK);
    // Success with by-ref args and unchanged VTs: keeps hr.
    assert_eq!(callee_result(S_OK, &before, &before, 1), S_OK);
    // Failure: the check is skipped, so hr is kept (oracle P08 / P11 show the same).
    assert_eq!(
        callee_result(DISP_E_EXCEPTION, &before, &changed, 1),
        DISP_E_EXCEPTION
    );
}

#[test]
fn deferred_fill_in_truth_table() {
    // Not oracle-derived beyond P09 / P10: exhaustive over the three inputs.
    assert!(deferred_fill_in_runs(DISP_E_EXCEPTION, true, true));
    assert!(!deferred_fill_in_runs(DISP_E_EXCEPTION, false, true));
    assert!(!deferred_fill_in_runs(DISP_E_EXCEPTION, true, false));
    assert!(!deferred_fill_in_runs(S_OK, true, true));
    assert!(!deferred_fill_in_runs(E_OUTOFMEMORY, true, true));
}

#[test]
fn succeeded_sign_bit() {
    // Not oracle-derived: `SUCCEEDED` is the sign bit test.
    assert!(succeeded(S_OK));
    assert!(succeeded(0x0000_0001));
    assert!(!succeeded(DISP_E_EXCEPTION));
    assert!(!succeeded(E_OUTOFMEMORY));
}
