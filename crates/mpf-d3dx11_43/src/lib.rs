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
    d3dx11_image_file_format_from_d3dx_image_file_format, d3dx11_image_info_from_d3dx_image,
    D3dx11ImageFileFormat, D3dxImage, D3dxImageFileFormat, D3dxResourceType, ImageInfo,
    ResourceDimension,
};
pub use load_info::{init_load_info, ImageLoadInfo};

/// `HRESULT`.
pub type HResult = i32;

/// `E_FAIL`.
pub const E_FAIL: HResult = 0x8000_4005_u32 as HResult;
