//! The uncompressed video format table and the stride and size helpers of `dlls/mfplat/mediatype.c`.

use crate::guid::Guid;
use crate::{media_subtype_guid, video_format_guid, Hresult, E_INVALIDARG, MF_E_INVALIDMEDIATYPE};

/// `D3DFMT_*` values from `include/d3d9types.h`, used as `Data1` of the `MFVideoFormat_*` GUIDs.
const D3DFMT_A1: u32 = 118;
const D3DFMT_A16B16G16R16F: u32 = 113;
const D3DFMT_A2B10G10R10: u32 = 31;
const D3DFMT_A2R10G10B10: u32 = 35;
const D3DFMT_A8B8G8R8: u32 = 32;
const D3DFMT_A8R8G8B8: u32 = 21;
const D3DFMT_D16: u32 = 80;
const D3DFMT_L16: u32 = 81;
const D3DFMT_L8: u32 = 50;
const D3DFMT_P8: u32 = 41;
const D3DFMT_R5G6B5: u32 = 23;
const D3DFMT_R8G8B8: u32 = 20;
const D3DFMT_X1R5G5B5: u32 = 24;
const D3DFMT_X8R8G8B8: u32 = 22;

/// FourCC values, the `MAKEFOURCC` of their letters in little-endian order.
const FOURCC_4PXX: u32 = u32::from_le_bytes(*b"4Pxx");
const FOURCC_AYUV: u32 = u32::from_le_bytes(*b"AYUV");
const FOURCC_I420: u32 = u32::from_le_bytes(*b"I420");
const FOURCC_IMC1: u32 = u32::from_le_bytes(*b"IMC1");
const FOURCC_IMC2: u32 = u32::from_le_bytes(*b"IMC2");
const FOURCC_IMC3: u32 = u32::from_le_bytes(*b"IMC3");
const FOURCC_IMC4: u32 = u32::from_le_bytes(*b"IMC4");
const FOURCC_IYUV: u32 = u32::from_le_bytes(*b"IYUV");
const FOURCC_NV11: u32 = u32::from_le_bytes(*b"NV11");
const FOURCC_NV12: u32 = u32::from_le_bytes(*b"NV12");
const FOURCC_P010: u32 = u32::from_le_bytes(*b"P010");
const FOURCC_UYVY: u32 = u32::from_le_bytes(*b"UYVY");
const FOURCC_YUY2: u32 = u32::from_le_bytes(*b"YUY2");
const FOURCC_YV12: u32 = u32::from_le_bytes(*b"YV12");
const FOURCC_YVYU: u32 = u32::from_le_bytes(*b"YVYU");

/// One row of `video_formats`: the layout of an uncompressed video format.
struct Format {
    subtype: Guid,
    /// Bits per pixel.
    bpp: u32,
    /// Row padding: a stride is rounded up to a multiple of `alignment + 1` bytes.
    alignment: u32,
    /// A bottom-up format has a negative stride, so its rows run from the last line up.
    bottom_up: bool,
    yuv: bool,
}

/// `video_formats` in `mediatype.c`. The C sorts this table for `bsearch`. `lookup` searches it
/// linearly, which finds the same row, because the subtypes are distinct.
static VIDEO_FORMATS: [Format; 34] = [
    Format {
        subtype: video_format_guid(D3DFMT_A1),
        bpp: 1,
        alignment: 0,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(FOURCC_4PXX),
        bpp: 4,
        alignment: 0,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_R8G8B8),
        bpp: 24,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_A8R8G8B8),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_X8R8G8B8),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_R5G6B5),
        bpp: 16,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_X1R5G5B5),
        bpp: 16,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_A8B8G8R8),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_A2B10G10R10),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_A2R10G10B10),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_P8),
        bpp: 8,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_L8),
        bpp: 8,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(FOURCC_AYUV),
        bpp: 32,
        alignment: 3,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_I420),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_IMC1),
        bpp: 16,
        alignment: 3,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_IMC2),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_IMC3),
        bpp: 16,
        alignment: 3,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_IMC4),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_IYUV),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_NV11),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_NV12),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_P010),
        bpp: 24,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(D3DFMT_D16),
        bpp: 16,
        alignment: 3,
        bottom_up: false,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(D3DFMT_L16),
        bpp: 16,
        alignment: 3,
        bottom_up: false,
        yuv: false,
    },
    Format {
        subtype: video_format_guid(FOURCC_UYVY),
        bpp: 16,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_YUY2),
        bpp: 16,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_YV12),
        bpp: 12,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(FOURCC_YVYU),
        bpp: 16,
        alignment: 0,
        bottom_up: false,
        yuv: true,
    },
    Format {
        subtype: video_format_guid(D3DFMT_A16B16G16R16F),
        bpp: 64,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: media_subtype_guid(0xe436eb7a),
        bpp: 8,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: media_subtype_guid(0xe436eb7b),
        bpp: 16,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: media_subtype_guid(0xe436eb7c),
        bpp: 16,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: media_subtype_guid(0xe436eb7d),
        bpp: 24,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
    Format {
        subtype: media_subtype_guid(0xe436eb7e),
        bpp: 32,
        alignment: 3,
        bottom_up: true,
        yuv: false,
    },
];

