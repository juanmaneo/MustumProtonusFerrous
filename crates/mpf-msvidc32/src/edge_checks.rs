//! Edge checks for the cases the oracle rows do not reach: strides, bounds, truncated streams,
//! skip runs, empty frames, the depth conversion limits and the driver's constant replies. The
//! values that the C code defines were also run through the verbatim C in
//! `target/canon/msvidc32`. The bounds that this crate adds are the deviations in the crate docs.

use super::*;
use crate::oracle_checks::{code_of, decode16, decode8, header, hex};

fn info(fcc_type: u32) -> IcInfo {
    IcInfo {
        dw_size: ICINFO_SIZE,
        fcc_type,
        fcc_handler: 0,
        flags: 0,
        version: ICVERSION,
        version_icm: ICVERSION,
    }
}

#[test]
fn stride_is_padded_to_a_multiple_of_four() {
    assert_eq!(get_stride(4, 8), 4);
    assert_eq!(get_stride(5, 8), 8);
    assert_eq!(get_stride(4, 16), 8);
    assert_eq!(get_stride(5, 16), 12);
    assert_eq!(get_stride(3, 24), 12);
    assert_eq!(get_stride(0, 8), 0);
    assert_eq!(get_stride(-4, 8), -4);
}

// Rust-only: Wine writes past a short pixel buffer instead of returning an error.
#[test]
fn decode_8bit_rejects_a_pixel_buffer_short_of_the_last_row() {
    let input = [0x3C, 0x8F];
    let mut short = vec![0xEE_u8; 15];
    assert_eq!(
        decode_8bit(4, 4, &input, &mut short, 4),
        Err(IcError::BadSize)
    );
    assert_eq!(short, vec![0xEE_u8; 15]);
}

// Rust-only: a negative width gives a negative stride, and the C does not check for it.
#[test]
fn decode_8bit_rejects_a_negative_stride() {
    let mut pixels = vec![0xEE_u8; 16];
    assert_eq!(
        decode_8bit(4, 4, &[0x3C, 0x8F], &mut pixels, -4),
        Err(IcError::BadSize)
    );
}

// Rust-only: Wine writes these bytes without checking the length of the buffer.
#[test]
fn decode_8bit_needs_the_unpadded_last_row_only() {
    let input = [0x3C, 0x8F];
    let mut short = vec![0xEE_u8; 91];
    assert_eq!(
        decode_8bit(8, 8, &input, &mut short, 12),
        Err(IcError::BadSize)
    );
    let mut exact = vec![0xEE_u8; 92];
    assert_eq!(decode_8bit(8, 8, &input, &mut exact, 12), Ok(()));
    assert_eq!(exact[0], 0x3C);
    assert_eq!(exact[91], 0xEE);
}

#[test]
fn frames_with_no_whole_block_decode_to_nothing() {
    let mut pixels = vec![0xEE_u8; 4];
    assert_eq!(decode_8bit(3, 4, &[0x3C, 0x8F], &mut pixels, -4), Ok(()));
    assert_eq!(decode_8bit(8, 0, &[0x3C, 0x8F], &mut pixels, 8), Ok(()));
    assert_eq!(decode_16bit(0, 8, &[0x3C, 0x8F], &mut pixels, 4), Ok(()));
    assert_eq!(pixels, vec![0xEE_u8; 4]);
}

// Rust-only: Wine does not check the dimensions against the buffer.
#[test]
fn decode_rejects_dimensions_too_large_for_the_buffer() {
    let mut pixels = vec![0xEE_u8; 16];
    assert_eq!(
        decode_8bit(i32::MAX, i32::MAX, &[], &mut pixels, 4),
        Err(IcError::BadSize)
    );
    assert_eq!(
        decode_16bit(i32::MAX, i32::MAX, &[], &mut pixels, 4),
        Err(IcError::BadSize)
    );
}

// Rust-only: Wine writes past a short pixel buffer instead of returning an error.
#[test]
fn decode_16bit_needs_a_word_for_every_pixel() {
    let input = [0x3C, 0x8F];
    let mut short = vec![0xEE_u8; 31];
    assert_eq!(
        decode_16bit(4, 4, &input, &mut short, 4),
        Err(IcError::BadSize)
    );
    let mut exact = vec![0xEE_u8; 33];
    assert_eq!(decode_16bit(4, 4, &input, &mut exact, 4), Ok(()));
    assert_eq!(exact[..32].to_vec(), hex(&"3c8f".repeat(16)));
    assert_eq!(exact[32], 0xEE);
}

