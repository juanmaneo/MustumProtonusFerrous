//! `video_format_from_codec_params` from `dlls/winedmo/unix_media_type.c`: the MFVIDEOFORMAT written
//! for a video stream.

use crate::codec::{CodecId, CodecParams, CodecTables};
use crate::guid::{media_guid, Guid};
use crate::pixel::{subtype_from_pixel_format, PixelFormat};
use crate::status::{check_capacity, FormatError};
use crate::time::Rational;

/// `sizeof(MFVIDEOFORMAT)`.
pub const MF_VIDEO_FORMAT_SIZE: usize = 168;

/// `MEDIASUBTYPE_MPEG1Payload`, which is not built from the `DEFINE_MEDIATYPE_GUID` pattern.
const MEDIASUBTYPE_MPEG1_PAYLOAD: Guid = Guid {
    data1: 0xe436_eb81,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

/// `MFVideoArea.Area`, a `SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aperture {
    pub cx: i32,
    pub cy: i32,
}

/// `MFRatio`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MfRatio {
    pub numerator: u32,
    pub denominator: u32,
}

/// The `MFVIDEOFORMAT` that the conversion writes, followed by the extradata.
pub struct VideoFormat {
    pub dw_size: u32,
    pub guid_format: Guid,
    pub width: u32,
    pub height: u32,
    pub minimum_display_aperture: Aperture,
    pub geometric_aperture: Aperture,
    pub pan_scan_aperture: Aperture,
    pub pixel_aspect_ratio: MfRatio,
    pub frames_per_second: MfRatio,
    pub extradata: Vec<u8>,
}

/// `video_format_from_codec_params`. The dimensions are `(size + align) & !align`, as in the C code,
/// not rounded up to a multiple: 1080 with `align` 16 gives 1096.
pub fn video_format_from_codec_params(
    params: &CodecParams,
    capacity: u32,
    sar: Rational,
    fps: Rational,
    align: u32,
    tables: &dyn CodecTables,
) -> Result<VideoFormat, FormatError> {
    let format_size = MF_VIDEO_FORMAT_SIZE + params.extradata.len();
    check_capacity(format_size, capacity)?;

    let guid_format =
        if params.codec_id == CodecId::RawVideo && params.pixel_format != PixelFormat::None {
            subtype_from_pixel_format(params.pixel_format)
        } else {
            match params.codec_id {
                CodecId::Mpeg1Video => MEDIASUBTYPE_MPEG1_PAYLOAD,
                CodecId::H264 => media_guid(u32::from_le_bytes(*b"H264")),
                CodecId::Vp9 => media_guid(u32::from_le_bytes(*b"VP90")),
                _ if params.codec_tag != 0 => media_guid(params.codec_tag),
                _ => media_guid(tables.video_format_tag(params.codec_id)),
            }
        };

    let width = (params.width as u32).wrapping_add(align) & !align;
    let height = (params.height as u32).wrapping_add(align) & !align;
    let aperture = if width != params.width as u32 || height != params.height as u32 {
        Aperture {
            cx: params.width,
            cy: params.height,
        }
    } else {
        Aperture { cx: 0, cy: 0 }
    };

    let pixel_aspect_ratio = if sar.num != 0 && sar.den != 0 {
        MfRatio {
            numerator: sar.num as u32,
            denominator: sar.den as u32,
        }
    } else {
        MfRatio {
            numerator: 1,
            denominator: 1,
        }
    };
    let frames_per_second = if fps.num != 0 && fps.den != 0 {
        MfRatio {
            numerator: fps.num as u32,
            denominator: fps.den as u32,
        }
    } else {
        MfRatio {
            numerator: 0,
            denominator: 0,
        }
    };

    Ok(VideoFormat {
        dw_size: format_size as u32,
        guid_format,
        width,
        height,
        minimum_display_aperture: aperture,
        geometric_aperture: aperture,
        pan_scan_aperture: aperture,
        pixel_aspect_ratio,
        frames_per_second,
        extradata: params.extradata.to_vec(),
    })
}
