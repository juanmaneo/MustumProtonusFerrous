//! The convert loops (`cvtXX...` in `msg711.c`) and `G711_StreamConvert`.
//! `fdwConvert` does not change the output: Wine ignores the BLOCKALIGN and
//! END flags, and START resets state that the codec does not keep. Unknown
//! flags are only logged by Wine, so they are ignored here too.

use crate::law::{alaw2linear, alaw2ulaw, linear2alaw, linear2ulaw, ulaw2alaw, ulaw2linear};

/// The convert loop that a stream was opened with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Convert {
    AlawToPcm16,
    PcmToAlaw,
    MulawToPcm16,
    PcmToMulaw,
    MulawToAlaw,
    AlawToMulaw,
}

/// `G711_StreamConvert`. Returns the bytes used from `src` and from `dst`.
/// The loop converts the whole samples that fit in both buffers.
pub fn stream_convert(convert: Convert, src: &[u8], dst: &mut [u8]) -> (usize, usize) {
    match convert {
        Convert::AlawToPcm16 => decode_pcm16(src, dst, alaw2linear),
        Convert::MulawToPcm16 => decode_pcm16(src, dst, ulaw2linear),
        Convert::PcmToAlaw => encode_pcm16(src, dst, |sample| linear2alaw(i32::from(sample))),
        Convert::PcmToMulaw => encode_pcm16(src, dst, linear2ulaw),
        Convert::MulawToAlaw => transcode(src, dst, ulaw2alaw),
        Convert::AlawToMulaw => transcode(src, dst, alaw2ulaw),
    }
}

fn decode_pcm16(src: &[u8], dst: &mut [u8], decode: fn(u8) -> i16) -> (usize, usize) {
    let len = src.len().min(dst.len() / 2);
    for (&byte, out) in src[..len].iter().zip(dst.as_chunks_mut::<2>().0.iter_mut()) {
        *out = decode(byte).to_le_bytes();
    }
    (len, len * 2)
}

fn encode_pcm16(src: &[u8], dst: &mut [u8], encode: fn(i16) -> u8) -> (usize, usize) {
    let len = (src.len() / 2).min(dst.len());
    for (pair, out) in src.as_chunks::<2>().0.iter().take(len).zip(dst.iter_mut()) {
        *out = encode(i16::from_le_bytes(*pair));
    }
    (len * 2, len)
}

fn transcode(src: &[u8], dst: &mut [u8], convert: fn(u8) -> u8) -> (usize, usize) {
    let len = src.len().min(dst.len());
    for (&byte, out) in src.iter().zip(dst.iter_mut()) {
        *out = convert(byte);
    }
    (len, len)
}
