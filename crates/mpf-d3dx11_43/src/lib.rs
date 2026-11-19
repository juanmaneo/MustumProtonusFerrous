//! Deterministic part of Wine's `dlls/d3dx11_43/texture.c`: the pixel and image file format
//! mappings to DXGI and D3DX11 values, the `D3DX11_IMAGE_INFO` derived from a parsed image
//! header, and the default-filling of `D3DX11_IMAGE_LOAD_INFO`.
//!
//! Ported:
//! - `dxgi_format_from_d3dx_pixel_format_id` and `dxgi_format_from_legacy_dds_d3dx_pixel_format_id`:
//!   pixel format id to `DXGI_FORMAT`, for DDS_DXT10 and for legacy DDS files.
//! - `d3dx11_image_file_format_from_d3dx_image_file_format`: image file format mapping. Wine
//!   returns `D3DX11_IFF_FORCE_DWORD` for unmapped formats, which is `None` here.
//! - `d3dx11_image_info_from_d3dx_image`: image info derivation and its failure rules.
//! - `init_load_info`: the default or copied `D3DX11_IMAGE_LOAD_INFO`.
//!
//! Skipped, not portable without OS, COM, file or decoder state:
//! - `async.c`: the `ID3DX11DataLoader` vtables, `D3DX11CreateAsync*Loader` (handles, resources,
//!   heap state) and `load_file`.
//! - `texture.c`: `get_image_info` and `D3DX11GetImageInfoFrom*` (they call `d3dx_image_init`),
//!   `load_texture_data` and `create_d3d_texture` (decoding and D3D11 device calls), `D3DX11Create*`,
//!   `D3DX11Save*`, the `dump_*` trace helpers and the `FIXME` stubs.
//!
//! Oracle provenance: the ported functions are copied verbatim from Wine's
//! `dlls/d3dx11_43/texture.c` and `dlls/d3dx9_36/d3dx_helpers.h`, and the enums and structs
//! come from the `include/` headers. The extract compiles under
//! `gcc -fsanitize=address,undefined` against Win32 scalar shims in `target/canon/d3dx11_43/`
//! (not tracked in git).

mod format;
mod image;
mod load_info;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use format::{
    dxgi_format_from_d3dx_pixel_format_id, dxgi_format_from_legacy_dds_d3dx_pixel_format_id,
    D3dxPixelFormatId, DxgiFormat,
};
pub use image::{
    d3dx11_image_file_format_from_d3dx_image_file_format, D3dx11ImageFileFormat, D3dxImage,
    D3dxImageFileFormat, D3dxResourceType, ImageInfo, ResourceDimension,
};
pub use load_info::{init_load_info, ImageLoadInfo};

/// `HRESULT`.
pub type HResult = i32;

/// `E_FAIL`.
pub const E_FAIL: HResult = 0x8000_4005_u32 as HResult;

use crate::format::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN};

/// `D3D11_RESOURCE_MISC_TEXTURECUBE`, from `include/d3d11.idl`.
const D3D11_RESOURCE_MISC_TEXTURECUBE: u32 = 0x0000_0004;

/// `d3dx11_image_info_from_d3dx_image`. Fails with `E_FAIL` for an image that has no
/// `D3DX11_IFF` equivalent, a pixel format without a DXGI format, or an unsupported resource
/// type. Wine leaves `info` partly written on failure, and callers discard it, so no partial
/// value is returned here.
pub fn d3dx11_image_info_from_d3dx_image(image: &D3dxImage) -> Result<ImageInfo, HResult> {
    let image_file_format =
        d3dx11_image_file_format_from_d3dx_image_file_format(image.image_file_format)
            .ok_or(E_FAIL)?;

    let format = match image.image_file_format {
        D3dxImageFileFormat::DdsDxt10 => dxgi_format_from_d3dx_pixel_format_id(image.format),
        D3dxImageFileFormat::Dds => dxgi_format_from_legacy_dds_d3dx_pixel_format_id(image.format),
        _ => DXGI_FORMAT_R8G8B8A8_UNORM,
    };
    if format == DXGI_FORMAT_UNKNOWN {
        return Err(E_FAIL);
    }

    let (resource_dimension, misc_flags) = match image.resource_type {
        D3dxResourceType::Texture2d => (ResourceDimension::Texture2d, 0),
        D3dxResourceType::CubeTexture => (
            ResourceDimension::Texture2d,
            D3D11_RESOURCE_MISC_TEXTURECUBE,
        ),
        D3dxResourceType::Texture3d => (ResourceDimension::Texture3d, 0),
        D3dxResourceType::Unknown | D3dxResourceType::Texture1d => return Err(E_FAIL),
    };

    Ok(ImageInfo {
        width: image.width,
        height: image.height,
        depth: image.depth,
        array_size: image.layer_count,
        mip_levels: image.mip_levels,
        misc_flags,
        format,
        resource_dimension,
        image_file_format,
    })
}
