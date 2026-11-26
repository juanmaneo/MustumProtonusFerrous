//! Oracle-derived test vectors for the ported mstask logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/mstask/task.c`,
//! `dlls/ntdll/time.c` and `dlls/kernelbase/file.c` with `gcc -fsanitize=address,undefined -g`
//! under `target/canon/mstask/` and running the harness. The 25 printed rows include one row
//! (`s2ft_year_32000_ubsan_expected`) that UBSan flags as signed overflow in
//! `RtlTimeFieldsToTime`. It is excluded here, so 24 rows remain.

use crate::{
    filetime_to_system_time, system_time_to_filetime, SystemTime, Task, TaskTrigger, TriggerType,
    E_INVALIDARG, SCHED_S_EVENT_TRIGGER, SCHED_S_TASK_DISABLED, SCHED_S_TASK_NO_VALID_TRIGGERS,
    TASK_FLAG_DISABLED, TASK_TRIGGER_SIZE, TRIGGER_FLAG_DISABLED, TRIGGER_FLAG_HAS_END_DATE,
};

/// The local time the oracle ran at: Thursday, 2026-11-26 09:15:42.
pub(crate) fn now() -> SystemTime {
    sys(4, [2026, 11, 26], [9, 15, 42, 0])
}

/// Builds a `SYSTEMTIME` from its day of week, `[year, month, day]` and `[hour, minute, second, ms]`.
pub(crate) fn sys(day_of_week: u16, date: [u16; 3], time: [u16; 4]) -> SystemTime {
    SystemTime {
        year: date[0],
        month: date[1],
        day_of_week,
        day: date[2],
        hour: time[0],
        minute: time[1],
        second: time[2],
        milliseconds: time[3],
    }
}

/// Builds a trigger with `[year, month, day]` begin and end dates and an `[hour, minute]` start.
pub(crate) fn trig(
    trigger_type: TriggerType,
    begin: [u16; 3],
    start: [u16; 2],
    end: [u16; 3],
    flags: u32,
    duration: u32,
    interval: u32,
) -> TaskTrigger {
    TaskTrigger {
        cb_trigger_size: TASK_TRIGGER_SIZE,
        begin_year: begin[0],
        begin_month: begin[1],
        begin_day: begin[2],
        end_year: end[0],
        end_month: end[1],
        end_day: end[2],
        start_hour: start[0],
        start_minute: start[1],
        minutes_duration: duration,
        minutes_interval: interval,
        flags,
        trigger_type,
    }
}

/// A task with `flags` and the given triggers stored as they would be by `set_trigger`.
pub(crate) fn task_with(flags: u32, triggers: &[TaskTrigger]) -> Task {
    let mut task = Task {
        flags,
        triggers: Vec::new(),
    };
    for trigger in triggers {
        let index = task.create_trigger(&now());
        task.set_trigger(index, trigger).unwrap();
    }
    task
}

/// A task holding one default trigger, with its index.
pub(crate) fn fresh_task() -> (Task, u16) {
    let mut task = Task::default();
    let index = task.create_trigger(&now());
    (task, index)
}

#[test]
fn oracle_set_get_rows() {
    let (mut task, index) = fresh_task();

    // ROW set_daily_roundtrip
    let t = trig(
        TriggerType::Daily { days_interval: 2 },
        [2024, 3, 15],
        [10, 30],
        [0, 0, 0],
        0,
        60,
        30,
    );
    assert_eq!(task.set_trigger(index, &t), Ok(()));
    assert_eq!(task.get_trigger(index), Ok(t));

    // ROW set_cb47
    let t = TaskTrigger {
        cb_trigger_size: 47,
        ..trig(
            TriggerType::Daily { days_interval: 1 },
            [2024, 3, 15],
            [10, 30],
            [0, 0, 0],
            0,
            0,
            0,
        )
    };
    assert_eq!(task.set_trigger(index, &t), Err(E_INVALIDARG));

    // ROW set_feb29_2023
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2023, 2, 29],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Ok(()));
    assert_eq!(task.get_trigger(index), Ok(t));

    // ROW set_end_invalid_flag
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [2023, 2, 30],
        TRIGGER_FLAG_HAS_END_DATE,
        0,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Err(E_INVALIDARG));

    // ROW set_duration_le_interval
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 3, 15],
        [10, 30],
        [0, 0, 0],
        0,
        10,
        10,
    );
    assert_eq!(task.set_trigger(index, &t), Err(E_INVALIDARG));

    // ROW set_weekly_roundtrip
    let t = trig(
        TriggerType::Weekly {
            weeks_interval: 2,
            days_of_week: 42,
        },
        [2025, 1, 6],
        [7, 15],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Ok(()));
    assert_eq!(task.get_trigger(index), Ok(t));

    // ROW set_begin_year_sign_wrap
    let t = trig(
        TriggerType::Daily { days_interval: 1 },
        [32768, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );
    assert_eq!(task.set_trigger(index, &t), Err(E_INVALIDARG));
}

