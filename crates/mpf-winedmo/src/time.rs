//! Time base conversions from `dlls/winedmo/unix_demuxer.c`, on FFmpeg's `av_rescale_q_rnd` with
//! `AV_ROUND_PASS_MINMAX`.

/// `AVRational`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational {
    pub num: i32,
    pub den: i32,
}

pub const AV_NOPTS_VALUE: i64 = i64::MIN;
pub const AV_TIME_BASE: i64 = 1_000_000;
pub const AV_TIME_BASE_Q: Rational = Rational {
    num: 1,
    den: 1_000_000,
};
/// `USER_TIME_BASE_Q`: 100 ns units, the time base of the Windows media APIs.
pub const USER_TIME_BASE_Q: Rational = Rational {
    num: 1,
    den: 10_000_000,
};

/// `av_rescale_rnd(a, b, c, AV_ROUND_PASS_MINMAX)`: `a * b / c` truncated toward zero. The result is
/// `INT64_MIN` when `c` or `b` is not usable, and when the quotient does not fit in 64 bits.
pub fn av_rescale_minmax(a: i64, b: i64, c: i64) -> i64 {
    if c <= 0 || b < 0 {
        return i64::MIN;
    }
    if a == i64::MIN || a == i64::MAX {
        return a;
    }
    i64::try_from(i128::from(a) * i128::from(b) / i128::from(c)).unwrap_or(i64::MIN)
}

/// `av_rescale_q_rnd(a, bq, cq, AV_ROUND_PASS_MINMAX)`.
pub fn rescale_q_minmax(a: i64, bq: Rational, cq: Rational) -> i64 {
    av_rescale_minmax(
        a,
        i64::from(bq.num) * i64::from(cq.den),
        i64::from(cq.num) * i64::from(bq.den),
    )
}

/// `get_user_time`: converts `time` in `time_base` to 100 ns units.
pub fn user_time(time: i64, time_base: Rational) -> i64 {
    rescale_q_minmax(time, time_base, USER_TIME_BASE_Q)
}

/// `get_stream_time`: a stream without a time base is read in microseconds.
pub fn stream_time(time_base: Rational, time: i64) -> i64 {
    if time_base.num != 0 && time_base.den != 0 {
        user_time(time, time_base)
    } else {
        user_time(time, AV_TIME_BASE_Q)
    }
}

/// `get_context_duration` over `(time_base, duration)` for each stream. The longest valid stream
/// duration wins, and the context duration is the fallback when no stream has one.
pub fn context_duration(streams: &[(Rational, i64)], ctx_duration: i64) -> i64 {
    let mut max_duration = AV_NOPTS_VALUE;
    for &(time_base, stream_duration) in streams {
        let duration = stream_time(time_base, stream_duration);
        if duration == AV_NOPTS_VALUE {
            continue;
        }
        // The C code also re-checks `max_duration == AV_NOPTS_VALUE` here. That cannot hold after
        // the update above, because `duration` is never `AV_NOPTS_VALUE`, so the check is omitted.
        if duration >= max_duration {
            max_duration = duration;
        }
    }
    if max_duration == AV_NOPTS_VALUE {
        return user_time(ctx_duration, AV_TIME_BASE_Q);
    }
    max_duration
}

/// The timestamp scale in `demuxer_seek`: 100 ns units to microseconds. The C multiplication is
/// signed and overflows undefined; the port wraps.
pub fn seek_timestamp(timestamp: i64) -> i64 {
    timestamp.wrapping_mul(AV_TIME_BASE) / 10_000_000
}
