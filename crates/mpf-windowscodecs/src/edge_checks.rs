//! Edge checks for the cases the oracle rows do not reach: truncated files, DX10 headers without
//! the FourCC flag, cube and uncompressed files that `parse_header` rejects, frame indices with
//! zero divisors, the legacy pixel-format table, the bounds that the copy routines check before
//! they write, and the derived traits of every public type. The deviations from Wine are marked
//! `Rust-only`.

use super::*;
use crate::oracle_checks::{dx10_header, file_bytes, legacy_header, rgb32_header};

fn exercise<T: Clone + std::fmt::Debug + PartialEq>(value: &T) {
    assert_eq!(value.clone(), *value);
    assert!(!format!("{value:?}").is_empty());
}

fn legacy_info(flags: u32, rgb_bit_count: u32, masks: [u32; 4]) -> DdsInfo {
    let [r_mask, g_mask, b_mask, a_mask] = masks;
    let header = DdsHeader {
        pixel_format: DdsPixelFormat {
            flags,
            rgb_bit_count,
            r_mask,
            g_mask,
            b_mask,
            a_mask,
            ..DdsPixelFormat::default()
        },
        ..DdsHeader::default()
    };
    dds_info(&header, None)
}

#[test]
fn parse_header_reports_a_short_file_as_a_stream_read_error() {
    let legacy = file_bytes(DDS_MAGIC, &legacy_header(8, 4, 1, FOURCC_DXT5), None, 128);
    for len in [0, 3, 4, 127] {
        assert_eq!(parse_header(&legacy[..len]), Err(WINCODEC_ERR_STREAMREAD));
    }
    let dx10 = file_bytes(
        DDS_MAGIC,
        &legacy_header(8, 4, 1, FOURCC_DX10),
        Some(&dx10_header(0x47, 3, 0, 1, 0)),
        148,
    );
    for len in [128, 147] {
        assert_eq!(parse_header(&dx10[..len]), Err(WINCODEC_ERR_STREAMREAD));
    }
}

#[test]
fn parse_header_rejects_cube_and_non_block_compressed_files() {
    let cube = DdsHeader {
        caps2: DDSCAPS2_CUBEMAP,
        ..legacy_header(8, 4, 1, FOURCC_DXT5)
    };
    for header in [cube, rgb32_header(), legacy_header(8, 4, 1, FOURCC_BC4U)] {
        let file = file_bytes(DDS_MAGIC, &header, None, 128);
        assert_eq!(parse_header(&file), Err(WINCODEC_ERR_BADHEADER));
    }
}

#[test]
fn dx10_four_cc_without_the_fourcc_flag_is_not_an_extended_header() {
    let header = DdsHeader {
        pixel_format: DdsPixelFormat {
            four_cc: FOURCC_DX10,
            ..DdsPixelFormat::default()
        },
        ..DdsHeader::default()
    };
    let dxt10 = dx10_header(0x1c, 3, 0, 0, 0);
    let info = dds_info(&header, Some(&dxt10));
    assert_eq!(
        (
            info.format,
            info.pixel_format,
            info.pixel_format_bpp,
            info.data_offset
        ),
        (0, PixelFormat::Undefined, 0, 128)
    );
    let info = dds_info(&header, None);
    assert_eq!(
        (info.format, info.pixel_format, info.pixel_format_bpp),
        (0, PixelFormat::Undefined, 0)
    );
}

#[test]
fn dx10_header_defaults_zero_depth_mips_and_array_size_to_one() {
    let header = legacy_header(8, 4, 0, FOURCC_DX10);
    let info = dds_info(&header, Some(&dx10_header(0x1c, 3, 0, 0, 0)));
    assert_eq!((info.depth, info.mip_levels, info.array_size), (1, 1, 1));
}

#[test]
fn dx10_uncompressed_format_reports_its_wic_format_and_block_size() {
    let header = legacy_header(8, 4, 1, FOURCC_DX10);
    let info = dds_info(&header, Some(&dx10_header(0x1c, 3, 0, 1, 0)));
    assert_eq!(
        (
            info.pixel_format,
            info.pixel_format_bpp,
            info.bytes_per_block,
            info.data_offset
        ),
        (PixelFormat::Bpp32RGBA, 32, 4, 148)
    );
}

