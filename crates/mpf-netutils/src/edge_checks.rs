//! Edge-case tests for the ported netutils logic.
//!
//! These cover the NULL checks, the zero-size branch, the failure paths, the `HeapSize`
//! truncation, and the export and stub tables. Not every expectation is oracle-derived. The ones
//! that are not say so in a comment.

use super::oracle_checks::{FakeHeap, ERROR_NOT_ENOUGH_MEMORY, STUB_ROWS};
use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the values come from include/winerror.h, include/lmerr.h,
    // include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(NERR_SUCCESS, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn allocate_null_out_pointer_is_invalid_parameter_and_skips_the_heap() {
    // The return value matches oracle row 03. The check that the heap is not called is not
    // oracle-derived.
    let mut heap = FakeHeap::default();
    assert_eq!(
        net_api_buffer_allocate(&mut heap, 16, None),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(heap.next, 0);
}

#[test]
fn allocate_failure_clears_a_previously_held_block() {
    // Not oracle-derived: the oracle printed the failure with a NULL sentinel (row 02). This
    // checks that a non-NULL out-parameter is overwritten with NULL.
    let mut heap = FakeHeap::default();
    heap.fail_alloc = true;
    let mut buffer = Some(7);
    assert_eq!(
        net_api_buffer_allocate(&mut heap, 16, Some(&mut buffer)),
        ERROR_NOT_ENOUGH_MEMORY
    );
    assert_eq!(buffer, None);
}

#[test]
fn reallocate_without_an_old_block_allocates() {
    // Not oracle-derived: the NULL-old branch of NetApiBufferReallocate. The oracle printed only
    // the zero-size NULL case (row 08).
    let mut heap = FakeHeap::default();
    let mut new = None;
    assert_eq!(net_api_buffer_reallocate(&mut heap, None, 8, &mut new), 0);
    assert_eq!(new, Some(1));
    assert_eq!(heap.live, vec![(1, 8)]);
}

#[test]
fn reallocate_failure_stores_null_and_keeps_the_old_block() {
    // Not oracle-derived: the failed HeapReAlloc branch. The C code stores the NULL result in
    // *NewBuffer, so the old pointer is lost while the old block stays allocated. The port keeps
    // that behaviour.
    let mut heap = FakeHeap::default();
    let old = heap.alloc(16);
    heap.fail_alloc = true;
    let mut new = Some(9);
    assert_eq!(
        net_api_buffer_reallocate(&mut heap, old, 32, &mut new),
        ERROR_NOT_ENOUGH_MEMORY
    );
    assert_eq!(new, None);
    assert_eq!(heap.live, vec![(1, 16)]);
}

#[test]
fn reallocate_zero_frees_a_live_block_and_clears_new_buffer() {
    // Not oracle-derived: the zero-size branch with a live old block. The oracle printed the
    // NULL-old form (row 08).
    let mut heap = FakeHeap::default();
    let old = heap.alloc(16);
    let mut new = Some(9);
    assert_eq!(net_api_buffer_reallocate(&mut heap, old, 0, &mut new), 0);
    assert_eq!(new, None);
    assert!(heap.live.is_empty());
}

