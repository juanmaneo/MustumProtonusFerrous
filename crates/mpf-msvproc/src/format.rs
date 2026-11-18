//! Pixel formats of the video processor's MF video subtypes, as Wine's
//! `pixel_format_from_video_subtype()` maps them onto FFmpeg's `AVPixelFormat`.
//!
//! Only the `Data1` field of the subtype GUID is consulted. For the `MFVideoFormat_*` FourCC
//! subtypes that field is the FourCC, and for the RGB subtypes it is the `D3DFMT_*` value.

/// The FFmpeg pixel format a subtype is converted through. Wine reports [`PixelFormat::None`] for
/// every subtype it does not support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    None,
    Yuv420p,
    Yuv411p,
    Nv12,
    Yuyv422,
    Yvyu422,
    Uyvy422,
    Ayuv,
    Y216,
    Rgba,
    Bgra,
    Bgr24,
    Bgr0,
    Rgb555,
    Rgb565,
    Rgb8,
}

/// `MAKEFOURCC` of each subtype: the four characters, first one in the lowest byte.
const AYUV: u32 = u32::from_le_bytes(*b"AYUV");
const I420: u32 = u32::from_le_bytes(*b"I420");
const IYUV: u32 = u32::from_le_bytes(*b"IYUV");
const NV11: u32 = u32::from_le_bytes(*b"NV11");
const NV12: u32 = u32::from_le_bytes(*b"NV12");
const UYVY: u32 = u32::from_le_bytes(*b"UYVY");
const Y216: u32 = u32::from_le_bytes(*b"Y216");
const YUY2: u32 = u32::from_le_bytes(*b"YUY2");
const YV12: u32 = u32::from_le_bytes(*b"YV12");
const YVYU: u32 = u32::from_le_bytes(*b"YVYU");

const D3DFMT_R8G8B8: u32 = 20;
const D3DFMT_A8R8G8B8: u32 = 21;
const D3DFMT_X8R8G8B8: u32 = 22;
const D3DFMT_R5G6B5: u32 = 23;
const D3DFMT_X1R5G5B5: u32 = 24;
const D3DFMT_A8B8G8R8: u32 = 32;
const D3DFMT_P8: u32 = 41;

/// Maps the `Data1` of a video subtype GUID to its pixel format. The `420O`, `P208`, `v410`,
/// `Y41P`, `Y41T` and `Y42T` subtypes and any unknown value map to [`PixelFormat::None`].
pub fn pixel_format_from_subtype(data1: u32) -> PixelFormat {
    match data1 {
        AYUV => PixelFormat::Ayuv,
        I420 | IYUV | YV12 => PixelFormat::Yuv420p,
        NV11 => PixelFormat::Yuv411p,
        NV12 => PixelFormat::Nv12,
        UYVY => PixelFormat::Uyvy422,
        Y216 => PixelFormat::Y216,
        YUY2 => PixelFormat::Yuyv422,
        YVYU => PixelFormat::Yvyu422,
        D3DFMT_A8B8G8R8 => PixelFormat::Rgba,
        D3DFMT_A8R8G8B8 => PixelFormat::Bgra,
        D3DFMT_R8G8B8 => PixelFormat::Bgr24,
        D3DFMT_X8R8G8B8 => PixelFormat::Bgr0,
        D3DFMT_X1R5G5B5 => PixelFormat::Rgb555,
        D3DFMT_R5G6B5 => PixelFormat::Rgb565,
        D3DFMT_P8 => PixelFormat::Rgb8,
        _ => PixelFormat::None,
    }
}
