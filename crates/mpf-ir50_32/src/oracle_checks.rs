//! Replays of Wine's oracle vectors. The oracle compiles the verbatim `ir50.c` in
//! `target/canon/ir50_32/harness.c` with `-fsanitize=address,undefined`, and mocks only the
//! outside world (Media Foundation calls, `LoadStringW`, the debug channel). Each test names the
//! oracle row it reproduces. Row X1 (`abs(INT_MIN)` in C) is excluded because UBSan aborts it.

use super::*;

const S_OK: i32 = 0;
const E_FAIL: i32 = -2_147_467_259;

fn header(
    width: i32,
    height: i32,
    bit_count: u16,
    compression: u32,
    size_image: u32,
) -> BitmapInfoHeader {
    BitmapInfoHeader {
        size: BITMAPINFOHEADER_SIZE,
        width,
        height,
        planes: 1,
        bit_count,
        compression,
        size_image,
        ..Default::default()
    }
}

fn bitmap_info(header: BitmapInfoHeader) -> BitmapInfo {
    BitmapInfo {
        header,
        masks: [0; 3],
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn code_of(result: IcResult<()>) -> LResult {
    match result {
        Ok(()) => ICERR_OK,
        Err(error) => error.code(),
    }
}

/// Runs one decode the way `IV50_Decompress` does, with the transform answering `first` and then
/// `second`. The input is 3x2 IV50 and the output is 3 wide and `out_height` high, with a 24-byte
/// sentinel buffer. The decoder writes `7 * i + 3` into its output buffer on success.
fn run_frame(first: i32, second: i32, out_height: i32) -> (LResult, Vec<u8>) {
    let out = header(3, out_height, 24, BI_RGB, 24);
    let params = DecompressParams::from_headers(header(3, 2, 12, IV50, 24), Some(bitmap_info(out)));
    let mut decoded = vec![0u8; 24];
    let mut replies = [first, second].into_iter();
    let action = process_output(|| {
        let hr = replies.next().unwrap();
        if hr >= 0 {
            for (i, byte) in decoded.iter_mut().enumerate() {
                *byte = (7 * i + 3) as u8;
            }
        }
        hr
    });
    let mut dst = vec![0xEE; 24];
    let code = match action {
        OutputAction::Copy => code_of(copy_frame(&out, params.dst_rect, &decoded, &mut dst)),
        OutputAction::Finish(result) => code_of(result),
    };
    (code, dst)
}

#[test]
fn oracle_constants_match_wine_abi() {
    assert_eq!(
        (
            DRV_LOAD,
            DRV_ENABLE,
            DRV_OPEN,
            DRV_CLOSE,
            DRV_DISABLE,
            DRV_FREE
        ),
        (1, 2, 3, 4, 5, 6)
    );
    assert_eq!((ICM_USER, ICM_RESERVED, ICM_GETINFO), (16384, 20480, 20482));
    assert_eq!(
        (
            ICM_COMPRESS_GET_FORMAT,
            ICM_COMPRESS_QUERY,
            ICM_COMPRESS,
            ICM_COMPRESS_END
        ),
        (16388, 16390, 16392, 16393)
    );
    assert_eq!(
        (
            ICM_DECOMPRESS_GET_FORMAT,
            ICM_DECOMPRESS_QUERY,
            ICM_DECOMPRESS_BEGIN,
            ICM_DECOMPRESS,
            ICM_DECOMPRESS_END,
            ICM_DECOMPRESS_GET_PALETTE
        ),
        (16394, 16395, 16396, 16397, 16398, 16414)
    );
    assert_eq!(
        (
            ICM_DECOMPRESSEX_BEGIN,
            ICM_DECOMPRESSEX_QUERY,
            ICM_DECOMPRESSEX,
            ICM_DECOMPRESSEX_END,
            ICM_CONFIGURE
        ),
        (16444, 16445, 16446, 16447, 20490)
    );
    assert_eq!(
        (ICTYPE_VIDEO, ICVERSION, ICINFO_SIZE, BITMAPINFOHEADER_SIZE),
        (0x6364_6976, 260, 568, 40)
    );
    assert_eq!(BITMAPINFO_PALETTE_SIZE, 1064);
    assert_eq!(
        (
            ICERR_OK,
            ICERR_UNSUPPORTED,
            ICERR_BADFORMAT,
            ICERR_INTERNAL,
            ICERR_BADPARAM,
            ICERR_BADSIZE
        ),
        (0, -1, -2, -4, -6, -7)
    );
    assert_eq!(
        (MF_E_TRANSFORM_STREAM_CHANGE, MF_E_TRANSFORM_NEED_MORE_INPUT),
        (-1_072_861_855, -1_072_861_838)
    );
    assert_eq!((BI_RGB, BI_BITFIELDS), (0, 3));
}

#[test]
fn oracle_a1_fourcc_ignores_case_bit() {
    assert_eq!(compare_fourcc(IV50, u32::from_le_bytes(*b"iv50")), 0);
}

#[test]
fn oracle_a2_fourcc_differs_in_last_digit() {
    assert_eq!(
        compare_fourcc(IV50, u32::from_le_bytes(*b"IV51")),
        16_777_216
    );
}

#[test]
fn oracle_b1_make_uint64_negative_low() {
    assert_eq!(make_uint64(320, (-480i32) as u32), 0x0000_0140_FFFF_FE20);
}

#[test]
fn oracle_c1_getinfo_null_is_size_query() {
    assert_eq!(get_info(false, 0), InfoResult::SizeQuery);
}

#[test]
fn oracle_c3_getinfo_fills_icinfo() {
    assert_eq!(
        get_info(true, 568),
        InfoResult::Filled(IcInfo {
            dw_size: 568,
            fcc_type: 0x6364_6976,
            fcc_handler: 0x3035_5649,
            flags: 0,
            version: 260,
            version_icm: 260,
        })
    );
}

#[test]
fn oracle_d1_open_accepts_vidc() {
    let vidc = IcInfo {
        fcc_type: u32::from_le_bytes(*b"vidc"),
        ..Default::default()
    };
    assert!(open_accepts(Some(&vidc)));
}

#[test]
fn oracle_d2_open_rejects_audc() {
    let audc = IcInfo {
        fcc_type: u32::from_le_bytes(*b"audc"),
        ..Default::default()
    };
    assert!(!open_accepts(Some(&audc)));
}

#[test]
fn oracle_e1_getformat_size_query() {
    assert_eq!(
        decompress_get_format(&header(320, -240, 12, IV50, 0), false),
        Ok(FormatQuery::PaletteSize(1064))
    );
}

#[test]
fn oracle_e2_getformat_output_header() {
    assert_eq!(
        decompress_get_format(&header(320, -240, 12, IV50, 0), true),
        Ok(FormatQuery::Output(header(320, 240, 24, BI_RGB, 230_400)))
    );
}

#[test]
fn oracle_e3_getformat_rejects_yuy2() {
    let yuy2 = header(320, 240, 12, u32::from_le_bytes(*b"YUY2"), 0);
    assert_eq!(code_of(decompress_get_format(&yuy2, false).map(|_| ())), -2);
}

#[test]
fn oracle_f3_query_rgb8_output_rejected() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(bitmap_info(header(320, 240, 8, BI_RGB, 0))),
    );
    assert_eq!(code_of(decompress_query(&params)), -2);
}

