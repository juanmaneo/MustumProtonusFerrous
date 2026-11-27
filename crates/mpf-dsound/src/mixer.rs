//! The volume and pan conversions, the sample readers and the ring buffer distance from
//! dlls/dsound/mixer.c, and DS_MAX_CHANNELS and DSVOLUMEPAN from dlls/dsound/dsound_private.h.

/// DS_MAX_CHANNELS: the number of entries in the amplitude factor array.
pub const DS_MAX_CHANNELS: usize = 8;

/// DSBVOLUME_MIN from dsound.h: the volume that silences a channel.
pub const DSBVOLUME_MIN: i32 = -10000;

/// DSVOLUMEPAN. The amplitude factors are 16.16 fixed point. Volume and pan are in hundredths
/// of a decibel.
pub struct VolumePan {
    pub total_amp_factor: [u32; DS_MAX_CHANNELS],
    pub volume: i32,
    pub pan: i32,
}

/// DSOUND_RecalcVolPan. A volume of DSBVOLUME_MIN zeroes every amplitude factor. Otherwise only
/// entries 0 and 1 are written, from the volume with the pan applied to each side.
pub fn recalc_vol_pan(volpan: &mut VolumePan) {
    if volpan.volume == DSBVOLUME_MIN {
        volpan.total_amp_factor = [0; DS_MAX_CHANNELS];
        return;
    }
    let left = volpan.volume.wrapping_sub(volpan.pan.max(0));
    volpan.total_amp_factor[0] = db_to_amp_factor(f64::from(left));
    let right = volpan.volume.wrapping_add(volpan.pan.min(0));
    volpan.total_amp_factor[1] = db_to_amp_factor(f64::from(right));
}

/// DSOUND_AmpFactorToVolPan. Only entries 0 and 1 are read.
///
/// The C code also clamps the volume to -10000. That clamp is not ported because it cannot
/// fire: a 32-bit factor gives at least about -9600 hundredths of a decibel, and silence gives
/// exactly -10000.
pub fn amp_factor_to_vol_pan(volpan: &mut VolumePan) {
    let left = amp_factor_to_db(volpan.total_amp_factor[0]);
    let right = amp_factor_to_db(volpan.total_amp_factor[1]);
    volpan.volume = (if left < right { right } else { left }) as i32;
    volpan.pan = ((right - left) as i32).max(DSBVOLUME_MIN);
}

fn db_to_amp_factor(db: f64) -> u32 {
    (2f64.powf(db / 600.0) * 65535.0) as u32
}

fn amp_factor_to_db(factor: u32) -> f64 {
    if factor == 0 {
        -10000.0
    } else {
        600.0 * (f64::from(factor) / 65535.0).ln() / 2f64.ln()
    }
}

/// Reads dst.len() samples of width bytes each from the interleaved buffer base, for the given
/// channel. Sample i starts at byte width * (channel + i * channels), as in the C readers.
fn read_samples(
    base: &[u8],
    dst: &mut [f32],
    width: usize,
    channels: usize,
    channel: usize,
    decode: fn(&[u8]) -> f32,
) {
    for (i, sample) in dst.iter_mut().enumerate() {
        let at = width * (channel + i * channels);
        *sample = decode(&base[at..at + width]);
    }
}

/// get8: unsigned 8-bit samples, centred on 0x80 and scaled by 1/128.
pub fn get8(base: &[u8], dst: &mut [f32], channels: usize, channel: usize) {
    read_samples(base, dst, 1, channels, channel, |b| {
        (f32::from(b[0]) - 128.0) / 128.0
    });
}

/// get16: signed 16-bit little-endian samples scaled by 1/32768.
pub fn get16(base: &[u8], dst: &mut [f32], channels: usize, channel: usize) {
    read_samples(base, dst, 2, channels, channel, |b| {
        f32::from(i16::from_le_bytes([b[0], b[1]])) / 32768.0
    });
}

/// get24: 24-bit little-endian samples, read as the top three bytes of a 32-bit value and
/// scaled by 1/2^31. The C expression shifts bits past 31 when the top byte is 0x80 or more,
/// which is undefined behaviour in C. Here the shifts are unsigned, and the bits are then
/// read as two's complement.
pub fn get24(base: &[u8], dst: &mut [f32], channels: usize, channel: usize) {
    read_samples(base, dst, 3, channels, channel, |b| {
        let bits = (u32::from(b[0]) << 8) | (u32::from(b[1]) << 16) | (u32::from(b[2]) << 24);
        (bits as i32 as f32) / 2147483648.0
    });
}

/// get32: signed 32-bit little-endian samples scaled by 1/2^31.
pub fn get32(base: &[u8], dst: &mut [f32], channels: usize, channel: usize) {
    read_samples(base, dst, 4, channels, channel, |b| {
        i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f32 / 2147483648.0
    });
}

/// getieee32: single precision samples, copied without scaling.
pub fn getieee32(base: &[u8], dst: &mut [f32], channels: usize, channel: usize) {
    read_samples(base, dst, 4, channels, channel, |b| {
        f32::from_le_bytes([b[0], b[1], b[2], b[3]])
    });
}

/// DSOUND_BufPtrDiff: the number of bytes from ptr2 forward to ptr1 in a ring buffer of buflen
/// bytes. Both pointers must be inside the buffer.
pub fn buf_ptr_diff(buflen: u32, ptr1: u32, ptr2: u32) -> u32 {
    assert!(ptr1 < buflen);
    assert!(ptr2 < buflen);
    if ptr1 >= ptr2 {
        ptr1 - ptr2
    } else {
        buflen.wrapping_add(ptr1).wrapping_sub(ptr2)
    }
}
