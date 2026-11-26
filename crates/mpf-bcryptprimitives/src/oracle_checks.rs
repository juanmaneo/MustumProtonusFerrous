//! Oracle-derived test vectors for the ported ProcessPrng loop.
//!
//! Vectors were captured by compiling the verbatim Wine C body of ProcessPrng
//! from dlls/bcryptprimitives/main.c with gcc -fsanitize=address,undefined -g
//! under target/canon/bcryptprimitives/ (oracle.c) and running the harness. The
//! harness fakes RtlGenRandom: it records (offset, len) and fails on the
//! fail_at-th call, so these rows check chunking and the return value, not the
//! random bytes. All 21 printed rows passed ASan and UBSan, so none are excluded.

use super::*;

/// (size, fail_at, ProcessPrng return, (offset, len) of each RtlGenRandom call).
/// fail_at is 1-based; 0 means the fake RtlGenRandom never fails.
type Row = (u64, usize, bool, &'static [(u64, u64)]);

/// Runs process_prng_chunks and returns its result and the (offset, len) of each call.
fn run(size: u64, fail_at: usize) -> (bool, Vec<(u64, u64)>) {
    let mut calls = Vec::new();
    let ret = process_prng_chunks(size, &mut |offset, len| {
        calls.push((offset, len));
        fail_at == 0 || calls.len() != fail_at
    });
    (ret, calls)
}

#[test]
fn oracle_process_prng_rows() {
    let rows: &[Row] = &[
        (0, 0, true, &[]),
        (0, 1, true, &[]),
        (0x1, 0, true, &[(0, 0x1)]),
        (0x1, 1, false, &[(0, 0x1)]),
        (0x7, 2, true, &[(0, 0x7)]),
        (0x20, 0, true, &[(0, 0x20)]),
        (0x1000, 0, true, &[(0, 0x1000)]),
        (0x10000, 0, true, &[(0, 0x10000)]),
        (0xffffffff, 0, true, &[(0, 0xffffffff)]),
        (0xffffffff, 1, false, &[(0, 0xffffffff)]),
        (0x100000000, 0, true, &[(0, 0xffffffff), (0xffffffff, 0x1)]),
        (0x100000000, 1, false, &[(0, 0xffffffff)]),
        (0x100000000, 2, false, &[(0, 0xffffffff), (0xffffffff, 0x1)]),
        (0x100000000, 3, true, &[(0, 0xffffffff), (0xffffffff, 0x1)]),
        (0x100000005, 0, true, &[(0, 0xffffffff), (0xffffffff, 0x6)]),
        (0x100000005, 2, false, &[(0, 0xffffffff), (0xffffffff, 0x6)]),
        (
            0x1fffffffe,
            0,
            true,
            &[(0, 0xffffffff), (0xffffffff, 0xffffffff)],
        ),
        (
            0x1fffffffe,
            2,
            false,
            &[(0, 0xffffffff), (0xffffffff, 0xffffffff)],
        ),
        (
            0x1ffffffff,
            0,
            true,
            &[
                (0, 0xffffffff),
                (0xffffffff, 0xffffffff),
                (0x1fffffffe, 0x1),
            ],
        ),
        (
            0x1ffffffff,
            3,
            false,
            &[
                (0, 0xffffffff),
                (0xffffffff, 0xffffffff),
                (0x1fffffffe, 0x1),
            ],
        ),
        (
            0x200000000,
            0,
            true,
            &[
                (0, 0xffffffff),
                (0xffffffff, 0xffffffff),
                (0x1fffffffe, 0x2),
            ],
        ),
    ];
    for &(size, fail_at, ret, calls) in rows {
        assert_eq!(
            run(size, fail_at),
            (ret, calls.to_vec()),
            "size={size:#x} fail_at={fail_at}"
        );
    }
}
