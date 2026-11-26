//! Oracle-derived test vectors for the ported winedmo logic.
//!
//! Each expected row is a verbatim line printed by the harness in `target/canon/winedmo/oracle.c`,
//! which compiles the Wine C bodies with `gcc -fsanitize=address,undefined -g`. The 181 rows span
//! 15 sections. No vector was excluded: the harness reported no ASan or UBSan findings. Each check
//! renders the port's result in the harness's printf format and compares it with the row.

use super::*;

const SEEK_SET: i32 = 0;
const SEEK_CUR: i32 = 1;
const SEEK_END: i32 = 2;

const ZERO: Rational = Rational { num: 0, den: 0 };
const SAR_4_3: Rational = Rational { num: 4, den: 3 };
const NTSC: Rational = Rational {
    num: 30000,
    den: 1001,
};
const FPS_25: Rational = Rational { num: 25, den: 1 };

const EXTRA2: [u8; 2] = [0x12, 0x10];
const EXTRA5: [u8; 5] = [1, 2, 3, 4, 5];
const EXTRA10: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/// The FFmpeg lookups that the harness reads from `av_codec_get_tag` and `av_get_bits_per_sample`.
/// Only the codecs the harness uses have entries.
pub(crate) struct Tables;

impl CodecTables for Tables {
    fn wave_format_tag(&self, id: CodecId) -> u32 {
        match id {
            CodecId::PcmS16le => 1,
            CodecId::Mp1 => 0x3170_6d2e,
            CodecId::Mp3 => 85,
            CodecId::Wmav1 => 352,
            CodecId::Wmav2 => 353,
            CodecId::Wmapro => 354,
            CodecId::Wmavoice => 10,
            CodecId::Wmalossless => 355,
            CodecId::Alac => 0x6361_6c61,
            _ => 0,
        }
    }

    fn video_format_tag(&self, id: CodecId) -> u32 {
        match id {
            CodecId::Hevc => 0x3176_6568,
            _ => 0,
        }
    }

    fn bits_per_sample(&self, id: CodecId) -> i32 {
        match id {
            CodecId::PcmS16le | CodecId::Alac => 16,
            _ => 0,
        }
    }
}

/// `CodecParams` with the fields the harness zeroes with `memset`.
pub(crate) fn base<'a>(codec_type: MediaKind, codec_id: CodecId) -> CodecParams<'a> {
    CodecParams {
        codec_type,
        codec_id,
        codec_tag: 0,
        channels: 0,
        channel_order_native: false,
        channel_mask: 0,
        sample_rate: 0,
        bits_per_coded_sample: 0,
        block_align: 0,
        bit_rate: 0,
        extradata: &[],
        width: 0,
        height: 0,
        pixel_format: PixelFormat::None,
    }
}