#[test]
fn oracle_f4_query_bitfields_565_accepted() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(BitmapInfo {
            header: header(320, 240, 16, BI_BITFIELDS, 0),
            masks: [0xf800, 0x07e0, 0x001f],
        }),
    );
    assert_eq!(code_of(decompress_query(&params)), 0);
}

#[test]
fn oracle_f5_query_bitfields_bad_mask_rejected() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(BitmapInfo {
            header: header(320, 240, 16, BI_BITFIELDS, 0),
            masks: [0xf800, 0x07e0, 0x001e],
        }),
    );
    assert_eq!(code_of(decompress_query(&params)), -2);
}

#[test]
fn oracle_f7_query_nontrivial_source_rect_rejected() {
    let mut params = DecompressParams::from_headers(header(320, 240, 12, IV50, 0), None);
    params.src_rect.x = 1;
    assert_eq!(code_of(decompress_query(&params)), -2);
}

#[test]
fn oracle_f8_query_dimension_mismatch_is_badparam() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(bitmap_info(header(320, 120, 24, BI_RGB, 0))),
    );
    assert_eq!(code_of(decompress_query(&params)), -6);
}

#[test]
fn oracle_g2_begin_rgb24() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(bitmap_info(header(320, 240, 24, BI_RGB, 0))),
    );
    assert_eq!(
        decompress_begin_plan(true, &params),
        Ok(BeginPlan {
            output_subtype: OutputSubtype::Rgb24,
            input_frame_size: 0x0000_0140_0000_00f0,
            output_frame_size: 0x0000_0140_0000_00f0,
            default_stride: 4_294_966_976,
        })
    );
}

