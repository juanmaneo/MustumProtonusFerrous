//! `TASK_TRIGGER` data and its validation, from `task_set_trigger` in Wine's `dlls/mstask/task.c`.

use crate::calendar::{date_time_to_filetime, SystemTime};
use crate::{
    HResult, E_INVALIDARG, TASK_TRIGGER_SIZE, TRIGGER_FLAG_DISABLED, TRIGGER_FLAG_HAS_END_DATE,
};

/// `TASK_TRIGGER_TYPE` with the fields of its `Type` union member.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TriggerType {
    #[default]
    Once,
    Daily {
        days_interval: u16,
    },
    Weekly {
        weeks_interval: u16,
        days_of_week: u16,
    },
    MonthlyDate {
        days: u32,
        months: u16,
    },
    MonthlyDow {
        which_week: u16,
        days_of_week: u16,
        months: u16,
    },
    OnIdle,
    AtSystemStart,
    AtLogon,
    /// A raw trigger type that is not a named variant. It is stored unchanged and never scheduled.
    Other(u32),
}

/// `TASK_TRIGGER`. The reserved fields and `wRandomMinutesInterval` are omitted because Wine
/// always stores them as zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskTrigger {
    pub cb_trigger_size: u16,
    pub begin_year: u16,
    pub begin_month: u16,
    pub begin_day: u16,
    pub end_year: u16,
    pub end_month: u16,
    pub end_day: u16,
    pub start_hour: u16,
    pub start_minute: u16,
    pub minutes_duration: u32,
    pub minutes_interval: u32,
    pub flags: u32,
    pub trigger_type: TriggerType,
}

/// The trigger that `ITask::CreateTrigger` appends: a disabled daily trigger starting at `now`.
pub(crate) fn default_trigger(now: &SystemTime) -> TaskTrigger {
    TaskTrigger {
        cb_trigger_size: TASK_TRIGGER_SIZE,
        begin_year: now.year,
        begin_month: now.month,
        begin_day: now.day,
        start_hour: now.hour,
        start_minute: now.minute,
        flags: TRIGGER_FLAG_DISABLED,
        trigger_type: TriggerType::Daily { days_interval: 1 },
        ..Default::default()
    }
}

/// The checks `task_set_trigger` makes, in Wine's order.
pub(crate) fn check_trigger(trigger: &TaskTrigger) -> Result<(), HResult> {
    if trigger.cb_trigger_size != TASK_TRIGGER_SIZE {
        return Err(E_INVALIDARG);
    }
    date_time_to_filetime(
        trigger.begin_year,
        trigger.begin_month,
        trigger.begin_day,
        trigger.start_hour,
        trigger.start_minute,
    )
    .ok_or(E_INVALIDARG)?;
    if trigger.flags & TRIGGER_FLAG_HAS_END_DATE != 0 {
        date_time_to_filetime(trigger.end_year, trigger.end_month, trigger.end_day, 0, 0)
            .ok_or(E_INVALIDARG)?;
    }
    if trigger.minutes_duration <= trigger.minutes_interval && trigger.minutes_interval > 0 {
        return Err(E_INVALIDARG);
    }
    Ok(())
}
