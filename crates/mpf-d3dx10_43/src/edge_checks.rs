//! Edge cases that the oracle rows do not name. The sweeps replay every value of the C enums
//! and the `d3dx_array_reserve` cases of `harness.c`, so each arm of the mappings is checked
//! against `oracle.out`.

use super::oracle_checks::{image, info, DEFAULT};
use super::*;
use crate::format::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN};

/// `(format, modern, legacy)` for every `enum d3dx_pixel_format_id` value in C order. `modern`
/// and `legacy` are the `pf` and `lg` lines of `oracle.out`.
const PIXEL_SWEEP: &[(D3dxPixelFormatId, DxgiFormat, DxgiFormat)] = &[
    (D3dxPixelFormatId::B8g8r8Unorm, 0, 28),          // 0
    (D3dxPixelFormatId::B8g8r8a8Unorm, 87, 28),       // 1
    (D3dxPixelFormatId::B8g8r8x8Unorm, 88, 28),       // 2
    (D3dxPixelFormatId::R8g8b8a8Unorm, 28, 28),       // 3
    (D3dxPixelFormatId::R8g8b8x8Unorm, 0, 28),        // 4
    (D3dxPixelFormatId::B5g6r5Unorm, 0, 28),          // 5
    (D3dxPixelFormatId::B5g5r5x1Unorm, 0, 28),        // 6
    (D3dxPixelFormatId::B5g5r5a1Unorm, 0, 28),        // 7
    (D3dxPixelFormatId::B2g3r3Unorm, 0, 28),          // 8
    (D3dxPixelFormatId::B2g3r3a8Unorm, 0, 28),        // 9
    (D3dxPixelFormatId::B4g4r4a4Unorm, 0, 28),        // 10
    (D3dxPixelFormatId::B4g4r4x4Unorm, 0, 28),        // 11
    (D3dxPixelFormatId::B10g10r10a2Unorm, 0, 24),     // 12
    (D3dxPixelFormatId::R10g10b10a2Unorm, 24, 24),    // 13
    (D3dxPixelFormatId::R16g16b16Unorm, 0, 0),        // 14
    (D3dxPixelFormatId::R16g16b16a16Unorm, 11, 11),   // 15
    (D3dxPixelFormatId::R8Unorm, 61, 0),              // 16
    (D3dxPixelFormatId::R8Snorm, 63, 0),              // 17
    (D3dxPixelFormatId::R8g8Unorm, 49, 0),            // 18
    (D3dxPixelFormatId::R16Unorm, 56, 0),             // 19
    (D3dxPixelFormatId::R16g16Unorm, 35, 35),         // 20
    (D3dxPixelFormatId::A8Unorm, 65, 65),             // 21
    (D3dxPixelFormatId::L8a8Unorm, 0, 28),            // 22
    (D3dxPixelFormatId::L4a4Unorm, 0, 28),            // 23
    (D3dxPixelFormatId::L8Unorm, 0, 28),              // 24
    (D3dxPixelFormatId::L16Unorm, 0, 11),             // 25
    (D3dxPixelFormatId::Dxt1Unorm, 71, 71),           // 26
    (D3dxPixelFormatId::Dxt2Unorm, 0, 74),            // 27
    (D3dxPixelFormatId::Dxt3Unorm, 74, 74),           // 28
    (D3dxPixelFormatId::Dxt4Unorm, 0, 77),            // 29
    (D3dxPixelFormatId::Dxt5Unorm, 77, 77),           // 30
    (D3dxPixelFormatId::Bc4Unorm, 80, 80),            // 31
    (D3dxPixelFormatId::Bc4Snorm, 81, 81),            // 32
    (D3dxPixelFormatId::Bc5Unorm, 83, 83),            // 33
    (D3dxPixelFormatId::Bc5Snorm, 84, 84),            // 34
    (D3dxPixelFormatId::R16Float, 54, 54),            // 35
    (D3dxPixelFormatId::R16g16Float, 34, 34),         // 36
    (D3dxPixelFormatId::R16g16b16a16Float, 10, 10),   // 37
    (D3dxPixelFormatId::R32Float, 41, 41),            // 38
    (D3dxPixelFormatId::R32g32Float, 16, 16),         // 39
    (D3dxPixelFormatId::R32g32b32Float, 6, 0),        // 40
    (D3dxPixelFormatId::R32g32b32a32Float, 2, 2),     // 41
    (D3dxPixelFormatId::P1Uint, 0, 0),                // 42
    (D3dxPixelFormatId::P2Uint, 0, 0),                // 43
    (D3dxPixelFormatId::P4Uint, 0, 0),                // 44
    (D3dxPixelFormatId::P8Uint, 0, 28),               // 45
    (D3dxPixelFormatId::P8UintA8Unorm, 0, 28),        // 46
    (D3dxPixelFormatId::U8v8w8q8Snorm, 31, 0),        // 47
    (D3dxPixelFormatId::U16v16w16q16Snorm, 13, 13),   // 48
    (D3dxPixelFormatId::U8v8Snorm, 51, 0),            // 49
    (D3dxPixelFormatId::U8v8SnormCx, 0, 0),           // 50
    (D3dxPixelFormatId::U16v16Snorm, 37, 0),          // 51
    (D3dxPixelFormatId::U8v8SnormL8x8Unorm, 0, 0),    // 52
    (D3dxPixelFormatId::U10v10w10SnormA2Unorm, 0, 0), // 53
    (D3dxPixelFormatId::R8g8B8g8Unorm, 68, 68),       // 54
    (D3dxPixelFormatId::G8r8G8b8Unorm, 69, 69),       // 55
    (D3dxPixelFormatId::Uyvy, 0, 0),                  // 56
    (D3dxPixelFormatId::Yuy2, 0, 0),                  // 57
];

