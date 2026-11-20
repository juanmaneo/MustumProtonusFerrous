//! The video format checks of `dlls/amstream/ddrawstream.c`: `subtype_from_pf`,
//! `is_format_compatible` and the decision of `ddraw_sink_QueryAccept`.

use crate::{
    Guid, Hresult, E_POINTER, FORMAT_VIDEOINFO, MEDIASUBTYPE_RGB24, MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB555, MEDIASUBTYPE_RGB565, MEDIASUBTYPE_RGB8, MEDIATYPE_VIDEO, S_OK,
    VFW_E_TYPE_NOT_ACCEPTED,
};

/// `DDPF_PALETTEINDEXED8`.
pub const DDPF_PALETTEINDEXED8: u32 = 0x0000_0020;
/// `DDPF_RGB`.
pub const DDPF_RGB: u32 = 0x0000_0040;
/// `DDPF_FOURCC`.
pub const DDPF_FOURCC: u32 = 0x0000_0004;
/// `DDSD_HEIGHT`.
pub const DDSD_HEIGHT: u32 = 0x0000_0002;
/// `DDSD_PIXELFORMAT`.
pub const DDSD_PIXELFORMAT: u32 = 0x0000_1000;

/// The fields of a `DDPIXELFORMAT` that the ported checks read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PixelFormat {
    pub flags: u32,
    pub rgb_bit_count: u32,
    pub r_mask: u32,
    pub g_mask: u32,
}

/// The fields of a `DDSURFACEDESC`-derived stream format that the ported checks read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamFormat {
    pub flags: u32,
    pub width: u32,
    pub height: u32,
    pub pf: PixelFormat,
}

/// The fields of an `AM_MEDIA_TYPE` and its `VIDEOINFOHEADER` that [`query_accept`] reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoMediaType {
    pub majortype: Guid,
    pub formattype: Guid,
    pub subtype: Guid,
    /// `bmiHeader.biHeight`.
    pub bi_height: i32,
}

/// Maps a DirectDraw pixel format to a media subtype, as `subtype_from_pf` does. Returns `None`
/// where Wine only reports the format with a FIXME and leaves the subtype unset.
pub fn subtype_from_pixel_format(pf: &PixelFormat) -> Option<Guid> {
    match (pf.rgb_bit_count, pf.r_mask) {
        (16, 0x7c00) => Some(MEDIASUBTYPE_RGB555),
        (16, 0xf800) => Some(MEDIASUBTYPE_RGB565),
        (24, _) => Some(MEDIASUBTYPE_RGB24),
        (32, _) => Some(MEDIASUBTYPE_RGB32),
        (8, _) if pf.flags & DDPF_PALETTEINDEXED8 != 0 => Some(MEDIASUBTYPE_RGB8),
        _ => None,
    }
}

/// Whether a connection with `width`, `height` and `connection_pf` fits the stream's format, as
/// `is_format_compatible` does.
pub fn is_format_compatible(
    format: &StreamFormat,
    width: u32,
    height: u32,
    connection_pf: &PixelFormat,
) -> bool {
    if format.flags & DDSD_HEIGHT != 0 && (format.width != width || format.height != height) {
        return false;
    }
    if format.flags & DDSD_PIXELFORMAT != 0 {
        if format.pf.flags & DDPF_FOURCC != 0 {
            return false;
        }
        if format.pf.rgb_bit_count != connection_pf.rgb_bit_count {
            return false;
        }
        if format.pf.rgb_bit_count == 16 && format.pf.g_mask != connection_pf.g_mask {
            return false;
        }
    }
    true
}

/// Accepts a video media type on the DirectDraw stream, as `ddraw_sink_QueryAccept` does. A
/// null media type gives `E_POINTER`; the other rejections give `VFW_E_TYPE_NOT_ACCEPTED`.
pub fn query_accept(
    mt: Option<&VideoMediaType>,
    format: &StreamFormat,
    connected: bool,
) -> Hresult {
    let Some(mt) = mt else {
        return E_POINTER;
    };
    if mt.majortype != MEDIATYPE_VIDEO || mt.formattype != FORMAT_VIDEOINFO || mt.bi_height < 0 {
        return VFW_E_TYPE_NOT_ACCEPTED;
    }
    if format.flags & DDSD_PIXELFORMAT != 0 {
        return match subtype_from_pixel_format(&format.pf) {
            Some(subtype) if subtype == mt.subtype => S_OK,
            _ => VFW_E_TYPE_NOT_ACCEPTED,
        };
    }
    if mt.subtype == MEDIASUBTYPE_RGB8 {
        return S_OK;
    }
    if !connected {
        return VFW_E_TYPE_NOT_ACCEPTED;
    }
    match mt.subtype {
        MEDIASUBTYPE_RGB555 | MEDIASUBTYPE_RGB565 | MEDIASUBTYPE_RGB24 | MEDIASUBTYPE_RGB32 => S_OK,
        _ => VFW_E_TYPE_NOT_ACCEPTED,
    }
}
