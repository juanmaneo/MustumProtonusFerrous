//! Replays rows of the Wine oracle (`target/canon/d3dx11_43`). Its C is verbatim from
//! `texture.c` and `d3dx_helpers.h`, compiled with `gcc -fsanitize=address,undefined`. No
//! vector was excluded for a sanitizer report. Each comment names the oracle output line.

use super::*;

pub(super) const DEFAULT: u32 = 0xffff_ffff;

pub(super) fn image(
    iff: D3dxImageFileFormat,
    format: D3dxPixelFormatId,
    resource_type: D3dxResourceType,
) -> D3dxImage {
    D3dxImage {
        resource_type,
        format,
        width: 5,
        height: 6,
        depth: 7,
        mip_levels: 4,
        layer_count: 3,
        image_file_format: iff,
    }
}

pub(super) fn info(
    format: DxgiFormat,
    resource_dimension: ResourceDimension,
    misc_flags: u32,
    image_file_format: D3dx11ImageFileFormat,
) -> ImageInfo {
    ImageInfo {
        width: 5,
        height: 6,
        depth: 7,
        array_size: 3,
        mip_levels: 4,
        misc_flags,
        format,
        resource_dimension,
        image_file_format,
    }
}

const MODERN_ROWS: &[(D3dxPixelFormatId, DxgiFormat)] = &[
    // pf 0 0
    (D3dxPixelFormatId::B8g8r8Unorm, 0),
    // pf 28 74
    (D3dxPixelFormatId::Dxt3Unorm, 74),
    // pf 47 31
    (D3dxPixelFormatId::U8v8w8q8Snorm, 31),
];

const LEGACY_ROWS: &[(D3dxPixelFormatId, DxgiFormat)] = &[
    // lg 0 28
    (D3dxPixelFormatId::B8g8r8Unorm, 28),
    // lg 27 74
    (D3dxPixelFormatId::Dxt2Unorm, 74),
    // lg 47 0
    (D3dxPixelFormatId::U8v8w8q8Snorm, 0),
];

const IFF_ROWS: &[(D3dxImageFileFormat, Option<D3dx11ImageFileFormat>)] = &[
    // iff 2 2147483647
    (D3dxImageFileFormat::Tga, None),
    // iff 100 4
    (
        D3dxImageFileFormat::DdsDxt10,
        Some(D3dx11ImageFileFormat::Dds),
    ),
];

#[test]
fn pixel_formats_match_oracle_rows() {
    for &(format, expected) in MODERN_ROWS {
        assert_eq!(
            dxgi_format_from_d3dx_pixel_format_id(format),
            expected,
            "{format:?}"
        );
    }
    for &(format, expected) in LEGACY_ROWS {
        assert_eq!(
            dxgi_format_from_legacy_dds_d3dx_pixel_format_id(format),
            expected,
            "{format:?}"
        );
    }
}

#[test]
fn image_file_formats_match_oracle_rows() {
    for &(iff, expected) in IFF_ROWS {
        assert_eq!(
            d3dx11_image_file_format_from_d3dx_image_file_format(iff),
            expected
        );
    }
}

#[test]
fn image_info_matches_oracle_rows() {
    use D3dx11ImageFileFormat as Out;
    use D3dxImageFileFormat as In;
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;
    use ResourceDimension::{Texture2d as Dim2, Texture3d as Dim3};

    let rows = [
        // img 0 0 1 E_FAIL
        (image(In::Bmp, Pf::B8g8r8Unorm, Rt::Texture1d), Err(E_FAIL)),
        // img 0 47 2 S_OK 5 6 7 3 4 0 28 3 0
        (
            image(In::Bmp, Pf::U8v8w8q8Snorm, Rt::Texture2d),
            Ok(info(28, Dim2, 0, Out::Bmp)),
        ),
        // img 2 0 2 E_FAIL
        (image(In::Tga, Pf::B8g8r8Unorm, Rt::Texture2d), Err(E_FAIL)),
        // img 4 0 0 E_FAIL
        (image(In::Dds, Pf::B8g8r8Unorm, Rt::Unknown), Err(E_FAIL)),
        // img 4 0 2 S_OK 5 6 7 3 4 0 28 3 4
        (
            image(In::Dds, Pf::B8g8r8Unorm, Rt::Texture2d),
            Ok(info(28, Dim2, 0, Out::Dds)),
        ),
        // img 12 1 2 S_OK 5 6 7 3 4 0 28 3 12
        (
            image(In::Wmp, Pf::B8g8r8a8Unorm, Rt::Texture2d),
            Ok(info(28, Dim2, 0, Out::Wmp)),
        ),
        // img 100 0 2 E_FAIL
        (
            image(In::DdsDxt10, Pf::B8g8r8Unorm, Rt::Texture2d),
            Err(E_FAIL),
        ),
        // img 100 1 2 S_OK 5 6 7 3 4 0 87 3 4
        (
            image(In::DdsDxt10, Pf::B8g8r8a8Unorm, Rt::Texture2d),
            Ok(info(87, Dim2, 0, Out::Dds)),
        ),
        // img 100 1 3 S_OK 5 6 7 3 4 0 87 4 4
        (
            image(In::DdsDxt10, Pf::B8g8r8a8Unorm, Rt::Texture3d),
            Ok(info(87, Dim3, 0, Out::Dds)),
        ),
        // img 100 1 4 S_OK 5 6 7 3 4 4 87 3 4
        (
            image(In::DdsDxt10, Pf::B8g8r8a8Unorm, Rt::CubeTexture),
            Ok(info(87, Dim2, 4, Out::Dds)),
        ),
        // img 100 26 2 S_OK 5 6 7 3 4 0 71 3 4
        (
            image(In::DdsDxt10, Pf::Dxt1Unorm, Rt::Texture2d),
            Ok(info(71, Dim2, 0, Out::Dds)),
        ),
        // img 100 47 2 S_OK 5 6 7 3 4 0 31 3 4
        (
            image(In::DdsDxt10, Pf::U8v8w8q8Snorm, Rt::Texture2d),
            Ok(info(31, Dim2, 0, Out::Dds)),
        ),
    ];

    for (input, expected) in rows {
        assert_eq!(
            d3dx11_image_info_from_d3dx_image(&input),
            expected,
            "{input:?}"
        );
    }
}

#[test]
fn load_info_matches_oracle_rows() {
    // load none 0 4294967295 ... -1 4294967295 4294967295 0
    assert_eq!(
        init_load_info(None),
        ImageLoadInfo {
            width: DEFAULT,
            height: DEFAULT,
            depth: DEFAULT,
            first_mip_level: DEFAULT,
            mip_levels: DEFAULT,
            usage: DEFAULT,
            bind_flags: DEFAULT,
            cpu_access_flags: DEFAULT,
            misc_flags: DEFAULT,
            format: DEFAULT,
            filter: DEFAULT,
            mip_filter: DEFAULT,
            src_info: None,
        }
    );

    let source = info(
        28,
        ResourceDimension::Texture2d,
        0,
        D3dx11ImageFileFormat::Dds,
    );
    // load some 1 1 2 3 4 5 6 7 8 9 10 11 12 1
    let given = ImageLoadInfo {
        width: 1,
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
        src_info: Some(&source),
    };
    assert_eq!(init_load_info(Some(&given)), given);

    // load nullsrc 1 4294967295 2 3 4 5 6 7 8 9 10 11 12 0
    let no_source = ImageLoadInfo {
        width: DEFAULT,
        src_info: None,
        ..given
    };
    assert_eq!(init_load_info(Some(&no_source)), no_source);
}
