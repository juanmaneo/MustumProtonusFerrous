//! Edge cases of the format and image mappings that the oracle rows do not name. The sweeps
//! replay every value of the C enums, so each arm of the mappings is checked against
//! `oracle.out`.

use super::oracle_checks::{image, info, DEFAULT};
use super::*;
use crate::format::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN};

/// `(format, modern, legacy)` for every `enum d3dx_pixel_format_id` value in C order, where
/// `modern` and `legacy` are the `pf` and `lg` lines of `oracle.out`.
const PIXEL_SWEEP: &[(D3dxPixelFormatId, DxgiFormat, DxgiFormat)] = &[
    (D3dxPixelFormatId::B8g8r8Unorm, 0, 28),
    (D3dxPixelFormatId::B8g8r8a8Unorm, 87, 28),
    (D3dxPixelFormatId::B8g8r8x8Unorm, 88, 28),
    (D3dxPixelFormatId::R8g8b8a8Unorm, 28, 28),
    (D3dxPixelFormatId::R8g8b8x8Unorm, 0, 28),
    (D3dxPixelFormatId::B5g6r5Unorm, 0, 28),
    (D3dxPixelFormatId::B5g5r5x1Unorm, 0, 28),
    (D3dxPixelFormatId::B5g5r5a1Unorm, 0, 28),
    (D3dxPixelFormatId::B2g3r3Unorm, 0, 28),
    (D3dxPixelFormatId::B2g3r3a8Unorm, 0, 28),
    (D3dxPixelFormatId::B4g4r4a4Unorm, 0, 28),
    (D3dxPixelFormatId::B4g4r4x4Unorm, 0, 28),
    (D3dxPixelFormatId::B10g10r10a2Unorm, 0, 24),
    (D3dxPixelFormatId::R10g10b10a2Unorm, 24, 24),
    (D3dxPixelFormatId::R16g16b16Unorm, 0, 0),
    (D3dxPixelFormatId::R16g16b16a16Unorm, 11, 11),
    (D3dxPixelFormatId::R8Unorm, 61, 0),
    (D3dxPixelFormatId::R8Snorm, 0, 0),
    (D3dxPixelFormatId::R8g8Unorm, 49, 0),
    (D3dxPixelFormatId::R16Unorm, 56, 0),
    (D3dxPixelFormatId::R16g16Unorm, 35, 35),
    (D3dxPixelFormatId::A8Unorm, 65, 65),
    (D3dxPixelFormatId::L8a8Unorm, 0, 28),
    (D3dxPixelFormatId::L4a4Unorm, 0, 28),
    (D3dxPixelFormatId::L8Unorm, 0, 28),
    (D3dxPixelFormatId::L16Unorm, 0, 11),
    (D3dxPixelFormatId::Dxt1Unorm, 71, 71),
    (D3dxPixelFormatId::Dxt2Unorm, 0, 74),
    (D3dxPixelFormatId::Dxt3Unorm, 74, 74),
    (D3dxPixelFormatId::Dxt4Unorm, 0, 77),
    (D3dxPixelFormatId::Dxt5Unorm, 77, 77),
    (D3dxPixelFormatId::Bc4Unorm, 80, 80),
    (D3dxPixelFormatId::Bc4Snorm, 81, 81),
    (D3dxPixelFormatId::Bc5Unorm, 83, 83),
    (D3dxPixelFormatId::Bc5Snorm, 84, 84),
    (D3dxPixelFormatId::R16Float, 54, 54),
    (D3dxPixelFormatId::R16g16Float, 34, 34),
    (D3dxPixelFormatId::R16g16b16a16Float, 10, 10),
    (D3dxPixelFormatId::R32Float, 41, 41),
    (D3dxPixelFormatId::R32g32Float, 16, 16),
    (D3dxPixelFormatId::R32g32b32Float, 6, 0),
    (D3dxPixelFormatId::R32g32b32a32Float, 2, 2),
    (D3dxPixelFormatId::P1Uint, 0, 0),
    (D3dxPixelFormatId::P2Uint, 0, 0),
    (D3dxPixelFormatId::P4Uint, 0, 0),
    (D3dxPixelFormatId::P8Uint, 0, 28),
    (D3dxPixelFormatId::P8UintA8Unorm, 0, 28),
    (D3dxPixelFormatId::U8v8w8q8Snorm, 31, 0),
    (D3dxPixelFormatId::U16v16w16q16Snorm, 13, 13),
    (D3dxPixelFormatId::U8v8Snorm, 51, 0),
    (D3dxPixelFormatId::U8v8SnormCx, 0, 0),
    (D3dxPixelFormatId::U16v16Snorm, 37, 0),
    (D3dxPixelFormatId::U8v8SnormL8x8Unorm, 0, 0),
    (D3dxPixelFormatId::U10v10w10SnormA2Unorm, 0, 0),
    (D3dxPixelFormatId::R8g8B8g8Unorm, 68, 68),
    (D3dxPixelFormatId::G8r8G8b8Unorm, 69, 69),
    (D3dxPixelFormatId::Uyvy, 0, 0),
    (D3dxPixelFormatId::Yuy2, 0, 0),
];

