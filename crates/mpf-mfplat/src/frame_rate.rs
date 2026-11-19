//! The frame-rate conversions of `dlls/mfplat/main.c`: `MFFrameRateToAverageTimePerFrame` and
//! `MFAverageTimePerFrameToFrameRate`.

/// Average frame times are in 100-nanosecond units, so one second is this many.
const UNITS_PER_SECOND: u64 = 10_000_000;

/// The largest average frame time that is reduced to a rate. Longer times are clamped to it.
const MAX_AVERAGE_TIME: u64 = 100_000_000;

/// A time within this many units of a known rate's time gives that rate.
const KNOWN_RATE_TOLERANCE: u64 = 30;

/// `known_rates`: (average frame time, numerator, denominator).
const KNOWN_RATES: [(u64, u32, u32); 8] = [
    (417188, 24000, 1001),
    (416667, 24, 1),
    (400000, 25, 1),
    (333667, 30000, 1001),
    (333333, 30, 1),
    (200000, 50, 1),
    (166833, 60000, 1001),
    (166667, 60, 1),
];

/// `MFFrameRateToAverageTimePerFrame`: the average frame time of `numerator / denominator`.
///
/// A known rate gives its table time. Otherwise the time is `denominator * 10^7 / numerator`, and a
/// zero numerator gives zero.
pub fn frame_rate_to_average_time(numerator: u32, denominator: u32) -> u64 {
    match KNOWN_RATES
        .iter()
        .find(|&&(_, n, d)| n == numerator && d == denominator)
    {
        Some(&(time, _, _)) => time,
        None if numerator == 0 => 0,
        None => u64::from(denominator) * UNITS_PER_SECOND / u64::from(numerator),
    }
}

/// `MFAverageTimePerFrameToFrameRate`: the frame rate `(numerator, denominator)` of an average frame
/// time.
///
/// A time within `KNOWN_RATE_TOLERANCE` of a known rate gives that rate. Any other non-zero time is
/// clamped to `MAX_AVERAGE_TIME` and divided by its gcd with 10^7. A zero time gives `(0, 0)`.
pub fn average_time_to_frame_rate(average_time: u64) -> (u32, u32) {
    if let Some(&(_, numerator, denominator)) = KNOWN_RATES
        .iter()
        .find(|&&(time, _, _)| time.abs_diff(average_time) <= KNOWN_RATE_TOLERANCE)
    {
        return (numerator, denominator);
    }
    if average_time == 0 {
        return (0, 0);
    }
    let time = average_time.min(MAX_AVERAGE_TIME);
    let gcd = gcd(UNITS_PER_SECOND, time);
    ((UNITS_PER_SECOND / gcd) as u32, (time / gcd) as u32)
}

/// `get_gcd`: Euclid's algorithm.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
