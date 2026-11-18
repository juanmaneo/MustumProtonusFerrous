//! A converter with scripted results, standing in for libswresample in the tests. Its state is
//! per thread, so each test calls [`reset`] first.

use crate::format::AudioSpec;
use crate::resampler::Converter;
use std::cell::Cell;

thread_local! {
    static RESULT: Cell<(i32, i32)> = const { Cell::new((0, 0)) };
    static REFUSE: Cell<bool> = const { Cell::new(false) };
    static LAST: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
}

/// Clears the script: no results, no refusal and no recorded call.
pub fn reset() {
    RESULT.set((0, 0));
    REFUSE.set(false);
    LAST.set((0, 0));
}

/// Scripts the next `convert` to return `ret`, and `pending` to report `pending` buffered frames.
pub fn set(ret: i32, pending: i32) {
    RESULT.set((ret, pending));
}

/// Makes `new` refuse the layouts, as `swr_init` does when it fails.
pub fn set_refuse(refuse: bool) {
    REFUSE.set(refuse);
}

/// The `(out_samples, in_samples)` arguments of the last `convert`.
pub fn last_call() -> (usize, usize) {
    LAST.get()
}

/// The stub converter. It writes no samples, only the scripted counts.
pub struct Stub;

impl Converter for Stub {
    fn new(_input: &AudioSpec, _output: &AudioSpec) -> Option<Self> {
        (!REFUSE.get()).then_some(Stub)
    }

    fn next_pts(&mut self, pts: i64) -> i64 {
        pts
    }

    fn convert(
        &mut self,
        _out: &mut [u8],
        out_samples: usize,
        _input: &[u8],
        in_samples: usize,
    ) -> i32 {
        LAST.set((out_samples, in_samples));
        RESULT.get().0
    }

    fn pending(&self) -> i32 {
        RESULT.get().1
    }
}