/// `(input, output)` for every `enum d3dx_image_file_format` value in C order, from the `iff`
/// lines of `oracle.out`. `None` is `D3DX11_IFF_FORCE_DWORD`.
const IFF_SWEEP: &[(D3dxImageFileFormat, Option<D3dx11ImageFileFormat>)] = &[
    (D3dxImageFileFormat::Bmp, Some(D3dx11ImageFileFormat::Bmp)),
    (D3dxImageFileFormat::Jpg, Some(D3dx11ImageFileFormat::Jpg)),
    (D3dxImageFileFormat::Tga, None),
    (D3dxImageFileFormat::Png, Some(D3dx11ImageFileFormat::Png)),
    (D3dxImageFileFormat::Dds, Some(D3dx11ImageFileFormat::Dds)),
    (D3dxImageFileFormat::Ppm, None),
    (D3dxImageFileFormat::Dib, None),
    (D3dxImageFileFormat::Hdr, None),
    (D3dxImageFileFormat::Pfm, None),
    (D3dxImageFileFormat::Tiff, Some(D3dx11ImageFileFormat::Tiff)),
    (D3dxImageFileFormat::Gif, Some(D3dx11ImageFileFormat::Gif)),
    (D3dxImageFileFormat::Wmp, Some(D3dx11ImageFileFormat::Wmp)),
    (
        D3dxImageFileFormat::DdsDxt10,
        Some(D3dx11ImageFileFormat::Dds),
    ),
];

