//! Time-format conversions and block alignment from mciwave.c.

use crate::device::WaveDevice;
use crate::{MCI_FORMAT_BYTES, MCI_FORMAT_MILLISECONDS, MCI_FORMAT_SAMPLES};

/// MulDiv from dlls/kernelbase/main.c: a * b / c rounded to nearest, or -1 on overflow.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (-i64::from(a), -i64::from(c))
    } else {
        (i64::from(a), i64::from(c))
    };
    let product = a * i64::from(b);
    let ret = if (a < 0) == (b < 0) {
        (product + c / 2) / c
    } else {
        (product - c / 2) / c
    };
    if !(-2_147_483_647..=2_147_483_647).contains(&ret) {
        -1
    } else {
        ret as i32
    }
}

/// WAVE_ConvertByteToTimeFormat: a byte offset in the device's time format.
pub fn byte_to_time(dev: &WaveDevice, val: u32) -> u32 {
    let avg = dev.format.avg_bytes_per_sec as i32;
    match dev.time_format {
        MCI_FORMAT_MILLISECONDS => mul_div(val as i32, 1000, avg) as u32,
        MCI_FORMAT_BYTES => val,
        MCI_FORMAT_SAMPLES => mul_div(val as i32, dev.format.samples_per_sec as i32, avg) as u32,
        _ => 0,
    }
}

/// WAVE_ConvertTimeFormatToByte: a time in the device's format as a byte offset.
/// A millisecond value that maps to the end of the chunk is clamped to it.
pub fn time_to_byte(dev: &WaveDevice, val: u32) -> u32 {
    let avg = dev.format.avg_bytes_per_sec as i32;
    match dev.time_format {
        MCI_FORMAT_MILLISECONDS => {
            let ret = mul_div(val as i32, avg, 1000) as u32;
            if ret > dev.cksize && val == byte_to_time(dev, dev.cksize) {
                dev.cksize
            } else {
                ret
            }
        }
        MCI_FORMAT_BYTES => val,
        MCI_FORMAT_SAMPLES => mul_div(val as i32, avg, dev.format.samples_per_sec as i32) as u32,
        _ => 0,
    }
}

/// WAVE_ALIGN_ON_BLOCK: rounds v up to a whole number of blocks, or None when
/// the block align is zero (the C macro divides by it).
pub fn align_up(v: u32, block_align: u16) -> Option<u32> {
    let ba = u32::from(block_align);
    if ba == 0 {
        return None;
    }
    let blocks = v.wrapping_add(ba - 1) / ba;
    Some(blocks.wrapping_mul(ba))
}

/// Rounds v down to a whole number of blocks, as play, record and seek do
/// before using a position. None when the block align is zero.
pub fn round_down(v: u32, block_align: u16) -> Option<u32> {
    let ba = u32::from(block_align);
    if ba == 0 {
        return None;
    }
    Some(v / ba * ba)
}
