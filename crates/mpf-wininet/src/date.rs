//! HTTP date parsing from Wine's `dlls/wininet/http.c` (`HTTP_ParseDate` and its helpers).
//!
//! The layouts are RFC 1123 (`Sun, 06 Nov 1994 08:49:37 GMT`), RFC 850 (`Sunday, 06-Nov-1994
//! 08:49:37 GMT`) and asctime (`Sun Nov  6 08:49:37 1994`), plus `0`. Results are FILETIME ticks:
//! 100 ns units since 1601-01-01 UTC.

use crate::wide::{at, eq_ci, eq_exact, find, is_space, until_nul, wcstoul_dec};

const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];
const SHORT_DAYS: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
const LONG_DAYS: [&str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];

/// The calendar fields of a parsed date, as the `SYSTEMTIME` in C holds them.
struct Fields {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

/// `HTTP_ParseDate`: the FILETIME ticks for `value`, or `None` when no layout accepts it. The
/// value ends at its first NUL, as the C string does.
pub fn parse_http_date(value: &[u16]) -> Option<u64> {
    let s = until_nul(value);
    if eq_exact(s, "0") {
        return Some(0);
    }
    match find(s, b',') {
        Some(comma) => parse_rfc1123(s, comma).or_else(|| parse_rfc850(s, comma)),
        None => parse_asctime(s),
    }
}

/// `HTTP_ParseRfc1123Date`: the weekday is exactly three letters, then the comma.
fn parse_rfc1123(s: &[u16], comma: usize) -> Option<u64> {
    if comma != 3 || !is_short_day(&s[..comma]) {
        return None;
    }
    let (day, end) = day_of_month(s, comma + 1)?;
    let month_start = skip_spaces(s, end);
    let month_end = word_end(s, month_start, is_space);
    let month = month_of(&s[month_start..month_end])?;
    let (year, end) = year_number(s, month_end)?;
    let (hour, minute, second, end) = parse_time(s, end)?;
    if !is_gmt(&s[skip_spaces(s, end)..]) {
        return None;
    }
    file_time(Fields {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

/// `HTTP_ParseRfc850Date`: the weekday is a short or a full name, and the date uses dashes.
fn parse_rfc850(s: &[u16], comma: usize) -> Option<u64> {
    let weekday = &s[..comma];
    if !is_short_day(weekday) && !is_long_day(weekday) {
        return None;
    }
    let (day, end) = day_of_month(s, comma + 1)?;
    if !is_dash(at(s, end)) {
        return None;
    }
    let month_start = end + 1;
    let month_end = word_end(s, month_start, is_dash);
    let month = month_of(&s[month_start..month_end])?;
    if !is_dash(at(s, month_end)) {
        return None;
    }
    let (year, end) = year_number(s, month_end + 1)?;
    let (hour, minute, second, end) = parse_time(s, end)?;
    if !is_gmt(&s[skip_spaces(s, end)..]) {
        return None;
    }
    file_time(Fields {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

/// `HTTP_ParseDateAsAsctime`: `Wkd Mon DD HH:MM:SS YYYY`. asctime has no zone, but the C code
/// accepts an optional `GMT` or `UTC` after the year, so this does too.
fn parse_asctime(s: &[u16]) -> Option<u64> {
    let day_end = word_end(s, 0, is_space);
    if !is_short_day(&s[..day_end]) {
        return None;
    }
    let month_start = skip_spaces(s, day_end);
    let month_end = word_end(s, month_start, is_space);
    let month = month_of(&s[month_start..month_end])?;
    let (day, end) = day_of_month(s, month_end)?;
    let (hour, minute, second, end) = parse_time(s, end)?;
    let (year, end) = year_number(s, end)?;
    let zone = &s[skip_spaces(s, end)..];
    if !zone.is_empty() && !is_gmt(zone) {
        return None;
    }
    file_time(Fields {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

/// `HTTP_ParseTime`: `HH:MM:SS` at `pos`, returning the fields and the position after the seconds.
fn parse_time(s: &[u16], pos: usize) -> Option<(u32, u32, u32, usize)> {
    let (hour, pos) = colon_field(s, pos, 23)?;
    let (minute, pos) = colon_field(s, pos, 59)?;
    let (second, end) = number(s, pos)?;
    (second <= 59).then_some((hour, minute, second, end))
}

/// A number of at most `max` followed by a colon. Returns the number and the position after the colon.
fn colon_field(s: &[u16], pos: usize, max: u32) -> Option<(u32, usize)> {
    let (value, end) = number(s, pos)?;
    (value <= max && is_colon(at(s, end))).then_some((value, end + 1))
}

/// The day of the month, 1 to 31.
fn day_of_month(s: &[u16], pos: usize) -> Option<(u32, usize)> {
    number(s, pos).filter(|&(day, _)| (1..=31).contains(&day))
}

/// A year that `SystemTimeToFileTime` can represent, 1601 to 30827.
fn year_number(s: &[u16], pos: usize) -> Option<(u32, usize)> {
    number(s, pos).filter(|&(year, _)| (1601..=30827).contains(&year))
}

/// `wcstoul(s + pos, &end, 10)`, where the C code requires a conversion. Returns the value and
/// the position after its digits. Leading white space and a sign are accepted, as `wcstoul` does.
fn number(s: &[u16], pos: usize) -> Option<(u32, usize)> {
    let (value, end) = wcstoul_dec(&s[pos..]);
    (end > 0).then_some((value, pos + end))
}

/// The end of the word that starts at `pos`: up to three units, stopping before a NUL or before a
/// unit that `stop` accepts. The C code copies words this way.
fn word_end(s: &[u16], pos: usize, stop: fn(u16) -> bool) -> usize {
    let mut end = pos;
    while end < pos + 3 && at(s, end) != 0 && !stop(at(s, end)) {
        end += 1;
    }
    end
}

fn skip_spaces(s: &[u16], mut pos: usize) -> usize {
    while is_space(at(s, pos)) {
        pos += 1;
    }
    pos
}

fn is_dash(c: u16) -> bool {
    c == u16::from(b'-')
}

fn is_colon(c: u16) -> bool {
    c == u16::from(b':')
}

/// `HTTP_ParseMonth`: the month number, 1 to 12.
fn month_of(name: &[u16]) -> Option<u32> {
    (1u32..)
        .zip(MONTHS)
        .find(|&(_, month)| eq_ci(name, month))
        .map(|(number, _)| number)
}

fn is_short_day(day: &[u16]) -> bool {
    SHORT_DAYS.iter().any(|&name| eq_ci(day, name))
}

fn is_long_day(day: &[u16]) -> bool {
    LONG_DAYS.iter().any(|&name| eq_ci(day, name))
}

/// `GMT` or `UTC`, exactly.
fn is_gmt(zone: &[u16]) -> bool {
    eq_exact(zone, "GMT") || eq_exact(zone, "UTC")
}

/// `RtlTimeFieldsToTime` and `SystemTimeToFileTime`. The parsers have checked the ranges, so only
/// the day against the month length is left. February has 29 days in every year, as in C.
fn file_time(fields: Fields) -> Option<u64> {
    if fields.day > days_in_month(fields.month) {
        return None;
    }
    let (month, year) = if fields.month < 3 {
        (i64::from(fields.month) + 13, i64::from(fields.year) - 1)
    } else {
        (i64::from(fields.month) + 1, i64::from(fields.year))
    };
    let cleaps = (3 * (year / 100) + 3) / 4;
    let days =
        (36525 * year) / 100 - cleaps + (1959 * month) / 64 + i64::from(fields.day) - 584_817;
    let seconds = days * 86_400
        + i64::from(fields.hour) * 3_600
        + i64::from(fields.minute) * 60
        + i64::from(fields.second);
    u64::try_from(seconds * 10_000_000).ok()
}

fn days_in_month(month: u32) -> u32 {
    match month {
        2 => 29,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
