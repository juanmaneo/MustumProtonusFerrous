//! Oracle rows for the wmvcore port, checked against Wine's `debugstr_time`,
//! `get_current_time` and `async_reader_get_wait_timeout`, compiled verbatim
//! under ASan and UBSan with the shim in `target/canon/wmvcore/`.
//!
//! The expected values are copied from the C oracle's output in
//! `target/canon/wmvcore/ok.txt`, which is not committed. Do not edit them by
//! hand. Inputs that make the C code undefined (negating `i64::MIN`, signed
//! overflow in the counter and clock start arithmetic, and a zero frequency)
//! are flagged by the sanitizers and left out of these rows.

use crate::clock::{ReaderClock, INFINITE};
use crate::current_time;
use crate::debugstr::debugstr_time;

const DEBUG_ROWS: [(i64, &str); 12] = [
    (0, "0.0"),
    (1, "0.0000001"),
    (9, "0.0000009"),
    (10, "0.000001"),
    (9_999_999, "0.9999999"),
    (10_000_000, "1.0"),
    (12_345_678, "1.2345678"),
    (-1, "-0.0000001"),
    (-10_000_000, "-1.0"),
    (-12_345_678_901, "-1234.5678901"),
    (1_234_567_890_123, "123456.7890123"),
    (9_223_372_036_854_775_807, "922337203685.4775807"),
];

/// `(counter, frequency, get_current_time)`.
const CURRENT_ROWS: [(i64, i64, i64); 4] = [
    (0, 10_000_000, 0),
    (1, 10_000_000, 0),
    (123_456_789, 10_000_000, 123_450_000),
    (9_007_199_254_740, 1_000_000_000, 90_071_990_000),
];

/// `(user_clock, user_time, clock_start, counter, frequency, pts, timeout)`.
const WAIT_ROWS: [(bool, u64, i64, i64, i64, u64, u32); 8] = [
    (true, 50_000_000, 0, 0, 10_000_000, 60_000_000, INFINITE),
    (true, 50_000_000, 0, 0, 10_000_000, 50_000_000, 0),
    (false, 0, 0, 0, 10_000_000, 5_000, 0),
    (false, 0, 0, 0, 10_000_000, 100_000_000, 10_000),
    (
        false, 0, 10_000_000, 20_000_000, 10_000_000, 50_000_000, 4_000,
    ),
    (false, 0, 30_000_000, 20_000_000, 10_000_000, 50_000_000, 0),
    (
        false,
        0,
        0,
        1_000_000_000_000,
        10_000_000,
        1_000_000_000_000_000,
        1_115_752_192,
    ),
    (true, 1 << 63, 0, 0, 10_000_000, (1 << 63) + 1, INFINITE),
];

#[test]
fn oracle_debugstr_time_rows() {
    for (time, expected) in DEBUG_ROWS {
        assert_eq!(debugstr_time(time), expected, "time {time}");
    }
}

#[test]
fn oracle_current_time_rows() {
    for (counter, frequency, expected) in CURRENT_ROWS {
        assert_eq!(
            current_time(counter, frequency),
            expected,
            "counter {counter}, frequency {frequency}"
        );
    }
}

#[test]
fn oracle_wait_timeout_rows() {
    for (user_clock, user_time, clock_start, counter, frequency, pts, expected) in WAIT_ROWS {
        let clock = ReaderClock {
            user_clock,
            user_time,
            clock_start,
        };
        let now = current_time(counter, frequency);
        assert_eq!(clock.wait_timeout(now, pts), expected, "pts {pts}");
    }
}