/// `print_guid` without its leading label.
fn guid_text(guid: Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// `%s` of a status: the harness prints `%#x`, which is `0` for success.
fn status_text(status: u32) -> String {
    if status == 0 {
        "0".to_string()
    } else {
        format!("{status:#x}")
    }
}

/// Comma-separated items, as the harness prints its arrays.
fn list<T: ToString>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// `print_bytes`.
fn byte_list(bytes: &[u8]) -> String {
    format!("[{}]", list(bytes))
}

fn le16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// The `A` row of `run_audio`.
fn audio_row(label: &str, params: &CodecParams, capacity: u32) -> String {
    match audio_format_from_codec_params(params, capacity, &Tables) {
        Err(error) => format!(
            "A {label} st={} size={}",
            status_text(error.status()),
            error.required_size().unwrap_or(capacity)
        ),
        Ok(format) => {
            let tag = le16(&format, 0);
            let mut row = format!(
                "A {label} st=0 size={} tag={tag} ch={} sps={} avg={} align={} bits={} cb={}",
                format.len(),
                le16(&format, 2),
                le32(&format, 4),
                le32(&format, 8),
                le16(&format, 12),
                le16(&format, 14),
                le16(&format, 16),
            );
            let header_size = match tag {
                0xfffe => {
                    row.push_str(&format!(
                        " valid={} mask={} sub={:08x}",
                        le16(&format, 18),
                        le32(&format, 20),
                        le32(&format, 24)
                    ));
                    40
                }
                0x1610 => 30,
                _ => 18,
            };
            if format.len() > header_size {
                row.push_str(&format!(" extra={}", byte_list(&format[header_size..])));
            }
            row
        }
    }
}

/// The `V` row of `run_video`.
fn video_row(
    label: &str,
    params: &CodecParams,
    capacity: u32,
    sar: Rational,
    fps: Rational,
    align: u32,
) -> String {
    match video_format_from_codec_params(params, capacity, sar, fps, align, &Tables) {
        Err(error) => format!(
            "V {label} st={} size={}",
            status_text(error.status()),
            error.required_size().unwrap_or(capacity)
        ),
        Ok(video) => {
            let mut row = format!(
                "V {label} st=0 size={0} dwSize={0} guid={1} w={2} h={3} ap={4},{5} par={6}/{7} fps={8}/{9}",
                video.dw_size,
                guid_text(video.guid_format),
                video.width,
                video.height,
                video.minimum_display_aperture.cx,
                video.minimum_display_aperture.cy,
                video.pixel_aspect_ratio.numerator,
                video.pixel_aspect_ratio.denominator,
                video.frames_per_second.numerator,
                video.frames_per_second.denominator,
            );
            if !video.extradata.is_empty() {
                row.push_str(&format!(" extra={}", byte_list(&video.extradata)));
            }
            row
        }
    }
}

/// The `M` row of `run_media`. A `None` major type prints as the zeroed GUID.
fn media_row(
    label: &str,
    params: &CodecParams,
    sar: Rational,
    fps: Rational,
    capacity: u32,
) -> String {
    let media = media_type_from_codec_params(params, sar, fps, 0, capacity, &Tables);
    let (status, size) = match &media.format {
        Ok(MediaFormat::Audio(format)) => (0, format.len() as u32),
        Ok(MediaFormat::Video(video)) => (0, video.dw_size),
        Err(error) => (error.status(), error.required_size().unwrap_or(capacity)),
    };
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    format!(
        "M {label} st={} size={}  major={}",
        status_text(status),
        size,
        guid_text(media.major.unwrap_or(zero))
    )
}

/// The `R` row of `run_rescale`.
fn rescale_row(time: i64, num: i32, den: i32) -> String {
    let result = user_time(time, Rational { num, den });
    format!("R {time} {num} {den} -> {result}")
}

/// The `ST` row of `run_stream_time`.
fn stream_time_row(time: i64, num: i32, den: i32) -> String {
    let result = stream_time(Rational { num, den }, time);
    format!("ST {time} {num} {den} -> {result}")
}

/// The `C` row of `run_context_duration`, with each stream as `(num, den, duration)`.
fn context_row(streams: &[(i32, i32, i64)], ctx: i64) -> String {
    let streams: Vec<(Rational, i64)> = streams
        .iter()
        .map(|&(num, den, duration)| (Rational { num, den }, duration))
        .collect();
    format!(
        "C n={} ctx={ctx} ->{}",
        streams.len(),
        context_duration(&streams, ctx)
    )
}

/// The `S` row of `run_seek`. `replies` is the dispatch script: `Some` is a reply, `None` a failed
/// dispatch. Once the script runs out, the dispatch echoes the requested offset.
fn seek_row(
    label: &str,
    mut ctx: StreamContext,
    offset: i64,
    whence: i32,
    replies: &[Option<u64>],
) -> String {
    let mut replies = replies.iter().copied();
    let mut requested: Vec<u64> = Vec::new();
    let ret = seek_callback(&mut ctx, offset, whence, &mut |request| {
        requested.push(request);
        replies.next().unwrap_or(Some(request))
    });
    format!(
        "S {label} ret={ret} pos={} size={} calls={} req=[{}]",
        ctx.position,
        ctx.size,
        requested.len(),
        list(&requested)
    )
}

fn seek_ctx(capacity: u32, length: u64, position: u64, size: u32) -> StreamContext {
    StreamContext {
        length,
        position,
        capacity,
        size,
        buffer: Vec::new(),
    }
}

/// The `RD` row of `run_read`. The buffer holds `(i * 7 + 3)` for each index, and the request is
/// `request` bytes. `replies` is the dispatch script, as in `seek_row`.
fn read_row(
    label: &str,
    capacity: u32,
    length: u64,
    position: u64,
    size: u32,
    request: usize,
    replies: &[Option<u32>],
) -> String {
    let mut ctx = StreamContext {
        length,
        position,
        capacity,
        size,
        buffer: (0..capacity).map(|i| (i * 7 + 3) as u8).collect(),
    };
    let mut out = vec![0xee; request];
    let mut replies = replies.iter().copied();
    let mut calls = 0;
    let ret = read_callback(&mut ctx, &mut out, &mut |_| {
        calls += 1;
        replies.next().flatten()
    });
    let shown: &[u8] = if ret > 0 { &out[..ret as usize] } else { &[] };
    format!(
        "RD {label} ret={ret} pos={} size={} calls={calls} out=[{}]",
        ctx.position,
        ctx.size,
        list(shown)
    )
}

/// The `T` row of `run_tag`. `language` picks the 32-byte `language` lookup, otherwise the 256-byte
/// `title` lookup. The text runs up to the first NUL.
fn tag_row(label: &str, metadata: &[(&str, &str)], language: bool) -> String {
    let mut buffer = vec![0; if language { 32 } else { 256 }];
    let status = if language {
        stream_language(metadata, &mut buffer)
    } else {
        stream_name(metadata, &mut buffer)
    };
    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    format!(
        "T {label} st={} str=\"{}\"",
        status_text(status),
        String::from_utf8_lossy(&buffer[..end])
    )
}

/// The `P` row of the pixel format section. `numeric` is the harness's `AVPixelFormat` value.
fn pixel_row(numeric: i32, format: PixelFormat) -> String {
    format!(
        "P {numeric} guid={}",
        guid_text(subtype_from_pixel_format(format))
    )
}

/// The `X` row of the sample aspect ratio section.
fn sar_row(stream: (i32, i32), codecpar: (i32, i32)) -> String {
    let result = stream_sample_aspect_ratio(
        Rational {
            num: stream.0,
            den: stream.1,
        },
        Rational {
            num: codecpar.0,
            den: codecpar.1,
        },
    );
    format!(
        "X s={}/{} c={}/{} -> {}/{}",
        stream.0, stream.1, codecpar.0, codecpar.1, result.num, result.den
    )
}

/// The `D` row of the MIME section.
fn demuxer_row(mime: &str) -> String {
    let status = demuxer_check(mime);
    format!(
        "D {mime} st={} found={}",
        status_text(status),
        u8::from(status == STATUS_SUCCESS)
    )
}

#[test]
fn rescale_rows() {
    assert_eq!(rescale_row(0, 1, 1000), "R 0 1 1000 -> 0");
    assert_eq!(rescale_row(1, 1, 1000), "R 1 1 1000 -> 10000");
    assert_eq!(rescale_row(1000, 1, 1000), "R 1000 1 1000 -> 10000000");
    assert_eq!(rescale_row(-1000, 1, 1000), "R -1000 1 1000 -> -10000000");
    assert_eq!(rescale_row(90000, 1, 90000), "R 90000 1 90000 -> 10000000");
    assert_eq!(rescale_row(5, 1, 3), "R 5 1 3 -> 16666666");
    assert_eq!(rescale_row(-5, 1, 3), "R -5 1 3 -> -16666666");
    assert_eq!(rescale_row(1, 3, 1), "R 1 3 1 -> 30000000");
    assert_eq!(
        rescale_row(123456789012, 1, 1000),
        "R 123456789012 1 1000 -> 1234567890120000"
    );
    assert_eq!(
        rescale_row(-123456789012, 1, 1000),
        "R -123456789012 1 1000 -> -1234567890120000"
    );
    assert_eq!(
        rescale_row(i64::MIN, 1, 1000),
        "R -9223372036854775808 1 1000 -> -9223372036854775808"
    );
    assert_eq!(
        rescale_row(i64::MAX, 1, 1000),
        "R 9223372036854775807 1 1000 -> 9223372036854775807"
    );
    assert_eq!(rescale_row(1, 0, 1), "R 1 0 1 -> 0");
    assert_eq!(rescale_row(1, 1, 0), "R 1 1 0 -> -9223372036854775808");
    assert_eq!(
        rescale_row(1, -1, 1000),
        "R 1 -1 1000 -> -9223372036854775808"
    );
    assert_eq!(
        rescale_row(1, 1, -1000),
        "R 1 1 -1000 -> -9223372036854775808"
    );
    assert_eq!(rescale_row(0, 0, 1), "R 0 0 1 -> 0");
    assert_eq!(rescale_row(3, 0, 0), "R 3 0 0 -> -9223372036854775808");
    assert_eq!(
        rescale_row(1099511627776, 1, 1000000),
        "R 1099511627776 1 1000000 -> 10995116277760"
    );
    assert_eq!(
        rescale_row(4611686018427387903, 1000000000, 1),
        "R 4611686018427387903 1000000000 1 -> -9223372036854775808"
    );
    assert_eq!(
        rescale_row(-4611686018427387903, 1000000000, 1),
        "R -4611686018427387903 1000000000 1 -> -9223372036854775808"
    );
    assert_eq!(
        rescale_row(1000000, 1001, 30000),
        "R 1000000 1001 30000 -> 333666666666"
    );
    assert_eq!(rescale_row(7, 2, 1), "R 7 2 1 -> 140000000");
}

#[test]
fn rescale_wide_rows() {
    assert_eq!(
        rescale_row(4000000000000000000, 1, 1000000000),
        "R 4000000000000000000 1 1000000000 -> 40000000000000000"
    );
    assert_eq!(
        rescale_row(-4000000000000000000, 1, 1000000000),
        "R -4000000000000000000 1 1000000000 -> -40000000000000000"
    );
    assert_eq!(
        rescale_row(9000000000000000000, 1, 1000000000),
        "R 9000000000000000000 1 1000000000 -> 90000000000000000"
    );
    assert_eq!(
        rescale_row(i64::MAX - 1, 1, 1000000000),
        "R 9223372036854775806 1 1000000000 -> 92233720368547758"
    );
    assert_eq!(
        rescale_row(123456789, 1, 1000000000),
        "R 123456789 1 1000000000 -> 1234567"
    );
}

#[test]
fn stream_time_rows() {
    assert_eq!(stream_time_row(100, 1, 90000), "ST 100 1 90000 -> 11111");
    assert_eq!(stream_time_row(100, 0, 0), "ST 100 0 0 -> 1000");
    assert_eq!(stream_time_row(100, 0, 1), "ST 100 0 1 -> 1000");
    assert_eq!(
        stream_time_row(i64::MIN, 1, 90000),
        "ST -9223372036854775808 1 90000 -> -9223372036854775808"
    );
    assert_eq!(stream_time_row(-100, 1, 90000), "ST -100 1 90000 -> -11111");
}

#[test]
fn context_duration_rows() {
    assert_eq!(context_row(&[], 5000000), "C n=0 ctx=5000000 ->50000000");
    assert_eq!(
        context_row(&[], i64::MIN),
        "C n=0 ctx=-9223372036854775808 ->-9223372036854775808"
    );
    assert_eq!(
        context_row(&[(1, 90000, 45000)], 0),
        "C n=1 ctx=0 ->5000000"
    );
    assert_eq!(
        context_row(&[(1, 90000, i64::MIN)], 777),
        "C n=1 ctx=777 ->7770"
    );
    assert_eq!(
        context_row(&[(1, 90000, 90000), (1, 1000, 5)], 0),
        "C n=2 ctx=0 ->10000000"
    );
    assert_eq!(
        context_row(&[(1, 90000, i64::MIN), (1, 1000, 7)], 0),
        "C n=2 ctx=0 ->70000"
    );
    assert_eq!(
        context_row(&[(1, 1000, i64::MIN), (1, 1000, 3), (1, 1000, 9)], 0),
        "C n=3 ctx=0 ->90000"
    );
    assert_eq!(
        context_row(
            &[(1, 1000, 2), (1, 1000, i64::MIN), (1, 1000, i64::MIN)],
            99
        ),
        "C n=3 ctx=99 ->20000"
    );
    assert_eq!(
        context_row(&[(1, 0, 10), (1, 2, -4), (0, 0, 5)], 1),
        "C n=3 ctx=1 ->100"
    );
    assert_eq!(
        context_row(&[(1, 1000, i64::MIN), (1, 1000, i64::MIN)], 12345),
        "C n=2 ctx=12345 ->123450"
    );
}

#[test]
fn seek_rows() {
    assert_eq!(
        seek_row("size_query", seek_ctx(64, 1000, 10, 3), 0, AVSEEK_SIZE, &[]),
        "S size_query ret=1000 pos=10 size=3 calls=0 req=[]"
    );
    assert_eq!(
        seek_row(
            "set_inside_same_chunk",
            seek_ctx(64, 1000, 10, 3),
            40,
            SEEK_SET,
            &[]
        ),
        "S set_inside_same_chunk ret=40 pos=40 size=3 calls=0 req=[]"
    );
    assert_eq!(
        seek_row(
            "set_past_end_clamp",
            seek_ctx(64, 1000, 10, 3),
            5000,
            SEEK_SET,
            &[]
        ),
        "S set_past_end_clamp ret=1000 pos=1000 size=0 calls=1 req=[960]"
    );
    assert_eq!(
        seek_row(
            "negative_set_clamp",
            seek_ctx(64, 1000, 10, 3),
            -5,
            SEEK_SET,
            &[]
        ),
        "S negative_set_clamp ret=1000 pos=1000 size=0 calls=1 req=[960]"
    );
    assert_eq!(
        seek_row(
            "cur_moves_chunk",
            seek_ctx(64, 1000, 10, 3),
            100,
            SEEK_CUR,
            &[]
        ),
        "S cur_moves_chunk ret=110 pos=110 size=0 calls=1 req=[64]"
    );
    assert_eq!(
        seek_row(
            "cur_negative",
            seek_ctx(64, 1000, 100, 3),
            -30,
            SEEK_CUR,
            &[]
        ),
        "S cur_negative ret=70 pos=70 size=3 calls=0 req=[]"
    );
    assert_eq!(
        seek_row("end_minus", seek_ctx(64, 1000, 10, 3), -1, SEEK_END, &[]),
        "S end_minus ret=999 pos=999 size=0 calls=1 req=[960]"
    );
    assert_eq!(
        seek_row(
            "end_plus_clamp",
            seek_ctx(64, 1000, 10, 3),
            500,
            SEEK_END,
            &[]
        ),
        "S end_plus_clamp ret=1000 pos=1000 size=0 calls=1 req=[960]"
    );
    assert_eq!(
        seek_row(
            "chunk_reply_exact",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[Some(100)]
        ),
        "S chunk_reply_exact ret=100 pos=100 size=0 calls=2 req=[192,64]"
    );
    assert_eq!(
        seek_row(
            "chunk_reply_moves",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[Some(64), Some(128)]
        ),
        "S chunk_reply_moves ret=128 pos=128 size=0 calls=3 req=[192,64,128]"
    );
    assert_eq!(
        seek_row(
            "chunk_reply_unaligned",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[Some(130)]
        ),
        "S chunk_reply_unaligned ret=130 pos=130 size=0 calls=2 req=[192,128]"
    );
    assert_eq!(
        seek_row(
            "status_failure",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[None]
        ),
        "S status_failure ret=-22 pos=0 size=3 calls=1 req=[192]"
    );
    assert_eq!(
        seek_row(
            "bad_length",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[None]
        ),
        "S bad_length ret=-22 pos=0 size=3 calls=1 req=[192]"
    );
    assert_eq!(
        seek_row(
            "reply_after_failure_chain",
            seek_ctx(64, 1000, 0, 3),
            200,
            SEEK_SET,
            &[Some(64), Some(192), Some(0)]
        ),
        "S reply_after_failure_chain ret=0 pos=0 size=0 calls=4 req=[192,64,192,0]"
    );
    assert_eq!(
        seek_row("capacity_one", seek_ctx(1, 10, 0, 3), 7, SEEK_SET, &[]),
        "S capacity_one ret=7 pos=7 size=0 calls=1 req=[7]"
    );
    assert_eq!(
        seek_row("zero_length_clamp", seek_ctx(16, 0, 0, 3), 9, SEEK_SET, &[]),
        "S zero_length_clamp ret=0 pos=0 size=3 calls=0 req=[]"
    );
    assert_eq!(
        seek_row("size_query_end", seek_ctx(16, 0, 0, 3), 0, AVSEEK_SIZE, &[]),
        "S size_query_end ret=0 pos=0 size=3 calls=0 req=[]"
    );
}

#[test]
fn tag_rows() {
    assert_eq!(
        tag_row("lang_plain", &[("language", "eng")], true),
        "T lang_plain st=0 str=\"eng\""
    );
    assert_eq!(
        tag_row("lang_absent", &[("title", "Movie")], true),
        "T lang_absent st=0xc0000225 str=\"\""
    );
    assert_eq!(
        tag_row("name_present", &[("title", "Movie")], false),
        "T name_present st=0 str=\"Movie\""
    );
    assert_eq!(
        tag_row("lang_case", &[("Language", "fre")], true),
        "T lang_case st=0 str=\"fre\""
    );
    assert_eq!(
        tag_row("lang_suffix", &[("language-eng", "x")], true),
        "T lang_suffix st=0 str=\"x\""
    );
    assert_eq!(
        tag_row("lang_longer", &[("languages", "x")], true),
        "T lang_longer st=0 str=\"x\""
    );
    assert_eq!(
        tag_row("lang_shorter", &[("lang", "x")], true),
        "T lang_shorter st=0xc0000225 str=\"\""
    );
    assert_eq!(
        tag_row("name_suffix", &[("title-x", "y")], false),
        "T name_suffix st=0 str=\"y\""
    );
    assert_eq!(
        tag_row("name_case", &[("TITLE", "Upper")], false),
        "T name_case st=0 str=\"Upper\""
    );
    assert_eq!(
        tag_row("lang_empty", &[("language", "")], true),
        "T lang_empty st=0 str=\"\""
    );
    assert_eq!(
        tag_row(
            "lang_truncated",
            &[("language", "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJ")],
            true
        ),
        "T lang_truncated st=0 str=\"abcdefghijklmnopqrstuvwxyz01234\""
    );
    // av_dict_set replaces the value of an existing key, so only "b" remains.
    assert_eq!(
        tag_row("lang_first_wins", &[("language", "b")], true),
        "T lang_first_wins st=0 str=\"b\""
    );
    assert_eq!(
        tag_row("lang_no_dict", &[], true),
        "T lang_no_dict st=0xc0000225 str=\"\""
    );
    assert_eq!(
        tag_row("name_no_dict", &[], false),
        "T name_no_dict st=0xc0000225 str=\"\""
    );
}

#[test]
fn pixel_rows() {
    assert_eq!(
        pixel_row(-1, PixelFormat::None),
        "P -1 guid=00000000-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(0, PixelFormat::Yuv420p),
        "P 0 guid=30323449-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(1, PixelFormat::Yuyv422),
        "P 1 guid=32595559-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(2, PixelFormat::Other),
        "P 2 guid=00000000-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(3, PixelFormat::Bgr24),
        "P 3 guid=00000014-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(5, PixelFormat::Other),
        "P 5 guid=00000000-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(12, PixelFormat::Yuvj420p),
        "P 12 guid=30323449-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(15, PixelFormat::Uyvy422),
        "P 15 guid=59565955-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(20, PixelFormat::Rgb8),
        "P 20 guid=00000029-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(23, PixelFormat::Nv12),
        "P 23 guid=3231564e-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(24, PixelFormat::Nv21),
        "P 24 guid=3132564e-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(26, PixelFormat::Rgba),
        "P 26 guid=00000020-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(28, PixelFormat::Bgra),
        "P 28 guid=00000015-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(37, PixelFormat::Rgb565le),
        "P 37 guid=00000017-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(39, PixelFormat::Rgb555le),
        "P 39 guid=00000018-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(121, PixelFormat::Bgr0),
        "P 121 guid=00000016-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(158, PixelFormat::P010le),
        "P 158 guid=30313050-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        pixel_row(9, PixelFormat::Other),
        "P 9 guid=00000000-0000-0010-8000-00aa00389b71"
    );
}

#[test]
fn video_rows() {
    let raw = |pixel_format, width, height| CodecParams {
        width,
        height,
        pixel_format,
        ..base(MediaKind::Video, CodecId::RawVideo)
    };
    assert_eq!(
        video_row("raw_i420_align16", &raw(PixelFormat::Yuv420p, 1920, 1080), 512, ZERO, ZERO, 16),
        "V raw_i420_align16 st=0 size=168 dwSize=168 guid=30323449-0000-0010-8000-00aa00389b71 w=1920 h=1096 ap=1920,1080 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row("raw_argb_align0", &raw(PixelFormat::Bgra, 640, 480), 512, SAR_4_3, NTSC, 0),
        "V raw_argb_align0 st=0 size=168 dwSize=168 guid=00000015-0000-0010-8000-00aa00389b71 w=640 h=480 ap=0,0 par=4/3 fps=30000/1001"
    );
    assert_eq!(
        video_row(
            "raw_none_no_tag",
            &raw(PixelFormat::None, 320, 240),
            512,
            Rational { num: 0, den: 1 },
            Rational { num: 25, den: 0 },
            8
        ),
        "V raw_none_no_tag st=0 size=168 dwSize=168 guid=00000000-0000-0010-8000-00aa00389b71 w=320 h=240 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "raw_none_codec_tag",
            &CodecParams {
                codec_tag: 0x1234_5678,
                ..raw(PixelFormat::None, 320, 240)
            },
            512,
            ZERO,
            ZERO,
            0
        ),
        "V raw_none_codec_tag st=0 size=168 dwSize=168 guid=12345678-0000-0010-8000-00aa00389b71 w=320 h=240 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row("raw_unmapped_fmt_align32", &raw(PixelFormat::Other, 33, 17), 512, ZERO, ZERO, 32),
        "V raw_unmapped_fmt_align32 st=0 size=168 dwSize=168 guid=00000000-0000-0010-8000-00aa00389b71 w=65 h=17 ap=33,17 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "mpeg1",
            &CodecParams {
                width: 352,
                height: 288,
                ..base(MediaKind::Video, CodecId::Mpeg1Video)
            },
            512,
            ZERO,
            ZERO,
            0
        ),
        "V mpeg1 st=0 size=168 dwSize=168 guid=e436eb81-524f-11ce-9f53-0020af0ba770 w=352 h=288 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "h264_extradata",
            &CodecParams {
                width: 1280,
                height: 720,
                extradata: &EXTRA10,
                ..base(MediaKind::Video, CodecId::H264)
            },
            512,
            ZERO,
            ZERO,
            16
        ),
        "V h264_extradata st=0 size=178 dwSize=178 guid=34363248-0000-0010-8000-00aa00389b71 w=1280 h=736 ap=1280,720 par=1/1 fps=0/0 extra=[1,2,3,4,5,6,7,8,9,10]"
    );
    assert_eq!(
        video_row(
            "vp9",
            &CodecParams {
                width: 16,
                height: 16,
                ..base(MediaKind::Video, CodecId::Vp9)
            },
            512,
            ZERO,
            ZERO,
            0
        ),
        "V vp9 st=0 size=168 dwSize=168 guid=30395056-0000-0010-8000-00aa00389b71 w=16 h=16 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "hevc_tag_lookup",
            &CodecParams {
                width: 16,
                height: 16,
                ..base(MediaKind::Video, CodecId::Hevc)
            },
            512,
            ZERO,
            ZERO,
            0
        ),
        "V hevc_tag_lookup st=0 size=168 dwSize=168 guid=31766568-0000-0010-8000-00aa00389b71 w=16 h=16 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "hevc_codec_tag",
            &CodecParams {
                codec_tag: 0x3163_6568,
                width: 16,
                height: 16,
                ..base(MediaKind::Video, CodecId::Hevc)
            },
            512,
            ZERO,
            ZERO,
            0
        ),
        "V hevc_codec_tag st=0 size=168 dwSize=168 guid=31636568-0000-0010-8000-00aa00389b71 w=16 h=16 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "capacity_too_small",
            &CodecParams {
                width: 1280,
                height: 720,
                extradata: &EXTRA10,
                ..base(MediaKind::Video, CodecId::H264)
            },
            100,
            ZERO,
            ZERO,
            0
        ),
        "V capacity_too_small st=0xc0000023 size=178"
    );
    assert_eq!(
        video_row(
            "mpeg1_exact_capacity",
            &CodecParams {
                width: 1,
                height: 1,
                ..base(MediaKind::Video, CodecId::Mpeg1Video)
            },
            MF_VIDEO_FORMAT_SIZE as u32,
            ZERO,
            ZERO,
            0
        ),
        "V mpeg1_exact_capacity st=0 size=168 dwSize=168 guid=e436eb81-524f-11ce-9f53-0020af0ba770 w=1 h=1 ap=0,0 par=1/1 fps=0/0"
    );
    assert_eq!(
        video_row(
            "zero_dims_align",
            &raw(PixelFormat::Rgb8, 0, 0),
            512,
            SAR_4_3,
            NTSC,
            16
        ),
        "V zero_dims_align st=0 size=168 dwSize=168 guid=00000029-0000-0010-8000-00aa00389b71 w=0 h=0 ap=0,0 par=4/3 fps=30000/1001"
    );
}