#[test]
fn legacy_pixel_format_rows_match_flags_bit_count_and_masks() {
    let rgba = legacy_info(DDPF_RGB, 32, [0xff, 0xff00, 0xff_0000, 0xff00_0000]);
    assert_eq!(
        (
            rgba.pixel_format,
            rgba.pixel_format_bpp,
            rgba.bytes_per_block
        ),
        (PixelFormat::Bpp32RGBA, 32, 4)
    );
    let alpha = legacy_info(DDPF_ALPHA, 8, [0, 0, 0, 0xff]);
    assert_eq!(
        (
            alpha.pixel_format,
            alpha.pixel_format_bpp,
            alpha.bytes_per_block
        ),
        (PixelFormat::Bpp8Alpha, 8, 1)
    );
    let gray = legacy_info(DDPF_LUMINANCE, 16, [0xffff, 0, 0, 0]);
    assert_eq!(
        (
            gray.pixel_format,
            gray.pixel_format_bpp,
            gray.bytes_per_block
        ),
        (PixelFormat::Bpp16Gray, 16, 2)
    );
    let unknown = legacy_info(DDPF_RGB, 12, [0; 4]);
    assert_eq!(
        (
            unknown.format,
            unknown.pixel_format,
            unknown.pixel_format_bpp,
            unknown.bytes_per_block
        ),
        (0, PixelFormat::Undefined, 0, 1)
    );
}

#[test]
fn frame_index_is_none_for_zero_divisors_and_depth() {
    let no_array = DdsInfo {
        depth: 4,
        mip_levels: 3,
        array_size: 0,
        frame_count: 14,
        dimension: Dimension::Texture2D,
        ..DdsInfo::default()
    };
    let no_cube_mips = DdsInfo {
        depth: 1,
        mip_levels: 0,
        array_size: 1,
        frame_count: 0,
        dimension: Dimension::Cube,
        ..DdsInfo::default()
    };
    let no_depth = DdsInfo {
        depth: 0,
        mip_levels: 3,
        array_size: 2,
        frame_count: 14,
        dimension: Dimension::Texture2D,
        ..DdsInfo::default()
    };
    for info in [no_array, no_cube_mips, no_depth] {
        assert_eq!(frame_dds_index(0, &info), None);
    }
}

#[test]
fn cube_frame_index_takes_its_frames_per_texture_from_the_mip_count() {
    let info = DdsInfo {
        depth: 1,
        mip_levels: 2,
        array_size: 1,
        frame_count: 12,
        dimension: Dimension::Cube,
        ..DdsInfo::default()
    };
    assert_eq!(
        frame_dds_index(5, &info),
        Some(FrameIndex {
            array_index: 2,
            mip_level: 1,
            slice_index: 0,
        })
    );
}

#[test]
fn dxgi_format_without_a_legacy_row_has_no_wic_format() {
    assert_eq!(
        dxgi_to_wic(DXGI_FORMAT_R8G8_B8G8_UNORM),
        PixelFormat::Undefined
    );
}

#[test]
fn palette_type_rejects_values_outside_the_predefined_eleven() {
    for wic in [0, 1, 0xd] {
        assert_eq!(PaletteType::from_wic(wic), Err(E_INVALIDARG));
    }
    assert_eq!(PaletteType::from_wic(0xc), Ok(PaletteType::Gray256));
}

#[test]
fn half_float_decodes_subnormal_normal_and_maximum_values() {
    assert_eq!(float_16_to_32(0x0400).to_bits(), 0x3880_0000);
    assert_eq!(float_16_to_32(0x7bff).to_bits(), 0x477f_e000);
    assert_eq!(float_16_to_32(0x8000).to_bits(), 0x8000_0000);
}

#[test]
fn palette_index_takes_the_earlier_of_equal_distances() {
    assert_eq!(rgb_to_palette_index([0, 0, 0], &[]), 0);
    assert_eq!(
        rgb_to_palette_index([0, 0, 0], &[0xff01_0000, 0xff00_0001]),
        0
    );
    assert_eq!(rgb_to_palette_index([0, 0, 0], &[0xff01_0000, 0]), 1);
}

#[test]
fn copy_palette_without_a_map_fills_only_the_requested_entries() {
    let mut colors = [0x1234_5678, 0x8765_4321];
    copy_palette(None, &[], 1, &mut colors);
    assert_eq!(colors, [0xff00_0000, 0x8765_4321]);
    let mut untouched = [0x1234_5678; 2];
    copy_palette(None, &[], 0, &mut untouched);
    assert_eq!(untouched, [0x1234_5678; 2]);
}

