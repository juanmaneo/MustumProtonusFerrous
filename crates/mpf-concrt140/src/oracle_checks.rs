//! Replays of Wine's oracle rows. The oracle compiles the verbatim ranges of `dlls/concrt140/details.c`
//! in `target/canon/concrt140/harness.c` with `-fsanitize=address,undefined`, and reports no sanitizer
//! findings for these 24 rows. Each test names the row (`O01` to `O24`) it reproduces, and the expected
//! values are the ones printed by that run.

use super::*;

/// Builds a segment table from a pattern: `N` is `Empty`, `M` is `AllocMarker`, `A` is `Allocated`.
pub(crate) fn table(pattern: &str) -> Vec<Segment> {
    pattern
        .chars()
        .map(|c| match c {
            'N' => Segment::Empty,
            'M' => Segment::AllocMarker,
            _ => Segment::Allocated,
        })
        .collect()
}

#[test]
fn oracle_o01() {
    assert_eq!(segment_index_of(0), 0);
}

#[test]
fn oracle_o02() {
    assert_eq!(segment_index_of(1), 0);
}

#[test]
fn oracle_o03() {
    assert_eq!(segment_index_of(3), 1);
}

#[test]
fn oracle_o04() {
    assert_eq!(segment_index_of(8), 3);
}

#[test]
fn oracle_o05() {
    assert_eq!(segment_index_of(0xffff_ffff), 31);
}

#[test]
fn oracle_o06() {
    assert_eq!(segment_index_of(0x1_0000_0000), 32);
}

#[test]
fn oracle_o07() {
    assert_eq!(segment_index_of(u64::MAX), 63);
}

#[test]
fn oracle_o08() {
    assert_eq!(queue_alloc_count(8), 32);
}

#[test]
fn oracle_o09() {
    assert_eq!(queue_alloc_count(9), 16);
}

#[test]
fn oracle_o10() {
    assert_eq!(queue_alloc_count(64), 4);
}

#[test]
fn oracle_o11() {
    assert_eq!(queue_alloc_count(65), 2);
}

#[test]
fn oracle_o12() {
    assert_eq!(queue_alloc_count(128), 2);
}

#[test]
fn oracle_o13() {
    assert_eq!(queue_alloc_count(129), 1);
}

#[test]
fn oracle_o14() {
    assert_eq!(queue_alloc_count(1000), 1);
}

#[test]
fn oracle_o15() {
    assert!(queue_is_empty(5, 5));
}

#[test]
fn oracle_o16() {
    assert_eq!(queue_size(10, 3), 18_446_744_073_709_551_609);
}

#[test]
fn oracle_o17() {
    assert_eq!(vector_capacity(&table("AAA"), true), 8);
}

#[test]
fn oracle_o18() {
    assert_eq!(vector_capacity(&table("ANA"), true), 2);
}

#[test]
fn oracle_o19() {
    assert_eq!(vector_capacity(&table("NAA"), true), 0);
}

#[test]
fn oracle_o20() {
    assert_eq!(vector_capacity(&table("AAM"), true), 4);
}

#[test]
fn oracle_o21() {
    assert_eq!(vector_capacity(&table("AAANAAAAAAA"), false), 8);
}

#[test]
fn oracle_o22() {
    assert_eq!(byte_reverse(0x01), 0x80);
}

#[test]
fn oracle_o23() {
    assert_eq!(byte_reverse(0x5a), 0x5a);
}

#[test]
fn oracle_o24() {
    assert_eq!(byte_reverse(0xff), 0xff);
}