#[test]
fn audio_rows() {
    let pcm = |channels| CodecParams {
        channels,
        sample_rate: 44100,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    assert_eq!(
        audio_row("pcm_s16_defaults", &pcm(2), 64),
        "A pcm_s16_defaults st=0 size=18 tag=1 ch=2 sps=44100 avg=176400 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "pcm_s16_explicit",
            &CodecParams {
                block_align: 4,
                bit_rate: 1411200,
                ..pcm(2)
            },
            64
        ),
        "A pcm_s16_explicit st=0 size=18 tag=1 ch=2 sps=44100 avg=176400 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "zero_channels_zero_bits",
            &CodecParams {
                channels: 0,
                sample_rate: 48000,
                bits_per_coded_sample: 0,
                ..base(MediaKind::Audio, CodecId::PcmS16le)
            },
            64
        ),
        "A zero_channels_zero_bits st=0 size=18 tag=1 ch=0 sps=48000 avg=0 align=0 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "zero_channels_coded_bits",
            &CodecParams {
                channels: 0,
                sample_rate: 48000,
                bits_per_coded_sample: 24,
                ..base(MediaKind::Audio, CodecId::PcmS16le)
            },
            64
        ),
        "A zero_channels_coded_bits st=0 size=18 tag=1 ch=0 sps=48000 avg=0 align=0 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "bit_rate_truncation",
            &CodecParams {
                bit_rate: 1411200 * 4096 + 8,
                ..pcm(2)
            },
            64
        ),
        "A bit_rate_truncation st=0 size=18 tag=1 ch=2 sps=44100 avg=722534401 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "pcm_with_extradata",
            &CodecParams {
                extradata: &EXTRA5,
                ..pcm(2)
            },
            64
        ),
        "A pcm_with_extradata st=0 size=23 tag=1 ch=2 sps=44100 avg=176400 align=4 bits=16 cb=5 extra=[1,2,3,4,5]"
    );
    assert_eq!(
        audio_row(
            "pcm_capacity_small",
            &CodecParams {
                extradata: &EXTRA5,
                ..pcm(2)
            },
            10
        ),
        "A pcm_capacity_small st=0xc0000023 size=23"
    );
    let aac = CodecParams {
        channels: 2,
        sample_rate: 48000,
        extradata: &EXTRA2,
        ..base(MediaKind::Audio, CodecId::Aac)
    };
    assert_eq!(
        audio_row("aac_heaac", &aac, 64),
        "A aac_heaac st=0 size=32 tag=5648 ch=2 sps=48000 avg=0 align=0 bits=0 cb=14 extra=[18,16]"
    );
    assert_eq!(
        audio_row("aac_capacity_small", &aac, 31),
        "A aac_capacity_small st=0xc0000023 size=32"
    );
    assert_eq!(
        audio_row(
            "aac_no_extradata",
            &CodecParams {
                extradata: &[],
                ..aac
            },
            64
        ),
        "A aac_no_extradata st=0 size=30 tag=5648 ch=2 sps=48000 avg=0 align=0 bits=0 cb=12"
    );
    let codec = |id| CodecParams {
        channels: 2,
        sample_rate: 44100,
        ..base(MediaKind::Audio, id)
    };
    assert_eq!(
        audio_row("mp1_size40", &codec(CodecId::Mp1), 64),
        "A mp1_size40 st=0 size=62 tag=65534 ch=2 sps=44100 avg=0 align=0 bits=0 cb=44 valid=0 mask=0 sub=31706d2e extra=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row("mp3_size30", &codec(CodecId::Mp3), 64),
        "A mp3_size30 st=0 size=30 tag=85 ch=2 sps=44100 avg=0 align=0 bits=0 cb=12 extra=[0,0,0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row("wmav1_size22", &codec(CodecId::Wmav1), 64),
        "A wmav1_size22 st=0 size=22 tag=352 ch=2 sps=44100 avg=0 align=0 bits=0 cb=4 extra=[0,0,0,0]"
    );
    assert_eq!(
        audio_row("wmav2_size28", &codec(CodecId::Wmav2), 64),
        "A wmav2_size28 st=0 size=28 tag=353 ch=2 sps=44100 avg=0 align=0 bits=0 cb=10 extra=[0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row("wmapro_size36", &codec(CodecId::Wmapro), 64),
        "A wmapro_size36 st=0 size=36 tag=354 ch=2 sps=44100 avg=0 align=0 bits=0 cb=18 extra=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row("wmavoice_size36", &codec(CodecId::Wmavoice), 64),
        "A wmavoice_size36 st=0 size=36 tag=10 ch=2 sps=44100 avg=0 align=0 bits=0 cb=18 extra=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row("wmalossless_size36", &codec(CodecId::Wmalossless), 64),
        "A wmalossless_size36 st=0 size=36 tag=355 ch=2 sps=44100 avg=0 align=0 bits=0 cb=18 extra=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
    );
    assert_eq!(
        audio_row(
            "mp3_extradata_larger_than_wfx",
            &CodecParams {
                extradata: &EXTRA5,
                ..codec(CodecId::Mp3)
            },
            64
        ),
        "A mp3_extradata_larger_than_wfx st=0 size=30 tag=85 ch=2 sps=44100 avg=0 align=0 bits=0 cb=12 extra=[1,2,3,4,5,0,0,0,0,0,0,0]"
    );
    let six = |native, mask, extradata: &'static [u8]| CodecParams {
        channels: 6,
        channel_order_native: native,
        channel_mask: mask,
        sample_rate: 48000,
        bits_per_coded_sample: 16,
        extradata,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    assert_eq!(
        audio_row("six_channels_native_mask", &six(true, 0x3f, &[]), 128),
        "A six_channels_native_mask st=0 size=40 tag=65534 ch=6 sps=48000 avg=576000 align=12 bits=16 cb=22 valid=0 mask=63 sub=00000001"
    );
    assert_eq!(
        audio_row("six_channels_non_native", &six(false, 0x3f, &[]), 128),
        "A six_channels_non_native st=0 size=18 tag=1 ch=6 sps=48000 avg=576000 align=12 bits=16 cb=0"
    );
    assert_eq!(
        audio_row(
            "stereo_native_mask",
            &CodecParams {
                channels: 2,
                ..six(true, 0x3, &[])
            },
            128
        ),
        "A stereo_native_mask st=0 size=18 tag=1 ch=2 sps=48000 avg=192000 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row("six_channels_small_capacity", &six(true, 0x3f, &[]), 20),
        "A six_channels_small_capacity st=0xc0000023 size=40"
    );
    assert_eq!(
        audio_row(
            "alac_tag_lookup",
            &CodecParams {
                channels: 2,
                sample_rate: 44100,
                bits_per_coded_sample: 16,
                ..base(MediaKind::Audio, CodecId::Alac)
            },
            128
        ),
        "A alac_tag_lookup st=0 size=40 tag=65534 ch=2 sps=44100 avg=176400 align=4 bits=16 cb=22 valid=0 mask=0 sub=63616c61"
    );
    assert_eq!(
        audio_row(
            "alac_six_channels_extensible_extra",
            &CodecParams {
                channels: 6,
                sample_rate: 44100,
                bits_per_coded_sample: 16,
                channel_order_native: true,
                channel_mask: 0x3f,
                extradata: &EXTRA5,
                ..base(MediaKind::Audio, CodecId::Alac)
            },
            128
        ),
        "A alac_six_channels_extensible_extra st=0 size=45 tag=65534 ch=6 sps=44100 avg=529200 align=12 bits=16 cb=27 valid=0 mask=63 sub=63616c61 extra=[1,2,3,4,5]"
    );
    assert_eq!(
        audio_row("six_channels_extensible_extra", &six(true, 0x3f, &EXTRA5), 128),
        "A six_channels_extensible_extra st=0 size=45 tag=65534 ch=6 sps=48000 avg=576000 align=12 bits=16 cb=27 valid=0 mask=63 sub=00000001 extra=[1,2,3,4,5]"
    );
    assert_eq!(
        audio_row(
            "six_channels_extensible_small_capacity",
            &six(true, 0x3f, &EXTRA5),
            40
        ),
        "A six_channels_extensible_small_capacity st=0xc0000023 size=45"
    );
    assert_eq!(
        audio_row(
            "zero_rate_zero_avg",
            &CodecParams {
                channels: 2,
                ..base(MediaKind::Audio, CodecId::PcmS16le)
            },
            64
        ),
        "A zero_rate_zero_avg st=0 size=18 tag=1 ch=2 sps=0 avg=0 align=4 bits=16 cb=0"
    );
}