/// `(iff, expected)` for every `enum d3dx_image_file_format` value in C order, from the `iff`
/// lines of `oracle.out`.
const IFF_SWEEP: &[(D3dxImageFileFormat, Option<D3dx10ImageFileFormat>)] = &[
    (D3dxImageFileFormat::Bmp, Some(D3dx10ImageFileFormat::Bmp)),
    (D3dxImageFileFormat::Jpg, Some(D3dx10ImageFileFormat::Jpg)),
    (D3dxImageFileFormat::Tga, None),
    (D3dxImageFileFormat::Png, Some(D3dx10ImageFileFormat::Png)),
    (D3dxImageFileFormat::Dds, Some(D3dx10ImageFileFormat::Dds)),
    (D3dxImageFileFormat::Ppm, None),
    (D3dxImageFileFormat::Dib, None),
    (D3dxImageFileFormat::Hdr, None),
    (D3dxImageFileFormat::Pfm, None),
    (D3dxImageFileFormat::Tiff, Some(D3dx10ImageFileFormat::Tiff)),
    (D3dxImageFileFormat::Gif, Some(D3dx10ImageFileFormat::Gif)),
    (D3dxImageFileFormat::Wmp, Some(D3dx10ImageFileFormat::Wmp)),
    (
        D3dxImageFileFormat::DdsDxt10,
        Some(D3dx10ImageFileFormat::Dds),
    ),
];

/// `(capacity, count, size, expected)` for the `d3dx_array_reserve` cases of `harness.c`, with
/// the results that the oracle prints. `None` is the `false` return.
const RESERVE_SWEEP: &[(usize, usize, usize, Option<usize>)] = &[
    (0, 0, 8, Some(0)),
    (0, 1, 8, Some(4)),
    (0, 4, 8, Some(4)),
    (0, 5, 8, Some(8)),
    (4, 4, 8, Some(4)),
    (4, 5, 8, Some(8)),
    (3, 9, 16, Some(16)),
    (100, 101, 4, Some(200)),
    (0, 1000, 4, Some(1024)),
    (4, 2, 0, Some(4)),
    (0, (1 << 20) + 1, 1, Some(1 << 21)),
    (0, usize::MAX, 1, Some(usize::MAX)),
    (0, usize::MAX / 8 + 1, 8, None),
    (0, usize::MAX / 8, 8, Some(usize::MAX / 8)),
];

#[test]
fn pixel_format_sweep_matches_oracle() {
    assert_eq!(PIXEL_SWEEP.len(), 58);
    for &(format, modern, legacy) in PIXEL_SWEEP {
        assert_eq!(
            dxgi_format_from_d3dx_pixel_format_id(format),
            modern,
            "{format:?}"
        );
        assert_eq!(
            dxgi_format_from_legacy_dds_d3dx_pixel_format_id(format),
            legacy,
            "{format:?}"
        );
    }
}

#[test]
fn image_file_format_sweep_matches_oracle() {
    for &(iff, expected) in IFF_SWEEP {
        assert_eq!(
            d3dx10_image_file_format_from_d3dx_image_file_format(iff),
            expected,
            "{iff:?}"
        );
    }
}

#[test]
fn image_info_resource_types_match_oracle() {
    use D3dx10ImageFileFormat as Out;
    use D3dxImageFileFormat as In;
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;
    use ResourceDimension::{Texture2d as Dim2, Texture3d as Dim3};

    let rows = [
        // img 0 3 0 E_FAIL
        (Rt::Unknown, Err(E_FAIL)),
        // img 0 3 1 E_FAIL
        (Rt::Texture1d, Err(E_FAIL)),
        // img 0 3 2 S_OK 5 6 7 3 4 0 28 3 0
        (Rt::Texture2d, Ok(info(28, Dim2, 0, Out::Bmp))),
        // img 0 3 3 S_OK 5 6 7 3 4 0 28 4 0
        (Rt::Texture3d, Ok(info(28, Dim3, 0, Out::Bmp))),
        // img 0 3 4 S_OK 5 6 7 3 4 4 28 3 0
        (Rt::CubeTexture, Ok(info(28, Dim2, 4, Out::Bmp))),
    ];
    for (resource_type, expected) in rows {
        let input = image(In::Bmp, Pf::R8g8b8a8Unorm, resource_type);
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&input),
            expected,
            "{resource_type:?}"
        );
    }
}

