//! The codec parameters that the conversions read, in place of FFmpeg's `AVCodecParameters`, and
//! the codec lookups that FFmpeg's tag tables and `av_get_bits_per_sample` provide.

use crate::pixel::PixelFormat;

/// The `AVMediaType` values that `media_type_from_codec_params` dispatches on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKind {
    Video,
    Audio,
    Other,
}

/// The `AVCodecID` values with a case of their own. `Other` covers every other codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecId {
    Mpeg1Video,
    H264,
    RawVideo,
    Vp9,
    Hevc,
    Mp1,
    Mp3,
    Aac,
    Wmav1,
    Wmav2,
    Wmapro,
    Wmavoice,
    Wmalossless,
    Alac,
    PcmS16le,
    Other,
}

impl CodecId {
    /// The size of the wave format structure that `audio_format_from_codec_params` starts from.
    /// Codecs without a structure of their own use `sizeof(WAVEFORMATEX)`, 18 bytes.
    pub fn wave_format_size(self) -> usize {
        match self {
            // sizeof(MPEG1WAVEFORMAT)
            Self::Mp1 => 40,
            // sizeof(MPEGLAYER3WAVEFORMAT)
            Self::Mp3 => 30,
            // sizeof(MSAUDIO1WAVEFORMAT)
            Self::Wmav1 => 22,
            // sizeof(WMAUDIO2WAVEFORMAT)
            Self::Wmav2 => 28,
            // sizeof(WMAUDIO3WAVEFORMAT)
            Self::Wmapro | Self::Wmavoice | Self::Wmalossless => 36,
            _ => 18,
        }
    }
}

/// The fields of `AVCodecParameters` that the conversions read. `channel_mask` only counts when
/// `channel_order_native` is set, which is `AV_CHANNEL_ORDER_NATIVE` in FFmpeg.
pub struct CodecParams<'a> {
    pub codec_type: MediaKind,
    pub codec_id: CodecId,
    pub codec_tag: u32,
    pub channels: i32,
    pub channel_order_native: bool,
    pub channel_mask: u64,
    pub sample_rate: i32,
    pub bits_per_coded_sample: i32,
    pub block_align: i32,
    pub bit_rate: i64,
    pub extradata: &'a [u8],
    pub width: i32,
    pub height: i32,
    pub pixel_format: PixelFormat,
}

/// The FFmpeg lookups that the conversions depend on: the RIFF and MOV tag tables, and
/// `av_get_bits_per_sample`. They are a trait so that the caller supplies the tables.
pub trait CodecTables {
    /// `av_codec_get_tag` over the RIFF and MOV audio tag tables.
    fn wave_format_tag(&self, id: CodecId) -> u32;
    /// `av_codec_get_tag` over the RIFF and MOV video tag tables.
    fn video_format_tag(&self, id: CodecId) -> u32;
    /// `av_get_bits_per_sample`.
    fn bits_per_sample(&self, id: CodecId) -> i32;
}
