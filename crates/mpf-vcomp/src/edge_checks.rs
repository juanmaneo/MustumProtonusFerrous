//! Edge checks for the inputs the oracle rows do not reach: every reduction slot at each integer
//! and float width, the wrapping cases the sanitizers flag (see `oracle_checks`), degenerate loop
//! bounds, and the dynamic and workshare state machines. The expected values follow the C in
//! Wine's `main.c`.

use crate::is_after;
use crate::oracle_checks::{drain, static32, static64};
use crate::reduction::{
    combine_f32, combine_f64, combine_i16, combine_i32, combine_i64, combine_i8,
};
use crate::schedule::{
    dynamic_init, dynamic_next, static_init, static_init_i8, static_simple_init,
    static_simple_init_i8, DynamicTask, LoopBounds, Pending, ThreadDynamic,
};
use crate::vect::{dynamic_chunks, nargs_supported, runs_serially, static_range};
use crate::workshare::{
    sections_init, sections_next, single_begin, WorkshareTask, WorkshareThread,
};

macro_rules! int_sweep {
    ($name:ident, $combine:ident, $t:ty) => {
        #[test]
        fn $name() {
            let (a, b): ($t, $t) = (5, 3);
            assert_eq!($combine(0x100, a, b), 8);
            assert_eq!($combine(0x200, a, b), 15);
            assert_eq!($combine(0x300, a, b), 1);
            assert_eq!($combine(0x400, a, b), 7);
            assert_eq!($combine(0x500, a, b), 6);
            assert_eq!($combine(0x600, a, b), 1);
            assert_eq!($combine(0x700, a, b), 5);
            assert_eq!($combine(0x700, 0, b), 1);
            assert_eq!($combine(0x800, 0, 0), 0);
        }
    };
}

int_sweep!(reduction_sweep_i8, combine_i8, i8);
int_sweep!(reduction_sweep_i16, combine_i16, i16);
int_sweep!(reduction_sweep_i32, combine_i32, i32);
int_sweep!(reduction_sweep_i64, combine_i64, i64);

#[test]
fn reduction_int_edges() {
    assert_eq!(combine_i8(0x200, 16, 16), 0);
    assert_eq!(combine_i16(0x200, 300, 300), 24464);
    assert_eq!(combine_i32(0x600, 0, 5), 0);
    assert_eq!(combine_i8(0x700, -3, 0), -3);
    assert_eq!(combine_i16(0x500, 240, 60), 204);
    assert_eq!(combine_i8(0x300, 0x0F, 0x3C), 12);
    assert_eq!(combine_i8(0x400, 0x0F, 0x30), 63);
    assert_eq!(combine_i64(0x500, 0xFF, 0x0F), 240);
    assert_eq!(combine_i16(0x600, 2, -1), 1);
    assert_eq!(combine_i64(0x700, 0, 0), 0);
    assert_eq!(combine_i32(0x800, 0, 0), 0);
    assert_eq!(combine_i32(0xF00, 0, 4), 1);
    assert_eq!(combine_i32(0x800, 2, 0), 2);
    assert_eq!(combine_i32(0x100, 2, 3), 5);
    assert_eq!(combine_i32(0x0FF, 2, 3), 5);
    assert_eq!(combine_i32(0x1000, 2, 3), 5);
}

// The oracle flags the signed overflow in `_vcomp_atomic_mul_i4` for this input. The port wraps.
#[test]
fn reduction_mul_wraps_like_c() {
    assert_eq!(combine_i32(0x200, 0x4000_0000, 4), 0);
}