// Rust-only: Wine reads colour-map entries past the end of a short map. Here the entries past the
// end are left unchanged.
#[test]
fn copy_palette_with_a_short_map_leaves_the_rest_alone() {
    let map = [Rgb {
        red: 1,
        green: 2,
        blue: 3,
    }];
    let mut colors = [0x1234_5678; 3];
    copy_palette(Some(map.as_slice()), &[], 3, &mut colors);
    assert_eq!(colors, [0xff01_0203, 0x1234_5678, 0x1234_5678]);
}

#[test]
fn copy_palette_ignores_blocks_that_are_not_a_transparent_graphic_control() {
    let ignored = [
        ExtensionBlock {
            function: 0xf8,
            bytes: vec![0, 0, 0, 1, 0, 0, 0, 0],
        },
        ExtensionBlock {
            function: GRAPHICS_EXT_FUNC_CODE,
            bytes: vec![0, 0, 0, 1, 0, 0, 0],
        },
        ExtensionBlock {
            function: GRAPHICS_EXT_FUNC_CODE,
            bytes: vec![0, 0, 0, 2, 0, 0, 0, 0],
        },
    ];
    for block in &ignored {
        let mut colors = [0_u32; 1];
        copy_palette(None, std::slice::from_ref(block), 1, &mut colors);
        assert_eq!(colors, [0xff00_0000]);
    }
}

// Rust-only: Wine writes past the palette at a transparency index beyond its end. Here the index is
// ignored.
#[test]
fn copy_palette_ignores_a_transparent_index_past_the_end() {
    let blocks = [ExtensionBlock {
        function: GRAPHICS_EXT_FUNC_CODE,
        bytes: vec![0, 0, 0, 1, 0, 0, 9, 0],
    }];
    let mut colors = [0_u32; 2];
    copy_palette(None, &blocks, 2, &mut colors);
    assert_eq!(colors, [0xff00_0000, 0xffff_ffff]);
}

#[test]
fn copy_palette_uses_the_first_transparent_block() {
    let blocks = [
        ExtensionBlock {
            function: GRAPHICS_EXT_FUNC_CODE,
            bytes: vec![0, 0, 0, 1, 0, 0, 0, 0],
        },
        ExtensionBlock {
            function: GRAPHICS_EXT_FUNC_CODE,
            bytes: vec![0, 0, 0, 1, 0, 0, 1, 0],
        },
    ];
    let mut colors = [0_u32; 2];
    copy_palette(None, &blocks, 2, &mut colors);
    assert_eq!(colors, [0x0000_0000, 0xffff_ffff]);
}

// Rust-only: negative sizes and rectangles outside the source are rejected before the first write.
// See the crate docs for the full list of deviations.
#[test]
fn copy_interlaced_pixels_rejects_a_bad_rect_before_writing() {
    let src: Vec<u8> = (0..88).collect();
    let rects = [
        WicRect {
            x: -1,
            y: 2,
            width: 3,
            height: 5,
        },
        WicRect {
            x: 1,
            y: -1,
            width: 3,
            height: 5,
        },
        WicRect {
            x: 1,
            y: 2,
            width: -1,
            height: 5,
        },
        WicRect {
            x: 1,
            y: 2,
            width: 3,
            height: -1,
        },
        WicRect {
            x: 3,
            y: 2,
            width: 3,
            height: 5,
        },
        WicRect {
            x: 1,
            y: 7,
            width: 3,
            height: 5,
        },
    ];
    for rect in rects {
        let mut dst = [0xaa_u8; 19];
        assert_eq!(
            copy_interlaced_pixels(&src, 5, 11, 8, Some(rect), 4, &mut dst),
            Err(E_INVALIDARG)
        );
        assert_eq!(dst, [0xaa; 19]);
    }
}

#[test]
fn copy_interlaced_pixels_checks_the_stride_the_destination_and_the_source() {
    let src: Vec<u8> = (0..88).collect();
    let rect = Some(WicRect {
        x: 1,
        y: 2,
        width: 3,
        height: 5,
    });
    let mut dst = [0xaa_u8; 19];
    assert_eq!(
        copy_interlaced_pixels(&src, 5, 11, 8, rect, 2, &mut dst),
        Err(E_INVALIDARG)
    );
    let mut short = [0xaa_u8; 18];
    assert_eq!(
        copy_interlaced_pixels(&src, 5, 11, 8, rect, 4, &mut short),
        Err(E_INVALIDARG)
    );
    assert_eq!(short, [0xaa; 18]);
    assert_eq!(
        copy_interlaced_pixels(&src[..60], 5, 11, 8, rect, 4, &mut dst),
        Err(E_INVALIDARG)
    );
    assert_eq!(dst, [0xaa; 19]);
}