#[test]
fn oracle_g3_begin_bitfields_negative_height() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(bitmap_info(header(321, -240, 16, BI_BITFIELDS, 0))),
    );
    assert_eq!(
        decompress_begin_plan(true, &params),
        Ok(BeginPlan {
            output_subtype: OutputSubtype::Rgb565,
            input_frame_size: 0x0000_0140_0000_00f0,
            output_frame_size: 0x0000_0141_0000_00f0,
            default_stride: 324,
        })
    );
}

#[test]
fn oracle_g4_begin_rgb8_rejected() {
    let params = DecompressParams::from_headers(
        header(320, 240, 12, IV50, 0),
        Some(bitmap_info(header(320, 240, 8, BI_RGB, 0))),
    );
    assert_eq!(
        decompress_begin_plan(true, &params),
        Err(IcError::BadFormat)
    );
}

#[test]
fn oracle_h1_frame_full_copy() {
    let (code, dst) = run_frame(S_OK, S_OK, 2);
    assert_eq!(code, 0);
    assert_eq!(
        hex(&dst),
        "030a11181f262d343beeeeee575e656c737a81888feeeeee"
    );
}

#[test]
fn oracle_h3_frame_stream_change_retry() {
    let (code, dst) = run_frame(MF_E_TRANSFORM_STREAM_CHANGE, S_OK, 2);
    assert_eq!(code, 0);
    assert_eq!(
        hex(&dst),
        "030a11181f262d343beeeeee575e656c737a81888feeeeee"
    );
}

#[test]
fn oracle_h4_frame_need_more_input() {
    let (code, dst) = run_frame(MF_E_TRANSFORM_NEED_MORE_INPUT, S_OK, 2);
    assert_eq!(code, 0);
    assert_eq!(hex(&dst), "ee".repeat(24));
}

#[test]
fn oracle_h5_frame_process_fail() {
    let (code, dst) = run_frame(E_FAIL, S_OK, 2);
    assert_eq!(code, -4);
    assert_eq!(hex(&dst), "ee".repeat(24));
}

#[test]
fn oracle_h7_icdecompress_negative_height() {
    let (code, dst) = run_frame(S_OK, S_OK, -2);
    assert_eq!(code, 0);
    assert_eq!(
        hex(&dst),
        "030a11181f262d343beeeeee575e656c737a81888feeeeee"
    );
}

#[test]
fn oracle_i5_compress_query_badformat() {
    assert_eq!(driver_proc_constant_result(ICM_COMPRESS_QUERY), Some(-2));
}

#[test]
fn oracle_i7_unknown_message_unsupported() {
    assert_eq!(driver_proc_constant_result(0x7777), Some(-1));
}