#[test]
fn reduction_float_edges() {
    assert_eq!(combine_f32(0x000, 0.5, 0.25).to_bits(), 0.75f32.to_bits());
    assert_eq!(combine_f32(0x200, -2.0, 3.0).to_bits(), (-6.0f32).to_bits());
    assert_eq!(combine_f64(0x000, 1.0, 2.0).to_bits(), 3.0f64.to_bits());
    assert_eq!(combine_f64(0x200, 1.5, 4.0).to_bits(), 6.0f64.to_bits());
    assert_eq!(combine_f64(0x600, 1.0, 0.0).to_bits(), 0.0f64.to_bits());
    assert_eq!(combine_f64(0x600, 1.0, -2.5).to_bits(), 1.0f64.to_bits());
    assert_eq!(combine_f32(0x300, 0.0, 0.0).to_bits(), 0);
    assert_eq!(combine_f32(0x300, -0.0, 0.0).to_bits(), 0);
    assert!(combine_f32(0x300, f32::NAN, 0.0).is_nan());
    assert_eq!(combine_f64(0x600, f64::NAN, 0.0).to_bits(), 0);
    assert_eq!(
        combine_f64(0x600, f64::NAN, f64::NAN).to_bits(),
        1.0f64.to_bits()
    );
    assert_eq!(combine_f32(0x700, 0.0, 2.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(combine_f64(0x300, 0.0, 2.0).to_bits(), 1.0f64.to_bits());
    assert_eq!(combine_f32(0x800, 0.0, 0.0).to_bits(), 0);
    assert_eq!(combine_f32(0x600, 1.0, 1.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(combine_f32(0x600, 0.0, 1.0).to_bits(), 0);
    assert_eq!(combine_f64(0x700, 2.5, 0.0).to_bits(), 2.5f64.to_bits());
    assert_eq!(combine_f64(0x300, 0.0, 0.0).to_bits(), 0);
}

#[test]
fn static_simple_u32_edges() {
    assert_eq!(static_simple_init(5, 9, 1, true, 1, 0), (5, 9));
    assert_eq!(static_simple_init(9, 5, 1, false, 1, 0), (9, 5));
    assert_eq!(static_simple_init(0, 9, -3, false, 2, 1), (0, 1));
    assert_eq!(static_simple_init(0, 9, 0, true, 2, 1), (0, u32::MAX));
    assert_eq!(static_simple_init(0, 0, 1, true, 2, 1), (0, u32::MAX));
    assert_eq!(static_simple_init(0, 0, 1, false, 2, 1), (0, 1));
    assert_eq!(static_simple_init(0, 4, 1, true, 3, 2), (4, 4));
    assert_eq!(static_simple_init(0, 4, 1, true, 3, 0), (0, 1));
    assert_eq!(static_simple_init(10, 0, 3, false, 3, 2), (1, 1));
}

#[test]
fn static_simple_i8_edges() {
    assert_eq!(static_simple_init_i8(0, 4, 1, true, 3, 2), (4, 4));
    assert_eq!(static_simple_init_i8(0, 4, 1, true, 3, 0), (0, 1));
    assert_eq!(static_simple_init_i8(5, 9, 1, true, 1, 0), (5, 9));
    assert_eq!(static_simple_init_i8(0, 0, 1, true, 2, 1), (0, u64::MAX));
    assert_eq!(static_simple_init_i8(5, 5, 0, true, 2, 0), (0, u64::MAX));
    assert_eq!(static_simple_init_i8(9, 0, -1, false, 2, 0), (0, 1));
    assert_eq!(static_simple_init_i8(10, 0, 3, false, 3, 2), (1, 1));
}

#[test]
fn static_init_u32_edges() {
    assert_eq!(
        static32(static_init(0, 9, 1, 1, 1, 0)),
        (10, Some((0, 0, 1, 9)))
    );
    assert_eq!(
        static32(static_init(0, 9, 1, 3, 1, 0)),
        (1, Some((0, 9, 0, 0)))
    );
    assert_eq!(
        static32(static_init(0, 9, 1, 0, 1, 0)),
        (1, Some((0, 9, 0, 0)))
    );
    assert_eq!(
        static32(static_init(7, 7, 1, 1, 3, 0)),
        (1, Some((7, 7, 0, 7)))
    );
    assert_eq!(static32(static_init(7, 7, 1, 1, 3, 1)), (0, None));
    assert_eq!(static32(static_init(0, 9, -1, 1, 3, 0)), (0, None));
    assert_eq!(static32(static_init(9, 0, 0, 1, 3, 0)), (0, None));
    assert_eq!(
        static32(static_init(9, 0, 1, 2, 2, 1)),
        (2, Some((7, 6, -4, 1)))
    );
    assert_eq!(
        static32(static_init(9, 0, 1, 2, 2, 0)),
        (3, Some((9, 8, -4, 1)))
    );
    assert_eq!(
        static32(static_init(0, 9, 1, 4, 3, 2)),
        (1, Some((8, 11, 12, 8)))
    );
    assert_eq!(
        static32(static_init(0, 9, 1, 0, 3, 2)),
        (3, Some((2, 2, 3, 9)))
    );
}

#[test]
fn static_init_i8_edges() {
    assert_eq!(
        static64(static_init_i8(0, 9, 1, 1, 1, 0)),
        (10, Some((0, 0, 1, 9)))
    );
    assert_eq!(
        static64(static_init_i8(0, 9, 1, 3, 1, 0)),
        (1, Some((0, 9, 0, 0)))
    );
    assert_eq!(
        static64(static_init_i8(7, 7, 1, 1, 3, 0)),
        (1, Some((7, 7, 0, 7)))
    );
    assert_eq!(static64(static_init_i8(7, 7, 1, 1, 3, 1)), (0, None));
    assert_eq!(static64(static_init_i8(0, 9, -1, 1, 3, 0)), (0, None));
    assert_eq!(
        static64(static_init_i8(9, 0, 1, 2, 2, 1)),
        (2, Some((7, 6, -4, 1)))
    );
}

fn bounds(first: u32, last: u32, step: i32) -> LoopBounds {
    LoopBounds { first, last, step }
}

// `(kind, begin, end)`: 0 idle, 1 static block, 2 chunked, 3 guided.
fn pending_of(thread: &ThreadDynamic) -> (u8, u32, u32) {
    match thread.pending {
        Pending::Idle => (0, 0, 0),
        Pending::Static { begin, end } => (1, begin, end),
        Pending::Chunked => (2, 0, 0),
        Pending::Guided => (3, 0, 0),
    }
}

#[test]
fn dynamic_init_step_not_positive_is_idle() {
    let mut thread = ThreadDynamic {
        generation: 1,
        pending: Pending::Chunked,
    };
    let mut task = DynamicTask::default();
    dynamic_init(&mut thread, &mut task, 0x42, bounds(0, 9, 0), 0, 2, 0);
    assert_eq!(pending_of(&thread), (0, 0, 0));
    assert_eq!(thread.generation, 1);
    assert_eq!(task.generation, 0);
}

#[test]
fn dynamic_static_block_by_thread() {
    let mut thread = ThreadDynamic::default();
    let mut task = DynamicTask::default();
    dynamic_init(&mut thread, &mut task, 0x01, bounds(0, 0, 1), 0, 2, 1);
    assert_eq!(pending_of(&thread), (0, 0, 0));

    dynamic_init(&mut thread, &mut task, 0x01, bounds(0, 0, 1), 0, 2, 0);
    assert_eq!(pending_of(&thread), (1, 0, 0));
    assert_eq!(dynamic_next(&mut thread, &mut task, 2), Some((0, 0)));
    assert_eq!(dynamic_next(&mut thread, &mut task, 2), None);
}

#[test]
fn dynamic_guided_publishes_task() {
    let mut thread = ThreadDynamic::default();
    let mut task = DynamicTask::default();
    dynamic_init(&mut thread, &mut task, 0x44, bounds(0, 9, 1), 2, 2, 0);
    assert_eq!(pending_of(&thread), (3, 0, 0));
    assert_eq!(thread.generation, 2);
    assert_eq!(task.generation, 2);
    assert_eq!(task.iterations, 10);
    assert_eq!(task.step, 1);
    assert_eq!(task.chunksize, 2);
}

#[test]
fn dynamic_init_keeps_newer_task() {
    let mut thread = ThreadDynamic::default();
    let mut task = DynamicTask {
        generation: 5,
        first: 7,
        last: 8,
        iterations: 1,
        step: 1,
        chunksize: 1,
    };
    dynamic_init(&mut thread, &mut task, 0x02, bounds(0, 9, 1), 1, 2, 0);
    assert_eq!(pending_of(&thread), (2, 0, 0));
    assert_eq!(thread.generation, 2);
    assert_eq!((task.generation, task.first, task.iterations), (5, 7, 1));
}

#[test]
fn dynamic_init_generation_wraps() {
    let mut thread = ThreadDynamic {
        generation: u32::MAX,
        pending: Pending::Idle,
    };
    let mut task = DynamicTask {
        generation: u32::MAX,
        ..DynamicTask::default()
    };
    dynamic_init(&mut thread, &mut task, 0x42, bounds(0, 9, 1), 1, 2, 0);
    assert_eq!(thread.generation, 0);
    assert_eq!(task.generation, 0);
    assert_eq!(task.iterations, 10);
}

#[test]
fn dynamic_next_edges() {
    let mut thread = ThreadDynamic::default();
    let mut task = DynamicTask::default();
    dynamic_init(&mut thread, &mut task, 0x42, bounds(0, 9, 1), 0, 2, 0);
    assert_eq!(dynamic_next(&mut thread, &mut task, 2), None);
    assert_eq!(task.iterations, 10);

    assert_eq!(drain(0x44, bounds(0, 9, 1), 10, 2), [(0, 9)]);
    assert_eq!(
        drain(0x43, bounds(0, 9, 1), 0, 2),
        [(0, 4), (5, 7), (8, 8), (9, 9)]
    );
    assert_eq!(drain(0x42, bounds(0, 10, 3), 4, 1), [(0, 10)]);

    let mut thread = ThreadDynamic {
        generation: 1,
        pending: Pending::Chunked,
    };
    let mut task = DynamicTask {
        generation: 2,
        first: 0,
        last: 9,
        iterations: 10,
        step: 1,
        chunksize: 4,
    };
    assert_eq!(dynamic_next(&mut thread, &mut task, 2), None);
    assert_eq!((task.iterations, task.first), (10, 0));
}

#[test]
fn dynamic_threads_share_one_task() {
    let mut task = DynamicTask::default();
    let mut a = ThreadDynamic::default();
    let mut b = ThreadDynamic::default();
    dynamic_init(&mut a, &mut task, 0x42, bounds(0, 9, 1), 4, 2, 0);
    dynamic_init(&mut b, &mut task, 0x42, bounds(0, 9, 1), 4, 2, 1);
    assert_eq!(dynamic_next(&mut a, &mut task, 2), Some((0, 3)));
    assert_eq!(dynamic_next(&mut b, &mut task, 2), Some((4, 7)));
    assert_eq!(dynamic_next(&mut a, &mut task, 2), Some((8, 9)));
    assert_eq!(dynamic_next(&mut b, &mut task, 2), None);
}

#[test]
fn vect_nargs_and_serial_edges() {
    assert!(nargs_supported(128));
    assert!(!nargs_supported(129));
    assert!(nargs_supported(-1));
    assert!(!runs_serially(0, 10, 1, 2));
    assert!(runs_serially(10, 0, 1, 2));
    assert!(runs_serially(0, 10, -1, 2));
    assert!(!runs_serially(10, 0, -1, 2));
    assert!(runs_serially(0, 1, 1, 2));
    assert!(!runs_serially(0, 4, 2, 2));
    assert!(runs_serially(0, 3, 2, 2));
    assert!(runs_serially(0, 10, 1, -1));
    assert!(!runs_serially(0, 10, 1, 0));
    // The oracle flags the signed overflow in this predicate. The port wraps: `end - start` is
    // -2, so the `< 2` test takes the serial path.
    assert!(runs_serially(-2147483647, 2147483647, 1, 2));
}

#[test]
fn vect_static_range_edges() {
    assert_eq!(static_range(0, 10, 1, true, 2, 1), Some((6, 10)));
    assert_eq!(static_range(0, 2, 2, true, 4, 3), None);
    assert_eq!(static_range(0, 2, 2, false, 4, 3), None);
    assert_eq!(static_range(0, 2, 1, false, 4, 3), Some((0, 0)));
    assert_eq!(static_range(10, 0, -1, false, 2, 1), Some((5, 0)));
    assert_eq!(static_range(10, 0, -1, false, 2, 0), Some((10, 5)));
    assert_eq!(static_range(0, 3, 1, false, 2, 0), Some((0, 2)));
    assert_eq!(static_range(0, 3, 1, false, 2, 1), Some((2, 3)));
    assert_eq!(static_range(0, 0, 1, false, 2, 1), Some((0, 0)));
    assert_eq!(static_range(0, 0, -1, false, 2, 1), Some((0, 0)));
}

fn chunks(
    start: i32,
    end: i32,
    step: i32,
    end_included: bool,
    thread_count: u32,
) -> Vec<(i32, i32)> {
    dynamic_chunks(start, end, step, end_included, thread_count).collect()
}

#[test]
fn vect_dynamic_chunks_edges() {
    assert_eq!(chunks(0, 9, 3, true, 4), [(0, 0), (3, 3), (6, 6), (9, 9)]);
    assert_eq!(chunks(5, 5, 1, false, 2), []);
    assert_eq!(chunks(0, 0, 1, true, 2), [(0, 0)]);
    assert_eq!(chunks(10, 0, -1, false, 1), [(10, 0)]);
    assert_eq!(
        chunks(0, 10, 1, true, 3),
        [(0, 3), (4, 6), (7, 8), (9, 9), (10, 10)]
    );
}

#[test]
fn workshare_single_and_sections_edges() {
    let mut task = WorkshareTask::default();
    let mut a = WorkshareThread::default();
    let mut b = WorkshareThread::default();
    assert!(single_begin(&mut a, &mut task));
    assert!(single_begin(&mut a, &mut task));
    assert!(!single_begin(&mut b, &mut task));

    let mut task = WorkshareTask::default();
    let mut a = WorkshareThread::default();
    sections_init(&mut a, &mut task, 0);
    assert_eq!(sections_next(&a, &mut task), -1);

    let thread = WorkshareThread::default();
    let mut task = WorkshareTask::default();
    assert_eq!(sections_next(&thread, &mut task), -1);
    assert_eq!(task.section_index, 0);

    let mut task = WorkshareTask::default();
    let mut a = WorkshareThread::default();
    sections_init(&mut a, &mut task, -1);
    assert_eq!(sections_next(&a, &mut task), 0);
    assert_eq!(sections_next(&a, &mut task), 1);
}

#[test]
fn is_after_wraps() {
    assert!(is_after(0, u32::MAX));
    assert!(!is_after(5, 5));
}
