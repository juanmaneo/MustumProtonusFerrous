//! Pure validation logic from Wine's `dlls/taskschd`.
//!
//! The ported helper validates `DailyTrigger_put_DaysInterval`: taskschd
//! accepts positive signed 16-bit day intervals and returns `E_INVALIDARG`
//! for zero or negative values. Trigger boundary strings and time-like
//! durations in this Wine component are stored without validation, while
//! other date/time behavior is stubbed or requires COM/OS state, so they are
//! outside this crate's pure-validation scope.

/// `S_OK`, returned when the requested interval is accepted.
pub const S_OK: i32 = 0;
/// `E_INVALIDARG`, returned when the requested interval is zero or negative.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;

/// Validates the day interval passed to Wine's `DailyTrigger_put_DaysInterval`.
///
/// On success this returns `S_OK`; on failure it returns `E_INVALIDARG`.
pub fn validate_daily_days_interval(days: i16) -> i32 {
    if days <= 0 {
        E_INVALIDARG
    } else {
        S_OK
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
