//! Edge-case tests for the ported mstask logic.
//!
//! These cover field limits, index errors, every trigger variant, the scheduling branches and the
//! derived trait impls. Expectations marked `// not oracle:` are not derived from the C harness.

use crate::calendar::is_leap_year;
use crate::oracle_checks::{fresh_task, now, sys, task_with, trig};
use crate::{
    filetime_to_system_time, system_time_to_filetime, SystemTime, Task, TaskTrigger, TriggerType,
    E_FAIL, E_INVALIDARG, SCHED_E_TRIGGER_NOT_FOUND, SCHED_S_EVENT_TRIGGER, SCHED_S_TASK_DISABLED,
    SCHED_S_TASK_NO_VALID_TRIGGERS, TASK_FLAG_DISABLED, TRIGGER_FLAG_DISABLED,
    TRIGGER_FLAG_HAS_END_DATE,
};

/// One trigger of each variant, with the fields the variant needs set to non-default values.
fn all_trigger_types() -> [TriggerType; 9] {
    [
        TriggerType::Once,
        TriggerType::Daily { days_interval: 7 },
        TriggerType::Weekly {
            weeks_interval: 3,
            days_of_week: 0x7f,
        },
        TriggerType::MonthlyDate {
            days: 0x8000_0001,
            months: 0x0fff,
        },
        TriggerType::MonthlyDow {
            which_week: 4,
            days_of_week: 0x41,
            months: 0x0aaa,
        },
        TriggerType::OnIdle,
        TriggerType::AtSystemStart,
        TriggerType::AtLogon,
        TriggerType::Other(42),
    ]
}

/// A task that holds `trigger` without going through `set_trigger`, so its fields are not checked.
fn unchecked_task(trigger: TaskTrigger) -> Task {
    Task {
        flags: 0,
        triggers: vec![trigger],
    }
}

#[test]
fn is_leap_year_rules() {
    // not oracle: Gregorian leap-year rules
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(2023));
}

#[test]
fn out_of_range_fields_are_rejected() {
    let base = sys(0, [2024, 1, 15], [10, 20, 30, 456]);
    assert!(system_time_to_filetime(&base).is_some());

    // not oracle: each field outside its range makes RtlTimeFieldsToTime fail
    let mutations: [fn(&mut SystemTime); 16] = [
        |t| t.milliseconds = 1000,
        |t| t.milliseconds = 0x8000,
        |t| t.second = 60,
        |t| t.second = 0x8000,
        |t| t.minute = 60,
        |t| t.minute = 0x8000,
        |t| t.hour = 24,
        |t| t.hour = 0x8000,
        |t| t.month = 0,
        |t| t.month = 13,
        |t| t.day = 0,
        |t| t.day = 32,
        |t| {
            t.month = 2;
            t.day = 30;
        },
        |t| {
            t.month = 4;
            t.day = 31;
        },
        |t| t.year = 1600,
        |t| t.year = 0x8000,
    ];
    for mutate in mutations {
        let mut t = base;
        mutate(&mut t);
        assert_eq!(system_time_to_filetime(&t), None);
    }
}

#[test]
fn accepted_times_round_trip() {
    // not oracle: the day of week in each input is the correct one for its date
    for t in [
        sys(1, [2025, 1, 6], [10, 0, 0, 0]),
        sys(4, [2024, 2, 29], [12, 0, 0, 0]),
        sys(4, [2026, 11, 26], [9, 15, 42, 123]),
        sys(2, [2024, 12, 31], [23, 59, 59, 999]),
        sys(2, [2024, 4, 30], [0, 0, 0, 0]),
    ] {
        assert_eq!(
            system_time_to_filetime(&t).and_then(filetime_to_system_time),
            Some(t)
        );
    }
    // not oracle: February 29 is accepted in any year, leap or not
    assert!(system_time_to_filetime(&sys(0, [2023, 2, 29], [0, 0, 0, 0])).is_some());
}

#[test]
fn largest_filetime_round_trips() {
    // not oracle: the largest FILETIME below 2^63 is 30828-09-14 02:48:05.477
    let last = sys(4, [30828, 9, 14], [2, 48, 5, 477]);
    assert_eq!(filetime_to_system_time(i64::MAX as u64), Some(last));
    assert_eq!(system_time_to_filetime(&last), Some(i64::MAX as u64 - 5807));
}