#[test]
fn pixel_sweep_matches_every_oracle_pixel_format() {
    assert_eq!(PIXEL_SWEEP.len(), 58);
    for (index, &(format, modern, legacy)) in PIXEL_SWEEP.iter().enumerate() {
        assert_eq!(format as usize, index, "{format:?}");
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
fn image_file_format_sweep_matches_every_oracle_format() {
    assert_eq!(IFF_SWEEP.len(), 13);
    for &(iff, expected) in IFF_SWEEP {
        assert_eq!(
            d3dx11_image_file_format_from_d3dx_image_file_format(iff),
            expected,
            "{iff:?}"
        );
    }
}

#[test]
fn non_dds_files_ignore_the_pixel_format() {
    use D3dx11ImageFileFormat as Out;
    use D3dxImageFileFormat as In;

    let files = [
        (In::Bmp, Out::Bmp),
        (In::Jpg, Out::Jpg),
        (In::Png, Out::Png),
        (In::Tiff, Out::Tiff),
        (In::Gif, Out::Gif),
        (In::Wmp, Out::Wmp),
    ];
    for (iff, out) in files {
        for &(format, _, _) in PIXEL_SWEEP {
            assert_eq!(
                d3dx11_image_info_from_d3dx_image(&image(iff, format, D3dxResourceType::Texture2d)),
                Ok(info(
                    DXGI_FORMAT_R8G8B8A8_UNORM,
                    ResourceDimension::Texture2d,
                    0,
                    out
                )),
                "{iff:?} {format:?}"
            );
        }
    }
}

#[test]
fn unmapped_image_file_formats_fail_for_every_pixel_format() {
    use D3dxImageFileFormat as In;

    for iff in [In::Tga, In::Ppm, In::Dib, In::Hdr, In::Pfm] {
        for &(format, _, _) in PIXEL_SWEEP {
            assert_eq!(
                d3dx11_image_info_from_d3dx_image(&image(iff, format, D3dxResourceType::Texture2d)),
                Err(E_FAIL),
                "{iff:?} {format:?}"
            );
        }
    }
}

#[test]
fn unknown_and_1d_resources_fail_for_every_non_cube_file_format() {
    use D3dxImageFileFormat as In;
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;

    for iff in [In::Bmp, In::Dds, In::DdsDxt10] {
        for resource_type in [Rt::Unknown, Rt::Texture1d] {
            assert_eq!(
                d3dx11_image_info_from_d3dx_image(&image(iff, Pf::R8g8b8a8Unorm, resource_type)),
                Err(E_FAIL),
                "{iff:?} {resource_type:?}"
            );
        }
    }
}

#[test]
fn legacy_dds_cube_and_volume_keep_their_dimension() {
    use D3dx11ImageFileFormat as Out;
    use D3dxImageFileFormat as In;
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;
    use ResourceDimension::{Texture2d as Dim2, Texture3d as Dim3};

    assert_eq!(
        d3dx11_image_info_from_d3dx_image(&image(In::Dds, Pf::R8g8b8a8Unorm, Rt::CubeTexture)),
        Ok(info(DXGI_FORMAT_R8G8B8A8_UNORM, Dim2, 4, Out::Dds))
    );
    assert_eq!(
        d3dx11_image_info_from_d3dx_image(&image(In::Dds, Pf::R8g8b8a8Unorm, Rt::Texture3d)),
        Ok(info(DXGI_FORMAT_R8G8B8A8_UNORM, Dim3, 0, Out::Dds))
    );
}

#[test]
fn dds_images_fail_exactly_where_their_pixel_format_has_no_dxgi_format() {
    use D3dxImageFileFormat as In;
    use D3dxResourceType as Rt;

    for &(format, modern, legacy) in PIXEL_SWEEP {
        let legacy_format =
            d3dx11_image_info_from_d3dx_image(&image(In::Dds, format, Rt::Texture2d))
                .map(|image_info| image_info.format)
                .ok();
        assert_eq!(
            legacy_format,
            (legacy != DXGI_FORMAT_UNKNOWN).then_some(legacy),
            "{format:?}"
        );

        let modern_format =
            d3dx11_image_info_from_d3dx_image(&image(In::DdsDxt10, format, Rt::Texture2d))
                .map(|image_info| image_info.format)
                .ok();
        assert_eq!(
            modern_format,
            (modern != DXGI_FORMAT_UNKNOWN).then_some(modern),
            "{format:?}"
        );
    }
}

#[test]
fn load_info_copy_keeps_the_source_reference() {
    let source = info(
        DXGI_FORMAT_R8G8B8A8_UNORM,
        ResourceDimension::Texture2d,
        0,
        D3dx11ImageFileFormat::Dds,
    );
    let given = ImageLoadInfo {
        width: 9,
        src_info: Some(&source),
        ..init_load_info(None)
    };

    let copied = init_load_info(Some(&given));
    assert_eq!(copied.width, 9);
    assert_eq!(copied.format, DEFAULT);
    assert!(std::ptr::eq(copied.src_info.expect("source"), &source));
}
