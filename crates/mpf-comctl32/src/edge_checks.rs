//! Cases the oracle rows do not reach: argument checks, the grow and shrink edges, the panics
//! that stand in for C crashes, and the export table. Each expectation is the port's own
//! behaviour, derived from the verbatim C path named in its comment. None of these are oracle
//! rows.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::oracle_checks::{alloc_fn, cmp_fn, dpa_enum_fn, dpa_of, dsa_enum_fn, dsa_of, Row};
use crate::{
    dpa_clone, dpa_clone_into, dpa_delete_ptr, dpa_enum_callback, dpa_get_ptr, dpa_grow,
    dpa_insert_ptr, dpa_search, dpa_set_ptr, dpa_sort, dsa_delete_item, dsa_enum_callback,
    dsa_get_item, dsa_get_item_ptr, dsa_insert_item, dsa_set_item, export_by_ordinal, export_kind,
    str_get_ptr_a, str_get_ptr_w, str_set_ptr_a, str_set_ptr_w, wine_spec_unimplemented_stub,
    ExportKind, StubException, DPAS_INSERTBEFORE, DPAS_SORTED, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, EXPORTS,
};

#[test]
fn dpa_negative_index_fails_without_change() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1], &mut alloc);
    assert_eq!(dpa_insert_ptr(dpa.as_deref_mut(), -1, 5, &mut alloc), -1);
    assert!(!dpa_set_ptr(dpa.as_deref_mut(), -1, 5, &mut alloc));
    assert_eq!(dpa_delete_ptr(dpa.as_deref_mut(), -1, &mut alloc), 0);
    assert_eq!(dpa_get_ptr(dpa.as_deref(), -1), 0);
    assert_eq!(dpa.as_deref().map(|d| d.item_count), Some(1));
    assert_eq!(row.allocs(), "allocs=[32,128]");
}

#[test]
fn dpa_set_ptr_alloc_failure_keeps_count_and_max() {
    // Allocations 1 and 2 make the handle. Allocation 3 grows the block for index 16 and fails.
    let row = Row::new(3);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2], &mut alloc);
    assert!(!dpa_set_ptr(dpa.as_deref_mut(), 16, 9, &mut alloc));
    let dpa = dpa.expect("dpa");
    assert_eq!((dpa.item_count, dpa.max_count), (2, 16));
    assert_eq!(row.allocs(), "allocs=[32,128,192!]");
}

#[test]
fn dpa_delete_ptr_out_of_range_leaves_items() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2], &mut alloc);
    assert_eq!(dpa_delete_ptr(dpa.as_deref_mut(), 2, &mut alloc), 0);
    assert_eq!(dpa_delete_ptr(dpa.as_deref_mut(), 5, &mut alloc), 0);
    assert_eq!(dpa.as_deref().map(|d| d.item_count), Some(2));
    assert_eq!(row.allocs(), "allocs=[32,128]");
}

#[test]
fn dpa_delete_last_ptr_shifts_nothing() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3], &mut alloc);
    assert_eq!(dpa_delete_ptr(dpa.as_deref_mut(), 2, &mut alloc), 3);
    let dpa = dpa.expect("dpa");
    assert_eq!((dpa.item_count, &dpa.ptrs[..2]), (2, &[1, 2][..]));
    assert_eq!(row.allocs(), "allocs=[32,128,128]");
}

#[test]
fn dpa_delete_ptr_within_slack_keeps_block() {
    // Ten items in a block of 16 leave 7 free slots after one delete, which is below `grow`.
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &mut alloc);
    assert_eq!(dpa_delete_ptr(dpa.as_deref_mut(), 1, &mut alloc), 2);
    let dpa = dpa.expect("dpa");
    assert_eq!((dpa.item_count, dpa.max_count), (9, 16));
    assert_eq!(row.allocs(), "allocs=[32,128]");
}

