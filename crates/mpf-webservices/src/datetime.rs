//! Port of the date helpers in `dlls/webservices/reader.c`: `str_to_datetime`, and the
//! conversions that `WsDateTimeToFileTime` and `WsFileTimeToDateTime` perform.
//!
//! Ticks count 100-nanosecond intervals from 0001-01-01, as in `webservices_private.h`.

use crate::text::trim;
use crate::Error;

const TICKS_PER_SEC: u64 = 10_000_000;
const TICKS_PER_MIN: u64 = 60 * TICKS_PER_SEC;
const TICKS_PER_HOUR: u64 = 60 * TICKS_PER_MIN;
const TICKS_PER_DAY: u64 = 24 * TICKS_PER_HOUR;
const TICKS_MAX: u64 = 3_155_378_975_999_999_999;
const TICKS_1601_01_01: u64 = 504_911_232_000_000_000;

/// Days in each month, for a common year (row 0) and a leap year (row 1).
const MONTH_DAYS: [[u64; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

/// `WS_DATETIME_FORMAT`: whether the ticks are UTC, or local time with an offset applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeFormat {
    Utc = 0,
    Local = 1,
}

/// A `WS_DATETIME`: ticks since 0001-01-01 and the format that the ticks are in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub ticks: u64,
    pub format: DateTimeFormat,
}

/// `str_to_datetime`: parses an ISO 8601 date and time with a `Z` or `+hh:mm` / `-hh:mm` zone.
/// The result is in UTC when the zone is `Z`, and in local time otherwise.
pub fn str_to_datetime(text: &[u8]) -> Result<DateTime, Error> {
    parse_datetime(trim(text)).ok_or(Error::InvalidFormat)
}

/// `WsDateTimeToFileTime`: the FILETIME, in 100-nanosecond intervals from 1601-01-01. The
/// format is not used, so local ticks are converted as they stand.
pub fn datetime_to_filetime(datetime: DateTime) -> Result<u64, Error> {
    datetime
        .ticks
        .checked_sub(TICKS_1601_01_01)
        .ok_or(Error::InvalidFormat)
}

/// `WsFileTimeToDateTime`: the UTC date and time of a FILETIME.
pub fn filetime_to_datetime(filetime: u64) -> Result<DateTime, Error> {
    let ticks = filetime
        .checked_add(TICKS_1601_01_01)
        .ok_or(Error::NumericOverflow)?;
    if ticks > TICKS_MAX {
        return Err(Error::InvalidFormat);
    }
    Ok(DateTime {
        ticks,
        format: DateTimeFormat::Utc,
    })
}

/// Reads exactly `width` ASCII digits, and returns their value and the rest of the text.
fn digits(text: &[u8], width: usize) -> Option<(u64, &[u8])> {
    let (field, rest) = text.split_at_checked(width)?;
    if !field.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let value = field
        .iter()
        .fold(0, |value, &byte| value * 10 + u64::from(byte - b'0'));
    Some((value, rest))
}

fn eat(text: &[u8], byte: u8) -> Option<&[u8]> {
    text.strip_prefix(&[byte])
}

/// The optional fraction of a second: a `.` and one to seven digits, one tick per digit place.
fn fraction(text: &[u8]) -> Option<(u64, &[u8])> {
    let Some(body) = text.strip_prefix(b".") else {
        return Some((0, text));
    };
    let run = body.iter().take_while(|byte| byte.is_ascii_digit()).count();
    if !(1..=7).contains(&run) {
        return None;
    }
    let value = body[..run]
        .iter()
        .fold(0, |value, &byte| value * 10 + u64::from(byte - b'0'));
    Some((
        value * (TICKS_PER_SEC / 10u64.pow(run as u32)),
        &body[run..],
    ))
}

/// The zone designator. Returns the offset in ticks, whether it is negative, and the format
/// that it implies. `Z` gives UTC. An offset of `hh:mm` gives local time, with hours up to 14.
fn zone(text: &[u8]) -> Option<(u64, bool, DateTimeFormat)> {
    if text == b"Z" {
        return Some((0, false, DateTimeFormat::Utc));
    }
    let (&sign, body) = text.split_first()?;
    let negative = match sign {
        b'+' => false,
        b'-' => true,
        _ => return None,
    };
    let (hours, body) = digits(body, 2)?;
    let body = eat(body, b':')?;
    let (minutes, body) = digits(body, 2)?;
    if !body.is_empty() || hours > 14 || minutes > 59 || (hours == 14 && minutes > 0) {
        return None;
    }
    let offset = hours * TICKS_PER_HOUR + minutes * TICKS_PER_MIN;
    Some((offset, negative, DateTimeFormat::Local))
}

fn is_leap(year: u64) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn parse_datetime(text: &[u8]) -> Option<DateTime> {
    let (year, rest) = digits(text, 4)?;
    let rest = eat(rest, b'-')?;
    let (month, rest) = digits(rest, 2)?;
    let rest = eat(rest, b'-')?;
    let (day, rest) = digits(rest, 2)?;
    let rest = eat(rest, b'T')?;
    let (hour, rest) = digits(rest, 2)?;
    let rest = eat(rest, b':')?;
    let (minute, rest) = digits(rest, 2)?;
    let rest = eat(rest, b':')?;
    let (second, rest) = digits(rest, 2)?;
    let (fraction, rest) = fraction(rest)?;
    let (offset, negative, format) = zone(rest)?;

    if year == 0 || !(1..=12).contains(&month) {
        return None;
    }
    let leap = usize::from(is_leap(year));
    let month_index = (month - 1) as usize;
    let days_in_month = MONTH_DAYS[leap][month_index];
    if day == 0 || day > days_in_month || hour > 24 || minute > 59 || second > 59 {
        return None;
    }
    if hour == 24 && (minute > 0 || second > 0) {
        return None;
    }

    let prior = year - 1;
    let leap_days = prior / 4 - prior / 100 + prior / 400;
    let months_before: u64 = MONTH_DAYS[leap][..month_index].iter().sum();
    let days = prior * 365 + leap_days + months_before + day - 1;
    let local = days * TICKS_PER_DAY
        + hour * TICKS_PER_HOUR
        + minute * TICKS_PER_MIN
        + second * TICKS_PER_SEC
        + fraction;

    let ticks = if negative {
        let utc = local + offset;
        (utc <= TICKS_MAX).then_some(utc)?
    } else {
        local.checked_sub(offset)?
    };
    Some(DateTime { ticks, format })
}