#[test]
fn reallocate_zero_free_failure_returns_the_heap_error_and_keeps_new_buffer() {
    // Not oracle-derived: HeapFree of a block the heap does not know. The C code returns before
    // it writes *NewBuffer. The error value is the one FakeHeap reports, not a value taken from a
    // real heap.
    let mut heap = FakeHeap::default();
    let mut new = Some(9);
    assert_eq!(
        net_api_buffer_reallocate(&mut heap, Some(99), 0, &mut new),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(new, Some(9));
}

#[test]
fn free_ignores_the_heap_result() {
    // Not oracle-derived: NetApiBufferFree returns NERR_Success even when HeapFree fails, and
    // when the block is NULL. Oracle row 07 printed only the success case.
    let mut heap = FakeHeap::default();
    assert_eq!(net_api_buffer_free(&mut heap, Some(99)), NERR_SUCCESS);
    assert_eq!(net_api_buffer_free(&mut heap, None), NERR_SUCCESS);
}

#[test]
fn size_of_an_unknown_block_reports_zero() {
    // Not oracle-derived: a failed HeapSize. The FakeHeap returns usize::MAX, which the C code
    // truncates to 0xFFFFFFFF and then reports as 0.
    let mut heap = FakeHeap::default();
    let mut bytes = 7;
    assert_eq!(
        net_api_buffer_size(&mut heap, Some(99), &mut bytes),
        NERR_SUCCESS
    );
    assert_eq!(bytes, 0);
}

#[test]
fn size_keeps_only_the_low_dword_of_the_heap_size() {
    // Not oracle-derived: the C code assigns the SIZE_T from HeapSize to a DWORD, which drops the
    // high 32 bits. The oracle heap never returns more than 4 GiB. A block of exactly 0xFFFF_FFFF
    // bytes also reads back as 0, the same as a failed HeapSize.
    let mut heap = FakeHeap::default();
    let big = heap.alloc(0x1_0000_0005).unwrap();
    let max = heap.alloc(0xFFFF_FFFF).unwrap();
    let mut bytes = 7;
    assert_eq!(
        net_api_buffer_size(&mut heap, Some(big), &mut bytes),
        NERR_SUCCESS
    );
    assert_eq!(bytes, 5);
    assert_eq!(
        net_api_buffer_size(&mut heap, Some(max), &mut bytes),
        NERR_SUCCESS
    );
    assert_eq!(bytes, 0);
}

#[test]
fn export_table_matches_the_spec_order() {
    // Not oracle-derived: the 22 lines of dlls/netutils/netutils.spec, in order.
    let names: Vec<&str> = EXPORTS.iter().map(|&(name, _)| name).collect();
    assert_eq!(
        names,
        [
            "NetApiBufferAllocate",
            "NetApiBufferFree",
            "NetApiBufferReallocate",
            "NetApiBufferSize",
            "NetRemoteComputerSupports",
            "NetapipBufferAllocate",
            "NetpIsComputerNameValid",
            "NetpIsDomainNameValid",
            "NetpIsGroupNameValid",
            "NetpIsRemote",
            "NetpIsRemoteNameValid",
            "NetpIsShareNameValid",
            "NetpIsUncComputerNameValid",
            "NetpIsUserNameValid",
            "NetpwListCanonicalize",
            "NetpwListTraverse",
            "NetpwNameCanonicalize",
            "NetpwNameCompare",
            "NetpwNameValidate",
            "NetpwPathCanonicalize",
            "NetpwPathCompare",
            "NetpwPathType",
        ]
    );
}

#[test]
fn export_kind_looks_names_up_case_sensitively() {
    // Not oracle-derived: lookups over the spec table. The alias is NetapipBufferAllocate, which
    // the spec maps to NetApiBufferAllocate.
    assert_eq!(export_kind("NetApiBufferSize"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("NetapipBufferAllocate"),
        Some(ExportKind::Alias("NetApiBufferAllocate"))
    );
    assert_eq!(export_kind("NetpIsRemote"), Some(ExportKind::Stub));
    assert_eq!(export_kind("netpisremote"), None);
    assert_eq!(export_kind("NoSuchExport"), None);
}

#[test]
fn stub_helpers_match_the_stub_entries_of_the_spec_in_order() {
    // Not oracle-derived: the stub entries of EXPORTS are the same 17 names, in the same order, as
    // the helpers that model them.
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|&(name, _)| name)
        .collect();
    let helper_names: Vec<&str> = STUB_ROWS.iter().map(|&(_, name)| name).collect();
    assert_eq!(stub_names, helper_names);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the stub record keeps any names it is given. The code and flags do not
    // depend on the names.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "NoSuchName"),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "other.dll",
            function: "NoSuchName",
        }
    );
}
