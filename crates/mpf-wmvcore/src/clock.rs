//! The clock fields of Wine's async reader and the wait timeout they give.

/// `INFINITE`: the wait for a sample that is not yet due on a user clock.
pub const INFINITE: u32 = u32::MAX;

/// The clock fields of `struct async_reader` that the wait timeout reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReaderClock {
    /// `user_clock`: the application supplies the time with `DeliverTime`.
    pub user_clock: bool,
    /// `user_time`: the last time the application delivered, in 100 ns units.
    pub user_time: u64,
    /// `clock_start`: the counter time when the reader started, in 100 ns units.
    pub clock_start: i64,
}

impl ReaderClock {
    /// `async_reader_get_wait_timeout`: the number of milliseconds to wait
    /// before a sample at `pts` (in 100 ns units) is due, or 0 when it is due.
    ///
    /// `now` is `get_current_time` of the performance counter. It is not read
    /// when `user_clock` is set.
    pub fn wait_timeout(&self, now: i64, pts: u64) -> u32 {
        if self.user_clock {
            return if pts > self.user_time { INFINITE } else { 0 };
        }
        let current = now.wrapping_sub(self.clock_start) as u64;
        if pts > current {
            ((pts - current) / 10_000) as u32
        } else {
            0
        }
    }
}
