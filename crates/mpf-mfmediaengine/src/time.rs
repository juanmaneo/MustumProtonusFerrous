/// `mftime_to_seconds` from `dlls/mfmediaengine/main.c`: `MFTIME` ticks of 100 ns to seconds.
pub fn mftime_to_seconds(time: i64) -> f64 {
    time as f64 / 10_000_000.0
}
