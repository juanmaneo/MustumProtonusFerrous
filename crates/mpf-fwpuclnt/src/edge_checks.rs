//! Edge-case tests for the ported fwpuclnt stubs.
//!
//! The stubs take no inputs, so the edge cases are the boundary values of the
//! one input that is echoed back (`FwpmFilterAdd0`'s `*pFilterId`) and the
//! status and NULL values every export shares. Expectations that are not
//! oracle-derived say so in a comment.

use super::*;

#[test]
fn status_value_matches_winerror() {
    // Oracle-derived: `RPC_S_CANNOT_SUPPORT` is 1764 in `winerror.h`, and the
    // harness prints 1764 for every export.
    assert_eq!(RPC_S_CANNOT_SUPPORT, 1764);
}

#[test]
fn every_status_is_non_success() {
    // Not oracle-derived: a check that the shared status is not `ERROR_SUCCESS`.
    // Every export in the oracle rows returns 1764, so this holds for each one.
    let statuses = [
        fwpm_engine_open0().0,
        fwpm_engine_close0(),
        fwpm_sub_layer_add0(),
        fwpm_sub_layer_get_by_key0().0,
        fwpm_filter_create_enum_handle0().0,
        fwpm_filter_destroy_enum_handle0(),
        fwpm_filter_enum0().status,
        fwpm_filter_add0(0).0,
    ];
    for status in statuses {
        assert_ne!(status, 0);
        assert_eq!(status, RPC_S_CANNOT_SUPPORT);
    }
}

#[test]
fn null_out_handles_are_zero() {
    // Oracle-derived: the harness prints `(nil)` for each handle the C body
    // writes as NULL, and `(nil)` is the zero pointer.
    assert_eq!(NULL, 0);
    assert_eq!(fwpm_engine_open0().1, NULL);
    assert_eq!(fwpm_sub_layer_get_by_key0().1, NULL);
    assert_eq!(fwpm_filter_create_enum_handle0().1, NULL);
}

#[test]
fn filter_enum_writes_zero_count_and_null_entries() {
    // Oracle-derived: `*entries` is NULL and `*nEntriesReturned` is 0 for every
    // `nEntriesRequested` the harness passed (0, 1 and 0xFFFFFFFF).
    let out = fwpm_filter_enum0();
    assert_eq!(out.entries, NULL);
    assert_eq!(out.n_entries_returned, 0);
    assert_eq!(out.status, RPC_S_CANNOT_SUPPORT);
}

#[test]
fn filter_add_echoes_filter_id_at_boundaries() {
    // Oracle-derived for `0xAAAA`, the one value the harness used. The other
    // values (0, 1, u64::MAX) are not oracle-derived; they test that the stub
    // leaves `*pFilterId` alone whatever it held before the call.
    for id in [0xAAAA, 0, 1, u64::MAX] {
        assert_eq!(fwpm_filter_add0(id), (RPC_S_CANNOT_SUPPORT, id));
    }
}

#[test]
fn free_memory_has_no_observable_effect() {
    // Oracle-derived: the harness leaves `*mem` at 0x1111 after the call, and
    // this function has no return value to check.
    fwpm_free_memory0();
    fwpm_free_memory0();
}

#[test]
fn filter_enum_struct_is_copyable_and_comparable() {
    // Not oracle-derived: checks the derive set so callers can copy and
    // compare the out-values.
    let a = fwpm_filter_enum0();
    let b = a;
    assert_eq!(a, b);
    assert_ne!(
        a,
        FilterEnum {
            status: 0,
            entries: NULL,
            n_entries_returned: 0,
        }
    );
}