#[test]
fn truncated_8bit_streams_leave_the_frame_untouched() {
    let cases: [&[u8]; 3] = [&[0x3C], &[0xA5, 0x12, 0x20], &[0x00, 0x90, 1, 2, 3]];
    for input in cases {
        assert_eq!(decode8(4, 4, input, 4), vec![0xEE_u8; 16]);
    }
}

#[test]
fn truncated_8bit_stream_keeps_the_blocks_it_decoded() {
    assert_eq!(
        decode8(8, 4, &[0x3C, 0x8F, 0x12], 8),
        hex(&"3c3c3c3ceeeeeeee".repeat(4))
    );
}

#[test]
fn skip_codes_leave_their_blocks_untouched() {
    let mut expected = vec![0xEE_u8; 32];
    expected.extend(hex(&"3c3c3c3ceeeeeeee".repeat(4)));
    assert_eq!(decode8(8, 8, &[0x02, 0x84, 0x3C, 0x8F], 8), expected);
    assert_eq!(
        decode8(8, 4, &[0x03, 0x84, 0x3C, 0x8F], 8),
        vec![0xEE_u8; 32]
    );
}

#[test]
fn truncated_16bit_streams_leave_the_frame_untouched() {
    assert_eq!(decode16(4, 4, &[0x1B, 0x12, 0x1F], 4), vec![0xEE_u8; 32]);
    assert_eq!(decode16(4, 4, &[0x55, 0x80], 4), hex(&"5580".repeat(16)));
    assert_eq!(
        decode16(4, 4, &[0x01, 0x00, 0x00, 0x80, 0x12, 0x34], 4),
        vec![0xEE_u8; 32]
    );
}

#[test]
fn convert_does_nothing_outside_16_to_24_bits_or_without_pixels() {
    let input = [0x00, 0x7C];
    let mut out = vec![0xEE_u8; 3];
    assert_eq!(
        convert_depth(&input, 8, &mut out, &header(1, 1, 24, 0, 0)),
        Ok(())
    );
    assert_eq!(
        convert_depth(&input, 16, &mut out, &header(1, 1, 8, 0, 0)),
        Ok(())
    );
    assert_eq!(
        convert_depth(&input, 16, &mut out, &header(0, 1, 24, 0, 0)),
        Ok(())
    );
    assert_eq!(
        convert_depth(&input, 16, &mut out, &header(1, 0, 24, 0, 0)),
        Ok(())
    );
    assert_eq!(
        convert_depth(&input, 16, &mut out, &header(1, -2, 24, 0, 0)),
        Ok(())
    );
    assert_eq!(out, vec![0xEE_u8; 3]);
}

#[test]
fn convert_expands_each_channel_from_five_bits() {
    let mut red = vec![0xEE_u8; 3];
    convert_depth(&[0x00, 0x7C], 16, &mut red, &header(1, 1, 24, 0, 0)).unwrap();
    assert_eq!(red, hex("ff0000"));

    let mut out = vec![0xEE_u8; 6];
    let pixels = [0xE0, 0x03, 0x1F, 0x00];
    convert_depth(&pixels, 16, &mut out, &header(2, 1, 24, 0, 0)).unwrap();
    assert_eq!(out, hex("00ff000000ff"));
}

// Rust-only: Wine's convert writes without checking either buffer.
#[test]
fn convert_checks_that_both_frames_hold_their_rows() {
    let mut out = vec![0xEE_u8; 24];
    assert_eq!(
        convert_depth(&[0_u8; 15], 16, &mut out, &header(4, 2, 24, 0, 0)),
        Err(IcError::BadSize)
    );
    let mut short = vec![0xEE_u8; 23];
    assert_eq!(
        convert_depth(&[0_u8; 16], 16, &mut short, &header(4, 2, 24, 0, 0)),
        Err(IcError::BadSize)
    );
    assert_eq!(short, vec![0xEE_u8; 23]);
    assert_eq!(
        convert_depth(&[0_u8; 16], 16, &mut out, &header(0x0800_0001, 2, 24, 0, 0)),
        Err(IcError::BadSize)
    );
}