#[test]
fn oracle_trigger_list_rows() {
    // ROW create_default
    let (task, index) = fresh_task();
    assert_eq!(index, 0);
    assert_eq!(task.trigger_count(), 1);
    assert_eq!(
        task.get_trigger(0),
        Ok(trig(
            TriggerType::Daily { days_interval: 1 },
            [2026, 11, 26],
            [9, 15],
            [0, 0, 0],
            TRIGGER_FLAG_DISABLED,
            0,
            0,
        ))
    );

    // ROW delete_middle
    let disabled = |day| {
        trig(
            TriggerType::Daily { days_interval: 1 },
            [2026, 11, day],
            [9, 0],
            [0, 0, 0],
            TRIGGER_FLAG_DISABLED,
            0,
            0,
        )
    };
    let mut task = task_with(0, &[disabled(10), disabled(11), disabled(12)]);
    assert_eq!(task.delete_trigger(1), Ok(()));
    assert_eq!(task.trigger_count(), 2);
    assert_eq!(
        [0, 1].map(|i| task.get_trigger(i).unwrap().begin_day),
        [10, 12]
    );

    // ROW flags_mask
    let mut task = Task::default();
    task.set_flags(0xffff_ffff);
    assert_eq!(task.get_flags(), 32767);
    task.set_flags(4);
    assert_eq!(task.get_flags(), 4);
}

#[test]
fn oracle_calendar_rows() {
    // ROW s2ft_1601_epoch
    assert_eq!(
        system_time_to_filetime(&sys(0, [1601, 1, 1], [0, 0, 0, 0])),
        Some(0)
    );
    // ROW s2ft_1970_unix_epoch
    assert_eq!(
        system_time_to_filetime(&sys(0, [1970, 1, 1], [0, 0, 0, 0])),
        Some(116_444_736_000_000_000)
    );
    // ROW s2ft_2001_feb29
    assert_eq!(
        system_time_to_filetime(&sys(0, [2001, 2, 29], [12, 0, 0, 0])),
        Some(126_279_216_000_000_000)
    );
    // ROW ft2s_zero
    assert_eq!(
        filetime_to_system_time(0),
        Some(sys(1, [1601, 1, 1], [0, 0, 0, 0]))
    );
    // ROW ft2s_high_bit
    assert_eq!(filetime_to_system_time(1 << 63), None);
}

#[test]
fn oracle_next_run_time_rows() {
    let at_nine = sys(4, [2026, 11, 26], [9, 0, 0, 0]);
    let daily_1030 = trig(
        TriggerType::Daily { days_interval: 1 },
        [2024, 1, 1],
        [10, 30],
        [0, 0, 0],
        0,
        0,
        0,
    );

    // ROW next_disabled_task
    let task = task_with(TASK_FLAG_DISABLED, &[daily_1030]);
    assert_eq!(task.next_run_time(&at_nine), Err(SCHED_S_TASK_DISABLED));

    // ROW next_daily_later_today
    let task = task_with(0, &[daily_1030]);
    assert_eq!(
        task.next_run_time(&at_nine),
        Ok(sys(4, [2026, 11, 26], [10, 30, 0, 0]))
    );

    // ROW next_daily_tomorrow
    assert_eq!(
        task.next_run_time(&sys(4, [2026, 11, 26], [11, 0, 0, 0])),
        Ok(sys(5, [2026, 11, 27], [10, 30, 0, 0]))
    );

    // ROW next_daily_interval3_future_begin
    let task = task_with(
        0,
        &[trig(
            TriggerType::Daily { days_interval: 3 },
            [2027, 1, 1],
            [8, 0],
            [0, 0, 0],
            0,
            0,
            0,
        )],
    );
    assert_eq!(
        task.next_run_time(&at_nine),
        Ok(sys(5, [2027, 1, 1], [8, 0, 0, 0]))
    );

    // ROW next_weekly_mon_fri
    let task = task_with(
        0,
        &[trig(
            TriggerType::Weekly {
                weeks_interval: 1,
                days_of_week: 34,
            },
            [2024, 1, 1],
            [10, 0],
            [0, 0, 0],
            0,
            0,
            0,
        )],
    );
    assert_eq!(
        task.next_run_time(&sys(3, [2026, 11, 25], [12, 0, 0, 0])),
        Ok(sys(5, [2026, 11, 27], [10, 0, 0, 0]))
    );

    // ROW next_weekly_sunday_jump_end
    let task = task_with(
        0,
        &[trig(
            TriggerType::Weekly {
                weeks_interval: 2,
                days_of_week: 2,
            },
            [2024, 1, 1],
            [6, 0],
            [2026, 11, 30],
            TRIGGER_FLAG_HAS_END_DATE,
            0,
            0,
        )],
    );
    assert_eq!(
        task.next_run_time(&sys(4, [2026, 11, 26], [12, 0, 0, 0])),
        Err(SCHED_S_TASK_NO_VALID_TRIGGERS)
    );

    // ROW next_once_future
    let task = task_with(
        0,
        &[trig(
            TriggerType::Once,
            [2027, 5, 5],
            [9, 0],
            [0, 0, 0],
            0,
            0,
            0,
        )],
    );
    assert_eq!(
        task.next_run_time(&at_nine),
        Ok(sys(3, [2027, 5, 5], [9, 0, 0, 0]))
    );

    // ROW next_event_only
    let task = task_with(
        0,
        &[trig(
            TriggerType::OnIdle,
            [2026, 11, 26],
            [0, 0],
            [0, 0, 0],
            0,
            0,
            0,
        )],
    );
    assert_eq!(task.next_run_time(&at_nine), Err(SCHED_S_EVENT_TRIGGER));

    // ROW next_two_daily_min
    let task = task_with(
        0,
        &[
            daily_1030,
            trig(
                TriggerType::Daily { days_interval: 1 },
                [2024, 1, 1],
                [9, 30],
                [0, 0, 0],
                0,
                0,
                0,
            ),
        ],
    );
    assert_eq!(
        task.next_run_time(&at_nine),
        Ok(sys(4, [2026, 11, 26], [9, 30, 0, 0]))
    );
}