#[test]
fn dpa_grow_within_capacity_keeps_block() {
    // A grow of 4 is raised to 8, and the 16 slots already fit whole groups of 8.
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[], &mut alloc);
    assert!(dpa_grow(dpa.as_deref_mut(), 4, &mut alloc));
    let dpa = dpa.expect("dpa");
    assert_eq!((dpa.grow, dpa.max_count), (8, 16));
    assert_eq!(row.allocs(), "allocs=[32,128]");
}

#[test]
fn dpa_clone_of_nothing_makes_no_allocation() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    assert!(dpa_clone(None, &mut alloc).is_none());
    let mut dst = dpa_of(&[], &mut alloc).expect("dst");
    assert!(!dpa_clone_into(None, &mut dst, &mut alloc));
    assert_eq!(row.allocs(), "allocs=[32,128]");
}

#[test]
#[should_panic(expected = "header allocation failed")]
fn dpa_clone_header_failure_panics() {
    // The C code dereferences the NULL header, so the port panics at the same point.
    let setup = Row::new(0);
    let src = dpa_of(&[1], &mut alloc_fn(&setup));
    let row = Row::new(1);
    dpa_clone(src.as_deref(), &mut alloc_fn(&row));
}

#[test]
#[should_panic(expected = "out of range")]
fn dpa_clone_block_failure_with_items_panics() {
    // The block allocation fails, and the copy then writes items into an empty block.
    let setup = Row::new(0);
    let src = dpa_of(&[1, 2, 3], &mut alloc_fn(&setup));
    let row = Row::new(2);
    dpa_clone(src.as_deref(), &mut alloc_fn(&row));
}

#[test]
fn dpa_clone_block_failure_without_items_keeps_handle() {
    // The header is allocation 3 and the 8-slot block is allocation 4, which fails.
    // DPA_Clone sets nMaxCount whether or not the block allocation succeeds.
    let row = Row::new(4);
    let mut alloc = alloc_fn(&row);
    let src = dpa_of(&[], &mut alloc);
    let clone = dpa_clone(src.as_deref(), &mut alloc).expect("clone");
    assert_eq!((clone.item_count, clone.max_count), (0, 8));
    assert!(clone.ptrs.is_empty());
    assert_eq!(row.allocs(), "allocs=[32,128,32,64!]");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn dpa_set_ptr_through_null_block_panics() {
    // A failed block allocation leaves an empty block, and the write then indexes it.
    let row = Row::new(2);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[], &mut alloc);
    dpa_set_ptr(dpa.as_deref_mut(), 0, 1, &mut alloc);
}

#[test]
fn dpa_insert_alloc_failure_on_full_block_keeps_items() {
    // Sixteen items fill the block, so the next insert grows it, and that grow fails.
    let setup = Row::new(0);
    let values: Vec<usize> = (1..=16).collect();
    let mut dpa = dpa_of(&values, &mut alloc_fn(&setup));
    let row = Row::new(1);
    assert_eq!(
        dpa_insert_ptr(dpa.as_deref_mut(), 0, 99, &mut alloc_fn(&row)),
        -1
    );
    assert_eq!(row.allocs(), "allocs=[192!]");
    let dpa = dpa.expect("dpa");
    assert_eq!((dpa.item_count, dpa.max_count), (16, 16));
    assert_eq!(dpa.ptrs[0], 1);
}

#[test]
fn dpa_sort_one_item_makes_no_comparison() {
    let row = Row::new(0);
    let mut dpa = dpa_of(&[7], &mut alloc_fn(&row));
    assert!(dpa_sort(dpa.as_deref_mut(), &mut cmp_fn(&row)));
    assert_eq!(row.tail(), "allocs=[32,128] log=[]");
}

#[test]
fn dpa_sort_twenty_reversed_items() {
    let row = Row::new(0);
    let values: Vec<usize> = (1..=20).rev().collect();
    let mut dpa = dpa_of(&values, &mut alloc_fn(&row));
    assert!(dpa_sort(dpa.as_deref_mut(), &mut cmp_fn(&row)));
    let sorted = dpa.map(|dpa| dpa.ptrs[..20].to_vec());
    assert_eq!(sorted, Some((1..=20).collect::<Vec<usize>>()));
}

