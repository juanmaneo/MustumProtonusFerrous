//! Boundary checks beyond the oracle rows.

use crate::clock::{ReaderClock, INFINITE};
use crate::current_time;
use crate::debugstr::debugstr_time;

fn clock(user_clock: bool, user_time: u64, clock_start: i64) -> ReaderClock {
    ReaderClock {
        user_clock,
        user_time,
        clock_start,
    }
}

#[test]
fn debugstr_time_trims_trailing_zeros_after_one_fraction_digit() {
    assert_eq!(debugstr_time(15_000_000), "1.5");
    assert_eq!(debugstr_time(100_000_000), "10.0");
    assert_eq!(debugstr_time(10_000_001), "1.0000001");
    assert_eq!(debugstr_time(-15_000_000), "-1.5");
}

#[test]
fn debugstr_time_formats_the_minimum_value_without_overflow() {
    assert_eq!(debugstr_time(i64::MIN), "-922337203685.4775808");
}

#[test]
#[should_panic]
fn current_time_panics_on_zero_frequency() {
    current_time(5, 0);
}

#[test]
fn current_time_wraps_where_wine_overflows() {
    // i64::MAX * 1000 wraps to -1000 in two's complement.
    assert_eq!(current_time(i64::MAX, 1), -10_000_000);
}

#[test]
fn wait_timeout_ignores_now_on_a_user_clock() {
    let user = clock(true, 100, 0);
    assert_eq!(user.wait_timeout(i64::MIN, 101), INFINITE);
    assert_eq!(user.wait_timeout(i64::MAX, 100), 0);
}

#[test]
fn wait_timeout_returns_zero_when_pts_is_not_after_now() {
    let system = clock(false, 0, 0);
    assert_eq!(system.wait_timeout(0, 0), 0);
    assert_eq!(system.wait_timeout(1_000, 999), 0);
}

#[test]
fn wait_timeout_rounds_down_to_whole_milliseconds() {
    let system = clock(false, 0, 0);
    assert_eq!(system.wait_timeout(0, 9_999), 0);
    assert_eq!(system.wait_timeout(0, 10_000), 1);
    assert_eq!(system.wait_timeout(0, 19_999), 1);
}

#[test]
fn wait_timeout_measures_from_the_clock_start() {
    let system = clock(false, 0, 500_000);
    assert_eq!(system.wait_timeout(1_000_000, 1_010_000), 51);
}

#[test]
fn wait_timeout_truncates_the_milliseconds_to_32_bits() {
    let system = clock(false, 0, 0);
    let pts = (1_u64 << 32) * 10_000 + 10_000;
    assert_eq!(system.wait_timeout(0, pts), 1);
}
