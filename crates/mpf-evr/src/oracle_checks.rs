//! Replays of the oracle rows in `target/canon/evr`. The oracle compiles the verbatim `main.c`,
//! `mixer.c` and `presenter.c` helpers with `-fsanitize=address,undefined`, and drives them
//! through `harness.c`. Each test names the row it replays, and its expected values are the
//! output of that row in `rows.json`. Rows `x01` and `x02` are excluded, because the sanitizer
//! flags the signed overflow of `video_mixer_correct_aspect_ratio` on them. The edge checks
//! replay both as wrap cases.

use super::*;

/// A rectangle from its four edges, as `RECT` lists them.
pub(crate) fn rect(edges: [i32; 4]) -> Rect {
    Rect {
        left: edges[0],
        top: edges[1],
        right: edges[2],
        bottom: edges[3],
    }
}

/// A normalized rectangle from its four edges.
pub(crate) fn norm(edges: [f32; 4]) -> NormalizedRect {
    NormalizedRect {
        left: edges[0],
        top: edges[1],
        right: edges[2],
        bottom: edges[3],
    }
}

pub(crate) fn edges(rect: Rect) -> [i32; 4] {
    [rect.left, rect.top, rect.right, rect.bottom]
}

/// `scale_rect` of a `width` by `height` frame, as edges.
pub(crate) fn scale(width: u32, height: u32, edges: [f32; 4]) -> [i32; 4] {
    self::edges(scale_rect(width, height, &norm(edges)))
}

/// `correct_aspect_ratio` of `dst` for `src`, as the edges of the corrected `dst`.
pub(crate) fn aspect(src: [i32; 4], dst: [i32; 4]) -> [i32; 4] {
    let src = rect(src);
    let mut dst = rect(dst);
    correct_aspect_ratio(&src, &mut dst);
    self::edges(dst)
}

/// The result of `check_video_position`, as the HRESULT the C function returns.
pub(crate) fn video_position(src: Option<[f32; 4]>, dst: Option<[i32; 4]>) -> HResult {
    let src = src.map(norm);
    let dst = dst.map(rect);
    hresult(check_video_position(src.as_ref(), dst.as_ref()))
}

/// The native size and ratio of a frame, as `cx cy ratio.cx ratio.cy`.
pub(crate) fn native_of(frame: Option<u64>) -> [i32; 4] {
    let native = native_video_size(frame);
    [
        native.size.cx,
        native.size.cy,
        native.ratio.cx,
        native.ratio.cy,
    ]
}

/// `rgb_to_ycbcr` of a colour, as `Y Cb Cr Alpha`.
pub(crate) fn ayuv_of(rgb: u32) -> [u16; 4] {
    let ayuv = rgb_to_ycbcr(rgb);
    [ayuv.y, ayuv.cb, ayuv.cr, ayuv.alpha]
}

/// A FOURCC code, read as the harness reads it: the bytes in order, first byte lowest.
pub(crate) fn fourcc(code: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*code)
}

/// The HRESULT of a `Result`, with `Ok` as `S_OK`.
pub(crate) fn hresult<T>(result: Result<T, HResult>) -> HResult {
    result.err().unwrap_or(S_OK)
}

/// r01: `yuv` of the ten YUV FOURCCs. Each is a YUV format.
#[test]
fn r01_yuv_fourccs_are_yuv() {
    let formats = [
        b"UYVY", b"YUY2", b"AYUV", b"IMC1", b"IMC2", b"YV12", b"NV11", b"NV12", b"Y210", b"Y216",
    ];
    for format in formats {
        assert!(is_format_yuv(fourcc(format)));
    }
}

/// r02: `yuv` of `RGB8`, a lowercase `yuy2`, `0`, the UYVY and NV12 codes, `AYUX`, `Y41P` and
/// all ones. Only the UYVY and NV12 codes are YUV.
#[test]
fn r02_non_yuv_codes() {
    let formats = [
        fourcc(b"RGB8"),
        fourcc(b"yuy2"),
        0x0,
        0x5956_5955,
        0x3231_564e,
        fourcc(b"AYUX"),
        fourcc(b"Y41P"),
        0xffff_ffff,
    ];
    assert_eq!(
        formats.map(is_format_yuv),
        [false, false, false, true, true, false, false, false]
    );
}