#[test]
fn sar_rows() {
    assert_eq!(sar_row((4, 3), (1, 1)), "X s=4/3 c=1/1 -> 4/3");
    assert_eq!(sar_row((0, 0), (16, 9)), "X s=0/0 c=16/9 -> 16/9");
    assert_eq!(sar_row((0, 5), (16, 9)), "X s=0/5 c=16/9 -> 16/9");
    assert_eq!(sar_row((5, 0), (16, 9)), "X s=5/0 c=16/9 -> 16/9");
    assert_eq!(sar_row((0, 0), (0, 0)), "X s=0/0 c=0/0 -> 0/0");
    assert_eq!(sar_row((-2, 3), (0, 0)), "X s=-2/3 c=0/0 -> -2/3");
}

#[test]
fn audio_extra_rows() {
    let pcm = |channels, channel_order_native, channel_mask, sample_rate, bit_rate| CodecParams {
        channels,
        channel_order_native,
        channel_mask,
        sample_rate,
        bits_per_coded_sample: 16,
        bit_rate,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    assert_eq!(
        audio_row("bit_rate_dword_truncation_to_16", &pcm(2, false, 0, 44100, (1 << 35) * 8 + 16 * 8), 64),
        "A bit_rate_dword_truncation_to_16 st=0 size=18 tag=1 ch=2 sps=44100 avg=16 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row("bit_rate_dword_truncation_to_zero", &pcm(2, false, 0, 44100, (1 << 35) * 8), 64),
        "A bit_rate_dword_truncation_to_zero st=0 size=18 tag=1 ch=2 sps=44100 avg=176400 align=4 bits=16 cb=0"
    );
    assert_eq!(
        audio_row("zero_channels_native_mask_default_one", &pcm(0, true, 0x3f, 48000, 0), 64),
        "A zero_channels_native_mask_default_one st=0 size=18 tag=1 ch=0 sps=48000 avg=0 align=0 bits=16 cb=0"
    );
    assert_eq!(
        audio_row("three_channels_no_mask", &pcm(3, false, 0, 48000, 0), 64),
        "A three_channels_no_mask st=0 size=18 tag=1 ch=3 sps=48000 avg=288000 align=6 bits=16 cb=0"
    );
}

#[test]
fn media_rows() {
    let audio = CodecParams {
        channels: 2,
        sample_rate: 44100,
        bits_per_coded_sample: 16,
        ..base(MediaKind::Audio, CodecId::PcmS16le)
    };
    assert_eq!(
        media_row("audio", &audio, ZERO, FPS_25, 64),
        "M audio st=0 size=18  major=73647561-0000-0010-8000-00aa00389b71"
    );
    let video = CodecParams {
        width: 320,
        height: 240,
        ..base(MediaKind::Video, CodecId::H264)
    };
    assert_eq!(
        media_row("video", &video, SAR_4_3, FPS_25, 512),
        "M video st=0 size=168  major=73646976-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        media_row(
            "subtitle_not_implemented",
            &CodecParams {
                codec_type: MediaKind::Other,
                ..video
            },
            SAR_4_3,
            FPS_25,
            512
        ),
        "M subtitle_not_implemented st=0xc0000002 size=512  major=00000000-0000-0000-0000-000000000000"
    );
    assert_eq!(
        media_row("video_too_small", &video, SAR_4_3, FPS_25, 10),
        "M video_too_small st=0xc0000023 size=168  major=73646976-0000-0010-8000-00aa00389b71"
    );
}

