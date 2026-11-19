//! Replays rows of the Wine oracle (`target/canon/d3dx10_43`). Its C is verbatim from
//! `texture.c`, `d3dx10_43_main.c`, `sprite.c` and `async.c`, compiled with
//! `gcc -fsanitize=address,undefined`. No vector was flagged by the sanitizers. Each comment
//! names the oracle output line.

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
    image_file_format: D3dx10ImageFileFormat,
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
    // pf 1 87
    (D3dxPixelFormatId::B8g8r8a8Unorm, 87),
    // pf 17 63
    (D3dxPixelFormatId::R8Snorm, 63),
    // pf 47 31
    (D3dxPixelFormatId::U8v8w8q8Snorm, 31),
    // pf 5 0
    (D3dxPixelFormatId::B5g6r5Unorm, 0),
];

const LEGACY_ROWS: &[(D3dxPixelFormatId, DxgiFormat)] = &[
    // lg 0 28
    (D3dxPixelFormatId::B8g8r8Unorm, 28),
    // lg 27 74
    (D3dxPixelFormatId::Dxt2Unorm, 74),
    // lg 25 11
    (D3dxPixelFormatId::L16Unorm, 11),
];

const IFF_ROWS: &[(D3dxImageFileFormat, Option<D3dx10ImageFileFormat>)] = &[
    // iff 2 2147483647
    (D3dxImageFileFormat::Tga, None),
    // iff 100 4
    (
        D3dxImageFileFormat::DdsDxt10,
        Some(D3dx10ImageFileFormat::Dds),
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
            d3dx10_image_file_format_from_d3dx_image_file_format(iff),
            expected
        );
    }
}

#[test]
fn image_info_matches_oracle_rows() {
    use D3dx10ImageFileFormat as Out;
    use D3dxImageFileFormat as In;
    use D3dxPixelFormatId as Pf;
    use D3dxResourceType as Rt;
    use ResourceDimension::{Texture2d as Dim2, Texture3d as Dim3};

    let rows = [
        // img 100 47 2 S_OK 5 6 7 3 4 0 31 3 4
        (
            image(In::DdsDxt10, Pf::U8v8w8q8Snorm, Rt::Texture2d),
            Ok(info(31, Dim2, 0, Out::Dds)),
        ),
        // img 4 27 2 S_OK 5 6 7 3 4 0 74 3 4
        (
            image(In::Dds, Pf::Dxt2Unorm, Rt::Texture2d),
            Ok(info(74, Dim2, 0, Out::Dds)),
        ),
        // img 0 3 4 S_OK 5 6 7 3 4 4 28 3 0
        (
            image(In::Bmp, Pf::R8g8b8a8Unorm, Rt::CubeTexture),
            Ok(info(28, Dim2, 4, Out::Bmp)),
        ),
        // img 0 3 3 S_OK 5 6 7 3 4 0 28 4 0
        (
            image(In::Bmp, Pf::R8g8b8a8Unorm, Rt::Texture3d),
            Ok(info(28, Dim3, 0, Out::Bmp)),
        ),
        // img 2 3 2 E_FAIL
        (
            image(In::Tga, Pf::R8g8b8a8Unorm, Rt::Texture2d),
            Err(E_FAIL),
        ),
        // img 3 3 1 E_FAIL
        (
            image(In::Png, Pf::R8g8b8a8Unorm, Rt::Texture1d),
            Err(E_FAIL),
        ),
    ];
    for (input, expected) in rows {
        assert_eq!(
            d3dx10_image_info_from_d3dx_image(&input),
            expected,
            "{input:?}"
        );
    }
}

#[test]
fn load_info_matches_oracle_rows() {
    // load none 0 4294967295 4294967295 4294967295 4294967295 4294967295 4294967295 4294967295
    // 4294967295 4294967295 -1 4294967295 4294967295 0 -1
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

    // load some 1 1 2 3 4 5 6 7 8 9 10 11 12 1 1
    let src = info(
        28,
        ResourceDimension::Texture2d,
        0,
        D3dx10ImageFileFormat::Bmp,
    );
    let input = ImageLoadInfo {
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
        src_info: Some(&src),
    };
    let out = init_load_info(Some(&input));
    assert_eq!(out, input);
    assert!(std::ptr::eq(out.src_info.unwrap(), &src));
}

#[test]
fn version_matches_oracle_rows() {
    // version 29 43 1
    assert!(d3dx10_check_version(29, 43));
    // version 29 42 0
    assert!(!d3dx10_check_version(29, 42));
}

#[test]
fn thread_counts_match_oracle_rows() {
    // thread 0 0 8 S_OK 1 8
    assert_eq!(thread_pump_thread_counts(0, 0, 8), Ok((1, 8)));
    // thread 1024 1 8 E_FAIL
    assert_eq!(thread_pump_thread_counts(1024, 1, 8), Err(E_FAIL));
    // thread 5 6 0 S_OK 5 6
    assert_eq!(thread_pump_thread_counts(5, 6, 0), Ok((5, 6)));
}

#[test]
fn array_reserve_matches_oracle_rows() {
    // reserve 100 101 4 1 200 800
    assert_eq!(array_reserve_capacity(100, 101, 4), Some(200));
    // reserve 0 2305843009213693952 8 0 0 0
    assert_eq!(array_reserve_capacity(0, 1 << 61, 8), None);
}