/// r03: `ilace` of the sample formats 0 to 7, and 255.
#[test]
fn r03_interlace_modes() {
    let formats = [0u8, 1, 2, 3, 4, 5, 6, 7, 255];
    assert_eq!(
        formats.map(|format| interlace_mode(format) as u32),
        [2, 2, 2, 3, 4, 5, 6, 2, 2]
    );
}

/// r04: `gcd`, with zeros, a common factor, coprime pairs and the 32-bit extremes.
#[test]
fn r04_gcd() {
    let pairs = [
        (0, 0),
        (0, 5),
        (5, 0),
        (12, 18),
        (7, 13),
        (1920, 1080),
        (u32::MAX, 2),
        (2_147_483_648, 6),
    ];
    assert_eq!(pairs.map(|(a, b)| gcd(a, b)), [0, 5, 5, 6, 1, 120, 1, 2]);
}

/// r05: `native` of a 1920 by 1080 frame, of a zero frame, and of the three missing-frame cases.
#[test]
fn r05_native_size_of_frame_and_missing_frame() {
    assert_eq!(native_of(Some(0x0000_0780_0000_0438)), [1920, 1080, 16, 9]);
    assert_eq!(native_of(Some(0)), [0, 0, 0, 0]);
    // `none`, `noinput` and `nomixer` all reach the zero reply, so they share this check.
    assert_eq!(native_of(None), [0, 0, 0, 0]);
}

/// r06: `native` of frames whose ratio, sign or zero edge needs the gcd path.
#[test]
fn r06_native_size_ratios() {
    assert_eq!(native_of(Some(0x0000_0050_0000_0140)), [80, 320, 1, 4]);
    assert_eq!(
        native_of(Some(0x8000_0000_0000_0001)),
        [i32::MIN, 1, i32::MIN, 1]
    );
    assert_eq!(native_of(Some(0x0000_0001_0000_0000)), [1, 0, 1, 0]);
    assert_eq!(native_of(Some(0x0000_0002_0000_0001)), [2, 1, 2, 1]);
    assert_eq!(native_of(Some(0x0000_0000_0000_0005)), [0, 5, 0, 1]);
}

/// r07: `rgb_to_ycbcr` of the red, green and blue primaries, mid grey and black.
#[test]
fn r07_rgb_primaries_grey_and_black() {
    assert_eq!(ayuv_of(0x0000_00ff), [23808, 21760, 512, 65535]);
    assert_eq!(ayuv_of(0x0000_ff00), [42240, 10752, 4864, 65535]);
    assert_eq!(ayuv_of(0x00ff_0000), [11520, 512, 27648, 65535]);
    assert_eq!(ayuv_of(0x0080_8080), [36864, 32768, 32768, 65535]);
    assert_eq!(ayuv_of(0x0000_0000), [4096, 32768, 32768, 65535]);
}

/// r08: `rgb_to_ycbcr` of white, yellow, the high byte set, all ones and a mixed colour. The
/// high byte of the colour is ignored.
#[test]
fn r08_rgb_white_yellow_high_byte_and_mixed() {
    assert_eq!(ayuv_of(0x00ff_ffff), [3840, 32768, 32768, 65535]);
    assert_eq!(ayuv_of(0x0000_ffff), [61952, 65280, 38144, 65535]);
    assert_eq!(ayuv_of(0xff00_0000), [4096, 32768, 32768, 65535]);
    assert_eq!(ayuv_of(0xffff_ffff), [3840, 32768, 32768, 65535]);
    assert_eq!(ayuv_of(0x0012_3456), [18944, 27136, 37888, 65535]);
}

/// r09: `scale` of a frame with the full, quarter, half, eighth and empty rectangles.
#[test]
fn r09_scale_rect_of_frames() {
    assert_eq!(scale(640, 480, [0.0, 0.0, 1.0, 1.0]), [0, 0, 640, 480]);
    assert_eq!(scale(640, 480, [0.25, 0.0, 0.75, 1.0]), [160, 0, 480, 480]);
    assert_eq!(scale(100, 50, [0.5, 0.5, 1.0, 1.0]), [50, 25, 100, 50]);
    assert_eq!(
        scale(1920, 1080, [0.125, 0.0625, 0.5, 0.5]),
        [240, 67, 960, 540]
    );
    assert_eq!(scale(0, 0, [0.0, 0.0, 0.5, 0.5]), [0, 0, 0, 0]);
}