#[test]
fn dpa_search_zero_find_returns_minus_one_without_comparing() {
    let row = Row::new(0);
    let dpa = dpa_of(&[1, 2], &mut alloc_fn(&row));
    assert_eq!(dpa_search(dpa.as_deref(), 0, 0, &mut cmp_fn(&row), 0), -1);
    assert_eq!(
        dpa_search(dpa.as_deref(), 0, 0, &mut cmp_fn(&row), DPAS_SORTED),
        -1
    );
    assert_eq!(row.tail(), "allocs=[32,128] log=[]");
}

#[test]
fn dpa_search_empty_sorted_insert_before_returns_zero() {
    let row = Row::new(0);
    let dpa = dpa_of(&[], &mut alloc_fn(&row));
    assert_eq!(
        dpa_search(
            dpa.as_deref(),
            5,
            0,
            &mut cmp_fn(&row),
            DPAS_SORTED | DPAS_INSERTBEFORE
        ),
        0
    );
    assert_eq!(
        dpa_search(dpa.as_deref(), 5, 0, &mut cmp_fn(&row), DPAS_SORTED),
        -1
    );
    assert_eq!(row.tail(), "allocs=[32,128] log=[]");
}

#[test]
fn dpa_search_linear_start_is_clamped_and_bounded() {
    // A negative start searches from 0. A start past the end finds nothing.
    let row = Row::new(0);
    let dpa = dpa_of(&[5, 2, 8], &mut alloc_fn(&row));
    assert_eq!(dpa_search(dpa.as_deref(), 5, -1, &mut cmp_fn(&row), 0), 0);
    assert_eq!(dpa_search(dpa.as_deref(), 8, 3, &mut cmp_fn(&row), 0), -1);
}

#[test]
fn dpa_enum_runs_to_end_and_skips_empty_handle() {
    let row = Row::new(0);
    let dpa = dpa_of(&[4, 5, 6], &mut alloc_fn(&row));
    dpa_enum_callback(dpa.as_deref(), &mut dpa_enum_fn(&row, usize::MAX));
    let empty = dpa_of(&[], &mut alloc_fn(&row));
    dpa_enum_callback(empty.as_deref(), &mut dpa_enum_fn(&row, usize::MAX));
    assert_eq!(row.tail(), "allocs=[32,128,32,128] log=[4,5,6]");
}

#[test]
fn dsa_create_clamps_grow_to_one() {
    let row = Row::new(0);
    let dsa = dsa_of(4, 0, &mut alloc_fn(&row)).expect("dsa");
    assert_eq!((dsa.item_size, dsa.grow), (4, 1));
}

#[test]
fn dsa_negative_index_reads_nothing() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc);
    let mut buf = [0x58u8; 4];
    assert!(!dsa_get_item(dsa.as_deref(), -1, &mut buf));
    assert!(dsa_get_item_ptr(dsa.as_deref(), -1).is_none());
    assert_eq!(buf, [0x58; 4]);
}

#[test]
fn dsa_set_item_overwrite_does_not_allocate() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    assert!(dsa_set_item(
        dsa.as_deref_mut(),
        0,
        &[1, 2, 3, 4],
        &mut alloc
    ));
    assert!(dsa_set_item(
        dsa.as_deref_mut(),
        0,
        &[5, 6, 7, 8],
        &mut alloc
    ));
    let dsa = dsa.expect("dsa");
    assert_eq!(dsa.item_count, 1);
    assert_eq!(&dsa.data[..4], &[5, 6, 7, 8]);
    assert_eq!(row.allocs(), "allocs=[32,8]");
}

