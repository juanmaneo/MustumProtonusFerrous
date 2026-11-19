//! Bit access to report bytes, least significant bit first, as `hidp.c` addresses them.

pub(crate) fn get_bit(report: &[u8], bit: usize) -> bool {
    report[bit / 8] & (1 << (bit % 8)) != 0
}

pub(crate) fn set_bit(report: &mut [u8], bit: usize, on: bool) {
    let mask = 1 << (bit % 8);
    if on {
        report[bit / 8] |= mask;
    } else {
        report[bit / 8] &= !mask;
    }
}

/// Copies `count` bits of `src`, from bit `src_bit`, into `dst` at bit `dst_bit`.
pub(crate) fn copy_bits(dst: &mut [u8], dst_bit: usize, src: &[u8], src_bit: usize, count: usize) {
    for i in 0..count {
        set_bit(dst, dst_bit + i, get_bit(src, src_bit + i));
    }
}
