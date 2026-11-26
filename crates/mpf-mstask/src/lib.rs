//! Port of the portable logic in Wine's `dlls/mstask/task.c`, with the calendar conversions
//! from `dlls/ntdll/time.c` and `dlls/kernelbase/file.c`.
//!
//! Ported:
//! - `ITask::SetFlags` / `GetFlags`: [`Task::set_flags`], [`Task::get_flags`]
//! - `ITask::GetTriggerCount`, `CreateTrigger`, `DeleteTrigger`: [`Task::trigger_count`],
//!   [`Task::create_trigger`], [`Task::delete_trigger`]
//! - `task_set_trigger` / `task_get_trigger` (validation and copy rules):
//!   [`Task::set_trigger`], [`Task::get_trigger`]
//! - `MSTASK_ITask_GetNextRunTime` and its helpers: [`Task::next_run_time`]
//! - `RtlTimeToTimeFields`, `RtlTimeFieldsToTime`, `SystemTimeToFileTime`,
//!   `FileTimeToSystemTime`: [`system_time_to_filetime`], [`filetime_to_system_time`]
//!
//! Skipped:
//! - `GetTriggerString` (task.c and task_trigger.c), `GetRunTimes`, and every other `FIXME` stub.
//! - `.job` file load and save (`load_job_data`, `write_triggers`): a byte codec tied to file I/O.
//!   This is a follow-up candidate.
//! - Metadata setters and getters (comment, creator, application, parameters, working directory,
//!   account, idle wait, retry, priority, max run time, work item data): storage over Wine's
//!   `TaskImpl`, not run-time logic.
//! - COM plumbing: `QueryInterface`, `AddRef`/`Release`, `IPersistFile`, the `ITaskTrigger` wrapper,
//!   `TaskConstructor`, `TaskTriggerConstructor`.
//!
//! Deviations:
//! - When Wine's `SystemTimeToFileTime` or `FileTimeToSystemTime` fails, Wine keeps a stale or
//!   uninitialised value and carries on. Here the call returns `E_INVALIDARG` or `E_FAIL`. A
//!   candidate run time past 30828-09-14 is reported as `E_FAIL` instead of `S_OK` with the caller's
//!   `SYSTEMTIME` left unchanged.
//! - Wine's `RtlTimeFieldsToTime` multiplies by 10000 in signed 64-bit arithmetic, which overflows
//!   from year 30828 on. That is undefined behaviour in C. Here the multiply wraps.
//! - `is_leap_year` is kept as in Wine. It never changes a result, because February always takes 29 days.
//! - A weekly trigger with `weeks > 1`, a mask without Sunday and no end date stays on Sundays, as in
//!   Wine. Here the loop ends with `E_FAIL` once the FILETIME passes 2^63; Wine continues with a stale weekday.

mod calendar;
mod schedule;
mod trigger;

pub use calendar::{filetime_to_system_time, system_time_to_filetime, SystemTime};
pub use trigger::{TaskTrigger, TriggerType};

/// `HRESULT`, as used by Wine's `mstask`.
pub type HResult = i32;

/// `E_FAIL`.
pub const E_FAIL: HResult = 0x8000_4005_u32 as i32;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: HResult = 0x8007_0057_u32 as i32;
/// `SCHED_E_TRIGGER_NOT_FOUND`.
pub const SCHED_E_TRIGGER_NOT_FOUND: HResult = 0x8004_1309_u32 as i32;
/// `SCHED_S_TASK_DISABLED`.
pub const SCHED_S_TASK_DISABLED: HResult = 0x0004_1302;
/// `SCHED_S_TASK_NO_VALID_TRIGGERS`.
pub const SCHED_S_TASK_NO_VALID_TRIGGERS: HResult = 0x0004_1307;
/// `SCHED_S_EVENT_TRIGGER`.
pub const SCHED_S_EVENT_TRIGGER: HResult = 0x0004_1308;

/// `TASK_FLAG_DISABLED`, in the task flags.
pub const TASK_FLAG_DISABLED: u32 = 0x4;
/// `TASK_TRIGGER_FLAG_HAS_END_DATE`, in the trigger flags.
pub const TRIGGER_FLAG_HAS_END_DATE: u32 = 0x1;
/// `TASK_TRIGGER_FLAG_DISABLED`, in the trigger flags.
pub const TRIGGER_FLAG_DISABLED: u32 = 0x4;
/// `sizeof(TASK_TRIGGER)`, which is what `cbTriggerSize` must hold.
pub const TASK_TRIGGER_SIZE: u16 = 48;

/// The state of one `ITask` object: its flags and trigger list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Task {
    flags: u32,
    triggers: Vec<TaskTrigger>,
}

impl Task {
    /// `MSTASK_ITask_SetFlags`: replaces the low 15 bits and keeps the rest.
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = (self.flags & 0xffff_8000) | (flags & 0x7fff);
    }

    /// `MSTASK_ITask_GetFlags`.
    pub fn get_flags(&self) -> u32 {
        self.flags & 0xffff
    }

    /// `MSTASK_ITask_GetTriggerCount`.
    pub fn trigger_count(&self) -> usize {
        self.triggers.len()
    }

    /// `MSTASK_ITask_CreateTrigger`, with `now` in place of `GetLocalTime`. The index
    /// wraps to 16 bits, as Wine's `WORD *idx` does.
    pub fn create_trigger(&mut self, now: &SystemTime) -> u16 {
        let index = self.triggers.len() as u16;
        self.triggers.push(trigger::default_trigger(now));
        index
    }

    /// `MSTASK_ITask_DeleteTrigger`.
    pub fn delete_trigger(&mut self, index: u16) -> Result<(), HResult> {
        let index = usize::from(index);
        if index >= self.triggers.len() {
            return Err(SCHED_E_TRIGGER_NOT_FOUND);
        }
        self.triggers.remove(index);
        Ok(())
    }

    /// `task_set_trigger`: validates `trigger` and stores it at `index`.
    pub fn set_trigger(&mut self, index: u16, trigger: &TaskTrigger) -> Result<(), HResult> {
        let slot = self.triggers.get_mut(usize::from(index)).ok_or(E_FAIL)?;
        trigger::check_trigger(trigger)?;
        *slot = *trigger;
        Ok(())
    }

    /// `task_get_trigger`.
    pub fn get_trigger(&self, index: u16) -> Result<TaskTrigger, HResult> {
        self.triggers
            .get(usize::from(index))
            .copied()
            .ok_or(SCHED_E_TRIGGER_NOT_FOUND)
    }

    /// `MSTASK_ITask_GetNextRunTime`, with `now` in place of `GetLocalTime`.
    pub fn next_run_time(&self, now: &SystemTime) -> Result<SystemTime, HResult> {
        schedule::next_run_time(self.flags, &self.triggers, now)
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