#[test]
fn first_millisecond_after_epoch() {
    // not oracle: 1601-01-01 00:00:00.001 is 10000 ticks after the epoch
    let t = sys(1, [1601, 1, 1], [0, 0, 0, 1]);
    assert_eq!(system_time_to_filetime(&t), Some(10_000));
    assert_eq!(filetime_to_system_time(10_000), Some(t));
}

#[test]
fn set_trigger_accepts_duration_with_zero_interval() {
    // not oracle: a zero interval turns off the duration-versus-interval check
    let (mut task, index) = fresh_task();
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [0, 0, 0],
        0,
        5,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Ok(()));
    assert_eq!(task.get_trigger(index), Ok(t));
}

#[test]
fn set_trigger_ignores_end_date_without_flag() {
    // not oracle: the end date is only checked when HAS_END_DATE is set
    let (mut task, index) = fresh_task();
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [2023, 2, 30],
        0,
        0,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Ok(()));
    assert_eq!(task.get_trigger(index), Ok(t));
}

#[test]
fn index_errors_on_empty_task() {
    // not oracle: index errors on a task with no triggers
    let mut task = Task::default();
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(task.set_trigger(0, &t), Err(E_FAIL));
    assert_eq!(task.get_trigger(0), Err(SCHED_E_TRIGGER_NOT_FOUND));
    assert_eq!(task.delete_trigger(0), Err(SCHED_E_TRIGGER_NOT_FOUND));
    assert_eq!(task.create_trigger(&now()), 0);
    assert_eq!(task.set_trigger(1, &t), Err(E_FAIL));
}

#[test]
fn create_trigger_appends_in_order() {
    // not oracle: each new trigger takes the next index
    let mut task = Task::default();
    assert_eq!(task.create_trigger(&now()), 0);
    assert_eq!(task.create_trigger(&now()), 1);
    assert_eq!(task.trigger_count(), 2);
}

#[test]
fn every_trigger_type_round_trips() {
    // not oracle: set_trigger stores every variant unchanged
    for trigger_type in all_trigger_types() {
        let (mut task, index) = fresh_task();
        let t = trig(trigger_type, [2024, 3, 15], [10, 30], [0, 0, 0], 0, 0, 0);
        assert_eq!(task.set_trigger(index, &t), Ok(()));
        assert_eq!(task.get_trigger(index), Ok(t));
    }
}

#[test]
fn set_flags_keeps_high_bits() {
    // not oracle: SetFlags replaces only the low 15 bits
    let mut task = Task {
        flags: 0x1234_8001,
        triggers: Vec::new(),
    };
    task.set_flags(2);
    assert_eq!(task.flags, 0x1234_8002);
    assert_eq!(task.get_flags(), 0x8002);
}

#[test]
fn disabled_task_is_checked_before_now() {
    // not oracle: the disabled check runs before `now` is converted
    let bad_now = sys(0, [2026, 2, 30], [9, 0, 0, 0]);
    let disabled = Task {
        flags: TASK_FLAG_DISABLED,
        triggers: Vec::new(),
    };
    assert_eq!(disabled.next_run_time(&bad_now), Err(SCHED_S_TASK_DISABLED));
    assert_eq!(Task::default().next_run_time(&bad_now), Err(E_INVALIDARG));
}