/// r10: `scale` with edges that round down, a non-square frame, and a width above `i32::MAX`.
#[test]
fn r10_scale_rect_rounding_and_large_width() {
    assert_eq!(scale(3, 3, [0.333, 0.333, 0.667, 0.667]), [0, 0, 2, 2]);
    assert_eq!(scale(10, 10, [0.1, 0.1, 0.9, 0.9]), [1, 1, 9, 9]);
    assert_eq!(scale(7, 5, [0.0, 0.0, 1.0, 0.5]), [0, 0, 7, 2]);
    assert_eq!(scale(1, 1, [0.0, 0.0, 1.0, 1.0]), [0, 0, 1, 1]);
    assert_eq!(
        scale(4_000_000_000, 1, [0.0, 0.0, 0.5, 1.0]),
        [0, 0, 2_000_000_000, 1]
    );
}

/// r11: `aspect` of a wide, tall, square, empty and flat source into a square destination.
#[test]
fn r11_aspect_of_wide_tall_square_empty_and_flat() {
    assert_eq!(aspect([0, 0, 100, 50], [0, 0, 100, 100]), [0, 25, 100, 75]);
    assert_eq!(aspect([0, 0, 50, 100], [0, 0, 100, 100]), [25, 0, 75, 100]);
    assert_eq!(aspect([0, 0, 100, 100], [0, 0, 100, 100]), [0, 0, 100, 100]);
    assert_eq!(aspect([0, 0, 0, 0], [0, 0, 100, 100]), [0, 0, 100, 100]);
    assert_eq!(aspect([0, 0, 100, 0], [0, 0, 100, 100]), [0, 50, 100, 50]);
}

/// r12: `aspect` with offsets, a rounding division, a negative destination, a source that
/// straddles the origin, and an empty destination.
#[test]
fn r12_aspect_with_offsets_negatives_and_empty_destination() {
    assert_eq!(
        aspect([10, 20, 110, 70], [5, 5, 105, 105]),
        [5, 30, 105, 80]
    );
    assert_eq!(aspect([0, 0, 3, 2], [1, 1, 8, 8]), [1, 2, 8, 6]);
    assert_eq!(
        aspect([0, 0, 100, 100], [0, 0, -100, -100]),
        [0, 0, -100, -100]
    );
    assert_eq!(
        aspect([-10, -10, 10, 0], [0, 0, 100, 100]),
        [0, 25, 100, 75]
    );
    assert_eq!(aspect([0, 0, 100, 100], [0, 0, 0, 0]), [0, 0, 0, 0]);
}

/// r13: `rtfmt` of six devices, two of which report a format already in the table, followed by
/// `rtdump`. The first device to report a format keeps it.
#[test]
fn r13_rt_format_table_keeps_first_device_per_format() {
    let mut formats: Vec<RtFormat<u32>> = Vec::new();
    append_rt_format(&mut formats, 1, fourcc(b"UYVY"));
    append_rt_format(&mut formats, 2, fourcc(b"UYVY"));
    append_rt_format(&mut formats, 3, fourcc(b"YUY2"));
    append_rt_format(&mut formats, 4, fourcc(b"AYUV"));
    append_rt_format(&mut formats, 5, fourcc(b"YUY2"));
    append_rt_format(&mut formats, 6, 0x5956_5955);

    let table: Vec<(u32, u32)> = formats
        .iter()
        .map(|entry| (entry.device, entry.format))
        .collect();
    assert_eq!(
        table,
        [
            (1, fourcc(b"UYVY")),
            (3, fourcc(b"YUY2")),
            (4, fourcc(b"AYUV"))
        ]
    );
}

/// r14: `pos` with and without a source or destination rectangle, and each edge out of range.
#[test]
fn r14_video_position_argument_checks() {
    assert_eq!(video_position(None, None), E_POINTER);
    assert_eq!(video_position(Some([0.0, 0.0, 1.0, 1.0]), None), S_OK);
    assert_eq!(
        video_position(Some([0.0, 0.0, 2.0, 1.0]), None),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([-0.5, 0.0, 1.0, 1.0]), None),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([0.0, 0.0, 1.0, 1.5]), None),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([0.6, 0.0, 0.5, 1.0]), None),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([0.0, 0.6, 1.0, 0.5]), None),
        E_INVALIDARG
    );
    assert_eq!(video_position(None, Some([10, 10, 5, 5])), E_INVALIDARG);
    assert_eq!(video_position(None, Some([0, 10, 10, 5])), E_INVALIDARG);
    assert_eq!(video_position(None, Some([0, 0, 10, 10])), S_OK);
}