#[test]
fn query_checks_the_signature_and_the_format() {
    let cram = Msvideo1Context::opened();
    let eight = header(4, 4, 8, CRAM_MAGIC, 0);
    let sixteen = header(4, 4, 16, CRAM_MAGIC, 0);
    let rgb24 = header(4, 4, 24, 0, 0);
    let bad = Msvideo1Context { magic: 0, depth: 0 };
    assert_eq!(code_of(bad.decompress_query(&eight, None)), ICERR_BADPARAM);
    assert_eq!(
        code_of(cram.decompress_query(&header(4, 4, 8, 0x1234, 0), None)),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&header(4, 4, 24, CRAM_MAGIC, 0), None)),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&eight, Some(&eight))),
        ICERR_OK
    );
    assert_eq!(code_of(cram.decompress_query(&eight, None)), ICERR_OK);
    assert_eq!(
        code_of(cram.decompress_query(&eight, Some(&rgb24))),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&sixteen, Some(&eight))),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&sixteen, Some(&rgb24))),
        ICERR_OK
    );
    let other_planes = BitmapInfoHeader { planes: 2, ..rgb24 };
    assert_eq!(
        code_of(cram.decompress_query(&sixteen, Some(&other_planes))),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&sixteen, Some(&header(5, 4, 24, 0, 0)))),
        ICERR_BADFORMAT
    );
    assert_eq!(
        code_of(cram.decompress_query(&sixteen, Some(&header(4, 2, 24, 0, 0)))),
        ICERR_BADFORMAT
    );
}

#[test]
fn get_format_reports_the_size_or_the_output_header() {
    let cram = Msvideo1Context::opened();
    let bad = Msvideo1Context { magic: 0, depth: 0 };
    let eight = header(4, 4, 8, CRAM_MAGIC, 0);
    assert_eq!(
        bad.decompress_get_format(&eight, false),
        Err(IcError::BadParam)
    );
    assert_eq!(
        cram.decompress_get_format(&header(4, 4, 24, CRAM_MAGIC, 0), false),
        Err(IcError::BadFormat)
    );
    let two_colours = BitmapInfoHeader {
        clr_used: 2,
        ..eight
    };
    assert_eq!(
        cram.decompress_get_format(&two_colours, false),
        Ok(FormatQuery::Size(48))
    );
    let many_colours = BitmapInfoHeader {
        clr_used: u32::MAX,
        ..eight
    };
    assert_eq!(
        cram.decompress_get_format(&many_colours, false),
        Ok(FormatQuery::Size(36))
    );
    assert_eq!(
        cram.decompress_get_format(&header(5, 5, 8, CRAM_MAGIC, 0), true),
        Ok(FormatQuery::Output(header(4, 4, 8, BI_RGB, 20)))
    );
    // Oracle E3: Wine overflows the signed product of biHeight and the stride here. The port
    // wraps, and this test pins the wrapped value (0).
    let tall = header(4, 0x4000_0000, 16, CRAM_MAGIC, 0);
    assert_eq!(
        cram.decompress_get_format(&tall, true),
        Ok(FormatQuery::Output(header(4, 0x4000_0000, 16, BI_RGB, 0)))
    );
}

#[test]
fn begin_chooses_the_depth_or_clears_it() {
    let mut cram = Msvideo1Context::opened();
    assert_eq!(
        cram.decompress_begin(&header(4, 4, 8, CRAM_MAGIC, 0)),
        Ok(())
    );
    assert_eq!(cram.depth, 8);
    assert_eq!(
        cram.decompress_begin(&header(4, 4, 24, CRAM_MAGIC, 0)),
        Ok(())
    );
    assert_eq!(cram.depth, 0);
    let mut bad = Msvideo1Context {
        magic: 0,
        depth: 16,
    };
    assert_eq!(
        bad.decompress_begin(&header(4, 4, 8, CRAM_MAGIC, 0)),
        Err(IcError::BadParam)
    );
    assert_eq!(bad.depth, 16);
}

#[test]
fn decompress_checks_the_signature_and_the_depth_first() {
    let input = header(4, 4, 16, CRAM_MAGIC, 2);
    let output = header(4, 4, 16, 0, 0);
    let mut out = vec![0xEE_u8; 32];
    let bad = Msvideo1Context {
        magic: 0,
        depth: 16,
    };
    assert_eq!(
        bad.decompress(&input, &[0x3C, 0x8F], &output, &mut out),
        Err(IcError::BadParam)
    );
    let unset = Msvideo1Context::opened();
    assert_eq!(
        unset.decompress(&input, &[0x3C, 0x8F], &output, &mut out),
        Err(IcError::BadFormat)
    );
    assert_eq!(out, vec![0xEE_u8; 32]);
}

