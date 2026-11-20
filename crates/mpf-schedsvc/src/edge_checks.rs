use super::{next_runtime, Date, DateTime, Trigger, TIME_TRIGGER_DAILY, TIME_TRIGGER_ONCE};

fn date(year: u16, month: u16, day: u16) -> Date {
    Date { year, month, day }
}

fn time(date: Date, hour: u16, minute: u16, second: u16, millisecond: u16) -> DateTime {
    DateTime {
        date,
        hour,
        minute,
        second,
        millisecond,
    }
}

fn daily(begin: Date, end: Option<Date>, interval: u16, hour: u16, minute: u16) -> Trigger {
    Trigger {
        trigger_type: TIME_TRIGGER_DAILY,
        flags: u32::from(end.is_some()),
        begin,
        end,
        start_hour: hour,
        start_minute: minute,
        daily_days_interval: interval,
        weekly_weeks_interval: 0,
        weekly_days_of_week: 0,
    }
}

#[test]
fn milliseconds_are_truncated_in_the_returned_runtime() {
    let trigger = Trigger {
        trigger_type: TIME_TRIGGER_ONCE,
        flags: 0,
        begin: date(2024, 2, 29),
        end: Some(date(2024, 3, 1)),
        start_hour: 12,
        start_minute: 5,
        daily_days_interval: 0,
        weekly_weeks_interval: 0,
        weekly_days_of_week: 0,
    };
    let current = time(date(2024, 2, 29), 12, 0, 59, 999);
    assert_eq!(
        next_runtime(&trigger, current),
        Some(time(date(2024, 2, 29), 12, 5, 0, 0))
    );
}

#[test]
fn leap_day_and_year_boundary_are_preserved_by_daily_iteration() {
    let trigger = daily(date(2024, 2, 29), Some(date(2024, 3, 2)), 1, 8, 0);
    let current = time(date(2024, 2, 28), 23, 59, 0, 0);
    assert_eq!(
        next_runtime(&trigger, current),
        Some(time(date(2024, 2, 29), 8, 0, 0, 0))
    );

    let new_year = daily(date(2024, 12, 31), Some(date(2025, 1, 2)), 1, 8, 0);
    let before = time(date(2024, 12, 30), 0, 0, 0, 0);
    assert_eq!(
        next_runtime(&new_year, before),
        Some(time(date(2024, 12, 31), 8, 0, 0, 0))
    );
}

#[test]
fn once_trigger_respects_a_subminute_begin_boundary() {
    let trigger = Trigger {
        trigger_type: TIME_TRIGGER_ONCE,
        flags: 0,
        begin: date(2024, 3, 1),
        end: None,
        start_hour: 10,
        start_minute: 0,
        daily_days_interval: 0,
        weekly_weeks_interval: 0,
        weekly_days_of_week: 0,
    };
    let current = time(date(2024, 3, 1), 10, 0, 0, 1);
    assert_eq!(next_runtime(&trigger, current), None);
}
