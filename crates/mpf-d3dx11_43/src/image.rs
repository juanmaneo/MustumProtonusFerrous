//! Image info types and the image file format mapping from `texture.c`. The image info
//! derivation itself lives in the crate root.

use crate::format::{D3dxPixelFormatId, DxgiFormat};

/// `enum d3dx_image_file_format` of `dlls/d3dx9_36/d3dx_helpers.h`, without the `FORCE_DWORD`
/// sentinel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3dxImageFileFormat {
    Bmp,
    Jpg,
    Tga,
    Png,
    Dds,
    Ppm,
    Dib,
    Hdr,
    Pfm,
    Tiff,
    Gif,
    Wmp,
    DdsDxt10,
}

/// `D3DX11_IMAGE_FILE_FORMAT` of `include/d3dx11tex.h`, without the `D3DX11_IFF_FORCE_DWORD`
/// sentinel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3dx11ImageFileFormat {
    Bmp,
    Jpg,
    Png,
    Dds,
    Tiff,
    Gif,
    Wmp,
}

/// `enum d3dx_resource_type` of `dlls/d3dx9_36/d3dx_helpers.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3dxResourceType {
    Unknown,
    Texture1d,
    Texture2d,
    Texture3d,
    CubeTexture,
}

/// The `D3D11_RESOURCE_DIMENSION` values that image info can report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceDimension {
    Texture2d,
    Texture3d,
}

/// The fields of Wine's `struct d3dx_image` that determine the image info. The pixel and palette
/// buffers are not part of the derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3dxImage {
    pub resource_type: D3dxResourceType,
    pub format: D3dxPixelFormatId,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub layer_count: u32,
    pub image_file_format: D3dxImageFileFormat,
}

/// `D3DX11_IMAGE_INFO`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_size: u32,
    pub mip_levels: u32,
    pub misc_flags: u32,
    pub format: DxgiFormat,
    pub resource_dimension: ResourceDimension,
    pub image_file_format: D3dx11ImageFileFormat,
}

/// `d3dx11_image_file_format_from_d3dx_image_file_format`. `None` where Wine returns
/// `D3DX11_IFF_FORCE_DWORD`, for formats that D3DX11 cannot report.
pub fn d3dx11_image_file_format_from_d3dx_image_file_format(
    iff: D3dxImageFileFormat,
) -> Option<D3dx11ImageFileFormat> {
    match iff {
        D3dxImageFileFormat::Bmp => Some(D3dx11ImageFileFormat::Bmp),
        D3dxImageFileFormat::Jpg => Some(D3dx11ImageFileFormat::Jpg),
        D3dxImageFileFormat::Png => Some(D3dx11ImageFileFormat::Png),
        D3dxImageFileFormat::Dds => Some(D3dx11ImageFileFormat::Dds),
        D3dxImageFileFormat::Tiff => Some(D3dx11ImageFileFormat::Tiff),
        D3dxImageFileFormat::Gif => Some(D3dx11ImageFileFormat::Gif),
        D3dxImageFileFormat::Wmp => Some(D3dx11ImageFileFormat::Wmp),
        D3dxImageFileFormat::DdsDxt10 => Some(D3dx11ImageFileFormat::Dds),
        D3dxImageFileFormat::Tga
        | D3dxImageFileFormat::Ppm
        | D3dxImageFileFormat::Dib
        | D3dxImageFileFormat::Hdr
        | D3dxImageFileFormat::Pfm => None,
    }
}