// Rust-only: Wine would decode into the 15-byte buffer and write past its end.
#[test]
fn decompress_checks_the_output_before_decoding() {
    let eight = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 8,
    };
    let input = header(4, 4, 8, CRAM_MAGIC, 2);
    let mut short = vec![0xEE_u8; 15];
    assert_eq!(
        eight.decompress(&input, &[0x3C, 0x8F], &header(4, 4, 8, 0, 0), &mut short),
        Err(IcError::BadSize)
    );
    assert_eq!(short, vec![0xEE_u8; 15]);
}

// Rust-only: the C passes a huge size to HeapAlloc here, and ASan aborts on it, so the
// cross-check row DX16neg was dropped. Without ASan, HeapAlloc returns NULL and the C returns
// ICERR_MEMORY, which is the reply that this test expects (inferred, not run).
#[test]
fn decompress_refuses_a_negative_scratch_size() {
    let sixteen = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 16,
    };
    let input = header(4, 4, 16, CRAM_MAGIC, 2);
    let mut out = vec![0xEE_u8; 48];
    assert_eq!(
        sixteen.decompress(&input, &[0x3C, 0x8F], &header(-4, 4, 24, 0, 0), &mut out),
        Err(IcError::Memory)
    );
}

#[test]
fn decompress_limits_the_stream_to_the_input_size_image() {
    let eight = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 8,
    };
    let input = header(8, 4, 8, CRAM_MAGIC, 2);
    let mut out = vec![0xEE_u8; 32];
    assert_eq!(
        eight.decompress(
            &input,
            &[0x3C, 0x8F, 0x3C, 0x8F],
            &header(8, 4, 8, 0, 0),
            &mut out
        ),
        Ok(())
    );
    assert_eq!(out, hex(&"3c3c3c3ceeeeeeee".repeat(4)));

    let none = header(8, 4, 8, CRAM_MAGIC, u32::MAX);
    let mut untouched = vec![0xEE_u8; 32];
    assert_eq!(
        eight.decompress(&none, &[0x3C, 0x8F], &header(8, 4, 8, 0, 0), &mut untouched),
        Ok(())
    );
    assert_eq!(untouched, vec![0xEE_u8; 32]);
}

// Rust-only: Wine writes past the output or the scratch buffer in these cases (oracle E2 is
// the 5x4 one, excluded).
#[test]
fn decompress_16bit_writes_the_decoded_pixels() {
    let sixteen = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 16,
    };
    let stream = [0xA5, 0x12, 0x1F, 0x7C, 0xE0, 0x03];
    let mut out = vec![0xEE_u8; 32];
    assert_eq!(
        sixteen.decompress(
            &header(4, 4, 16, CRAM_MAGIC, 6),
            &stream,
            &header(4, 4, 16, 0, 0),
            &mut out
        ),
        Ok(())
    );
    assert_eq!(
        out,
        hex("1f7ce0031f7ce003e0031f7ce0031f7ce0031f7ce003e0031f7ce003e003e003")
    );
}

#[test]
fn decompress_checks_the_scratch_and_converted_frames() {
    let sixteen = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 16,
    };
    let stream = [0xA5, 0x12, 0x1F, 0x7C, 0xE0, 0x03];
    let mut big = vec![0xEE_u8; 64];
    assert_eq!(
        sixteen.decompress(
            &header(5, 4, 16, CRAM_MAGIC, 6),
            &stream,
            &header(5, 4, 24, 0, 0),
            &mut big
        ),
        Err(IcError::BadSize)
    );
    assert_eq!(big, vec![0xEE_u8; 64]);

    let input = header(4, 4, 16, CRAM_MAGIC, 6);
    let mut short = vec![0xEE_u8; 47];
    assert_eq!(
        sixteen.decompress(&input, &stream, &header(4, 4, 24, 0, 0), &mut short),
        Err(IcError::BadSize)
    );
    assert_eq!(short, vec![0xEE_u8; 47]);
}

#[test]
fn decompress_without_a_depth_change_skips_the_conversion() {
    let eight = Msvideo1Context {
        magic: CRAM_MAGIC,
        depth: 8,
    };
    let mut out = vec![0xEE_u8; 48];
    assert_eq!(
        eight.decompress(
            &header(4, 4, 8, CRAM_MAGIC, 2),
            &[0x3C, 0x8F],
            &header(4, 4, 24, 0, 0),
            &mut out
        ),
        Ok(())
    );
    assert_eq!(out, vec![0xEE_u8; 48]);
}

