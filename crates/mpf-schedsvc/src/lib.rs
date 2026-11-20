//! Deterministic task-trigger runtime calculation from Wine's `dlls/schedsvc/atsvc.c`.
//!
//! Arithmetic uses Windows `FILETIME` ticks (100 ns since 1601-01-01), with caller-supplied
//! calendar values treated as local time just as `get_next_runtime` does after its timezone
//! conversion. Trigger calculations mirror Wine's supported subset and recurrence behavior; no
//! service, clock, timezone, or filesystem access is performed.

const TICKS_PER_SECOND: u64 = 10_000_000;
const TICKS_PER_MINUTE: u64 = 60 * TICKS_PER_SECOND;
const TICKS_PER_DAY: u64 = 24 * 60 * TICKS_PER_MINUTE;
const WINDOWS_EPOCH_UNIX_DAYS: i64 = -134_774;

pub const TRIGGER_FLAG_HAS_END_DATE: u32 = 0x1;
pub const TRIGGER_FLAG_DISABLED: u32 = 0x4;
pub const TIME_TRIGGER_ONCE: u16 = 0;
pub const TIME_TRIGGER_DAILY: u16 = 1;
pub const TIME_TRIGGER_WEEKLY: u16 = 2;
pub const TIME_TRIGGER_MONTHLY_DATE: u16 = 3;
pub const TIME_TRIGGER_MONTHLY_DOW: u16 = 4;
pub const EVENT_TRIGGER_ON_IDLE: u16 = 5;
pub const EVENT_TRIGGER_AT_SYSTEM_START: u16 = 6;
pub const EVENT_TRIGGER_AT_LOGON: u16 = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Date {
    pub year: u16,
    pub month: u16,
    pub day: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DateTime {
    pub date: Date,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub millisecond: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Trigger {
    pub trigger_type: u16,
    pub flags: u32,
    pub begin: Date,
    pub end: Option<Date>,
    pub start_hour: u16,
    pub start_minute: u16,
    pub daily_days_interval: u16,
    pub weekly_weeks_interval: u16,
    /// Sunday is bit 0, Monday bit 1, through Saturday bit 6.
    pub weekly_days_of_week: u16,
}

/// Calculates the next runtime using Wine's deterministic trigger rules.
pub fn next_runtime(trigger: &Trigger, current: DateTime) -> Option<DateTime> {
    if trigger.flags & TRIGGER_FLAG_DISABLED != 0 {
        return None;
    }

    let current_ft = system_time_to_filetime(current);
    let current_st = filetime_to_system_time(current_ft);
    let mut begin_ft = midnight_filetime(trigger.begin);
    if begin_ft < current_ft {
        begin_ft = current_ft;
    }
    let end_ft = if trigger.flags & TRIGGER_FLAG_HAS_END_DATE != 0 {
        trigger.end.map_or(u64::MAX, midnight_filetime)
    } else {
        u64::MAX
    };

    let mut trigger_ft;
    match trigger.trigger_type {
        EVENT_TRIGGER_ON_IDLE | EVENT_TRIGGER_AT_SYSTEM_START | EVENT_TRIGGER_AT_LOGON => {
            return None;
        }
        TIME_TRIGGER_ONCE => {
            trigger_ft = at_start_time(current_st, trigger.start_hour, trigger.start_minute);
            if begin_ft <= trigger_ft && trigger_ft < end_ft {
                return Some(filetime_to_system_time(trigger_ft));
            }
        }
        TIME_TRIGGER_DAILY => {
            if trigger.daily_days_interval == 0 {
                return None;
            }
            trigger_ft = at_start_time(current_st, trigger.start_hour, trigger.start_minute);
            while trigger_ft < end_ft {
                if trigger_ft >= begin_ft {
                    return Some(filetime_to_system_time(trigger_ft));
                }
                trigger_ft = add_days(trigger_ft, i64::from(trigger.daily_days_interval));
            }
        }
        TIME_TRIGGER_WEEKLY => {
            if trigger.weekly_days_of_week == 0 {
                return None;
            }
            trigger_ft = at_start_time(current_st, trigger.start_hour, trigger.start_minute);
            while trigger_ft < end_ft {
                let trigger_st = filetime_to_system_time(trigger_ft);
                let weekday = weekday_sunday_zero(trigger_st.date);
                if trigger_ft >= begin_ft && trigger.weekly_days_of_week & (1 << weekday) != 0 {
                    return Some(trigger_st);
                }
                if weekday == 0 && trigger.weekly_weeks_interval > 1 {
                    trigger_ft =
                        add_days(trigger_ft, i64::from(trigger.weekly_weeks_interval - 1) * 7);
                } else {
                    trigger_ft = add_days(trigger_ft, 1);
                }
            }
        }
        _ => return None,
    }
    None
}

fn at_start_time(date_time: DateTime, hour: u16, minute: u16) -> u64 {
    system_time_to_filetime(DateTime {
        date: date_time.date,
        hour,
        minute,
        second: 0,
        millisecond: 0,
    })
}

fn midnight_filetime(date: Date) -> u64 {
    system_time_to_filetime(DateTime {
        date,
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
    })
}

fn system_time_to_filetime(time: DateTime) -> u64 {
    let days = days_from_civil(
        i64::from(time.date.year),
        i64::from(time.date.month),
        i64::from(time.date.day),
    ) - WINDOWS_EPOCH_UNIX_DAYS;
    (days as u64) * TICKS_PER_DAY
        + u64::from(time.hour) * 60 * TICKS_PER_MINUTE
        + u64::from(time.minute) * TICKS_PER_MINUTE
        + u64::from(time.second) * TICKS_PER_SECOND
        + u64::from(time.millisecond) * 10_000
}

fn filetime_to_system_time(filetime: u64) -> DateTime {
    let days = (filetime / TICKS_PER_DAY) as i64;
    let mut remainder = filetime % TICKS_PER_DAY;
    let (year, month, day) = civil_from_days(days + WINDOWS_EPOCH_UNIX_DAYS);
    let hour = remainder / (60 * TICKS_PER_MINUTE);
    remainder %= 60 * TICKS_PER_MINUTE;
    let minute = remainder / TICKS_PER_MINUTE;
    remainder %= TICKS_PER_MINUTE;
    let second = remainder / TICKS_PER_SECOND;
    remainder %= TICKS_PER_SECOND;
    DateTime {
        date: Date {
            year: year as u16,
            month: month as u16,
            day: day as u16,
        },
        hour: hour as u16,
        minute: minute as u16,
        second: second as u16,
        millisecond: (remainder / 10_000) as u16,
    }
}

fn add_days(filetime: u64, days: i64) -> u64 {
    filetime + (days as u64) * TICKS_PER_DAY
}

fn weekday_sunday_zero(date: Date) -> u16 {
    let days = days_from_civil(
        i64::from(date.year),
        i64::from(date.month),
        i64::from(date.day),
    ) - WINDOWS_EPOCH_UNIX_DAYS;
    ((days + 1).rem_euclid(7)) as u16
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i64, i64, i64) {
    let days = days_since_unix_epoch + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    (year + i64::from(month <= 2), month, day)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