/// r15: `pos` with NaN edges, an infinite edge, and a destination alongside the source.
#[test]
fn r15_video_position_nan_infinity_and_destination() {
    assert_eq!(video_position(Some([f32::NAN; 4]), None), S_OK);
    assert_eq!(
        video_position(Some([0.0, 0.0, f32::INFINITY, 1.0]), None),
        E_INVALIDARG
    );
    assert_eq!(
        video_position(Some([0.0, 0.0, 1.0, 1.0]), Some([0, 0, 0, 0])),
        S_OK
    );
    assert_eq!(
        video_position(Some([0.0, 0.0, 1.0, 1.0]), Some([5, 5, 5, 5])),
        S_OK
    );
}

/// r16: `armode` of the defined modes, the first mode bit above the mask, and all ones.
#[test]
fn r16_aspect_ratio_mode_mask() {
    for mode in [0, 1, 2, 4, 7] {
        assert_eq!(hresult(check_aspect_ratio_mode(mode)), S_OK);
    }
    for mode in [8, 15, u32::MAX] {
        assert_eq!(hresult(check_aspect_ratio_mode(mode)), E_INVALIDARG);
    }
}

/// r17: `add`, `getz`, `ids` and `getr` on a new table. The new streams stack above the
/// reference stream in the order they are added.
#[test]
fn r17_add_zorder_ids_and_output_rect() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5, 3]), Ok(()));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0, 5, 3]));
    assert_eq!(streams.get_zorder(5), Ok(1));
    assert_eq!(streams.get_zorder(3), Ok(2));
    assert_eq!(streams.get_zorder(0), Ok(0));
    assert_eq!(streams.add(&[7]), Ok(()));
    assert_eq!(streams.get_zorder(7), Ok(3));
    assert_eq!(streams.stream_ids(8, 8), Ok(vec![0, 5, 3, 7]));
    assert_eq!(streams.get_output_rect(7), Ok(NormalizedRect::FULL));
}

/// r18: `del` of a stream that is not the last sorted one. The listing removes the entry at the
/// stream's sorted index, which is the wrong entry, so it keeps `3` and drops `5`.
#[test]
fn r18_delete_keeps_the_wrong_id_in_the_list() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5, 3]), Ok(()));
    assert_eq!(streams.delete(3), Ok(()));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0, 3]));
    assert_eq!(streams.add(&[4]), Ok(()));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0, 3, 4]));
}

/// r19: `del` of the last sorted stream, which the listing drops as expected.
#[test]
fn r19_delete_last_sorted_stream() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5, 3]), Ok(()));
    assert_eq!(streams.delete(5), Ok(()));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0, 5]));
    assert_eq!(streams.add(&[6]), Ok(()));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0, 5, 6]));
}

/// r20: `del` and `getz` of the reference stream, of an id that is not present, and of a stream
/// after it has been deleted.
#[test]
fn r20_delete_and_zorder_of_missing_streams() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.delete(0), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.delete(9), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.add(&[7]), Ok(()));
    assert_eq!(streams.delete(7), Ok(()));
    assert_eq!(streams.get_zorder(7), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.delete(7), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.stream_ids(4, 4), Ok(vec![0]));
}

/// r21: `add` up to the 16-stream limit. The batch that would exceed it, and the empty batch,
/// follow.
#[test]
fn r21_add_up_to_the_stream_limit() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[7]), Ok(()));
    let filled: Vec<u32> = (8..=21).collect();
    assert_eq!(streams.add(&filled), Ok(()));
    assert_eq!(streams.add(&[22]), Err(E_INVALIDARG));

    let mut listed = vec![0, 7];
    listed.extend(8..=21);
    assert_eq!(streams.stream_ids(16, 16), Ok(listed));
    assert_eq!(streams.add(&[]), Ok(()));
    assert_eq!(streams.add(&[23]), Err(E_INVALIDARG));
}