#[test]
fn opened_context_carries_the_cram_signature() {
    assert_eq!(
        Msvideo1Context::opened(),
        Msvideo1Context {
            magic: CRAM_MAGIC,
            depth: 0
        }
    );
}

#[test]
fn open_accepts_only_video_codecs() {
    assert!(open_accepts(None));
    assert!(open_accepts(Some(&info(ICTYPE_VIDEO))));
    assert!(open_accepts(Some(&info(u32::from_le_bytes(*b"VIDC")))));
    assert!(!open_accepts(Some(&info(u32::from_le_bytes(*b"msvc")))));
}

#[test]
fn get_info_sizes_then_fills_the_buffer() {
    let wham = Msvideo1Context {
        magic: WHAM_MAGIC,
        depth: 0,
    };
    assert_eq!(get_info(None, None), InfoResult::SizeQuery);
    assert_eq!(get_info(None, Some(ICINFO_SIZE - 1)), InfoResult::TooSmall);
    assert_eq!(
        get_info(Some(&wham), Some(ICINFO_SIZE)),
        InfoResult::Filled(IcInfo {
            fcc_handler: WHAM_MAGIC,
            ..info(ICTYPE_VIDEO)
        })
    );
    assert_eq!(
        get_info(None, Some(600)),
        InfoResult::Filled(IcInfo {
            fcc_handler: CRAM_MAGIC,
            ..info(ICTYPE_VIDEO)
        })
    );
}

#[test]
fn driver_proc_constant_replies_match_the_dispatch() {
    for msg in [
        DRV_ENABLE,
        DRV_CLOSE,
        DRV_DISABLE,
        DRV_FREE,
        ICM_CONFIGURE,
        ICM_DECOMPRESS_GET_PALETTE,
        ICM_DECOMPRESSEX_QUERY,
        ICM_DECOMPRESSEX_BEGIN,
        ICM_COMPRESS_GET_FORMAT,
        ICM_COMPRESS_END,
        ICM_COMPRESS,
        0x7FFF,
    ] {
        assert_eq!(driver_proc_constant_result(msg), Some(ICERR_UNSUPPORTED));
    }
    assert_eq!(
        driver_proc_constant_result(ICM_DECOMPRESS_END),
        Some(ICERR_OK)
    );
    assert_eq!(driver_proc_constant_result(DRV_LOAD), Some(1));
    assert_eq!(
        driver_proc_constant_result(ICM_COMPRESS_QUERY),
        Some(ICERR_BADFORMAT)
    );
    for msg in [
        DRV_OPEN,
        ICM_GETINFO,
        ICM_DECOMPRESS_QUERY,
        ICM_DECOMPRESS_GET_FORMAT,
        ICM_DECOMPRESS_BEGIN,
        ICM_DECOMPRESS,
        ICM_DECOMPRESSEX,
    ] {
        assert_eq!(driver_proc_constant_result(msg), None);
    }
}

#[test]
fn error_codes_match_the_icm_values() {
    assert_eq!(
        [
            IcError::BadFormat.code(),
            IcError::BadParam.code(),
            IcError::BadSize.code(),
            IcError::Memory.code(),
        ],
        [-2, -6, -7, -3]
    );
}

#[test]
fn value_types_debug_print_and_clone() {
    let ctx = Msvideo1Context::opened();
    assert_eq!(ctx.clone(), ctx);
    assert!(format!("{ctx:?}").starts_with("Msvideo1Context"));

    let query = FormatQuery::Size(40);
    assert_eq!(query.clone(), query);
    assert!(format!("{query:?}").starts_with("Size"));

    let header_copy = header(4, 4, 8, CRAM_MAGIC, 0);
    assert_eq!(header_copy.clone(), header_copy);
    assert!(format!("{:?}", BitmapInfoHeader::default()).starts_with("BitmapInfoHeader"));

    let icinfo = info(ICTYPE_VIDEO);
    assert_eq!(icinfo.clone(), icinfo);
    assert!(format!("{icinfo:?}").starts_with("IcInfo"));

    let result = InfoResult::Filled(icinfo);
    assert_eq!(result.clone(), result);
    assert!(format!("{result:?}").starts_with("Filled"));
    assert!(format!("{:?}", InfoResult::SizeQuery).starts_with("SizeQuery"));

    let error = IcError::Memory;
    assert_eq!(error.clone(), error);
    assert_eq!(format!("{error:?}"), "Memory");
}