#[test]
fn copy_interlaced_pixels_accepts_an_empty_rect_and_writes_nothing() {
    let src: Vec<u8> = (0..88).collect();
    let rect = Some(WicRect {
        x: 1,
        y: 2,
        width: 3,
        height: 0,
    });
    let mut dst = [0xaa_u8; 19];
    assert_eq!(
        copy_interlaced_pixels(&src, 5, 11, 8, rect, 4, &mut dst),
        Ok(())
    );
    assert_eq!(dst, [0xaa; 19]);
}

#[test]
fn pixel_format_follows_wic_declaration_order() {
    let all = [
        PixelFormat::Undefined,
        PixelFormat::Bpp32PBGRA,
        PixelFormat::Bpp32BGRA,
        PixelFormat::Bpp32FourChannels,
        PixelFormat::Bpp64RGBA,
        PixelFormat::Bpp16GrayHalf,
        PixelFormat::Bpp64RGBAHalf,
        PixelFormat::Bpp32GrayFloat,
        PixelFormat::Bpp128RGBAFloat,
        PixelFormat::Bpp32RGBA,
        PixelFormat::Bpp32RGB,
        PixelFormat::Bpp32BGR,
        PixelFormat::Bpp32R10G10B10A2,
        PixelFormat::Bpp32RGBA1010102,
        PixelFormat::Bpp24BGR,
        PixelFormat::Bpp24RGB,
        PixelFormat::Bpp16BGR565,
        PixelFormat::Bpp16BGR555,
        PixelFormat::Bpp16BGRA5551,
        PixelFormat::Bpp8Alpha,
        PixelFormat::Bpp16Gray,
        PixelFormat::Bpp8Gray,
        PixelFormat::Bpp32RGBE,
        PixelFormat::Bpp32RGBA1010102XR,
        PixelFormat::Bpp96RGBFloat,
    ];
    for (index, format) in all.iter().enumerate() {
        assert_eq!(*format as usize, index);
        exercise(format);
    }
    assert_eq!(PixelFormat::default(), PixelFormat::Undefined);
}

#[test]
fn dimension_and_alpha_mode_values_and_derives() {
    let dimensions = [
        Dimension::Texture1D,
        Dimension::Texture2D,
        Dimension::Texture3D,
        Dimension::Cube,
    ];
    for dimension in &dimensions {
        exercise(dimension);
    }
    assert_eq!(Dimension::default(), Dimension::Texture1D);
    let modes = [
        AlphaMode::Unknown,
        AlphaMode::Straight,
        AlphaMode::Premultiplied,
    ];
    let values: Vec<u32> = modes.iter().map(|&mode| mode as u32).collect();
    assert_eq!(values, [0, 1, 2]);
    for mode in &modes {
        exercise(mode);
    }
}

#[test]
fn palette_type_derives_are_exercised() {
    let all = [
        PaletteType::BlackWhite,
        PaletteType::Halftone8,
        PaletteType::Halftone27,
        PaletteType::Halftone64,
        PaletteType::Halftone125,
        PaletteType::Halftone216,
        PaletteType::Halftone252,
        PaletteType::Halftone256,
        PaletteType::Gray4,
        PaletteType::Gray16,
        PaletteType::Gray256,
    ];
    for palette_type in &all {
        exercise(palette_type);
    }
}

#[test]
fn plain_values_clone_compare_and_print() {
    exercise(&WicRect {
        x: 1,
        y: 2,
        width: 3,
        height: 4,
    });
    exercise(&Rgb {
        red: 1,
        green: 2,
        blue: 3,
    });
    exercise(&Rgb::default());
    exercise(&ExtensionBlock {
        function: GRAPHICS_EXT_FUNC_CODE,
        bytes: vec![0; 8],
    });
    exercise(&ExtensionBlock::default());
    exercise(&DdsPixelFormat::default());
    exercise(&DdsHeader::default());
    exercise(&DdsHeaderDxt10::default());
    exercise(&DdsInfo::default());
    exercise(&FrameIndex::default());
    exercise(&FrameIndex {
        array_index: 1,
        mip_level: 2,
        slice_index: 3,
    });
}
