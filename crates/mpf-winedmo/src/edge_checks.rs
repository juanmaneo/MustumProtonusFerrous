//! Boundary and branch checks for the ported winedmo logic. Expectations marked "not oracle-derived"
//! come from the C source or from FFmpeg and Windows definitions, not from the harness rows.

use super::*;
use crate::oracle_checks::{base, Tables};

const NONE: Rational = Rational { num: 0, den: 0 };

/// A table with fixed answers, for the tag and bit-count branches that the harness never reaches.
struct Custom {
    wave_tag: u32,
    video_tag: u32,
    bits: i32,
}

impl CodecTables for Custom {
    fn wave_format_tag(&self, _id: CodecId) -> u32 {
        self.wave_tag
    }

    fn video_format_tag(&self, _id: CodecId) -> u32 {
        self.video_tag
    }

    fn bits_per_sample(&self, _id: CodecId) -> i32 {
        self.bits
    }
}

/// `(wFormatTag, nChannels, nSamplesPerSec, nAvgBytesPerSec, nBlockAlign, wBitsPerSample, cbSize)`.
type Wfx = (u16, u16, u32, u32, u16, u16, u16);

fn wfx(bytes: &[u8]) -> Wfx {
    let u16_at = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
    let u32_at =
        |at: usize| u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
    (
        u16_at(0),
        u16_at(2),
        u32_at(4),
        u32_at(8),
        u16_at(12),
        u16_at(14),
        u16_at(16),
    )
}

fn stream(length: u64, position: u64, capacity: u32, size: u32) -> StreamContext {
    StreamContext {
        length,
        position,
        capacity,
        size,
        buffer: (0..capacity).map(|i| i as u8).collect(),
    }
}

/// Runs `seek_callback` with the dispatch script. An exhausted script echoes the request.
fn seek_script(
    ctx: &mut StreamContext,
    offset: i64,
    whence: i32,
    replies: &[Option<u64>],
) -> (i64, Vec<u64>) {
    let mut replies = replies.iter().copied();
    let mut requested = Vec::new();
    let ret = seek_callback(ctx, offset, whence, &mut |request| {
        requested.push(request);
        replies.next().unwrap_or(Some(request))
    });
    (ret, requested)
}

/// Runs `read_callback` with the dispatch script. An exhausted script gives no reply.
fn read_script(
    ctx: &mut StreamContext,
    out: &mut [u8],
    replies: &[Option<u32>],
) -> (i32, Vec<u32>) {
    let mut replies = replies.iter().copied();
    let mut requested = Vec::new();
    let ret = read_callback(ctx, out, &mut |request| {
        requested.push(request);
        replies.next().flatten()
    });
    (ret, requested)
}

fn video_guid(params: &CodecParams, tables: &dyn CodecTables) -> Guid {
    video_format_from_codec_params(params, 168, NONE, NONE, 0, tables)
        .unwrap()
        .guid_format
}

#[test]
fn format_error_status_and_size() {
    // Not oracle-derived: the NTSTATUS mapping of the C error returns.
    let too_small = FormatError::BufferTooSmall(40);
    assert_eq!(too_small.status(), STATUS_BUFFER_TOO_SMALL);
    assert_eq!(too_small.required_size(), Some(40));
    assert_eq!(FormatError::NotImplemented.status(), STATUS_NOT_IMPLEMENTED);
    assert_eq!(FormatError::NotImplemented.required_size(), None);
    assert_eq!(status::check_capacity(40, 40), Ok(()));
    assert_eq!(
        status::check_capacity(41, 40),
        Err(FormatError::BufferTooSmall(41))
    );
}

#[test]
fn rescale_minmax_edges() {
    // Not oracle-derived: FFmpeg `av_rescale_rnd` with `AV_ROUND_PASS_MINMAX`.
    assert_eq!(time::av_rescale_minmax(1, 1, 0), i64::MIN);
    assert_eq!(time::av_rescale_minmax(1, -1, 1), i64::MIN);
    assert_eq!(time::av_rescale_minmax(i64::MIN, 3, 2), i64::MIN);
    assert_eq!(time::av_rescale_minmax(i64::MAX, 3, 2), i64::MAX);
    assert_eq!(time::av_rescale_minmax(1 << 62, 4, 1), i64::MIN);
}

