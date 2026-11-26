/// The value `init_thread.c` stores in `*once` while one thread runs the initializer.
const IN_PROGRESS: i32 = -1;

/// What [`InitThreadState::header`] tells the caller to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderOutcome {
    /// `*once` was 0. It is now -1, so the caller runs the initializer and then calls `footer` or `abort`.
    Claimed,
    /// `*once` held a completed epoch. The thread epoch is now the global epoch, and no initializer runs.
    Done,
    /// `*once` is -1, so another thread is initializing. The C loop waits on the condition variable and
    /// calls `_Init_thread_header` again. The port returns this without changing the state, so the caller can wait.
    Wait,
}

/// The epoch counters of `init_thread.c`: `_Init_global_epoch` and the calling thread's `_Init_thread_epoch`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitThreadState {
    pub global_epoch: i32,
    pub thread_epoch: i32,
}

impl Default for InitThreadState {
    /// Both counters start at `INT_MIN`, as in `init_thread.c`.
    fn default() -> Self {
        Self {
            global_epoch: i32::MIN,
            thread_epoch: i32::MIN,
        }
    }
}

impl InitThreadState {
    /// `_Init_thread_header` without the SRW lock, which the caller holds.
    pub fn header(&mut self, once: &mut i32) -> HeaderOutcome {
        if *once == IN_PROGRESS {
            HeaderOutcome::Wait
        } else if *once != 0 {
            self.thread_epoch = self.global_epoch;
            HeaderOutcome::Done
        } else {
            *once = IN_PROGRESS;
            HeaderOutcome::Claimed
        }
    }

    /// `_Init_thread_footer` without the lock. The global epoch wraps, where the C `++` is undefined
    /// on overflow; see edge_checks.rs.
    pub fn footer(&mut self, once: &mut i32) {
        self.global_epoch = self.global_epoch.wrapping_add(1);
        self.thread_epoch = self.global_epoch;
        *once = self.global_epoch;
    }

    /// `_Init_thread_abort` without the lock: resets `*once` so that the next caller can run the initializer.
    pub fn abort(&mut self, once: &mut i32) {
        *once = 0;
    }
}