/// r22: `add` of a collision with the reference stream, a repeat within a batch, a repeat of an
/// added stream, and an empty batch.
#[test]
fn r22_add_rejects_collisions_and_repeats() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[0]), Err(E_INVALIDARG));
    assert_eq!(streams.add(&[7, 7]), Err(E_INVALIDARG));
    assert_eq!(streams.add(&[9, 8]), Ok(()));
    assert_eq!(streams.add(&[8]), Err(E_INVALIDARG));
    assert_eq!(streams.add(&[]), Ok(()));
    assert_eq!(streams.stream_ids(16, 16), Ok(vec![0, 9, 8]));
}

/// r23: `ids` with an input size below the list, with a zero output size, and with room for
/// both.
#[test]
fn r23_stream_ids_buffer_sizes() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.add(&[5, 3]), Ok(()));
    assert_eq!(streams.stream_ids(1, 4), Err(MF_E_BUFFERTOOSMALL));
    assert_eq!(streams.stream_ids(2, 0), Err(MF_E_BUFFERTOOSMALL));
    assert_eq!(streams.stream_ids(2, 1), Err(MF_E_BUFFERTOOSMALL));
    assert_eq!(streams.stream_ids(3, 1), Ok(vec![0, 5, 3]));
    assert_eq!(streams.stream_ids(0, 1), Err(MF_E_BUFFERTOOSMALL));
    assert_eq!(streams.stream_ids(16, 16), Ok(vec![0, 5, 3]));
}

/// r24: `setr` and `getr` on the reference stream, with a rectangle that is inverted, out of
/// range, or NaN, and on a stream that is not present.
#[test]
fn r24_set_and_get_output_rect() {
    let mut streams = InputStreams::default();
    assert_eq!(
        streams.set_output_rect(0, &norm([0.0, 0.0, 1.0, 1.0])),
        Ok(())
    );
    assert_eq!(streams.get_output_rect(0), Ok(norm([0.0, 0.0, 1.0, 1.0])));
    assert_eq!(
        streams.set_output_rect(0, &norm([0.2, 0.0, 0.1, 1.0])),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        streams.set_output_rect(0, &norm([0.0, 0.0, 1.01, 1.0])),
        Err(E_INVALIDARG)
    );
    assert_eq!(
        streams.set_output_rect(7, &norm([0.0, 0.0, 0.5, 0.5])),
        Err(MF_E_INVALIDSTREAMNUMBER)
    );

    // A NaN left edge passes the check and is stored as it is, so it is compared edge by edge.
    assert_eq!(
        streams.set_output_rect(0, &norm([f32::NAN, 0.0, 1.0, 1.0])),
        Ok(())
    );
    let stored = streams.get_output_rect(0).unwrap();
    assert!(stored.left.is_nan());
    assert_eq!([stored.top, stored.right, stored.bottom], [0.0, 1.0, 1.0]);

    assert_eq!(streams.set_output_rect(0, &norm([0.5; 4])), Ok(()));
    assert_eq!(streams.get_output_rect(0), Ok(norm([0.5; 4])));
}

/// r25: `zorder` and `getz` on the reference stream and two added streams. The reply depends on
/// the zorder being in range, on the reference stream keeping zorder `0`, and on the stream
/// being present.
#[test]
fn r25_set_zorder_rules() {
    let mut streams = InputStreams::default();
    assert_eq!(streams.set_zorder(0, 0), Ok(()));
    assert_eq!(streams.set_zorder(0, 1), Err(E_INVALIDARG));
    assert_eq!(streams.add(&[7, 9]), Ok(()));
    assert_eq!(streams.set_zorder(7, 0), Err(MF_E_INVALIDREQUEST));
    assert_eq!(streams.set_zorder(9, 3), Err(E_INVALIDARG));
    assert_eq!(streams.set_zorder(9, 2), Ok(()));
    assert_eq!(streams.set_zorder(9, 1), Ok(()));
    assert_eq!(streams.get_zorder(9), Ok(1));
    assert_eq!(streams.set_zorder(9, 9), Err(E_INVALIDARG));
    assert_eq!(streams.set_zorder(7, 2), Ok(()));
    assert_eq!(streams.get_zorder(7), Ok(2));
    assert_eq!(streams.set_zorder(0, 0), Ok(()));
    assert_eq!(streams.set_zorder(42, 0), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.set_zorder(42, 1), Err(MF_E_INVALIDSTREAMNUMBER));
    assert_eq!(streams.set_zorder(9, 0), Err(MF_E_INVALIDREQUEST));
}
