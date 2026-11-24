//! Loop schedulers of Wine's `main.c`: the static and dynamic `for` loop bounds.
//!
//! The static functions compute one thread's share from its thread number. The dynamic functions
//! keep a per-thread handle and a team-wide task record. The caller provides the mutual exclusion
//! that Wine takes with `vcomp_section`.

use crate::{is_after, DYNAMIC_FLAGS_CHUNKED, DYNAMIC_FLAGS_INCREMENT, DYNAMIC_FLAGS_STATIC};

/// One thread's block when `iterations` are split statically: `[begin, end]` in the loop's
/// step, or `None` when the thread gets no iterations.
fn block(
    first: u64,
    iterations: u64,
    step: u64,
    num_threads: u64,
    thread_num: u64,
) -> Option<(u64, u64)> {
    let per_thread = iterations / num_threads;
    let remaining = iterations - per_thread * num_threads;
    let (first, count) = if thread_num < remaining {
        (first, per_thread + 1)
    } else if per_thread != 0 {
        (first.wrapping_add(remaining.wrapping_mul(step)), per_thread)
    } else {
        return None;
    };
    let begin = first.wrapping_add(thread_num.wrapping_mul(count).wrapping_mul(step));
    let end = begin.wrapping_add((count - 1).wrapping_mul(step));
    Some((begin, end))
}

macro_rules! define_static_simple {
    ($name:ident, $u:ty, $s:ty) => {
        /// `_vcomp_for_static_simple_init{,_i8}`: the `(begin, end)` of `thread_num`. Counting
        /// wraps in `$u` as it does in C.
        pub fn $name(
            first: $u,
            last: $u,
            step: $s,
            increment: bool,
            num_threads: u32,
            thread_num: u32,
        ) -> ($u, $u) {
            if num_threads == 1 {
                return (first, last);
            }
            if step <= 0 {
                return (0, if increment { <$u>::MAX } else { 1 });
            }
            let magnitude = step as $u;
            let (span, step) = if increment {
                (last.wrapping_sub(first), magnitude)
            } else {
                (first.wrapping_sub(last), magnitude.wrapping_neg())
            };
            let iterations = (span / magnitude).wrapping_add(1);
            match block(
                first as u64,
                iterations as u64,
                step as u64,
                u64::from(num_threads),
                u64::from(thread_num),
            ) {
                Some((begin, end)) => (begin as $u, end as $u),
                None => (first, first.wrapping_sub(step)),
            }
        }
    };
}

define_static_simple!(static_simple_init, u32, i32);
define_static_simple!(static_simple_init_i8, u64, i64);

/// The chunks of one thread, as set by `_vcomp_for_static_init{,_i8}`. `loops` is the number of
/// chunks; `range` is `None` when the C leaves the bounds untouched.
pub struct StaticChunks<U, S> {
    pub loops: U,
    pub range: Option<ChunkRange<S>>,
}

/// The first chunk of a thread, the distance to its next chunk, and the last chunk of the loop.
pub struct ChunkRange<S> {
    pub begin: S,
    pub end: S,
    pub next: S,
    pub lastchunk: S,
}

macro_rules! define_static_init {
    ($name:ident, $u:ty, $s:ty) => {
        /// `_vcomp_for_static_init{,_i8}`: the chunk count and chunk bounds of `thread_num`.
        pub fn $name(
            first: $s,
            last: $s,
            step: $s,
            chunksize: $s,
            num_threads: u32,
            thread_num: u32,
        ) -> StaticChunks<$u, $s> {
            if num_threads == 1 && chunksize != 1 {
                return StaticChunks {
                    loops: 1,
                    range: Some(ChunkRange {
                        begin: first,
                        end: last,
                        next: 0,
                        lastchunk: first,
                    }),
                };
            }
            if first == last {
                let owner = thread_num == 0;
                return StaticChunks {
                    loops: <$u>::from(owner),
                    range: owner.then_some(ChunkRange {
                        begin: first,
                        end: last,
                        next: 0,
                        lastchunk: first,
                    }),
                };
            }
            if step <= 0 {
                return StaticChunks {
                    loops: 0,
                    range: None,
                };
            }
            let distance = (i128::from(last) - i128::from(first)).unsigned_abs();
            let iterations = (distance / u128::from(step.unsigned_abs()) + 1) as u64;
            let step = if first < last { step } else { -step };
            let chunksize = chunksize.max(1);
            let num_chunks = iterations.div_ceil(chunksize as u64) as $u;
            let threads = num_threads as $u;
            let per_thread = num_chunks / threads;
            let remaining = num_chunks - per_thread * threads;
            let begin = first.wrapping_add(
                (thread_num as $s)
                    .wrapping_mul(chunksize)
                    .wrapping_mul(step),
            );
            let end = begin.wrapping_add((chunksize - 1).wrapping_mul(step));
            let next = chunksize.wrapping_mul(num_threads as $s).wrapping_mul(step);
            let lastchunk = first.wrapping_add(
                (num_chunks.wrapping_sub(1) as $s)
                    .wrapping_mul(chunksize)
                    .wrapping_mul(step),
            );
            StaticChunks {
                loops: per_thread + <$u>::from((thread_num as $u) < remaining),
                range: Some(ChunkRange {
                    begin,
                    end,
                    next,
                    lastchunk,
                }),
            }
        }
    };
}