#[test]
fn disabled_triggers_are_skipped() {
    // not oracle: a disabled trigger never produces a run time
    let daily = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        TRIGGER_FLAG_DISABLED,
        0,
        0,
    );
    let idle = trig(
        TriggerType::OnIdle,
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        TRIGGER_FLAG_DISABLED,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[daily]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
    assert_eq!(
        task_with(0, &[idle]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
}

#[test]
fn event_triggers_report_event_status() {
    // not oracle: event triggers are never scheduled, and their dates are not read
    for trigger_type in [
        TriggerType::AtSystemStart,
        TriggerType::AtLogon,
        TriggerType::OnIdle,
    ] {
        let t = trig(
            trigger_type,
            [1600, 1, 1],
            [0, 0],
            [2023, 2, 30],
            TRIGGER_FLAG_HAS_END_DATE,
            0,
            0,
        );
        assert_eq!(
            unchecked_task(t).next_run_time(&now()),
            Err(SCHED_S_EVENT_TRIGGER)
        );
    }
}

#[test]
fn idle_trigger_does_not_hide_daily_run_time() {
    // not oracle: an event trigger alongside a time trigger yields the time trigger's run time
    let idle = trig(
        TriggerType::OnIdle,
        [2026, 11, 26],
        [0, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    let daily = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[idle, daily]).next_run_time(&now()),
        Ok(sys(4, [2026, 11, 26], [10, 30, 0, 0]))
    );
}

#[test]
fn once_trigger_compares_against_now() {
    // not oracle: a Once trigger that started before now is not scheduled
    let once = trig(
        TriggerType::Once,
        [2026, 11, 26],
        [9, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[once]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
    let at_nine = sys(4, [2026, 11, 26], [9, 0, 0, 0]);
    assert_eq!(task_with(0, &[once]).next_run_time(&at_nine), Ok(at_nine));

    // not oracle: a Once trigger whose end date has passed is not scheduled
    let ended = trig(
        TriggerType::Once,
        [2026, 11, 26],
        [9, 0],
        [2026, 11, 26],
        TRIGGER_FLAG_HAS_END_DATE,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[ended]).next_run_time(&at_nine),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
}

#[test]
fn zero_interval_or_empty_mask_has_no_run_time() {
    // not oracle: a zero day interval or an empty weekday mask is never scheduled
    let daily = trig(
        TriggerType::Daily { days_interval: 0 },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    let weekly = trig(
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 0,
        },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[daily]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
    assert_eq!(
        task_with(0, &[weekly]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
}

#[test]
fn daily_end_date_limits_the_run_times() {
    // not oracle: the end date is exclusive, and a begin date after it leaves nothing to run
    let ended_today = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [2026, 11, 26],
        TRIGGER_FLAG_HAS_END_DATE,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[ended_today]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
    let begins_after_end = trig(
        TriggerType::Daily { days_interval: 1 },
        [2030, 1, 1],
        [10, 30],
        [2026, 11, 28],
        TRIGGER_FLAG_HAS_END_DATE,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[begins_after_end]).next_run_time(&now()),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );
}

#[test]
fn weekly_steps_to_the_next_listed_day() {
    // not oracle: weekly runs land on the next listed weekday, and an interval above one
    // week skips ahead a week after each Sunday
    let monday = trig(
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 2,
        },
        [2024, 1, 1],
        [10, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[monday]).next_run_time(&now()),
        Ok(sys(1, [2026, 11, 30], [10, 0, 0, 0]))
    );

    let sunday = trig(
        TriggerType::Weekly {
            weeks_interval: 2,
            days_of_week: 1,
        },
        [2024, 1, 1],
        [10, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[sunday]).next_run_time(&now()),
        Ok(sys(0, [2026, 11, 29], [10, 0, 0, 0]))
    );

    let late_begin = trig(
        TriggerType::Weekly {
            weeks_interval: 2,
            days_of_week: 1,
        },
        [2026, 12, 1],
        [10, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[late_begin]).next_run_time(&now()),
        Ok(sys(0, [2026, 12, 6], [10, 0, 0, 0]))
    );
}

#[test]
fn monthly_and_other_triggers_are_never_scheduled() {
    // not oracle: monthly and unknown trigger types yield no run time
    for trigger_type in [
        TriggerType::MonthlyDate {
            days: 1,
            months: 0x0fff,
        },
        TriggerType::MonthlyDow {
            which_week: 1,
            days_of_week: 2,
            months: 0x0fff,
        },
        TriggerType::Other(42),
    ] {
        let t = trig(trigger_type, [2024, 1, 1], [10, 30], [0, 0, 0], 0, 0, 0);
        assert_eq!(
            task_with(0, &[t]).next_run_time(&now()),
            Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
        );
    }
}

#[test]
fn earliest_candidate_wins_across_trigger_types() {
    // not oracle: a Once run at 09:45 today is earlier than a daily run at 10:30
    let once = trig(
        TriggerType::Once,
        [2026, 11, 26],
        [9, 45],
        [0, 0, 0],
        0,
        0,
        0,
    );
    let daily = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(
        task_with(0, &[once, daily]).next_run_time(&now()),
        Ok(sys(4, [2026, 11, 26], [9, 45, 0, 0]))
    );
}

#[test]
fn unchecked_bad_begin_date_fails() {
    // not oracle: a begin date before 1601 fails the window for each time trigger type
    for trigger_type in [
        TriggerType::Daily { days_interval: 1 },
        TriggerType::Once,
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 0x7f,
        },
    ] {
        let t = trig(trigger_type, [1600, 1, 1], [10, 30], [0, 0, 0], 0, 0, 0);
        assert_eq!(unchecked_task(t).next_run_time(&now()), Err(E_INVALIDARG));
    }
}

#[test]
fn unchecked_bad_end_date_fails() {
    // not oracle: an invalid end date fails the window when HAS_END_DATE is set
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [2023, 2, 30],
        TRIGGER_FLAG_HAS_END_DATE,
        0,
        0,
    );
    assert_eq!(unchecked_task(t).next_run_time(&now()), Err(E_INVALIDARG));
}

#[test]
fn unchecked_bad_start_time_fails() {
    // not oracle: an invalid start hour fails each time trigger that uses it
    for trigger_type in [
        TriggerType::Daily { days_interval: 1 },
        TriggerType::Once,
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 0x7f,
        },
    ] {
        let t = trig(trigger_type, [2024, 1, 1], [24, 0], [0, 0, 0], 0, 0, 0);
        assert_eq!(unchecked_task(t).next_run_time(&now()), Err(E_INVALIDARG));
    }
    // not oracle: a zero interval or empty mask returns before the start time is read
    for trigger_type in [
        TriggerType::Daily { days_interval: 0 },
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 0,
        },
    ] {
        let t = trig(trigger_type, [2024, 1, 1], [24, 0], [0, 0, 0], 0, 0, 0);
        assert_eq!(
            unchecked_task(t).next_run_time(&now()),
            Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
        );
    }
}

#[test]
fn run_time_past_2_63_fails() {
    // not oracle: a run time past 30828-09-14 has no SYSTEMTIME, so the call fails with E_FAIL
    let once = trig(TriggerType::Once, [32767, 1, 1], [0, 0], [0, 0, 0], 0, 0, 0);
    assert_eq!(task_with(0, &[once]).next_run_time(&now()), Err(E_FAIL));

    let weekly = trig(
        TriggerType::Weekly {
            weeks_interval: 1,
            days_of_week: 0x7f,
        },
        [2024, 1, 1],
        [0, 0],
        [0, 0, 0],
        0,
        0,
        0,
    );
    let far_now = sys(0, [32767, 1, 1], [0, 0, 0, 0]);
    assert_eq!(task_with(0, &[weekly]).next_run_time(&far_now), Err(E_FAIL));
}

#[test]
fn derived_impls_are_usable() {
    // not oracle: the derived Clone, Debug, Default and PartialEq impls
    for trigger_type in all_trigger_types() {
        assert_eq!(Clone::clone(&trigger_type), trigger_type);
        assert!(!format!("{trigger_type:?}").is_empty());
    }
    assert_eq!(format!("{:?}", TriggerType::Other(42)), "Other(42)");
    assert_eq!(TriggerType::default(), TriggerType::Once);

    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(Clone::clone(&t), t);
    assert!(format!("{t:?}").contains("TaskTrigger"));
    assert_eq!(TaskTrigger::default().cb_trigger_size, 0);

    let task = task_with(0, &[t]);
    assert_eq!(task.clone(), task);
    assert!(format!("{task:?}").contains("triggers"));
    assert_eq!(Task::default().trigger_count(), 0);

    let st = now();
    assert_eq!(Clone::clone(&st), st);
    assert!(format!("{st:?}").contains("day_of_week"));
    assert_eq!(SystemTime::default().year, 0);
}