#[test]
fn stream_time_without_time_base_uses_microseconds() {
    // Not oracle-derived: a zero numerator or denominator selects `AV_TIME_BASE_Q`, so 10 s in
    // microseconds is 1e8 units of 100 ns.
    assert_eq!(
        stream_time(Rational { num: 0, den: 5 }, 10_000_000),
        100_000_000
    );
    assert_eq!(
        stream_time(Rational { num: 1, den: 0 }, 10_000_000),
        100_000_000
    );
}

#[test]
fn seek_timestamp_wraps_the_multiplication() {
    // Not oracle-derived: 1e13 * 1e6 overflows i64 and wraps to -8446744073709551616 before the
    // divide by 1e7, which truncates toward zero.
    assert_eq!(seek_timestamp(10_000_000_000_000), -844_674_407_370);
    assert_eq!(seek_timestamp(i64::MIN), 0);
}

#[test]
fn find_tag_needs_a_long_enough_matching_prefix() {
    // Not oracle-derived: a key longer than the entry, and a same-length entry that differs.
    assert_eq!(find_tag(&[("lang", "x")], "language"), None);
    assert_eq!(find_tag(&[("languaxe", "x")], "language"), None);
    assert_eq!(find_tag(&[("LANGUAGE_X", "fre")], "language"), Some("fre"));
}

#[test]
fn lstrcpyn_truncates_and_always_writes_nul() {
    // Not oracle-derived: the `lstrcpynA` copy limits for empty, one-byte and short buffers.
    let mut empty: [u8; 0] = [];
    lstrcpyn_a(&mut empty, "abc");
    let mut one = [0xaa];
    lstrcpyn_a(&mut one, "abc");
    assert_eq!(one, [0]);
    let mut four = [0xaa; 4];
    lstrcpyn_a(&mut four, "abcdef");
    assert_eq!(four, *b"abc\0");
    let mut four_short = [0xaa; 4];
    lstrcpyn_a(&mut four_short, "ab");
    assert_eq!(four_short, [b'a', b'b', 0, 0xaa]);
}

#[test]
fn demuxer_check_is_exact_and_case_sensitive() {
    // Not oracle-derived: near-miss MIME types, which the rows do not contain.
    assert_eq!(input_format("video/mp4"), Some("mp4"));
    assert_eq!(input_format("video/mp4 "), None);
    assert_eq!(input_format("VIDEO/MP4"), None);
    assert_eq!(demuxer_check("video/mp4"), STATUS_SUCCESS);
    assert_eq!(demuxer_check(""), STATUS_NOT_SUPPORTED);
}

#[test]
fn wave_format_sizes_per_codec() {
    // Not oracle-derived: the sizes of the MPEG1, MP3 and WMA structures in the C headers.
    for (id, size) in [
        (CodecId::Mp1, 40),
        (CodecId::Mp3, 30),
        (CodecId::Wmav1, 22),
        (CodecId::Wmav2, 28),
        (CodecId::Wmapro, 36),
        (CodecId::Wmavoice, 36),
        (CodecId::Wmalossless, 36),
        (CodecId::Alac, 18),
    ] {
        assert_eq!(id.wave_format_size(), size);
    }
}

#[test]
fn harness_tables_have_zero_for_other_codecs() {
    // Not oracle-derived: the fallback arms of the fixture table.
    assert_eq!(Tables.wave_format_tag(CodecId::Other), 0);
    assert_eq!(Tables.video_format_tag(CodecId::Other), 0);
    assert_eq!(Tables.bits_per_sample(CodecId::Other), 0);
}

