//! Stream open checks and the block decoders of 4-bit Microsoft ADPCM to 8- and
//! 16-bit PCM (`ADPCM_StreamOpen`, `ADPCM_StreamConvert`, `cvtSSms16K`, `cvtMMms16K`).

use crate::format::{format_index, Coef, WaveFormat, MS_COEFS};
use crate::{Error, WAVE_FORMAT_ADPCM, WAVE_FORMAT_PCM};

/// `ACM_STREAMOPENF_ASYNC`.
pub const STREAMOPEN_ASYNC: u32 = 0x2;

/// `MS_Delta`: the step multiplier for each nibble.
const MS_DELTA: [i32; 16] = [
    230, 230, 230, 230, 307, 409, 512, 614, 768, 614, 512, 409, 307, 230, 230, 230,
];

/// The channel layout of a stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Mono,
    Stereo,
}

/// The state of an open stream: the layout and sizes the block decoders need.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decoder {
    kind: Kind,
    samples_per_block: usize,
    src_align: usize,
    dst_align: usize,
    frame: usize,
    bits: u16,
}

/// The bytes read and written by one `convert` call (`cbSrcLengthUsed` and `cbDstLengthUsed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    pub src_used: usize,
    pub dst_used: usize,
}

/// `ADPCM_StreamOpen`. The layout must be ADPCM to PCM at the same rate and
/// channel count, and the source blocks must hold every sample they declare.
pub fn stream_open(fdw_open: u32, src: &WaveFormat, dst: &WaveFormat) -> Result<Decoder, Error> {
    if fdw_open & STREAMOPEN_ASYNC != 0 {
        return Err(Error::NotSupported);
    }
    if format_index(src).is_none() || format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    if src.format_tag != WAVE_FORMAT_ADPCM || dst.format_tag != WAVE_FORMAT_PCM {
        return Err(Error::NotSupported);
    }
    if src.samples_per_sec != dst.samples_per_sec || src.channels != dst.channels {
        return Err(Error::NotSupported);
    }
    let kind = if dst.channels == 2 {
        Kind::Stereo
    } else {
        Kind::Mono
    };
    let frame = usize::from(dst.channels) * usize::from(dst.bits_per_sample / 8);
    let spb = usize::from(src.samples_per_block);
    let src_align = usize::from(src.block_align);
    let needed = match kind {
        Kind::Stereo => spb + 12,
        Kind::Mono => spb / 2 + 6,
    };
    if spb < 2
        || (kind == Kind::Mono && spb % 2 == 1)
        || needed > src_align
        || usize::from(dst.block_align) < frame
    {
        return Err(Error::NotSupported);
    }
    Ok(Decoder {
        kind,
        samples_per_block: spb,
        src_align,
        dst_align: usize::from(dst.block_align),
        frame,
        bits: dst.bits_per_sample,
    })
}

impl Decoder {
    /// The channel layout of the stream.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// `ADPCM_StreamConvert` with the convert flags ignored, as Wine does.
    /// Converts whole blocks while both buffers have room. A block whose header
    /// is out of range ends the conversion, and its samples are not counted.
    pub fn convert(&self, src: &[u8], dst: &mut [u8]) -> Usage {
        let spb = self.samples_per_block;
        let blocks = (src.len() / self.src_align).min(dst.len() / (spb * self.dst_align));
        let block_out = spb * self.frame;
        let mut usage = Usage {
            src_used: blocks * self.src_align,
            dst_used: blocks * spb * self.dst_align,
        };
        for index in 0..blocks {
            let block = &src[index * self.src_align..][..self.src_align];
            if !self.header_valid(block) {
                usage.dst_used -= (blocks - index) * (spb - 2) * self.dst_align;
                break;
            }
            let out = &mut dst[index * block_out..][..block_out];
            let mut writer = Writer {
                out,
                pos: 0,
                bits: self.bits,
            };
            match self.kind {
                Kind::Stereo => decode_stereo(block, spb, &mut writer),
                Kind::Mono => decode_mono(block, spb, &mut writer),
            }
        }
        usage
    }

    fn header_valid(&self, block: &[u8]) -> bool {
        match self.kind {
            Kind::Stereo => block[0] <= 6 && block[1] <= 6,
            Kind::Mono => block[0] <= 6,
        }
    }
}

/// One channel's ADPCM state.
struct State {
    coef: Coef,
    delta: i32,
    sample1: i32,
    sample2: i32,
}

impl State {
    /// `process_nibble`: decodes one 4-bit code and advances the state.
    fn process(&mut self, nibble: u8) -> i32 {
        let signed = if nibble & 0x08 != 0 {
            i32::from(nibble) - 16
        } else {
            i32::from(nibble)
        };
        let predicted = self
            .sample1
            .wrapping_mul(i32::from(self.coef.i_coef1))
            .wrapping_add(self.sample2.wrapping_mul(i32::from(self.coef.i_coef2)))
            / 256;
        let sample = predicted
            .wrapping_add(signed.wrapping_mul(self.delta))
            .clamp(-32768, 32767);
        self.sample2 = self.sample1;
        self.sample1 = sample;
        self.delta = (MS_DELTA[usize::from(nibble)].wrapping_mul(self.delta) / 256).max(16);
        sample
    }
}

/// Writes PCM samples in the stream's bit depth: unsigned 8-bit or little-endian 16-bit.
struct Writer<'a> {
    out: &'a mut [u8],
    pos: usize,
    bits: u16,
}

impl Writer<'_> {
    fn put(&mut self, sample: i32) {
        let s = sample as i16;
        if self.bits == 8 {
            self.out[self.pos] = ((s as u16) >> 8) as u8 ^ 0x80;
            self.pos += 1;
        } else {
            self.out[self.pos..self.pos + 2].copy_from_slice(&s.to_le_bytes());
            self.pos += 2;
        }
    }
}

fn read16(block: &[u8], at: usize) -> i32 {
    i32::from(i16::from_le_bytes([block[at], block[at + 1]]))
}

/// `cvtSSms16K`: a stereo block, with the left channel in the high nibbles and
/// the right channel in the low nibbles.
fn decode_stereo(block: &[u8], spb: usize, w: &mut Writer) {
    let mut left = State {
        coef: MS_COEFS[usize::from(block[0])],
        delta: read16(block, 2),
        sample1: read16(block, 6),
        sample2: read16(block, 10),
    };
    let mut right = State {
        coef: MS_COEFS[usize::from(block[1])],
        delta: read16(block, 4),
        sample1: read16(block, 8),
        sample2: read16(block, 12),
    };
    w.put(left.sample2);
    w.put(right.sample2);
    w.put(left.sample1);
    w.put(right.sample1);
    for &byte in &block[14..spb + 12] {
        let l = left.process(byte >> 4);
        w.put(l);
        let r = right.process(byte & 0x0f);
        w.put(r);
    }
}

/// `cvtMMms16K`: a mono block, two samples per byte, high nibble first.
fn decode_mono(block: &[u8], spb: usize, w: &mut Writer) {
    let mut st = State {
        coef: MS_COEFS[usize::from(block[0])],
        delta: read16(block, 1),
        sample1: read16(block, 3),
        sample2: read16(block, 5),
    };
    w.put(st.sample2);
    w.put(st.sample1);
    for &byte in &block[7..spb / 2 + 6] {
        let hi = st.process(byte >> 4);
        w.put(hi);
        let lo = st.process(byte & 0x0f);
        w.put(lo);
    }
}