define_static_init!(static_init, u32, i32);
define_static_init!(static_init_i8, u64, i64);

/// The per-thread dynamic state (`thread_data->dynamic*`). A new thread starts at generation 1.
pub struct ThreadDynamic {
    pub generation: u32,
    pub pending: Pending,
}

impl Default for ThreadDynamic {
    fn default() -> Self {
        Self {
            generation: 1,
            pending: Pending::Idle,
        }
    }
}

/// What `_vcomp_for_dynamic_next` hands this thread: nothing, one static block, or a share of
/// the team's chunked or guided loop.
pub enum Pending {
    Idle,
    Static { begin: u32, end: u32 },
    Chunked,
    Guided,
}

/// The loop state shared by the team (`task_data->dynamic*`).
#[derive(Default)]
pub struct DynamicTask {
    pub generation: u32,
    pub first: u32,
    pub last: u32,
    pub iterations: u32,
    pub step: i32,
    pub chunksize: u32,
}

/// The `first`, `last` and `step` of a `for` loop.
pub struct LoopBounds {
    pub first: u32,
    pub last: u32,
    pub step: i32,
}

/// `_vcomp_for_dynamic_init`: records how this thread will take its iterations, and publishes a
/// new team loop when this thread's generation is ahead of the task's.
pub fn dynamic_init(
    thread: &mut ThreadDynamic,
    task: &mut DynamicTask,
    flags: u32,
    bounds: LoopBounds,
    chunksize: u32,
    num_threads: u32,
    thread_num: u32,
) {
    let LoopBounds { first, last, step } = bounds;
    if step <= 0 {
        thread.pending = Pending::Idle;
        return;
    }
    let magnitude = step as u32;
    let (span, step) = if (flags & DYNAMIC_FLAGS_INCREMENT) != 0 {
        (last.wrapping_sub(first), step)
    } else {
        (first.wrapping_sub(last), step.wrapping_neg())
    };
    let iterations = (span / magnitude).wrapping_add(1);
    let kind = flags & !DYNAMIC_FLAGS_INCREMENT;
    if kind == DYNAMIC_FLAGS_STATIC {
        thread.pending = match block(
            u64::from(first),
            u64::from(iterations),
            u64::from(step as u32),
            u64::from(num_threads),
            u64::from(thread_num),
        ) {
            Some((begin, end)) => Pending::Static {
                begin: begin as u32,
                end: end as u32,
            },
            None => Pending::Idle,
        };
        return;
    }
    // Unknown kinds fall back to guided, as the C does.
    thread.pending = if kind == DYNAMIC_FLAGS_CHUNKED {
        Pending::Chunked
    } else {
        Pending::Guided
    };
    thread.generation = thread.generation.wrapping_add(1);
    if is_after(thread.generation, task.generation) {
        *task = DynamicTask {
            generation: thread.generation,
            first,
            last,
            iterations,
            step,
            chunksize,
        };
    }
}

/// `_vcomp_for_dynamic_next`: the next `(begin, end)` for this thread, or `None` once it has no
/// more iterations.
pub fn dynamic_next(
    thread: &mut ThreadDynamic,
    task: &mut DynamicTask,
    num_threads: u32,
) -> Option<(u32, u32)> {
    match thread.pending {
        Pending::Idle => None,
        Pending::Static { begin, end } => {
            thread.pending = Pending::Idle;
            Some((begin, end))
        }
        Pending::Chunked | Pending::Guided => {
            let guided = matches!(thread.pending, Pending::Guided);
            if thread.generation != task.generation || task.iterations == 0 {
                return None;
            }
            let mut iterations = task.iterations.min(task.chunksize);
            if guided && task.iterations > num_threads.wrapping_mul(task.chunksize) {
                iterations =
                    task.iterations.wrapping_add(num_threads).wrapping_sub(1) / num_threads;
            }
            if iterations == 0 {
                return None;
            }
            let stride = task.step as u32;
            let begin = task.first;
            let mut end = begin.wrapping_add((iterations - 1).wrapping_mul(stride));
            task.iterations = task.iterations.wrapping_sub(iterations);
            task.first = task.first.wrapping_add(iterations.wrapping_mul(stride));
            if task.iterations == 0 {
                end = task.last;
            }
            Some((begin, end))
        }
    }
}