#[test]
fn extensible_by_format_tag_fffe() {
    // Not oracle-derived: a table tag of 0xfffe selects WAVE_FORMAT_EXTENSIBLE directly.
    let params = CodecParams {
        channels: 2,
        sample_rate: 48000,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let custom = Custom {
        wave_tag: 0xfffe,
        video_tag: 0,
        bits: 16,
    };
    let format = audio_format_from_codec_params(&params, 64, &custom).unwrap();
    assert_eq!(format.len(), 40);
    assert_eq!(wfx(&format), (0xfffe, 2, 48000, 192000, 4, 16, 22));
    assert_eq!(format[18..20], [0, 0]);
    assert_eq!(format[20..24], [0, 0, 0, 0]);
    assert_eq!(format[24..40], media_guid(0xfffe).to_bytes());
}

#[test]
fn extensible_by_wide_format_tag() {
    // Not oracle-derived: a table tag above 16 bits selects the extensible path, and the tag is
    // the subtype's Data1.
    let params = CodecParams {
        channels: 2,
        sample_rate: 48000,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let custom = Custom {
        wave_tag: 0x1_0000,
        video_tag: 0,
        bits: 16,
    };
    let format = audio_format_from_codec_params(&params, 64, &custom).unwrap();
    assert_eq!(wfx(&format).0, 0xfffe);
    assert_eq!(format[24..28], [0, 0, 1, 0]);
}

#[test]
fn extensible_capacity_and_mask_truncation() {
    // Not oracle-derived: the capacity check of the extensible path, and the mask keeps its low
    // 32 bits (`dwChannelMask` is a DWORD).
    let params = CodecParams {
        channels: 3,
        channel_order_native: true,
        channel_mask: 0x1_0000_0007,
        sample_rate: 48000,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    assert_eq!(
        audio_format_from_codec_params(&params, 39, &Tables),
        Err(FormatError::BufferTooSmall(40))
    );
    let format = audio_format_from_codec_params(&params, 40, &Tables).unwrap();
    assert_eq!(format[20..24], 7u32.to_le_bytes());
}

#[test]
fn many_channels_without_native_order_stay_plain() {
    // Not oracle-derived: a channel mask is ignored unless the layout is native, so six channels
    // keep the plain 18-byte format.
    let params = CodecParams {
        channels: 6,
        channel_order_native: false,
        channel_mask: 0x3f,
        sample_rate: 48000,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let format = audio_format_from_codec_params(&params, 18, &Tables).unwrap();
    assert_eq!(wfx(&format), (1, 6, 48000, 576000, 12, 16, 0));
}

#[test]
fn heaac_capacity_is_checked_and_extradata_follows_the_header() {
    // Not oracle-derived: the capacity check of the HE-AAC path, and the 30-byte header.
    let extradata = [0x12, 0x10];
    let params = CodecParams {
        extradata: &extradata,
        ..base(MediaKind::Audio, CodecId::Aac)
    };
    assert_eq!(
        audio_format_from_codec_params(&params, 31, &Tables),
        Err(FormatError::BufferTooSmall(32))
    );
    let format = audio_format_from_codec_params(&params, 32, &Tables).unwrap();
    assert_eq!(format.len(), 32);
    assert_eq!(wfx(&format).0, 0x1610);
    assert_eq!(format[30..32], extradata);
}

#[test]
fn coded_bits_only_when_the_table_gives_zero() {
    // Not oracle-derived: the table's zero falls back to the coded bit count.
    let params = CodecParams {
        channels: 2,
        sample_rate: 8000,
        bits_per_coded_sample: 24,
        ..base(MediaKind::Audio, CodecId::Other)
    };
    let format = audio_format_from_codec_params(&params, 64, &Tables).unwrap();
    assert_eq!(wfx(&format), (0, 2, 8000, 48000, 6, 24, 0));
}

#[test]
fn block_align_wraps_to_16_bits() {
    // Not oracle-derived: 16 * 65535 / 8 = 131070 does not fit in the 16-bit block align.
    let params = CodecParams {
        channels: 65535,
        sample_rate: 44100,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let format = audio_format_from_codec_params(&params, 64, &Tables).unwrap();
    assert_eq!(wfx(&format), (1, 65535, 44100, 2890049400, 65534, 16, 0));
}

#[test]
fn bytes_per_second_wraps_and_bit_rate_is_truncated() {
    // Not oracle-derived: 2e9 * 4 wraps the 32-bit bytes per second, and a negative bit rate
    // becomes a DWORD without a fallback because it is not zero.
    let wrapping = CodecParams {
        channels: 2,
        sample_rate: 2_000_000_000,
        block_align: 4,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let format = audio_format_from_codec_params(&wrapping, 64, &Tables).unwrap();
    assert_eq!(wfx(&format).3, 3_705_032_704);
    let negative = CodecParams {
        channels: 2,
        sample_rate: 48000,
        bit_rate: -8,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    let format = audio_format_from_codec_params(&negative, 64, &Tables).unwrap();
    assert_eq!(wfx(&format).3, u32::MAX);
}

#[test]
fn video_capacity_is_checked() {
    // Not oracle-derived: the capacity error comes before any field is written.
    let params = CodecParams {
        width: 2,
        height: 2,
        ..base(MediaKind::Video, CodecId::H264)
    };
    assert_eq!(
        video_format_from_codec_params(&params, 167, NONE, NONE, 0, &Tables).err(),
        Some(FormatError::BufferTooSmall(168))
    );
    let format = video_format_from_codec_params(&params, 168, NONE, NONE, 0, &Tables).unwrap();
    assert_eq!(format.dw_size, 168);
}

#[test]
fn video_guid_follows_codec_then_tag_then_table() {
    // Not oracle-derived: the codec arms, the explicit codec tag, and the table fallback. The
    // MPEG-1 subtype is MEDIASUBTYPE_MPEG1Payload, {E436EB81-524F-11CE-9F53-0020AF0BA770}.
    let custom = Custom {
        wave_tag: 0,
        video_tag: 0x77,
        bits: 0,
    };
    let mpeg1 = CodecParams {
        ..base(MediaKind::Video, CodecId::Mpeg1Video)
    };
    assert_eq!(
        video_guid(&mpeg1, &custom),
        Guid {
            data1: 0xe436_eb81,
            data2: 0x524f,
            data3: 0x11ce,
            data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
        }
    );
    let h264 = base(MediaKind::Video, CodecId::H264);
    assert_eq!(video_guid(&h264, &custom), media_guid(0x3436_3248));
    let vp9 = base(MediaKind::Video, CodecId::Vp9);
    assert_eq!(video_guid(&vp9, &custom), media_guid(0x3039_5056));
    let tagged = CodecParams {
        codec_tag: 0x1234,
        ..base(MediaKind::Video, CodecId::Hevc)
    };
    assert_eq!(video_guid(&tagged, &custom), media_guid(0x1234));
    let untagged = base(MediaKind::Video, CodecId::Hevc);
    assert_eq!(video_guid(&untagged, &custom), media_guid(0x77));
}

#[test]
fn video_ratios_fall_back_unless_both_terms_are_set() {
    // Not oracle-derived: a ratio with a zero term uses the default for its kind.
    let params = base(MediaKind::Video, CodecId::Hevc);
    let format = |sar: Rational, fps: Rational| {
        video_format_from_codec_params(&params, 168, sar, fps, 0, &Tables).unwrap()
    };
    let both = format(
        Rational { num: 4, den: 3 },
        Rational {
            num: 30000,
            den: 1001,
        },
    );
    assert_eq!(
        (
            both.pixel_aspect_ratio.numerator,
            both.pixel_aspect_ratio.denominator
        ),
        (4, 3)
    );
    assert_eq!(
        (
            both.frames_per_second.numerator,
            both.frames_per_second.denominator
        ),
        (30000, 1001)
    );
    let partial = format(Rational { num: 4, den: 0 }, Rational { num: 30, den: 0 });
    assert_eq!(
        (
            partial.pixel_aspect_ratio.numerator,
            partial.pixel_aspect_ratio.denominator
        ),
        (1, 1)
    );
    assert_eq!(
        (
            partial.frames_per_second.numerator,
            partial.frames_per_second.denominator
        ),
        (0, 0)
    );
    let zero_num = format(Rational { num: 0, den: 3 }, Rational { num: 0, den: 1 });
    assert_eq!(
        (
            zero_num.pixel_aspect_ratio.numerator,
            zero_num.pixel_aspect_ratio.denominator
        ),
        (1, 1)
    );
    assert_eq!(
        (
            zero_num.frames_per_second.numerator,
            zero_num.frames_per_second.denominator
        ),
        (0, 0)
    );
}

#[test]
fn video_without_alignment_has_no_aperture() {
    // Not oracle-derived: a width that the alignment leaves unchanged has a zero aperture.
    let params = CodecParams {
        width: 16,
        height: 16,
        ..base(MediaKind::Video, CodecId::H264)
    };
    let format = video_format_from_codec_params(&params, 168, NONE, NONE, 0, &Tables).unwrap();
    assert_eq!((format.width, format.height), (16, 16));
    assert_eq!(format.geometric_aperture, Aperture { cx: 0, cy: 0 });
}

#[test]
fn unmapped_pixel_formats_use_the_base_subtype() {
    // Not oracle-derived: `Other` and `None` have no mapping, and a raw format with `None` uses
    // the codec path.
    assert_eq!(subtype_from_pixel_format(PixelFormat::Other), media_guid(0));
    assert_eq!(subtype_from_pixel_format(PixelFormat::None), media_guid(0));
    let raw_other = CodecParams {
        pixel_format: PixelFormat::Other,
        ..base(MediaKind::Video, CodecId::RawVideo)
    };
    assert_eq!(video_guid(&raw_other, &Tables), media_guid(0));
}

#[test]
fn media_type_without_a_kind_has_no_major_type() {
    // Not oracle-derived: streams that are neither audio nor video have no format.
    let params = base(MediaKind::Other, CodecId::Other);
    let media = media_type_from_codec_params(&params, NONE, NONE, 0, 64, &Tables);
    assert_eq!(media.major, None);
    assert!(matches!(media.format, Err(FormatError::NotImplemented)));
}

#[test]
fn stream_sar_falls_back_when_either_term_is_zero() {
    // Not oracle-derived: the stream's ratio is used only when both of its terms are set.
    let codecpar = Rational { num: 5, den: 4 };
    assert_eq!(
        stream_sample_aspect_ratio(Rational { num: 0, den: 1 }, codecpar),
        codecpar
    );
    assert_eq!(
        stream_sample_aspect_ratio(Rational { num: 1, den: 0 }, codecpar),
        codecpar
    );
    assert_eq!(
        stream_sample_aspect_ratio(Rational { num: 2, den: 3 }, codecpar),
        Rational { num: 2, den: 3 }
    );
}

#[test]
fn stream_media_type_uses_the_codecpar_ratio_when_the_stream_has_none() {
    // Not oracle-derived: `demuxer_stream_type` passes the fallback ratio to the video format.
    let params = base(MediaKind::Video, CodecId::H264);
    let media = stream_media_type(
        &params,
        NONE,
        Rational { num: 5, den: 4 },
        Rational { num: 25, den: 1 },
        168,
        &Tables,
    );
    assert_eq!(media.major, Some(MFMEDIATYPE_VIDEO));
    assert!(matches!(
        &media.format,
        Ok(MediaFormat::Video(format))
            if (format.pixel_aspect_ratio.numerator, format.pixel_aspect_ratio.denominator) == (5, 4)
                && (format.frames_per_second.numerator, format.frames_per_second.denominator) == (25, 1)
    ));
}

#[test]
fn seek_unknown_whence_is_treated_as_set() {
    // Not oracle-derived: any `whence` that is not SEEK_END or SEEK_CUR keeps the offset.
    let mut ctx = stream(100, 0, 16, 0);
    assert_eq!(seek_script(&mut ctx, 40, 99, &[Some(32)]), (40, vec![32]));
    assert_eq!(ctx.position, 40);
}

#[test]
fn seek_clamps_to_length() {
    // Not oracle-derived: an offset past the end is clamped to the stream length.
    let mut ctx = stream(100, 0, 16, 0);
    assert_eq!(seek_script(&mut ctx, 500, 0, &[]), (100, vec![96]));
    assert_eq!(ctx.position, 100);
}

#[test]
fn seek_failed_dispatch_leaves_the_context_alone() {
    // Not oracle-derived: a failed dispatch makes the seek return AVERROR_EINVAL.
    let mut ctx = stream(100, 0, 16, 0);
    assert_eq!(
        seek_script(&mut ctx, 40, 0, &[None]),
        (i64::from(AVERROR_EINVAL), vec![32])
    );
    assert_eq!((ctx.position, ctx.size), (0, 0));
}

#[test]
fn seek_retries_with_the_reply_offset() {
    // Not oracle-derived: a reply that differs from the chunk moves the target, and the loop asks
    // again for the chunk of the new target.
    let mut ctx = stream(100, 0, 16, 5);
    assert_eq!(
        seek_script(&mut ctx, 40, 0, &[Some(16), Some(16)]),
        (16, vec![32, 16])
    );
    assert_eq!((ctx.position, ctx.size), (16, 0));
}

#[test]
fn read_at_the_end_or_with_no_room_is_eof() {
    // Not oracle-derived: a zero request, and a position at the end, return AVERROR_EOF without
    // dispatching.
    let mut ctx = stream(64, 0, 16, 0);
    assert_eq!(read_script(&mut ctx, &mut [], &[]), (AVERROR_EOF, vec![]));
    let mut at_end = stream(8, 8, 16, 0);
    assert_eq!(
        read_script(&mut at_end, &mut [0; 4], &[]),
        (AVERROR_EOF, vec![])
    );
}

#[test]
fn read_failed_dispatch_is_einval() {
    // Not oracle-derived: a failed dispatch for the chunk returns AVERROR_EINVAL.
    let mut ctx = stream(64, 0, 16, 0);
    let mut out = [0; 4];
    assert_eq!(
        read_script(&mut ctx, &mut out, &[None]),
        (AVERROR_EINVAL, vec![16])
    );
}

#[test]
fn read_with_a_stale_size_at_the_offset_is_eof() {
    // Not oracle-derived: the stale size equals the buffer offset, so nothing can be copied.
    let mut ctx = stream(64, 4, 16, 4);
    let mut out = [0; 8];
    assert_eq!(read_script(&mut ctx, &mut out, &[]), (AVERROR_EOF, vec![]));
    assert_eq!(ctx.position, 4);
}

#[test]
fn read_with_a_stale_size_below_the_offset_copies_the_request() {
    // Not oracle-derived: a stale size below the buffer offset wraps, so the request is copied.
    let mut ctx = stream(64, 4, 16, 2);
    let mut out = [0; 8];
    assert_eq!(read_script(&mut ctx, &mut out, &[]), (8, vec![]));
    assert_eq!(out, [4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(ctx.position, 12);
}

#[test]
fn exports_map_to_ported_functions() {
    // Not oracle-derived: the names of the eight `@ cdecl` exports in winedmo.spec.
    assert_eq!(EXPORTS.len(), 8);
    assert_eq!(ported_as("winedmo_demuxer_check"), Some("demuxer_check"));
    assert_eq!(ported_as("winedmo_demuxer_seek"), Some("seek_timestamp"));
    assert_eq!(
        ported_as("winedmo_demuxer_stream_lang"),
        Some("stream_language")
    );
    assert_eq!(
        ported_as("winedmo_demuxer_stream_name"),
        Some("stream_name")
    );
    assert_eq!(
        ported_as("winedmo_demuxer_stream_type"),
        Some("stream_media_type")
    );
    assert_eq!(ported_as("winedmo_demuxer_create"), None);
    assert_eq!(ported_as("winedmo_demuxer_destroy"), None);
    assert_eq!(ported_as("winedmo_demuxer_read"), None);
    assert_eq!(ported_as("winedmo_unknown"), None);
}