#[test]
fn dsa_set_item_negative_item_size_panics_after_alloc() {
    // The block request is the item size times the slot count, as a DWORD. The copy then panics.
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(-4, 2, &mut alloc);
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        dsa_set_item(dsa.as_deref_mut(), 0, &[0; 4], &mut alloc)
    }));
    assert!(outcome.is_err());
    assert_eq!(row.allocs(), "allocs=[32,4294967288]");
}

#[test]
fn dsa_insert_negative_index_fails_before_alloc() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    assert_eq!(
        dsa_insert_item(dsa.as_deref_mut(), -1, &[1; 4], &mut alloc),
        -1
    );
    assert_eq!(row.allocs(), "allocs=[32]");
}

#[test]
fn dsa_insert_size_overflow_fails_before_alloc() {
    // Two items of 0x4000_0000 bytes overflow INT, so DSA_InsertItem's size check returns -1.
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(0x4000_0000, 2, &mut alloc);
    assert_eq!(dsa_insert_item(dsa.as_deref_mut(), 0, &[], &mut alloc), -1);
    assert_eq!(row.allocs(), "allocs=[32]");
}

#[test]
#[should_panic(expected = "divide by zero")]
fn dsa_insert_zero_item_size_divides_by_zero() {
    // The C size check divides by the item size, so a zero size faults there too.
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(0, 2, &mut alloc);
    dsa_insert_item(dsa.as_deref_mut(), 0, &[], &mut alloc);
}

#[test]
fn dsa_delete_item_out_of_range_fails() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 1, &[1; 4], &mut alloc);
    assert_eq!(dsa_delete_item(dsa.as_deref_mut(), 2, &mut alloc), -1);
    assert_eq!(dsa_delete_item(dsa.as_deref_mut(), -1, &mut alloc), -1);
    assert_eq!(dsa.as_deref().map(|d| d.item_count), Some(2));
    assert_eq!(row.allocs(), "allocs=[32,8]");
}

#[test]
fn dsa_delete_last_item_releases_block() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 1, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc);
    assert_eq!(dsa_delete_item(dsa.as_deref_mut(), 0, &mut alloc), 0);
    let dsa = dsa.expect("dsa");
    assert_eq!((dsa.item_count, dsa.max_count), (0, 0));
    assert_eq!(row.allocs(), "allocs=[32,4,0]");
}

#[test]
fn dsa_enum_visits_items_then_empty_handle_is_skipped() {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    for index in 0..3 {
        dsa_set_item(
            dsa.as_deref_mut(),
            index,
            &(index + 1).to_ne_bytes(),
            &mut alloc,
        );
    }
    dsa_enum_callback(dsa.as_deref(), &mut dsa_enum_fn(&row, -1));
    let empty = dsa_of(4, 2, &mut alloc);
    dsa_enum_callback(empty.as_deref(), &mut dsa_enum_fn(&row, -1));
    assert_eq!(row.tail(), "allocs=[32,8,16,32] log=[1,2,3]");
}

#[test]
fn str_get_ptr_a_zero_max_returns_size_and_keeps_buffer() {
    let mut buf = [0x58u8; 4];
    assert_eq!(str_get_ptr_a(Some(b"abc\0"), Some(&mut buf), 0), 4);
    assert_eq!(str_get_ptr_a(None, Some(&mut buf), 0), 0);
    assert_eq!(str_get_ptr_a(None, None, 0), 0);
    assert_eq!(buf, [0x58; 4]);
}

#[test]
fn str_get_ptr_a_exact_fit_keeps_nul() {
    let mut buf = [0x58u8; 4];
    assert_eq!(str_get_ptr_a(Some(b"abc\0"), Some(&mut buf), 4), 4);
    assert_eq!(buf, [97, 98, 99, 0]);
}

#[test]
#[should_panic(expected = "negative max_len")]
fn str_get_ptr_a_negative_max_panics() {
    let mut buf = [0u8; 4];
    str_get_ptr_a(Some(b"abc\0"), Some(&mut buf), -1);
}

