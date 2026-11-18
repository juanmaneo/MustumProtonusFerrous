//! Replays of Wine's oracle vectors. The oracle compiles the verbatim `msvideo1.c` in
//! `target/canon/msvidc32/harness.c` with `-fsanitize=address,undefined`, and mocks only the
//! outside world (the Win32 heap, `LoadStringW` and `DisableThreadLibraryCalls`). Each test names
//! the oracle row it reproduces. The expected bytes are the `out=` fields of `oracle.txt`. Rows
//! E1, E2 and E3 abort under the sanitizers, so they have no test here.

use super::*;

pub(crate) fn header(
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

pub(crate) fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

pub(crate) fn code_of(result: IcResult<()>) -> LResult {
    match result {
        Ok(()) => ICERR_OK,
        Err(error) => error.code(),
    }
}

/// Decodes into a buffer of `stride` bytes per row, filled with 0xEE as the oracle fills it.
pub(crate) fn decode8(width: i32, height: i32, input: &[u8], stride: i32) -> Vec<u8> {
    let mut pixels = vec![0xEE_u8; (stride * height) as usize];
    decode_8bit(width, height, input, &mut pixels, stride).unwrap();
    pixels
}

/// Decodes into a buffer of `stride` words per row, filled with 0xEE as the oracle fills it.
pub(crate) fn decode16(width: i32, height: i32, input: &[u8], stride: i32) -> Vec<u8> {
    let mut pixels = vec![0xEE_u8; (stride * height * 2) as usize];
    decode_16bit(width, height, input, &mut pixels, stride).unwrap();
    pixels
}

#[test]
fn oracle_d1() {
    let input = [0xA5, 0x12, 0x20, 0xC0];
    assert_eq!(
        decode8(4, 4, &input, 4),
        hex("20c020c0c020c020c020c0c020c0c0c0")
    );
}

#[test]
fn oracle_d2() {
    let input = [0x1B, 0x9C, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
    assert_eq!(
        decode8(4, 4, &input, 4),
        hex("10104030102040406060707050608070")
    );
}

#[test]
fn oracle_d3() {
    let input = [0x3C, 0x8F];
    assert_eq!(
        decode8(4, 4, &input, 4),
        hex("3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c")
    );
}

#[test]
fn oracle_d4() {
    let input = [0x02, 0x84, 0x3C, 0x8F];
    assert_eq!(
        decode8(12, 4, &input, 12),
        hex("eeeeeeeeeeeeeeee3c3c3c3ceeeeeeeeeeeeeeee3c3c3c3ceeeeeeeeeeeeeeee3c3c3c3ceeeeeeeeeeeeeeee3c3c3c3c")
    );
}

#[test]
fn oracle_d5() {
    let input = [0x00, 0x84, 0x3C, 0x8F];
    assert_eq!(decode8(12, 4, &input, 12), vec![0xEE_u8; 48]);
}

#[test]
fn oracle_d6() {
    let input = [0x11, 0x8F, 0x22, 0x8F, 0x33, 0x8F, 0x44, 0x8F, 0x00, 0x00];
    assert_eq!(
        decode8(8, 8, &input, 12),
        hex("1111111122222222eeeeeeee1111111122222222eeeeeeee1111111122222222eeeeeeee1111111122222222eeeeeeee3333333344444444eeeeeeee3333333344444444eeeeeeee3333333344444444eeeeeeee3333333344444444eeeeeeee")
    );
}

#[test]
fn oracle_d7() {
    let input = [0x3C, 0x8F, 0x12, 0x00];
    assert_eq!(
        decode8(8, 4, &input, 8),
        hex("3c3c3c3ceeeeeeee3c3c3c3ceeeeeeee3c3c3c3ceeeeeeee3c3c3c3ceeeeeeee")
    );
}

#[test]
fn oracle_w1() {
    let input = [0xA5, 0x12, 0x1F, 0x7C, 0xE0, 0x03];
    assert_eq!(
        decode16(4, 4, &input, 4),
        hex("1f7ce0031f7ce003e0031f7ce0031f7ce0031f7ce003e0031f7ce003e003e003")
    );
}

#[test]
fn oracle_w2() {
    let input = [
        0x1B, 0x12, 0x1F, 0xFC, 0xE0, 0x03, 0xFF, 0x7F, 0x00, 0x00, 0x34, 0x12, 0x78, 0x56, 0xBC,
        0x9A, 0x01, 0x00,
    ];
    assert_eq!(
        decode16(4, 4, &input, 4),
        hex("1ffc1ffc0000ff7f1ffce0030000000078563412010001003412785601000100")
    );
}

#[test]
fn oracle_w3() {
    let input = [0x02, 0x84, 0x34, 0x9A];
    assert_eq!(
        decode16(12, 4, &input, 12),
        hex("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee349a349a349a349aeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee349a349a349a349aeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee349a349a349a349aeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee349a349a349a349a")
    );
}

#[test]
fn oracle_w4() {
    let input = [0x1B, 0x12, 0x1F, 0xFC, 0xE0, 0x03];
    assert_eq!(decode16(4, 4, &input, 4), vec![0xEE_u8; 32]);
}

#[test]
fn oracle_c1() {
    let input = [
        0xFF, 0x7F, 0x00, 0x00, 0x00, 0x7C, 0x1F, 0x00, 0xE0, 0x03, 0x10, 0x42, 0x18, 0x63, 0x21,
        0x04,
    ];
    let mut out = vec![0xEE_u8; 24];
    convert_depth(&input, 16, &mut out, &header(4, 2, 24, 0, 0)).unwrap();
    assert_eq!(out, hex("ffffff000000ff00000000ff00ff00848484c5c5c5080808"));
}

#[test]
fn oracle_c2() {
    let input = [
        0xFF, 0x7F, 0x21, 0x04, 0x18, 0x63, 0xAA, 0xAA, 0xE0, 0x03, 0x1F, 0x7C, 0x00, 0x00, 0x55,
        0x55,
    ];
    let mut out = vec![0xEE_u8; 24];
    convert_depth(&input, 16, &mut out, &header(3, 2, 24, 0, 0)).unwrap();
    assert_eq!(out, hex("ffffff080808c5c5c5eeeeee00ff00ff00ff000000eeeeee"));
}

#[test]
fn oracle_q1() {
    let ctx = Msvideo1Context::opened();
    let input = header(4, 4, 16, MSVC_MAGIC, 0);
    let output = header(4, 4, 24, 0, 0);
    assert_eq!(code_of(ctx.decompress_query(&input, Some(&output))), 0);
}

#[test]
fn oracle_q2() {
    let ctx = Msvideo1Context::opened();
    let input = header(4, 4, 8, CRAM_MAGIC, 0);
    let output = header(4, 4, 24, 0, 0);
    assert_eq!(
        code_of(ctx.decompress_query(&input, Some(&output))),
        ICERR_BADFORMAT
    );
}

#[test]
fn oracle_g1() {
    let ctx = Msvideo1Context::opened();
    let input = BitmapInfoHeader {
        clr_used: 256,
        ..header(4, 4, 8, CRAM_MAGIC, 0)
    };
    assert_eq!(
        ctx.decompress_get_format(&input, false),
        Ok(FormatQuery::Size(1064))
    );
}

#[test]
fn oracle_g2() {
    let ctx = Msvideo1Context::opened();
    let input = header(5, 3, 16, CRAM_MAGIC, 0);
    assert_eq!(
        ctx.decompress_get_format(&input, true),
        Ok(FormatQuery::Output(header(4, 2, 16, BI_RGB, 24)))
    );
}

#[test]
fn oracle_g3() {
    let ctx = Msvideo1Context::opened();
    let input = header(5, -3, 16, CRAM_MAGIC, 0);
    assert_eq!(
        ctx.decompress_get_format(&input, true),
        Ok(FormatQuery::Output(header(4, -4, 16, BI_RGB, 4294967272)))
    );
}

#[test]
fn oracle_b1() {
    let mut ctx = Msvideo1Context::opened();
    let input = header(4, 4, 16, CRAM_MAGIC, 0);
    assert_eq!(ctx.decompress_begin(&input), Ok(()));
    assert_eq!(ctx.depth, 16);
}

#[test]
fn oracle_i1() {
    assert_eq!(
        get_info(None, Some(ICINFO_SIZE)),
        InfoResult::Filled(IcInfo {
            dw_size: 568,
            fcc_type: 0x6364_6976,
            fcc_handler: 0x4d41_5243,
            flags: 0,
            version: 0x104,
            version_icm: 0x104,
        })
    );
}

#[test]
fn oracle_p1() {
    assert_eq!(driver_proc_constant_result(DRV_LOAD), Some(1));
}

#[test]
fn oracle_p2() {
    assert_eq!(
        driver_proc_constant_result(ICM_COMPRESS_QUERY),
        Some(ICERR_BADFORMAT)
    );
}

#[test]
fn oracle_p3() {
    assert_eq!(
        driver_proc_constant_result(DRV_CLOSE),
        Some(ICERR_UNSUPPORTED)
    );
}

#[test]
fn oracle_x1() {
    let mut ctx = Msvideo1Context::opened();
    let input = header(8, 4, 16, CRAM_MAGIC, 8);
    ctx.decompress_begin(&input).unwrap();
    let stream = [0x34, 0x9A, 0xA5, 0x12, 0x1F, 0x7C, 0xE0, 0x03];
    let mut out = vec![0xEE_u8; 64];
    let result = ctx.decompress(&input, &stream, &header(8, 4, 16, 0, 0), &mut out);
    assert_eq!(result, Ok(()));
    assert_eq!(
        out,
        hex("349a349a349a349a1f7ce0031f7ce003349a349a349a349ae0031f7ce0031f7c349a349a349a349ae0031f7ce003e003349a349a349a349a1f7ce003e003e003")
    );
}

#[test]
fn oracle_x2() {
    let mut ctx = Msvideo1Context::opened();
    let input = header(4, 4, 16, CRAM_MAGIC, 6);
    ctx.decompress_begin(&input).unwrap();
    let stream = [0xA5, 0x12, 0x1F, 0x7C, 0xE0, 0x03];
    let mut out = vec![0xEE_u8; 48];
    let result = ctx.decompress(&input, &stream, &header(4, 4, 24, 0, 0), &mut out);
    assert_eq!(result, Ok(()));
    assert_eq!(
        out,
        hex("ff00ff00ff00ff00ff00ff0000ff00ff00ff00ff00ff00ff00ff00ff00ff00ff0000ff00ff00ff00ff0000ff0000ff00")
    );
}

#[test]
fn oracle_k_constants() {
    assert_eq!(
        [
            ICERR_OK,
            ICERR_UNSUPPORTED,
            ICERR_BADFORMAT,
            ICERR_MEMORY,
            ICERR_BADPARAM,
            ICERR_BADSIZE
        ],
        [0, -1, -2, -3, -6, -7]
    );
    assert_eq!(
        [
            DRV_LOAD,
            DRV_ENABLE,
            DRV_OPEN,
            DRV_CLOSE,
            DRV_DISABLE,
            DRV_FREE
        ],
        [1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        [
            ICM_CONFIGURE,
            ICM_GETINFO,
            ICM_DECOMPRESS_GET_PALETTE,
            ICM_DECOMPRESS_QUERY,
            ICM_DECOMPRESS_GET_FORMAT,
            ICM_DECOMPRESS,
            ICM_DECOMPRESS_BEGIN,
            ICM_DECOMPRESSEX_QUERY,
            ICM_DECOMPRESSEX,
            ICM_DECOMPRESS_END,
            ICM_DECOMPRESSEX_BEGIN,
            ICM_COMPRESS_QUERY,
            ICM_COMPRESS_GET_FORMAT,
            ICM_COMPRESS_END,
            ICM_COMPRESS,
        ],
        [
            20490, 20482, 16414, 16395, 16394, 16397, 16396, 16445, 16446, 16398, 16444, 16390,
            16388, 16393, 16392,
        ]
    );
    assert_eq!([ICTYPE_VIDEO, ICVERSION], [0x6364_6976, 0x104]);
    assert_eq!(
        [CRAM_MAGIC, MSVC_MAGIC, WHAM_MAGIC],
        [0x4d41_5243, 0x4356_534d, 0x4d41_4857]
    );
    assert_eq!(
        [BI_RGB, ICINFO_SIZE, BITMAPINFOHEADER_SIZE, RGBQUAD_SIZE],
        [0, 568, 40, 4]
    );
}
