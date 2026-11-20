//! Position and time conversions of `dlls/quartz/dsoundrender.c`.

/// The number of 100-nanosecond units in a second.
const REFERENCE_TIMES_PER_SECOND: i64 = 10_000_000;

/// `time_from_pos`: a byte position in the buffer, as a `REFERENCE_TIME`. `None` when the average
/// byte rate is zero.
pub fn time_from_pos(pos: u32, avg_bytes_per_sec: u32) -> Option<i64> {
    (REFERENCE_TIMES_PER_SECOND * i64::from(pos)).checked_div(i64::from(avg_bytes_per_sec))
}

/// `pos_from_time`: a `REFERENCE_TIME` as a byte position, rounded down to a whole block. `None`
/// when the product overflows or the block alignment is zero.
pub fn pos_from_time(time: i64, avg_bytes_per_sec: u32, block_align: u16) -> Option<u32> {
    let mut ret = time.checked_mul(i64::from(avg_bytes_per_sec))? / REFERENCE_TIMES_PER_SECOND;
    ret -= ret.checked_rem(i64::from(block_align))?;
    Some(ret as u32)
}
