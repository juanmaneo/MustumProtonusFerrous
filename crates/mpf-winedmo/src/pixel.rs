//! `subtype_from_pixel_format` from `dlls/winedmo/unix_media_type.c`: the Media Foundation video
//! subtype of a raw FFmpeg pixel format.

use crate::guid::{media_guid, Guid};

const D3DFMT_R8G8B8: u32 = 20;
const D3DFMT_A8R8G8B8: u32 = 21;
const D3DFMT_X8R8G8B8: u32 = 22;
const D3DFMT_R5G6B5: u32 = 23;
const D3DFMT_X1R5G5B5: u32 = 24;
const D3DFMT_A8B8G8R8: u32 = 32;
const D3DFMT_P8: u32 = 41;

/// The `AVPixelFormat` values with a subtype mapping. `None` and `Other` have no mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    None,
    Yuv420p,
    Yuvj420p,
    Yuyv422,
    Uyvy422,
    Bgr0,
    Rgba,
    Bgra,
    Bgr24,
    Nv12,
    Nv21,
    P010le,
    Rgb565le,
    Rgb555le,
    Rgb8,
    Other,
}

/// `subtype_from_pixel_format`. Unmapped formats give `MFVideoFormat_Base`.
pub fn subtype_from_pixel_format(format: PixelFormat) -> Guid {
    let data1 = match format {
        PixelFormat::None | PixelFormat::Other => 0,
        PixelFormat::Yuv420p | PixelFormat::Yuvj420p => u32::from_le_bytes(*b"I420"),
        PixelFormat::Yuyv422 => u32::from_le_bytes(*b"YUY2"),
        PixelFormat::Uyvy422 => u32::from_le_bytes(*b"UYVY"),
        PixelFormat::Bgr0 => D3DFMT_X8R8G8B8,
        PixelFormat::Rgba => D3DFMT_A8B8G8R8,
        PixelFormat::Bgra => D3DFMT_A8R8G8B8,
        PixelFormat::Bgr24 => D3DFMT_R8G8B8,
        PixelFormat::Nv12 => u32::from_le_bytes(*b"NV12"),
        PixelFormat::Nv21 => u32::from_le_bytes(*b"NV21"),
        PixelFormat::P010le => u32::from_le_bytes(*b"P010"),
        PixelFormat::Rgb565le => D3DFMT_R5G6B5,
        PixelFormat::Rgb555le => D3DFMT_X1R5G5B5,
        PixelFormat::Rgb8 => D3DFMT_P8,
    };
    media_guid(data1)
}
