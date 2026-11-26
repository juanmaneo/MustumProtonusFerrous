//! `SYSTEMTIME` and `FILETIME` conversions: Wine's `RtlTimeToTimeFields` and
//! `RtlTimeFieldsToTime` from `dlls/ntdll/time.c`, behind the `SystemTimeToFileTime` and
//! `FileTimeToSystemTime` wrappers from `dlls/kernelbase/file.c`.

const TICKS_PER_SEC: i64 = 10_000_000;
const TICKS_PER_MSEC: i64 = 10_000;
const SECS_PER_DAY: i64 = 86_400;
const SECS_PER_HOUR: i64 = 3_600;
const SECS_PER_MIN: i64 = 60;
/// January 1, 1601 was a Monday.
const EPOCH_WEEKDAY: i64 = 1;
const DAYS_PER_WEEK: i64 = 7;
const DAYS_PER_QUADRICENTENNIUM: i64 = 365 * 400 + 97;
const DAYS_PER_NORMAL_QUADRENNIUM: i64 = 365 * 4 + 1;

/// Days per month for common years (row 0) and leap years (row 1).
const MONTH_LENGTHS: [[i16; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// Mirrors Win32 `SYSTEMTIME`. The name is shared with `std::time::SystemTime`, which is unrelated.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SystemTime {
    pub year: u16,
    pub month: u16,
    pub day_of_week: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub milliseconds: u16,
}

/// Mirrors Wine's `TIME_FIELDS`, whose members are signed 16-bit values.
struct TimeFields {
    year: i16,
    month: i16,
    day: i16,
    hour: i16,
    minute: i16,
    second: i16,
    milliseconds: i16,
    weekday: i16,
}

pub(crate) fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// `RtlTimeFieldsToTime`. Returns `None` where Wine returns FALSE.
fn rtl_time_fields_to_time(fields: &TimeFields) -> Option<i64> {
    // Wine indexes the table with `month == 2 || leap`, so February always has 29 days.
    if !(0..=999).contains(&fields.milliseconds)
        || !(0..=59).contains(&fields.second)
        || !(0..=59).contains(&fields.minute)
        || !(0..=23).contains(&fields.hour)
        || !(1..=12).contains(&fields.month)
        || fields.day < 1
        || fields.day
            > MONTH_LENGTHS[usize::from(fields.month == 2 || is_leap_year(i32::from(fields.year)))]
                [(fields.month - 1) as usize]
        || fields.year < 1601
    {
        return None;
    }

    // Count years from March so the leap day falls at the end of the year.
    let (month, year) = if fields.month < 3 {
        (i32::from(fields.month) + 13, i32::from(fields.year) - 1)
    } else {
        (i32::from(fields.month) + 1, i32::from(fields.year))
    };
    let cleaps = (3 * (year / 100) + 3) / 4;
    let day = 36525 * year / 100 - cleaps + 1959 * month / 64 + i32::from(fields.day) - 584_817;
    let seconds = ((i64::from(day) * 24 + i64::from(fields.hour)) * 60 + i64::from(fields.minute))
        * 60
        + i64::from(fields.second);
    let milliseconds = seconds * 1000 + i64::from(fields.milliseconds);
    // Wine multiplies in signed 64-bit arithmetic, which overflows from year 30828 on.
    Some(milliseconds.wrapping_mul(TICKS_PER_MSEC))
}

/// `RtlTimeToTimeFields`.
fn rtl_time_to_time_fields(time: i64) -> TimeFields {
    let milliseconds = time % TICKS_PER_SEC / TICKS_PER_MSEC;
    let seconds = time / TICKS_PER_SEC;
    let mut days = seconds / SECS_PER_DAY;
    let seconds_in_day = seconds % SECS_PER_DAY;
    let hour = seconds_in_day / SECS_PER_HOUR;
    let minute = seconds_in_day % SECS_PER_HOUR / SECS_PER_MIN;
    let second = seconds_in_day % SECS_PER_MIN;
    let weekday = (EPOCH_WEEKDAY + days) % DAYS_PER_WEEK;

    let cleaps = (3 * ((4 * days + 1227) / DAYS_PER_QUADRICENTENNIUM) + 3) / 4;
    days += 28188 + cleaps;
    let years = (20 * days - 2442) / (5 * DAYS_PER_NORMAL_QUADRENNIUM);
    let yearday = days - years * DAYS_PER_NORMAL_QUADRENNIUM / 4;
    let months = 64 * yearday / 1959;
    // The result counts years from March, so January and February belong to the previous year.
    let (month, year) = if months < 14 {
        (months - 1, years + 1524)
    } else {
        (months - 13, years + 1525)
    };
    let day = yearday - 1959 * months / 64;

    TimeFields {
        year: year as i16,
        month: month as i16,
        day: day as i16,
        hour: hour as i16,
        minute: minute as i16,
        second: second as i16,
        milliseconds: milliseconds as i16,
        weekday: weekday as i16,
    }
}

/// `SystemTimeToFileTime`. Returns `None` where Wine's call fails.
pub fn system_time_to_filetime(time: &SystemTime) -> Option<u64> {
    let fields = TimeFields {
        year: time.year as i16,
        month: time.month as i16,
        day: time.day as i16,
        hour: time.hour as i16,
        minute: time.minute as i16,
        second: time.second as i16,
        milliseconds: time.milliseconds as i16,
        weekday: 0,
    };
    rtl_time_fields_to_time(&fields).map(|ticks| ticks as u64)
}

/// `FileTimeToSystemTime`. Returns `None` where Wine's call fails, which is when the high bit is set.
pub fn filetime_to_system_time(filetime: u64) -> Option<SystemTime> {
    let fields = rtl_time_to_time_fields(i64::try_from(filetime).ok()?);
    Some(SystemTime {
        year: fields.year as u16,
        month: fields.month as u16,
        day_of_week: fields.weekday as u16,
        day: fields.day as u16,
        hour: fields.hour as u16,
        minute: fields.minute as u16,
        second: fields.second as u16,
        milliseconds: fields.milliseconds as u16,
    })
}

/// Converts a date and time with zero seconds and milliseconds, as the trigger code does.
pub(crate) fn date_time_to_filetime(
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
) -> Option<u64> {
    system_time_to_filetime(&SystemTime {
        year,
        month,
        day,
        hour,
        minute,
        ..Default::default()
    })
}
