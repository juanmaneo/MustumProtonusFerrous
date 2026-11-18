//! The timestamp arithmetic of Wine's `resampler.c`: `av_rescale_rnd` and the conversions
//! `resampler_time_to_user` and `resampler_time_from_user` built on it.

/// `AV_NOPTS_VALUE`, the "no timestamp" value, which is also `INT64_MIN`.
pub const NO_PTS: i64 = i64::MIN;
/// The DirectShow reference time unit: 100 ns ticks per second.
pub const TIME_BASE: i64 = 10_000_000;

/// `av_rescale_rnd(a, b, c, AV_ROUND_ZERO | AV_ROUND_PASS_MINMAX)`: `a * b / c` truncated toward
/// zero. `NO_PTS` for a nonpositive divisor, a negative multiplier, or a result outside `i64`.
/// `NO_PTS` and `i64::MAX` pass through unchanged.
pub fn rescale(a: i64, b: i64, c: i64) -> i64 {
    if c <= 0 || b < 0 {
        return NO_PTS;
    }
    if a == NO_PTS || a == i64::MAX {
        return a;
    }
    i64::try_from(i128::from(a) * i128::from(b) / i128::from(c)).unwrap_or(NO_PTS)
}

/// `resampler_time_to_user`: a stream timestamp in samples-per-second units to reference time.
pub fn to_user(time_den: i64, time: i64) -> i64 {
    rescale(time, TIME_BASE, time_den)
}

/// `resampler_time_from_user`: a reference time to the stream timestamp.
pub fn from_user(time_den: i64, time: i64) -> i64 {
    rescale(time, time_den, TIME_BASE)
}

/// `av_rescale(samples, TIME_BASE, rate)`: rounded half away from zero. Output only passes
/// positive sample counts, which cannot overflow here, so Wine's `INT64_MIN` overflow result is
/// unreachable.
pub fn duration(samples: i64, rate: i64) -> i64 {
    (samples * TIME_BASE + rate / 2) / rate
}