#[test]
fn seek_timestamp_rows() {
    assert_eq!(format!("TS 0 -> {}", seek_timestamp(0)), "TS 0 -> 0");
    assert_eq!(format!("TS 1 -> {}", seek_timestamp(1)), "TS 1 -> 0");
    assert_eq!(format!("TS 9 -> {}", seek_timestamp(9)), "TS 9 -> 0");
    assert_eq!(
        format!("TS 10000000 -> {}", seek_timestamp(10000000)),
        "TS 10000000 -> 1000000"
    );
    assert_eq!(
        format!("TS 5000000 -> {}", seek_timestamp(5000000)),
        "TS 5000000 -> 500000"
    );
    assert_eq!(
        format!("TS -10000000 -> {}", seek_timestamp(-10000000)),
        "TS -10000000 -> -1000000"
    );
    assert_eq!(format!("TS -15 -> {}", seek_timestamp(-15)), "TS -15 -> -1");
    assert_eq!(
        format!("TS 123456789 -> {}", seek_timestamp(123456789)),
        "TS 123456789 -> 12345678"
    );
    assert_eq!(
        format!("TS 9223372036854 -> {}", seek_timestamp(9223372036854)),
        "TS 9223372036854 -> 922337203685"
    );
}

#[test]
fn mime_rows() {
    assert_eq!(demuxer_row("video/mp4"), "D video/mp4 st=0 found=1");
    assert_eq!(demuxer_row("video/avi"), "D video/avi st=0 found=1");
    assert_eq!(demuxer_row("audio/wav"), "D audio/wav st=0 found=1");
    assert_eq!(
        demuxer_row("audio/x-ms-wma"),
        "D audio/x-ms-wma st=0 found=1"
    );
    assert_eq!(
        demuxer_row("video/x-ms-wmv"),
        "D video/x-ms-wmv st=0 found=1"
    );
    assert_eq!(
        demuxer_row("video/x-ms-asf"),
        "D video/x-ms-asf st=0 found=1"
    );
    assert_eq!(demuxer_row("video/mpeg"), "D video/mpeg st=0 found=1");
    assert_eq!(demuxer_row("audio/mp3"), "D audio/mp3 st=0 found=1");
    assert_eq!(
        demuxer_row("audio/mpeg"),
        "D audio/mpeg st=0xc00000bb found=0"
    );
    assert_eq!(
        demuxer_row("video/MP4"),
        "D video/MP4 st=0xc00000bb found=0"
    );
    assert_eq!(demuxer_row(""), "D  st=0xc00000bb found=0");
    assert_eq!(
        demuxer_row("video/mp4 "),
        "D video/mp4  st=0xc00000bb found=0"
    );
    assert_eq!(
        demuxer_row("audio/aac"),
        "D audio/aac st=0xc00000bb found=0"
    );
    assert_eq!(
        demuxer_row("video/quicktime"),
        "D video/quicktime st=0xc00000bb found=0"
    );
}

