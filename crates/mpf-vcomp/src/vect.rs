//! The `C2VectParallel` checks and range split of Wine's `main.c`.
//!
//! `C2VectParallel` runs its callback over `start..end` split across the team. The C falls back
//! to one call over the whole range in some cases. The split done by `c2vectparallel_wrapper` is
//! ported as pure functions: [`static_range`] gives one thread its range, and [`dynamic_chunks`]
//! yields the chunks that the shared counter hands out, in order.

/// `MAX_VECT_PARALLEL_CALLBACK_ARGS`.
pub const MAX_CALLBACK_ARGS: i32 = 128;

/// The `nargs > MAX_VECT_PARALLEL_CALLBACK_ARGS` check. The C drops larger calls.
pub fn nargs_supported(nargs: i32) -> bool {
    nargs <= MAX_CALLBACK_ARGS
}

/// The serial-fallback test: the C runs the callback once over the whole range when the range is
/// reversed or empty, has fewer than two steps, or the team size is negative. The
/// `(end - start) / step` term wraps as it does in the C.
pub fn runs_serially(start: i32, end: i32, step: i32, thread_count: i32) -> bool {
    (step > 0 && end < start)
        || (step < 0 && end > start)
        || end.wrapping_sub(start).wrapping_div(step) < 2
        || thread_count < 0
}

fn step_sign(step: i32) -> i32 {
    if step > 0 {
        1
    } else {
        -1
    }
}

fn range_offset(step: i32, end_included: bool) -> i32 {
    step_sign(step) * i32::from(!end_included)
}

fn step_count(start: i32, end: i32, range_offset: i32, step: i32) -> u32 {
    let range = end
        .wrapping_sub(start)
        .wrapping_add(step)
        .wrapping_sub(range_offset);
    if step < 0 {
        (range.wrapping_neg() as u32) / (step.wrapping_neg() as u32)
    } else {
        (range as u32) / (step as u32)
    }
}

/// The static split of `c2vectparallel_wrapper` for `thread`: the `(first, last)` of its range,
/// in the same convention as the callback's arguments. The result is `None` for a thread past the
/// last step when the range is inclusive or has a step other than +-1, where the C makes no call.
/// For a unit step over an exclusive range, the C still calls the callback for those threads with
/// an empty range.
pub fn static_range(
    start: i32,
    end: i32,
    step: i32,
    end_included: bool,
    thread_count: u32,
    thread: u32,
) -> Option<(i32, i32)> {
    let adjust = range_offset(step, end_included);
    let count = step_count(start, end, adjust, step);
    if thread >= count && (end_included || (step != 1 && step != -1)) {
        return None;
    }
    let per_thread = count / thread_count;
    let remainder = count % thread_count;
    let (offset, steps) = if thread < remainder {
        (
            thread.wrapping_mul(per_thread.wrapping_add(1)),
            per_thread.wrapping_add(1),
        )
    } else if thread < count {
        (
            remainder.wrapping_add(per_thread.wrapping_mul(thread)),
            per_thread,
        )
    } else {
        (0, 0)
    };
    let first = start.wrapping_add((offset as i32).wrapping_mul(step));
    let last_step = offset.wrapping_add(steps).wrapping_sub(1) as i32;
    Some((
        first,
        start
            .wrapping_add(last_step.wrapping_mul(step))
            .wrapping_add(adjust),
    ))
}

/// The chunks of the dynamic split in `c2vectparallel_wrapper`, in the order the shared counter
/// hands them out. The C claims each chunk with a compare-exchange on the shared start. A single
/// iterator has no contention, so every claim succeeds.
pub struct DynamicChunks {
    start: i32,
    end: i32,
    end_value: i32,
    step: i32,
    step_sign: i32,
    range_offset: i32,
    thread_count: u32,
}

/// The dynamic split of `c2vectparallel_wrapper` over `start..end`, drawn by `thread_count`
/// threads.
pub fn dynamic_chunks(
    start: i32,
    end: i32,
    step: i32,
    end_included: bool,
    thread_count: u32,
) -> DynamicChunks {
    DynamicChunks {
        start,
        end,
        end_value: end.wrapping_add(i32::from(end_included).wrapping_mul(step)),
        step,
        step_sign: step_sign(step),
        range_offset: range_offset(step, end_included),
        thread_count,
    }
}

impl Iterator for DynamicChunks {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.start == self.end_value {
            return None;
        }
        let count = step_count(self.start, self.end, self.range_offset, self.step);
        let chunk_steps = count.wrapping_add(self.thread_count).wrapping_sub(1) / self.thread_count;
        let mut curr_end = self
            .start
            .wrapping_add((chunk_steps as i32).wrapping_mul(self.step))
            .wrapping_add(self.range_offset);
        let next_start = if curr_end.wrapping_sub(self.end).wrapping_mul(self.step_sign) > 0 {
            curr_end = self.end;
            self.end_value
        } else {
            let next_start = curr_end.wrapping_sub(self.range_offset);
            curr_end = curr_end.wrapping_sub(self.step);
            next_start
        };
        let start = self.start;
        self.start = next_start;
        Some((start, curr_end))
    }
}
