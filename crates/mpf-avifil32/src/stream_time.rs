//! `AVIStreamSampleToTime` and `AVIStreamTimeToSample` (`api.c`): conversions between a stream's
//! sample numbers and milliseconds.
//!
//! Wine reads the stream's `AVISTREAMINFOW` through `IAVIStream::Info`, so the port takes the four
//! fields the conversions use as a [`StreamTiming`]. The arithmetic follows the C expressions
//! step by step in `f32`, the single precision that x86-64 SSE uses for `float`.

/// The `AVISTREAMINFOW` fields the conversions read.
pub struct StreamTiming {
    /// `dwStart`: the first sample of the stream.
    pub start: u32,
    /// `dwLength`: the number of samples in the stream.
    pub length: u32,
    /// `dwRate`: the numerator of the sample rate.
    pub rate: u32,
    /// `dwScale`: the denominator of the sample rate.
    pub scale: u32,
}

/// `AVIStreamSampleToTime`: the time in milliseconds at which `sample` starts.
///
/// The sample is clamped to the stream first. The result is truncated when the rate is under 1000
/// samples per second, and rounded up otherwise. Returns -1 when `rate` or `scale` is zero. C
/// divides by `scale` in that case and traps, so the zero-`scale` result is the port's choice.
pub fn sample_to_time(info: StreamTiming, sample: i32) -> i32 {
    if info.rate == 0 || info.scale == 0 {
        return -1;
    }
    let sample = clamp_to_stream(&info, sample as u32) as i32 as f32;
    let scaled = sample * info.scale as f32 * 1000.0;
    let time = if info.rate / info.scale < 1000 {
        scaled / info.rate as f32
    } else {
        (scaled + (info.rate - 1) as f32) / info.rate as f32
    };
    time as i32
}

/// `AVIStreamTimeToSample`: the sample at `time` milliseconds, clamped to the stream.
///
/// Returns -1 for a negative `time` or a zero `scale`. The rounding is the same as in
/// [`sample_to_time`], and a zero `rate` is accepted and gives the stream's start.
pub fn time_to_sample(info: StreamTiming, time: i32) -> i32 {
    if time < 0 || info.scale == 0 {
        return -1;
    }
    // C computes `dwScale * 1000` as an unsigned int, so it wraps.
    let scale_ms = info.scale.wrapping_mul(1000);
    let numerator = info.rate as f32 * time as f32;
    let sample = if info.rate / info.scale < 1000 {
        numerator / scale_ms as f32
    } else {
        (numerator + scale_ms.wrapping_sub(1) as f32) / scale_ms as f32
    };
    clamp_to_stream(&info, sample as i32 as u32) as i32
}

/// The clamp both conversions apply. C raises the sample to `dwStart`, then lowers it to
/// `dwStart + dwLength`, which wraps. Working on `u32` matches C's unsigned comparisons.
fn clamp_to_stream(info: &StreamTiming, sample: u32) -> u32 {
    let end = info.start.wrapping_add(info.length);
    sample.max(info.start).min(end)
}