/// The table row of `subtype`, if the format is uncompressed and listed.
fn lookup(subtype: &Guid) -> Option<&'static Format> {
    VIDEO_FORMATS
        .iter()
        .find(|format| format.subtype == *subtype)
}

/// `mf_get_stride_for_format`: the bytes in one row of `width` pixels.
///
/// Sub-byte formats pack `bpp` bits per pixel, P010 is two bytes per pixel, and every other format
/// is `bpp / 8` bytes per pixel, rounded up to the alignment.
fn stride_for_format(format: &Format, width: u32) -> u32 {
    if format.bpp < 8 {
        width.wrapping_mul(format.bpp) / 8
    } else if format.subtype == video_format_guid(FOURCC_P010) {
        width.wrapping_mul(2)
    } else {
        width
            .wrapping_mul(format.bpp / 8)
            .wrapping_add(format.alignment)
            & !format.alignment
    }
}

/// `MFGetStrideForBitmapInfoHeader`: the stride of the uncompressed format whose `Data1` is
/// `fourcc`, negative for a bottom-up format.
pub fn stride_for_bitmap_info_header(fourcc: u32, width: u32) -> Result<i32, Hresult> {
    let format = lookup(&video_format_guid(fourcc)).ok_or(MF_E_INVALIDMEDIATYPE)?;
    let stride = stride_for_format(format, width) as i32;
    Ok(if format.bottom_up {
        stride.wrapping_neg()
    } else {
        stride
    })
}

/// `mf_format_get_stride`: the stride and YUV flag of an uncompressed subtype, or `None` if the
/// subtype is not in the table.
pub fn format_stride(subtype: &Guid, width: u32) -> Option<(u32, bool)> {
    let format = lookup(subtype)?;
    Some((stride_for_format(format, width), format.yuv))
}

/// `MFCalculateImageSize`: the bytes of a `width` by `height` frame of an uncompressed subtype.
///
/// A zero height, or a subtype that is not in the table, is `E_INVALIDARG`. The arithmetic wraps at
/// 32 bits, as the C does.
pub fn image_size(subtype: &Guid, width: u32, height: u32) -> Result<u32, Hresult> {
    if height == 0 {
        return Err(E_INVALIDARG);
    }
    let format = lookup(subtype).ok_or(E_INVALIDARG)?;
    let even_width = width.wrapping_add(1) & !1;
    let size = match subtype.data1 {
        FOURCC_IMC2 | FOURCC_IMC4 | FOURCC_NV12 | FOURCC_YV12 | FOURCC_I420 | FOURCC_IYUV => {
            even_width.wrapping_mul(height).wrapping_mul(3) / 2
        }
        FOURCC_P010 => even_width.wrapping_mul(height).wrapping_mul(3),
        FOURCC_NV11 => {
            (width.wrapping_add(3) & !3)
                .wrapping_mul(height)
                .wrapping_mul(3)
                / 2
        }
        D3DFMT_L8 | D3DFMT_L16 | D3DFMT_D16 => {
            width.wrapping_mul(format.bpp / 8).wrapping_mul(height)
        }
        _ => stride_for_format(format, width).wrapping_mul(height),
    };
    Ok(size)
}

/// `MFGetPlaneSize`: the bytes of the first plane of the format whose `Data1` is `fourcc`.
///
/// A format that is not in the table has a stride of zero, so its plane is empty.
pub fn plane_size(fourcc: u32, width: u32, height: u32) -> u32 {
    let stride =
        lookup(&video_format_guid(fourcc)).map_or(0, |format| stride_for_format(format, width));
    match fourcc {
        FOURCC_IMC2 | FOURCC_IMC4 | FOURCC_NV12 | FOURCC_YV12 | FOURCC_I420 | FOURCC_IYUV
        | FOURCC_NV11 | FOURCC_P010 => stride.wrapping_mul(height).wrapping_mul(3) / 2,
        _ => stride.wrapping_mul(height),
    }
}
