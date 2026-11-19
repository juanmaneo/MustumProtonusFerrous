//! Edge checks for the cases the oracle rows do not reach: every power-of-two boundary of the segment
//! index, the queue sizing thresholds, wrapping queue positions, and short or saturated segment tables.
//! The values the C defines were also run through the verbatim C in `target/canon/concrt140`. The
//! Rust-only cases are labelled, because the C either has undefined behaviour there or has no
//! equivalent for the padded slice.

use super::*;
use crate::oracle_checks::table;

fn allocated_then_empty(allocated: usize) -> Vec<Segment> {
    let mut segments = vec![Segment::Allocated; allocated];
    segments.push(Segment::Empty);
    segments
}

#[test]
fn segment_index_is_the_bit_length_minus_one_at_every_power_of_two() {
    for k in 0..64u32 {
        assert_eq!(segment_index_of(1u64 << k), k);
        if k > 0 {
            assert_eq!(segment_index_of((1u64 << k) - 1), k - 1);
        }
    }
}

#[test]
fn queue_alloc_count_switches_at_each_size_threshold() {
    let cases = [
        (0, 32),
        (8, 32),
        (9, 16),
        (16, 16),
        (17, 8),
        (32, 8),
        (33, 4),
        (64, 4),
        (65, 2),
        (128, 2),
        (129, 1),
        (u64::MAX, 1),
    ];
    for (item_size, count) in cases {
        assert_eq!(queue_alloc_count(item_size), count, "item size {item_size}");
    }
}

#[test]
fn queue_is_empty_only_when_positions_match() {
    assert!(queue_is_empty(0, 0));
    assert!(!queue_is_empty(1, 2));
    assert!(!queue_is_empty(2, 1));
}

#[test]
fn queue_size_wraps_like_size_t_when_tail_is_behind_head() {
    assert_eq!(queue_size(u64::MAX, 0), 1);
    assert_eq!(queue_size(3, 10), 7);
    assert_eq!(queue_size(7, 7), 0);
}

#[test]
fn vector_capacity_of_an_empty_table_is_zero() {
    assert_eq!(vector_capacity(&table("NNN"), true), 0);
    assert_eq!(vector_capacity(&[], true), 0);
    assert_eq!(vector_capacity(&[], false), 0);
}

#[test]
fn vector_capacity_stops_at_a_marker_slot() {
    assert_eq!(vector_capacity(&table("MAA"), true), 0);
    assert_eq!(vector_capacity(&table("AMA"), true), 2);
    assert_eq!(vector_capacity(&table("AAMAN"), false), 4);
}

#[test]
fn vector_capacity_counts_a_heap_table_by_its_first_gap() {
    assert_eq!(vector_capacity(&allocated_then_empty(20), false), 1 << 20);
}

// Rust-only: the C shifts an `int` by 31 and by 63 here. UBSan flags the 31 case as
// `left shift of 1 by 31 places cannot be represented in type 'int'`.
#[test]
fn vector_capacity_keeps_the_exact_power_past_int_range() {
    assert_eq!(
        vector_capacity(&allocated_then_empty(31), false),
        1u64 << 31
    );
    assert_eq!(
        vector_capacity(&allocated_then_empty(63), false),
        1u64 << 63
    );
}

// Rust-only: all 64 heap slots allocated would need 2^64, which `size_t` cannot hold. The C
// shifts by 64 here, which is undefined behaviour.
#[test]
fn vector_capacity_saturates_when_every_heap_slot_is_allocated() {
    assert_eq!(vector_capacity(&[Segment::Allocated; 64], false), u64::MAX);
}

// Rust-only: the C reads the whole segment table. A short slice is padded with `Empty` here.
#[test]
fn vector_capacity_treats_slots_past_the_slice_as_empty() {
    assert_eq!(vector_capacity(&table("AA"), true), 4);
    assert_eq!(vector_capacity(&table("A"), false), 2);
}

#[test]
fn byte_reverse_matches_bit_reversal_for_every_byte() {
    for byte in 0..=u8::MAX {
        assert_eq!(byte_reverse(byte), byte.reverse_bits());
    }
}