#[test]
#[should_panic(expected = "NULL destination")]
fn str_get_ptr_a_null_destination_panics() {
    str_get_ptr_a(None, None, 4);
}

#[test]
#[should_panic(expected = "without a NUL")]
fn str_get_ptr_a_unterminated_source_panics() {
    str_get_ptr_a(Some(b"abc"), None, 0);
}

#[test]
fn str_get_ptr_w_zero_max_and_null_destination() {
    let mut buf = [7u16; 3];
    assert_eq!(str_get_ptr_w(Some(&[97, 98, 0]), Some(&mut buf), 0), 0);
    assert_eq!(str_get_ptr_w(None, None, 0), 0);
    assert_eq!(buf, [7; 3]);
}

#[test]
fn str_get_ptr_w_exact_fit_drops_nul() {
    let mut buf = [7u16; 3];
    assert_eq!(str_get_ptr_w(Some(&[97, 98, 99, 0]), Some(&mut buf), 3), 2);
    assert_eq!(buf, [97, 98, 0]);
}

#[test]
#[should_panic(expected = "NULL destination")]
fn str_get_ptr_w_null_destination_panics() {
    str_get_ptr_w(None, None, 3);
}

#[test]
#[should_panic(expected = "negative max_len")]
fn str_get_ptr_w_negative_max_panics() {
    let mut buf = [0u16; 3];
    str_get_ptr_w(Some(&[97, 0]), Some(&mut buf), -1);
}

#[test]
fn str_set_ptr_failure_keeps_old_value() {
    let row_a = Row::new(1);
    let mut dst = Some(b"old\0".to_vec());
    assert!(!str_set_ptr_a(
        &mut dst,
        Some(b"new\0"),
        &mut alloc_fn(&row_a)
    ));
    assert_eq!(dst, Some(b"old\0".to_vec()));
    assert_eq!(row_a.allocs(), "allocs=[4!]");

    let row_w = Row::new(1);
    let mut wide = Some(vec![111u16, 0]);
    assert!(!str_set_ptr_w(
        &mut wide,
        Some(&[110, 0]),
        &mut alloc_fn(&row_w)
    ));
    assert_eq!(wide, Some(vec![111, 0]));
    assert_eq!(row_w.allocs(), "allocs=[4!]");
}

#[test]
fn export_names_resolve_to_their_kinds() {
    assert_eq!(export_kind("MenuHelp"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("PropertySheet"),
        Some(ExportKind::Alias("PropertySheetA"))
    );
    assert_eq!(export_kind("StrChrA"), None);
    assert_eq!(export_kind("CreatePage"), None);
    assert_eq!(export_kind("NoSuchExport"), None);
}

#[test]
fn export_ordinals_resolve_to_their_entries() {
    assert_eq!(export_by_ordinal(2).map(|e| e.symbol), Some("MenuHelp"));
    let page = export_by_ordinal(163).expect("CreatePage");
    assert_eq!(
        (page.name, page.symbol, page.kind),
        (None, "CreatePage", ExportKind::Stub)
    );
    assert_eq!(
        export_by_ordinal(350).map(|e| e.kind),
        Some(ExportKind::Forward("kernelbase.StrChrA"))
    );
    assert!(export_by_ordinal(0).is_none());
    assert!(export_by_ordinal(u16::MAX).is_none());
}

#[test]
fn export_table_kind_counts() {
    let count = |kind: ExportKind| EXPORTS.iter().filter(|e| e.kind == kind).count();
    assert_eq!(count(ExportKind::Stdcall), 140);
    assert_eq!(count(ExportKind::Stub), 4);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|e| matches!(e.kind, ExportKind::Alias(_)))
            .count(),
        5
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|e| matches!(e.kind, ExportKind::Forward(_)))
            .count(),
        31
    );
}

#[test]
fn stub_record_carries_module_and_function() {
    assert_eq!(
        wine_spec_unimplemented_stub("comctl32.dll", "Example"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "comctl32.dll",
            function: "Example",
        }
    );
}
