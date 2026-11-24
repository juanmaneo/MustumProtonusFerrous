//! Rows taken from the native-compiled oracle in `target/canon/vcomp`, which runs Wine's
//! `main.c` under ASan and UBSan. Vectors that those tools flag are left out: the signed overflow
//! in `_vcomp_atomic_mul_i4` and in the `C2VectParallel` serial-fallback predicate. The edge checks
//! cover both as wrapping behaviour.

use crate::reduction::{combine_f32, combine_f64, combine_i32, combine_i64, combine_i8};
use crate::schedule::{
    dynamic_init, dynamic_next, static_init, static_init_i8, static_simple_init,
    static_simple_init_i8, DynamicTask, LoopBounds, StaticChunks, ThreadDynamic,
};
use crate::vect::{dynamic_chunks, static_range};
use crate::workshare::{
    sections_init, sections_next, single_begin, WorkshareTask, WorkshareThread,
};

pub(crate) fn static32(s: StaticChunks<u32, i32>) -> (u32, Option<(i32, i32, i32, i32)>) {
    (
        s.loops,
        s.range.map(|r| (r.begin, r.end, r.next, r.lastchunk)),
    )
}

pub(crate) fn static64(s: StaticChunks<u64, i64>) -> (u64, Option<(i64, i64, i64, i64)>) {
    (
        s.loops,
        s.range.map(|r| (r.begin, r.end, r.next, r.lastchunk)),
    )
}

pub(crate) fn drain(
    flags: u32,
    bounds: LoopBounds,
    chunksize: u32,
    num_threads: u32,
) -> Vec<(u32, u32)> {
    let mut thread = ThreadDynamic::default();
    let mut task = DynamicTask::default();
    dynamic_init(
        &mut thread,
        &mut task,
        flags,
        bounds,
        chunksize,
        num_threads,
        0,
    );
    std::iter::from_fn(|| dynamic_next(&mut thread, &mut task, num_threads)).collect()
}

#[test]
fn wine_reduction_int_oracle_rows() {
    assert_eq!(combine_i32(0x000, 7, 5), 12);
    assert_eq!(combine_i32(0x200, -3, 5), -15);
    assert_eq!(combine_i32(0x300, 240, 51), 48);
    assert_eq!(combine_i32(0x600, -4, 9), 1);
    assert_eq!(combine_i32(0x900, 0, 7), 1);
    assert_eq!(combine_i8(0x000, 100, 100), -56);
    assert_eq!(combine_i64(0x000, 1 << 40, 5), 1_099_511_627_781);
}

#[test]
fn wine_reduction_float_oracle_rows() {
    assert_eq!(combine_f32(0x000, 1.5, 2.25).to_bits(), 0x4070_0000);
    assert_eq!(combine_f32(0x200, 1.5, -4.0).to_bits(), 0xc0c0_0000);
    assert_eq!(combine_f32(0x400, 3.0, 0.0).to_bits(), 0x4040_0000);
    assert_eq!(combine_f64(0x600, -0.0, 3.0).to_bits(), 0);
}

#[test]
fn wine_static_simple_oracle_rows() {
    assert_eq!(static_simple_init(10, 0, 2, false, 3, 1), (6, 4));
    assert_eq!(
        static_simple_init(0, 0xFFFF_FFFF, 1, true, 2, 1),
        (0, 0xFFFF_FFFF)
    );
    assert_eq!(static_simple_init_i8(100, 10, 3, false, 2, 0), (100, 55));
}

#[test]
fn wine_static_init_oracle_rows() {
    assert_eq!(
        static32(static_init(0, 10, 3, 4, 3, 1)),
        (0, Some((12, 21, 36, 0)))
    );
    assert_eq!(
        static32(static_init(10, 0, 2, 2, 2, 0)),
        (2, Some((10, 8, -8, 2)))
    );
    assert_eq!(static32(static_init(5, 5, 1, 1, 2, 1)), (0, None));
    assert_eq!(
        static64(static_init_i8(-50, 50, 10, 1, 3, 0)),
        (4, Some((-50, -50, 30, 50)))
    );
}

#[test]
fn wine_vect_oracle_rows() {
    assert_eq!(static_range(0, 7, 1, false, 3, 2), Some((5, 7)));
    let chunks: Vec<(i32, i32)> = dynamic_chunks(0, 10, 1, false, 2).collect();
    assert_eq!(chunks, [(0, 5), (5, 8), (8, 9), (9, 10)]);
}

#[test]
fn wine_dynamic_oracle_rows() {
    assert_eq!(
        drain(
            0x42,
            LoopBounds {
                first: 0,
                last: 9,
                step: 1
            },
            3,
            2
        ),
        [(0, 2), (3, 5), (6, 8), (9, 9)]
    );
    assert_eq!(
        drain(
            0x43,
            LoopBounds {
                first: 0,
                last: 9,
                step: 1
            },
            2,
            2
        ),
        [(0, 4), (5, 7), (8, 9)]
    );
    assert_eq!(
        drain(
            0x02,
            LoopBounds {
                first: 10,
                last: 0,
                step: 2
            },
            2,
            3
        ),
        [(10, 8), (6, 4), (2, 0)]
    );
}

#[test]
fn wine_single_oracle_rows() {
    let mut task = WorkshareTask::default();
    let mut a = WorkshareThread::default();
    let mut b = WorkshareThread::default();
    assert!(single_begin(&mut a, &mut task));
    assert!(!single_begin(&mut b, &mut task));
    assert!(single_begin(&mut a, &mut task));
}

#[test]
fn wine_sections_oracle_rows() {
    let mut task = WorkshareTask::default();
    let mut a = WorkshareThread::default();
    let mut b = WorkshareThread::default();
    sections_init(&mut a, &mut task, 3);
    let got: Vec<i32> = (0..4).map(|_| sections_next(&a, &mut task)).collect();
    assert_eq!(got, [0, 1, 2, -1]);
    sections_init(&mut b, &mut task, 3);
    assert_eq!(sections_next(&b, &mut task), -1);
    sections_init(&mut a, &mut task, 5);
    assert_eq!(sections_next(&a, &mut task), 0);
}