#[test]
fn image_info_fails_exactly_for_unmapped_file_formats() {
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;

    for &(iff, expected) in IFF_SWEEP {
        let input = image(iff, Pf::R8g8b8a8Unorm, Rt::Texture2d);
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&input).is_ok(),
            expected.is_some(),
            "{iff:?}"
        );
    }
}

#[test]
fn image_info_format_follows_the_file_format_and_pixel_format() {
    use D3dxImageFileFormat as In;
    use D3dxResourceType as Rt;

    let expected_dxgi = |dxgi: DxgiFormat| {
        if dxgi == DXGI_FORMAT_UNKNOWN {
            Err(E_FAIL)
        } else {
            Ok(dxgi)
        }
    };
    for &(format, modern, legacy) in PIXEL_SWEEP {
        let dxt10 = image(In::DdsDxt10, format, Rt::Texture2d);
        let dds = image(In::Dds, format, Rt::Texture2d);
        let bmp = image(In::Bmp, format, Rt::Texture2d);
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&dxt10).map(|info| info.format),
            expected_dxgi(modern),
            "{format:?}"
        );
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&dds).map(|info| info.format),
            expected_dxgi(legacy),
            "{format:?}"
        );
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&bmp).map(|info| info.format),
            Ok(DXGI_FORMAT_R8G8B8A8_UNORM),
            "{format:?}"
        );
    }
}

#[test]
fn load_info_copies_a_null_source() {
    // load nullsrc 1 4294967295 2 3 4 5 6 7 8 9 10 11 12 0 1
    let input = ImageLoadInfo {
        width: DEFAULT,
        height: 2,
        depth: 3,
        first_mip_level: 4,
        mip_levels: 5,
        usage: 6,
        bind_flags: 7,
        cpu_access_flags: 8,
        misc_flags: 9,
        format: 10,
        filter: 11,
        mip_filter: 12,
        src_info: None,
    };
    assert_eq!(init_load_info(Some(&input)), input);
}

#[test]
fn array_reserve_sweep_matches_oracle() {
    for &(capacity, count, size, expected) in RESERVE_SWEEP {
        assert_eq!(
            array_reserve_capacity(capacity, count, size),
            expected,
            "{capacity} {count} {size}"
        );
    }
}

#[test]
fn array_growth_edges() {
    // (capacity, count, size, expected)
    let rows = [
        // A count that fits keeps the capacity, even for a zero size.
        (10, 3, 8, Some(10)),
        (0, 0, 0, Some(0)),
        (7, 7, 0, Some(7)),
        // Growth starts at four elements and doubles.
        (2, 3, 8, Some(4)),
        (4, 5, 8, Some(8)),
        (4, 9, 8, Some(16)),
        // A zero size cannot grow: the C divides by zero here.
        (0, 1, 0, None),
        // The largest count that fits in usize, and one count past what fits.
        (0, usize::MAX, 1, Some(usize::MAX)),
        (0, usize::MAX, 2, None),
        // Elements larger than SIZE_MAX / 4: the first step is clamped to the count that fits.
        // Wine's max(4, capacity) would give 4 here and wrap the byte count.
        (0, 1, 1 << 63, Some(1)),
        (0, 2, 1 << 62, Some(3)),
        (0, 3, 1 << 62, Some(3)),
        (0, 4, 1 << 62, None),
        (0, 2, 1 << 63, None),
    ];
    for (capacity, count, size, expected) in rows {
        assert_eq!(
            array_reserve_capacity(capacity, count, size),
            expected,
            "{capacity} {count} {size}"
        );
    }
}

#[test]
fn thread_count_boundaries() {
    // thread 1023 1023 8 S_OK 1023 1023
    assert_eq!(thread_pump_thread_counts(1023, 1023, 8), Ok((1023, 1023)));
    // Each count is checked before a zero count takes its default.
    assert_eq!(thread_pump_thread_counts(1024, 0, 8), Err(E_FAIL));
    assert_eq!(thread_pump_thread_counts(0, 1024, 8), Err(E_FAIL));
    // thread 4294967295 0 8 E_FAIL
    assert_eq!(thread_pump_thread_counts(u32::MAX, 0, 8), Err(E_FAIL));
    // thread 0 0 0 S_OK 1 0
    assert_eq!(thread_pump_thread_counts(0, 0, 0), Ok((1, 0)));
    // thread 2 0 4 S_OK 2 4
    assert_eq!(thread_pump_thread_counts(2, 0, 4), Ok((2, 4)));
    // thread 0 3 8 S_OK 1 3
    assert_eq!(thread_pump_thread_counts(0, 3, 8), Ok((1, 3)));
}

#[test]
fn version_neighbours() {
    // (d3d_sdk_version, d3dx_sdk_version, expected), from the `version` lines of `oracle.out`.
    let rows = [
        (29, 43, true),
        (28, 43, false),
        (30, 43, false),
        (0, 0, false),
        (29, 0, false),
        (u32::MAX, 43, false),
        (29, u32::MAX, false),
        (29, 44, false),
    ];
    for (d3d, d3dx, expected) in rows {
        assert_eq!(d3dx10_check_version(d3d, d3dx), expected, "{d3d} {d3dx}");
    }
}
