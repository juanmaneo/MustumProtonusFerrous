//! Edge-case tests for the ported ProcessPrng loop.
//!
//! These cover the ULONG_MAX chunk boundary, chunk contiguity for sizes that
//! need several calls, failure after several chunks, and the slice adapter.
//! Expectations that are not oracle-derived are marked with a comment.

use super::*;

/// Runs process_prng_chunks and returns its result and the (offset, len) of each call.
/// fail_at is 1-based; 0 means the fake RtlGenRandom never fails.
fn run(size: u64, fail_at: usize) -> (bool, Vec<(u64, u64)>) {
    let mut calls = Vec::new();
    let ret = process_prng_chunks(size, &mut |offset, len| {
        calls.push((offset, len));
        fail_at == 0 || calls.len() != fail_at
    });
    (ret, calls)
}

/// Runs process_prng over data. Each chunk is filled with 0xaa and its length is
/// recorded. The fake fails on every chunk when fail is set.
fn fill(data: &mut [u8], fail: bool) -> (bool, Vec<usize>) {
    let mut lens = Vec::new();
    let ret = process_prng(data, &mut |chunk: &mut [u8]| {
        lens.push(chunk.len());
        chunk.fill(0xaa);
        !fail
    });
    (ret, lens)
}

#[test]
fn chunk_len_is_capped_at_ulong_max() {
    // Not oracle-derived: direct check of min(size, (SIZE_T)ULONG_MAX) at the boundaries.
    assert_eq!(next_chunk_len(0), 0);
    assert_eq!(next_chunk_len(1), 1);
    assert_eq!(next_chunk_len(ULONG_MAX - 1), ULONG_MAX - 1);
    assert_eq!(next_chunk_len(ULONG_MAX), ULONG_MAX);
    assert_eq!(next_chunk_len(ULONG_MAX + 1), ULONG_MAX);
    assert_eq!(next_chunk_len(u64::MAX), ULONG_MAX);
}

#[test]
fn exact_multiples_of_ulong_max_have_no_empty_tail_call() {
    // Not oracle-derived: follows from the loop condition (while size).
    for k in 1..=3u64 {
        let (ret, calls) = run(k * ULONG_MAX, 0);
        assert!(ret);
        assert_eq!(calls.len(), k as usize);
        assert!(calls.iter().all(|&(_, len)| len == ULONG_MAX));
    }
}

#[test]
fn chunks_are_contiguous_and_cover_size() {
    // Not oracle-derived: offsets advance by the previous len and the lens sum to size.
    for size in [1, ULONG_MAX, ULONG_MAX + 1, 3 * ULONG_MAX + 17] {
        let (ret, calls) = run(size, 0);
        assert!(ret);
        let mut next = 0;
        for &(offset, len) in &calls {
            assert_eq!(offset, next);
            assert!((1..=ULONG_MAX).contains(&len));
            next += len;
        }
        assert_eq!(next, size);
    }
}

#[test]
fn failure_after_several_chunks_stops_the_loop() {
    // Not oracle-derived: the oracle has two-chunk failures; this one has three chunks.
    let (ret, calls) = run(3 * ULONG_MAX, 2);
    assert!(!ret);
    assert_eq!(calls, vec![(0, ULONG_MAX), (ULONG_MAX, ULONG_MAX)]);
}

#[test]
fn slice_empty_makes_no_call_and_succeeds() {
    // Not oracle-derived for the slice adapter. ProcessPrng(data, 0) returning TRUE
    // without a call is oracle row size 0.
    assert_eq!(fill(&mut [], false), (true, Vec::new()));
}

#[test]
fn slice_is_filled_in_one_call() {
    // Not oracle-derived: data.len() is passed as size, and the chunk is the whole slice.
    let mut buf = [0u8; 5];
    assert_eq!(fill(&mut buf, false), (true, vec![5]));
    assert_eq!(buf, [0xaa; 5]);
}

#[test]
fn slice_failure_returns_false() {
    // Not oracle-derived for the slice adapter. The loop-level failure is oracle-derived.
    let mut buf = [0u8; 3];
    assert_eq!(fill(&mut buf, true), (false, vec![3]));
}