#[test]
fn read_rows() {
    assert_eq!(
        read_row("cached_chunk", 8, 40, 2, 8, 4, &[]),
        "RD cached_chunk ret=4 pos=6 size=8 calls=0 out=[17,24,31,38]"
    );
    assert_eq!(
        read_row("first_chunk_partial", 8, 40, 0, 0, 5, &[Some(8)]),
        "RD first_chunk_partial ret=5 pos=5 size=8 calls=1 out=[3,10,17,24,31]"
    );
    assert_eq!(
        read_row("cross_chunk_two_loads", 8, 40, 0, 0, 12, &[Some(8), Some(8)]),
        "RD cross_chunk_two_loads ret=12 pos=12 size=8 calls=2 out=[3,10,17,24,31,38,45,52,3,10,17,24]"
    );
    assert_eq!(
        read_row("eof_clamp_reply_short", 8, 10, 8, 0, 16, &[Some(2)]),
        "RD eof_clamp_reply_short ret=2 pos=10 size=2 calls=1 out=[3,10]"
    );
    assert_eq!(
        read_row("at_end_eof", 8, 10, 10, 0, 4, &[]),
        "RD at_end_eof ret=-541478725 pos=10 size=0 calls=0 out=[]"
    );
    assert_eq!(
        read_row("zero_request_eof", 8, 40, 0, 0, 0, &[]),
        "RD zero_request_eof ret=-541478725 pos=0 size=0 calls=0 out=[]"
    );
    assert_eq!(
        read_row("dispatch_status_failure", 8, 40, 0, 0, 4, &[None]),
        "RD dispatch_status_failure ret=-22 pos=0 size=0 calls=1 out=[]"
    );
    assert_eq!(
        read_row("dispatch_bad_length", 8, 40, 0, 0, 4, &[None]),
        "RD dispatch_bad_length ret=-22 pos=0 size=0 calls=1 out=[]"
    );
    assert_eq!(
        read_row("zero_reply_eof", 8, 40, 0, 0, 4, &[Some(0)]),
        "RD zero_reply_eof ret=-541478725 pos=0 size=0 calls=1 out=[]"
    );
    assert_eq!(
        read_row("stale_size_below_offset", 8, 40, 5, 3, 3, &[]),
        "RD stale_size_below_offset ret=3 pos=8 size=0 calls=0 out=[38,45,52]"
    );
    assert_eq!(
        read_row("reload_after_chunk_boundary", 8, 40, 6, 8, 5, &[Some(8)]),
        "RD reload_after_chunk_boundary ret=5 pos=11 size=8 calls=1 out=[45,52,3,10,17]"
    );
    assert_eq!(
        read_row("two_chunks_partial_then_eof", 8, 16, 0, 0, 16, &[Some(8), Some(4)]),
        "RD two_chunks_partial_then_eof ret=12 pos=12 size=4 calls=2 out=[3,10,17,24,31,38,45,52,3,10,17,24]"
    );
}
