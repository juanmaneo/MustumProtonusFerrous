//! Edge-case tests for the ported actxprxy logic.
//!
//! These cover the NULL `pceltFetched` substitution, the `S_OK`-only count rule, and the
//! `RefCount` boundary. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

fn writes_celt_and_ok(celt: u32, celt_fetched: &mut u32) -> Hresult {
    *celt_fetched = celt;
    S_OK
}

fn writes_nothing_and_fails(_celt: u32, _celt_fetched: &mut u32) -> Hresult {
    -2147467259
}

fn returns_slot_value(_celt: u32, celt_fetched: &mut u32) -> Hresult {
    *celt_fetched as Hresult
}

fn returns_success_code_two(_celt: u32, celt_fetched: &mut u32) -> Hresult {
    *celt_fetched = 4;
    2
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
}

#[test]
fn proxy_with_null_slot_gives_callee_a_zeroed_local() {
    // Not oracle-derived: the C local is uninitialised. The port zeroes it, so the callee sees 0.
    assert_eq!(enum_next_proxy(5, None, returns_slot_value), 0);
}

#[test]
fn proxy_with_slot_passes_the_caller_pointer_through() {
    // Not oracle-derived: the proxy forwards the caller's slot, so the callee's write is visible.
    let mut slot = 0;
    assert_eq!(
        enum_next_proxy(5, Some(&mut slot), writes_celt_and_ok),
        S_OK
    );
    assert_eq!(slot, 5);
}

#[test]
fn shell_items_stub_counts_only_on_exact_s_ok() {
    // Not oracle-derived: the C test is `hr == S_OK`, so another success code (2) keeps the count
    // the callee wrote, and the count is not set to celt.
    let mut slot = 9;
    assert_eq!(
        enum_shell_items_next_stub(3, &mut slot, returns_success_code_two),
        2
    );
    assert_eq!(slot, 4);
}

#[test]
fn shell_items_stub_failure_clears_count_before_call() {
    // Not oracle-derived: the count is zeroed before the call and a failing callee writes nothing,
    // so the caller sees 0.
    let mut slot = 9;
    assert_eq!(
        enum_shell_items_next_stub(3, &mut slot, writes_nothing_and_fails),
        -2147467259
    );
    assert_eq!(slot, 0);
}

#[test]
fn objects_stub_zeroes_before_the_call() {
    // Not oracle-derived: the callee sees the zeroed count and leaves it unchanged, so the caller
    // sees 0.
    let mut slot = 9;
    assert_eq!(enum_objects_next_stub(3, &mut slot, returns_slot_value), 0);
    assert_eq!(slot, 0);
}

#[test]
fn can_unload_only_zero_is_s_ok() {
    // Not oracle-derived: the RefCount boundaries around zero, from the `!= 0` test in
    // NdrDllCanUnloadNow.
    assert_eq!(dll_can_unload_now(i32::MIN), S_FALSE);
    assert_eq!(dll_can_unload_now(-1), S_FALSE);
    assert_eq!(dll_can_unload_now(0), S_OK);
    assert_eq!(dll_can_unload_now(1), S_FALSE);
    assert_eq!(dll_can_unload_now(i32::MAX), S_FALSE);
}
