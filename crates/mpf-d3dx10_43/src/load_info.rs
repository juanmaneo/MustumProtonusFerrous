//! `init_load_info` from `texture.c`: the `D3DX10_IMAGE_LOAD_INFO` used when a caller passes
//! no load info.

use crate::format::DxgiFormat;
use crate::image::ImageInfo;

/// `D3DX10_DEFAULT`.
const D3DX10_DEFAULT: u32 = 0xffff_ffff;

/// `D3DX10_IMAGE_LOAD_INFO`. `usage` is a `D3D10_USAGE` value and `src_info` is the
/// `pSrcInfo` pointer, kept as a borrow of the caller's `D3DX10_IMAGE_INFO`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageLoadInfo<'a> {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub first_mip_level: u32,
    pub mip_levels: u32,
    pub usage: u32,
    pub bind_flags: u32,
    pub cpu_access_flags: u32,
    pub misc_flags: u32,
    pub format: DxgiFormat,
    pub filter: u32,
    pub mip_filter: u32,
    pub src_info: Option<&'a ImageInfo>,
}

/// `init_load_info`: copies `load_info` when the caller gave one, and otherwise returns the
/// load info with every field set to `D3DX10_DEFAULT` and no source info. The C sets `Format` to
/// `D3DX10_DEFAULT` on this path, not to the `DXGI_FORMAT_FROM_FILE` of the C++ constructor in
/// `include/d3dx10tex.h`, which is not ported.
pub fn init_load_info<'a>(load_info: Option<&ImageLoadInfo<'a>>) -> ImageLoadInfo<'a> {
    match load_info {
        Some(load_info) => *load_info,
        None => ImageLoadInfo {
            width: D3DX10_DEFAULT,
            height: D3DX10_DEFAULT,
            depth: D3DX10_DEFAULT,
            first_mip_level: D3DX10_DEFAULT,
            mip_levels: D3DX10_DEFAULT,
            usage: D3DX10_DEFAULT,
            bind_flags: D3DX10_DEFAULT,
            cpu_access_flags: D3DX10_DEFAULT,
            misc_flags: D3DX10_DEFAULT,
            format: D3DX10_DEFAULT,
            filter: D3DX10_DEFAULT,
            mip_filter: D3DX10_DEFAULT,
            src_info: None,
        },
    }
}
