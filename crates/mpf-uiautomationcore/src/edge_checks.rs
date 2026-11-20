//! Boundary checks beyond the recorded Wine rows in `oracle_checks.rs`.

use crate::{compare_int_safearrays, find_guid, guid, index_from_id, reserve_capacity, SafeArray};

const ARRAY: SafeArray = SafeArray {
    dims: 1,
    values: &[1, 2, 3],
};

fn sa(dims: u32, values: &[i32]) -> SafeArray<'_> {
    SafeArray { dims, values }
}

#[test]
fn guid_is_laid_out_in_memory_order() {
    let g = guid(0x1234_5678, 0xabcd, 0xef01, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        g,
        [0x78, 0x56, 0x34, 0x12, 0xcd, 0xab, 0x01, 0xef, 1, 2, 3, 4, 5, 6, 7, 8]
    );
}

#[test]
fn compare_empty_arrays() {
    assert_eq!(compare_int_safearrays(&sa(1, &[]), &sa(1, &[]), 0x10001), 0);
    assert_eq!(
        compare_int_safearrays(&sa(1, &[]), &sa(1, &[]), 0x10003),
        -1
    );
}

#[test]
fn compare_count_mismatch_decides_before_prop_type() {
    assert_eq!(
        compare_int_safearrays(&sa(1, &[1]), &sa(1, &[1, 2]), 0x10002),
        -1
    );
    assert_eq!(
        compare_int_safearrays(&sa(1, &[1, 2]), &sa(1, &[1]), 0x10002),
        1
    );
}

#[test]
fn compare_is_lexicographic_on_first_difference() {
    assert_eq!(
        compare_int_safearrays(&sa(1, &[1, 9]), &sa(1, &[2, 0]), 0x10001),
        -1
    );
    assert_eq!(
        compare_int_safearrays(&sa(1, &[3, 0]), &sa(1, &[2, 9]), 0x10001),
        1
    );
}

#[test]
fn compare_rejects_non_one_dimensional_arrays() {
    assert_eq!(
        compare_int_safearrays(&sa(0, &[1, 2, 3]), &ARRAY, 0x10001),
        -1
    );
    assert_eq!(
        compare_int_safearrays(&ARRAY, &sa(0, &[1, 2, 3]), 0x10001),
        -1
    );
    assert_eq!(
        compare_int_safearrays(&sa(2, &[]), &sa(2, &[9]), 0x10001),
        -1
    );
}

#[test]
fn reserve_keeps_capacity_without_dividing_by_size() {
    assert_eq!(reserve_capacity(2, 0, 0), Some(2));
    assert_eq!(reserve_capacity(2, 2, 0), Some(2));
}

#[test]
fn reserve_fails_when_growth_needs_zero_size() {
    assert_eq!(reserve_capacity(0, 1, 0), None);
}

#[test]
fn reserve_at_the_maximum_element_count() {
    assert_eq!(reserve_capacity(0, 1, usize::MAX), Some(1));
    assert_eq!(reserve_capacity(0, 2, usize::MAX), None);
}

#[test]
fn reserve_doubles_to_the_top_bit() {
    assert_eq!(reserve_capacity(1 << 62, (1 << 62) + 1, 1), Some(1 << 63));
    assert_eq!(reserve_capacity(1, 5, 8), Some(8));
    assert_eq!(reserve_capacity(0, 8, 8), Some(8));
}

#[test]
fn find_in_empty_table() {
    assert_eq!(find_guid(&[], &[0; 16]), None);
}

#[test]
fn find_outside_the_table_bounds() {
    let table = [(guid(1, 0, 0, [0; 8]), 7), (guid(9, 0, 0, [0; 8]), 8)];
    assert_eq!(find_guid(&table, &guid(0, 0, 0, [0; 8])), None);
    assert_eq!(find_guid(&table, &guid(0xff, 0, 0, [0; 8])), None);
    assert_eq!(find_guid(&table, &guid(9, 0, 0, [0; 8])), Some(8));
}

#[test]
fn index_at_both_ends_of_the_range() {
    let idx = [2usize, 0, 1];
    assert_eq!(index_from_id(100, 100, &idx), Some(2));
    assert_eq!(index_from_id(102, 100, &idx), Some(1));
    assert_eq!(index_from_id(99, 100, &idx), None);
    assert_eq!(index_from_id(103, 100, &idx), None);
}

#[test]
fn index_with_no_entries() {
    assert_eq!(index_from_id(0, 0, &[]), None);
}

#[test]
fn index_handles_extreme_ids() {
    let idx = [0usize];
    assert_eq!(index_from_id(i32::MAX, i32::MIN, &idx), None);
    assert_eq!(index_from_id(i32::MIN, i32::MIN, &idx), Some(0));
}
