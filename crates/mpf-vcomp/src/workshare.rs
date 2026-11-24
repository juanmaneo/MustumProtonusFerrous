//! The `single` and `sections` counters of Wine's `main.c`: `_vcomp_single_begin`,
//! `_vcomp_sections_init` and `_vcomp_sections_next`.
//!
//! Each thread counts the constructs it has passed. The first thread to reach a construct
//! claims it for the team, by the wrap-aware ordering of [`is_after`].

use crate::is_after;

/// The per-thread `single` and `section` counters. A new thread starts at 1, so its first
/// construct is one ahead of the team's initial value of 0.
pub struct WorkshareThread {
    pub single: u32,
    pub section: u32,
}

impl Default for WorkshareThread {
    fn default() -> Self {
        Self {
            single: 1,
            section: 1,
        }
    }
}

/// The team-wide `single` and `section` state (`task_data`).
#[derive(Default)]
pub struct WorkshareTask {
    pub single: u32,
    pub section: u32,
    pub num_sections: i32,
    pub section_index: i32,
}

/// `_vcomp_single_begin`: true for the first thread to reach this `single` construct.
pub fn single_begin(thread: &mut WorkshareThread, task: &mut WorkshareTask) -> bool {
    thread.single = thread.single.wrapping_add(1);
    if is_after(thread.single, task.single) {
        task.single = thread.single;
        true
    } else {
        false
    }
}

/// `_vcomp_sections_init`: publishes a `sections` construct of `count` sections when this thread
/// is the first to reach it.
pub fn sections_init(thread: &mut WorkshareThread, task: &mut WorkshareTask, count: i32) {
    thread.section = thread.section.wrapping_add(1);
    if is_after(thread.section, task.section) {
        task.section = thread.section;
        task.num_sections = count;
        task.section_index = 0;
    }
}

/// `_vcomp_sections_next`: the next section index for this thread, or -1 when none is left or the
/// thread is not in the published construct.
pub fn sections_next(thread: &WorkshareThread, task: &mut WorkshareTask) -> i32 {
    if thread.section == task.section && task.section_index != task.num_sections {
        let index = task.section_index;
        task.section_index = index.wrapping_add(1);
        index
    } else {
        -1
    }
}
