//! `MSTASK_ITask_GetNextRunTime` from Wine's `dlls/mstask/task.c`: the earliest run time among
//! the enabled triggers.

use crate::calendar::{
    date_time_to_filetime, filetime_to_system_time, system_time_to_filetime, SystemTime,
};
use crate::trigger::{TaskTrigger, TriggerType};
use crate::{
    HResult, E_FAIL, E_INVALIDARG, SCHED_S_EVENT_TRIGGER, SCHED_S_TASK_DISABLED,
    SCHED_S_TASK_NO_VALID_TRIGGERS, TASK_FLAG_DISABLED, TRIGGER_FLAG_DISABLED,
    TRIGGER_FLAG_HAS_END_DATE,
};

/// One day in FILETIME ticks.
const TICKS_PER_DAY: u64 = 86_400 * 10_000_000;

pub(crate) fn next_run_time(
    flags: u32,
    triggers: &[TaskTrigger],
    now: &SystemTime,
) -> Result<SystemTime, HResult> {
    if flags & TASK_FLAG_DISABLED != 0 {
        return Err(SCHED_S_TASK_DISABLED);
    }
    let current = system_time_to_filetime(now).ok_or(E_INVALIDARG)?;
    let mut hr = SCHED_S_TASK_NO_VALID_TRIGGERS;
    let mut best: Option<u64> = None;
    for trigger in triggers {
        if trigger.flags & TRIGGER_FLAG_DISABLED != 0 {
            continue;
        }
        let candidate = match trigger.trigger_type {
            // Wine never reads the date window of an event trigger. Reading it here could fail
            // where Wine succeeds.
            TriggerType::OnIdle | TriggerType::AtSystemStart | TriggerType::AtLogon => {
                hr = SCHED_S_EVENT_TRIGGER;
                None
            }
            _ => time_candidate(trigger, now, current)?,
        };
        best = best.into_iter().chain(candidate).min();
    }
    let ft = best.ok_or(hr)?;
    filetime_to_system_time(ft).ok_or(E_FAIL)
}

/// The earliest run time of one enabled time trigger, or `None` if it has none.
fn time_candidate(
    trigger: &TaskTrigger,
    now: &SystemTime,
    current: u64,
) -> Result<Option<u64>, HResult> {
    match trigger.trigger_type {
        TriggerType::Once => {
            let (begin, end) = window(trigger, current)?;
            let at = date_time_to_filetime(
                trigger.begin_year,
                trigger.begin_month,
                trigger.begin_day,
                trigger.start_hour,
                trigger.start_minute,
            )
            .ok_or(E_INVALIDARG)?;
            Ok((begin <= at && at < end).then_some(at))
        }
        TriggerType::Daily { days_interval } => {
            if days_interval == 0 {
                return Ok(None);
            }
            let (begin, end) = window(trigger, current)?;
            let mut at = start_today(now, trigger)?;
            while at < end {
                if at >= begin {
                    return Ok(Some(at));
                }
                at = add_days(at, u32::from(days_interval));
            }
            Ok(None)
        }
        TriggerType::Weekly {
            weeks_interval,
            days_of_week,
        } => {
            if days_of_week == 0 {
                return Ok(None);
            }
            let (begin, end) = window(trigger, current)?;
            let mut at = start_today(now, trigger)?;
            while at < end {
                let day = filetime_to_system_time(at).ok_or(E_FAIL)?.day_of_week;
                if at >= begin && days_of_week & (1 << day) != 0 {
                    return Ok(Some(at));
                }
                // After a Sunday, skip the rest of an interval week rather than moving one day.
                at = if day == 0 && weeks_interval > 1 {
                    add_weeks(at, weeks_interval - 1)
                } else {
                    add_days(at, 1)
                };
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// The trigger's date range as FILETIMEs. The begin date is not earlier than `current`, and the
/// end is `u64::MAX` when the trigger has no end date.
fn window(trigger: &TaskTrigger, current: u64) -> Result<(u64, u64), HResult> {
    let begin = date_time_to_filetime(
        trigger.begin_year,
        trigger.begin_month,
        trigger.begin_day,
        0,
        0,
    )
    .ok_or(E_INVALIDARG)?
    .max(current);
    let end = if trigger.flags & TRIGGER_FLAG_HAS_END_DATE != 0 {
        date_time_to_filetime(trigger.end_year, trigger.end_month, trigger.end_day, 0, 0)
            .ok_or(E_INVALIDARG)?
    } else {
        u64::MAX
    };
    Ok((begin, end))
}

/// Today's date at the trigger's start time.
fn start_today(now: &SystemTime, trigger: &TaskTrigger) -> Result<u64, HResult> {
    date_time_to_filetime(
        now.year,
        now.month,
        now.day,
        trigger.start_hour,
        trigger.start_minute,
    )
    .ok_or(E_INVALIDARG)
}

fn add_days(filetime: u64, days: u32) -> u64 {
    filetime.wrapping_add(u64::from(days).wrapping_mul(TICKS_PER_DAY))
}

fn add_weeks(filetime: u64, weeks: u16) -> u64 {
    add_days(filetime, u32::from(weeks) * 7)
}
