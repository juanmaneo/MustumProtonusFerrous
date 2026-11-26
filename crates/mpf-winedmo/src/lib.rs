//! Port of the portable logic in Wine's `dlls/winedmo/` (`unix_demuxer.c`, `unix_media_type.c`,
//! `unixlib.c`, `unixlib.h`) and its export table `dlls/winedmo/winedmo.spec`.
//!
//! The eight exports are all `@ cdecl`, so no `@ stub` model applies: `winebuild` does not route
//! any of them to `__wine_spec_unimplemented_stub`.
//!
//! Ported:
//! - `demuxer_check`: the MIME type to FFmpeg input format table: [`demuxer_check`], [`input_format`]
//! - `demuxer_stream_lang` and `demuxer_stream_name`: the metadata lookup and the `lstrcpynA` copy:
//!   [`stream_language`], [`stream_name`], [`find_tag`], [`lstrcpyn_a`]
//! - `demuxer_stream_type` through `media_type_from_codec_params`: the media type, the audio
//!   WAVEFORMATEX family, the MFVIDEOFORMAT, and the stream sample aspect ratio fallback:
//!   [`stream_media_type`], [`media_type_from_codec_params`], [`audio_format_from_codec_params`],
//!   [`video_format_from_codec_params`], [`stream_sample_aspect_ratio`]
//! - `subtype_from_pixel_format`: [`subtype_from_pixel_format`]
//! - The time base conversions `get_user_time`, `get_stream_time` and `get_context_duration`, and
//!   the timestamp scaling in `demuxer_seek`: [`user_time`], [`stream_time`], [`context_duration`],
//!   [`seek_timestamp`]
//! - The I/O callbacks `unix_seek_callback` and `unix_read_callback`, with the dispatch request
//!   modelled as a closure: [`seek_callback`], [`read_callback`], [`StreamContext`]
//! - The export table: [`EXPORTS`], [`ported_as`]
//!
//! Skipped:
//! - The FFmpeg I/O and packet pipeline: `avformat_open_input`, `av_read_frame`, `av_seek_frame`,
//!   `avio` setup, bitstream filters and the demuxer create, read, seek and destroy paths.
//! - The FFmpeg tag tables and `av_get_bits_per_sample`. The caller supplies them through
//!   [`CodecTables`].
//! - The Windows wrappers in `main.c`: COM and host marshalling, `UNIX_CALL` dispatch and the
//!   `WARN` traces.
//! - The buffer lock and unlock paths, and the `get_media_type` retry loop.
//! - The UTF-8 conversions, `TRACE` and `hexdump` output.
//! - The unixlib process attach and WOW64 thunks.
//! - The `AVChannelLayout` API in `ch_layout`. Only the channel count and the native mask are read.

mod audio;
mod codec;
mod demuxer;
mod exports;
mod guid;
mod media;
mod pixel;
mod seek;
mod status;
mod tags;
mod time;
mod video;

pub use audio::audio_format_from_codec_params;
pub use codec::{CodecId, CodecParams, CodecTables, MediaKind};
pub use demuxer::{demuxer_check, input_format};
pub use exports::{ported_as, EXPORTS};
pub use guid::{media_guid, Guid};
pub use media::{
    media_type_from_codec_params, stream_media_type, stream_sample_aspect_ratio, MediaFormat,
    MediaType, MFMEDIATYPE_AUDIO, MFMEDIATYPE_VIDEO,
};
pub use pixel::{subtype_from_pixel_format, PixelFormat};
pub use seek::{
    read_callback, seek_callback, StreamContext, AVERROR_EINVAL, AVERROR_EOF, AVSEEK_SIZE,
};
pub use status::{
    FormatError, STATUS_BUFFER_TOO_SMALL, STATUS_NOT_FOUND, STATUS_NOT_IMPLEMENTED,
    STATUS_NOT_SUPPORTED, STATUS_SUCCESS,
};
pub use tags::{find_tag, lstrcpyn_a, stream_language, stream_name};
pub use time::{
    context_duration, rescale_q_minmax, seek_timestamp, stream_time, user_time, Rational,
    AV_NOPTS_VALUE, AV_TIME_BASE, AV_TIME_BASE_Q, USER_TIME_BASE_Q,
};
pub use video::{
    video_format_from_codec_params, Aperture, MfRatio, VideoFormat, MF_VIDEO_FORMAT_SIZE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
