//! `media_type_from_codec_params` and `demuxer_stream_type` from `dlls/winedmo/unix_media_type.c`
//! and `dlls/winedmo/unix_demuxer.c`: the media type of a stream.

use crate::audio::audio_format_from_codec_params;
use crate::codec::{CodecParams, CodecTables, MediaKind};
use crate::guid::{media_guid, Guid};
use crate::status::FormatError;
use crate::time::Rational;
use crate::video::{video_format_from_codec_params, VideoFormat};

/// `MFMediaType_Audio`.
pub const MFMEDIATYPE_AUDIO: Guid = media_guid(u32::from_le_bytes(*b"auds"));
/// `MFMediaType_Video`.
pub const MFMEDIATYPE_VIDEO: Guid = media_guid(u32::from_le_bytes(*b"vids"));

/// The format block of a media type: the WAVEFORMAT bytes for audio, or the MFVIDEOFORMAT for video.
pub enum MediaFormat {
    Audio(Vec<u8>),
    Video(VideoFormat),
}

/// The output of `media_type_from_codec_params`. `major` is `None` for stream types that have no
/// major type, which the C code leaves zeroed.
pub struct MediaType {
    pub major: Option<Guid>,
    pub format: Result<MediaFormat, FormatError>,
}

/// `media_type_from_codec_params`. Only audio and video streams have a format.
pub fn media_type_from_codec_params(
    params: &CodecParams,
    sar: Rational,
    fps: Rational,
    align: u32,
    capacity: u32,
    tables: &dyn CodecTables,
) -> MediaType {
    match params.codec_type {
        MediaKind::Audio => MediaType {
            major: Some(MFMEDIATYPE_AUDIO),
            format: audio_format_from_codec_params(params, capacity, tables)
                .map(MediaFormat::Audio),
        },
        MediaKind::Video => MediaType {
            major: Some(MFMEDIATYPE_VIDEO),
            format: video_format_from_codec_params(params, capacity, sar, fps, align, tables)
                .map(MediaFormat::Video),
        },
        MediaKind::Other => MediaType {
            major: None,
            format: Err(FormatError::NotImplemented),
        },
    }
}

/// The sample aspect ratio that `demuxer_stream_type` uses: the stream's own ratio, unless its
/// numerator or denominator is zero, in which case the codec parameters' ratio is used.
pub fn stream_sample_aspect_ratio(stream_sar: Rational, codecpar_sar: Rational) -> Rational {
    if stream_sar.den == 0 || stream_sar.num == 0 {
        codecpar_sar
    } else {
        stream_sar
    }
}

/// `demuxer_stream_type`: the media type of the stream's output filter parameters, with the
/// stream's average frame rate and no alignment.
pub fn stream_media_type(
    par_out: &CodecParams,
    stream_sar: Rational,
    codecpar_sar: Rational,
    avg_frame_rate: Rational,
    capacity: u32,
    tables: &dyn CodecTables,
) -> MediaType {
    let sar = stream_sample_aspect_ratio(stream_sar, codecpar_sar);
    media_type_from_codec_params(par_out, sar, avg_frame_rate, 0, capacity, tables)
}
