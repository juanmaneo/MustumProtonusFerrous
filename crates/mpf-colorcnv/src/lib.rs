//! Deterministic part of Wine's `dlls/colorcnv/color_converter.c`: pixel-format mapping,
//! DMO/MF media-type validation, converter subtype lists, DMO-to-MF HRESULT mapping and the
//! frame plane layout used when wrapping a buffer.

mod layout;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use layout::{check_arrays_alignment, fill, Filled};

pub type HResult = u32;

pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const E_NOTIMPL: HResult = 0x8000_4001;
pub const E_INVALIDARG: HResult = 0x8007_0057;
pub const DMO_E_INVALIDSTREAMINDEX: HResult = 0x8004_0201;
pub const DMO_E_INVALIDTYPE: HResult = 0x8004_0202;
pub const DMO_E_TYPE_NOT_SET: HResult = 0x8004_0203;
pub const DMO_E_NOTACCEPTING: HResult = 0x8004_0204;
pub const DMO_E_TYPE_NOT_ACCEPTED: HResult = 0x8004_0205;
pub const DMO_E_NO_MORE_ITEMS: HResult = 0x8004_0206;
pub const MF_E_INVALIDSTREAMNUMBER: HResult = 0xC00D_36B3;
pub const MF_E_INVALIDMEDIATYPE: HResult = 0xC00D_36B4;
pub const MF_E_NOTACCEPTING: HResult = 0xC00D_36B5;
pub const MF_E_NO_MORE_TYPES: HResult = 0xC00D_36B9;
pub const MF_E_TRANSFORM_TYPE_NOT_SET: HResult = 0xC00D_6D60;
pub const MF_E_TRANSFORM_NEED_MORE_INPUT: HResult = 0xC00D_6D72;

/// `MFVideoFlag_BottomUpLinearRep`.
pub const BOTTOM_UP_LINEAR_REP: u64 = 0x0008_0000;

const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;

/// Offsets of the `BITMAPINFOHEADER` fields that the converter reads, relative to the header.
const BMI_WIDTH: usize = 4;
const BMI_HEIGHT: usize = 8;
const BMI_COMPRESSION: usize = 16;
const BMI_SIZE_IMAGE: usize = 20;

/// `VIDEOINFOHEADER` and `VIDEOINFOHEADER2` as laid out on x86_64.
const VIDEO_INFO_BMI: usize = 48;
const VIDEO_INFO_SIZE: usize = 88;
const VIDEO_INFO2_BMI: usize = 72;
const VIDEO_INFO2_SIZE: usize = 112;

/// `MFVIDEOFORMAT` offsets: `videoInfo.dwWidth`, `dwHeight`, `VideoFlags` and `guidFormat`.
const MF_VIDEO_FORMAT_SIZE: usize = 168;
const MF_WIDTH: usize = 8;
const MF_HEIGHT: usize = 12;
const MF_VIDEO_FLAGS: usize = 112;
const MF_SUBTYPE: usize = 120;

/// A COM `GUID`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// A subtype with the `MEDIASUBTYPE_` / `MFVideoFormat_` FourCC base GUID.
const fn mf_subtype(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    }
}

/// The DMO base GUID shared by the RGB `MEDIASUBTYPE_RGB*` values; `data1` is overridden.
const DMO_BASE: Guid = Guid {
    data1: 0,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

const MEDIATYPE_VIDEO: Guid = mf_subtype(0x7364_6976);
const MEDIASUBTYPE_AYUV: Guid = mf_subtype(0x5655_5941);
const MEDIASUBTYPE_I420: Guid = mf_subtype(0x3032_3449);
const MEDIASUBTYPE_IYUV: Guid = mf_subtype(0x5655_5949);
const MEDIASUBTYPE_NV11: Guid = mf_subtype(0x3131_564e);
const MEDIASUBTYPE_NV12: Guid = mf_subtype(0x3231_564e);
const MEDIASUBTYPE_UYVY: Guid = mf_subtype(0x5956_5955);
const MEDIASUBTYPE_V216: Guid = mf_subtype(0x3631_3256);
const MEDIASUBTYPE_V410: Guid = mf_subtype(0x3031_3456);
const MEDIASUBTYPE_Y41P: Guid = mf_subtype(0x5031_3459);
const MEDIASUBTYPE_Y41T: Guid = mf_subtype(0x5431_3459);
const MEDIASUBTYPE_Y42T: Guid = mf_subtype(0x5432_3459);
const MEDIASUBTYPE_YUY2: Guid = mf_subtype(0x3259_5559);
const MEDIASUBTYPE_YV12: Guid = mf_subtype(0x3231_5659);
const MEDIASUBTYPE_YVU9: Guid = mf_subtype(0x3955_5659);
const MEDIASUBTYPE_YVYU: Guid = mf_subtype(0x5559_5659);
const MEDIASUBTYPE_RGB8: Guid = Guid {
    data1: 0xe436_eb7a,
    ..DMO_BASE
};
const MEDIASUBTYPE_RGB555: Guid = Guid {
    data1: 0xe436_eb7c,
    ..DMO_BASE
};
const MEDIASUBTYPE_RGB565: Guid = Guid {
    data1: 0xe436_eb7b,
    ..DMO_BASE
};
const MEDIASUBTYPE_RGB24: Guid = Guid {
    data1: 0xe436_eb7d,
    ..DMO_BASE
};
const MEDIASUBTYPE_RGB32: Guid = Guid {
    data1: 0xe436_eb7e,
    ..DMO_BASE
};

const FORMAT_VIDEO_INFO: Guid = Guid {
    data1: 0x0558_9f80,
    data2: 0xc356,
    data3: 0x11ce,
    data4: [0xbf, 0x01, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a],
};
const FORMAT_VIDEO_INFO2: Guid = Guid {
    data1: 0xf72a_76a0,
    data2: 0xeb0a,
    data3: 0x11d0,
    data4: [0xac, 0xe4, 0x00, 0x00, 0xc0, 0xcc, 0x16, 0xba],
};
const FORMAT_MF_VIDEO_FORMAT: Guid = Guid {
    data1: 0xaed4_ab2d,
    data2: 0x7326,
    data3: 0x43cb,
    data4: [0x94, 0x64, 0xc8, 0x79, 0xca, 0xb9, 0xc4, 0x3d],
};

/// MF subtypes that the DMO RGB subtypes are normalised to, with the DMO subtype for each.
const DMO_SUBTYPES: [(Guid, Guid); 5] = [
    (mf_subtype(41), MEDIASUBTYPE_RGB8),
    (mf_subtype(24), MEDIASUBTYPE_RGB555),
    (mf_subtype(23), MEDIASUBTYPE_RGB565),
    (mf_subtype(20), MEDIASUBTYPE_RGB24),
    (mf_subtype(22), MEDIASUBTYPE_RGB32),
];

const CONVERTER_INPUT_TYPES: &[Guid] = &[
    MEDIASUBTYPE_YV12,
    MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_AYUV,
    MEDIASUBTYPE_NV12,
    MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB565,
    MEDIASUBTYPE_I420,
    MEDIASUBTYPE_IYUV,
    MEDIASUBTYPE_YVYU,
    MEDIASUBTYPE_RGB24,
    MEDIASUBTYPE_RGB555,
    MEDIASUBTYPE_RGB8,
    MEDIASUBTYPE_V216,
    MEDIASUBTYPE_V410,
    MEDIASUBTYPE_NV11,
    MEDIASUBTYPE_Y41P,
    MEDIASUBTYPE_Y41T,
    MEDIASUBTYPE_Y42T,
    MEDIASUBTYPE_YVU9,
];

const CONVERTER_OUTPUT_TYPES: &[Guid] = &[
    MEDIASUBTYPE_YV12,
    MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_AYUV,
    MEDIASUBTYPE_NV12,
    MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB565,
    MEDIASUBTYPE_I420,
    MEDIASUBTYPE_IYUV,
    MEDIASUBTYPE_YVYU,
    MEDIASUBTYPE_RGB24,
    MEDIASUBTYPE_RGB555,
    MEDIASUBTYPE_RGB8,
    MEDIASUBTYPE_V216,
    MEDIASUBTYPE_V410,
    MEDIASUBTYPE_NV11,
];

const RESIZER_INPUT_TYPES: &[Guid] = &[
    MEDIASUBTYPE_IYUV,
    MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_I420,
    MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB24,
    MEDIASUBTYPE_RGB565,
    MEDIASUBTYPE_RGB8,
    MEDIASUBTYPE_RGB555,
    MEDIASUBTYPE_AYUV,
    MEDIASUBTYPE_V216,
    MEDIASUBTYPE_YV12,
];

/// Wine's resizer output list is written out twice; the duplicate is kept so that the
/// enumeration order and count match the reference.
const RESIZER_OUTPUT_TYPES: &[Guid] = &[
    MEDIASUBTYPE_IYUV,
    MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_I420,
    MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB24,
    MEDIASUBTYPE_RGB565,
    MEDIASUBTYPE_RGB8,
    MEDIASUBTYPE_RGB555,
    MEDIASUBTYPE_AYUV,
    MEDIASUBTYPE_V216,
    MEDIASUBTYPE_YV12,
    MEDIASUBTYPE_IYUV,
    MEDIASUBTYPE_YUY2,
    MEDIASUBTYPE_UYVY,
    MEDIASUBTYPE_I420,
    MEDIASUBTYPE_RGB32,
    MEDIASUBTYPE_RGB24,
    MEDIASUBTYPE_RGB565,
    MEDIASUBTYPE_RGB8,
    MEDIASUBTYPE_RGB555,
    MEDIASUBTYPE_AYUV,
    MEDIASUBTYPE_V216,
    MEDIASUBTYPE_YV12,
];

/// Which converter DMO a subtype list belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConverterKind {
    Converter,
    Resizer,
}

impl ConverterKind {
    pub fn input_types(self) -> &'static [Guid] {
        match self {
            Self::Converter => CONVERTER_INPUT_TYPES,
            Self::Resizer => RESIZER_INPUT_TYPES,
        }
    }

    pub fn output_types(self) -> &'static [Guid] {
        match self {
            Self::Converter => CONVERTER_OUTPUT_TYPES,
            Self::Resizer => RESIZER_OUTPUT_TYPES,
        }
    }
}

/// The pixel layouts the converter can wrap, as libavutil names them.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Unsupported,
    Ayuv,
    Yuv420p,
    Yuv411p,
    Nv12,
    Uyvy422,
    Yuyv422,
    Yvyu422,
    Bgr24,
    Bgr0,
    Rgb555le,
    Rgb565le,
    Rgb8,
}

impl PixelFormat {
    pub fn name(self) -> Option<&'static str> {
        match self {
            Self::Unsupported => None,
            Self::Ayuv => Some("ayuv"),
            Self::Yuv420p => Some("yuv420p"),
            Self::Yuv411p => Some("yuv411p"),
            Self::Nv12 => Some("nv12"),
            Self::Uyvy422 => Some("uyvy422"),
            Self::Yuyv422 => Some("yuyv422"),
            Self::Yvyu422 => Some("yvyu422"),
            Self::Bgr24 => Some("bgr24"),
            Self::Bgr0 => Some("bgr0"),
            Self::Rgb555le => Some("rgb555le"),
            Self::Rgb565le => Some("rgb565le"),
            Self::Rgb8 => Some("rgb8"),
        }
    }
}

/// Maps a video subtype to a pixel layout. Only `Data1` is compared, as in Wine.
pub fn pixel_format_from_video_subtype(subtype: &Guid) -> PixelFormat {
    match subtype.data1 {
        0x5655_5941 => PixelFormat::Ayuv,
        0x3032_3449 | 0x5655_5949 => PixelFormat::Yuv420p,
        0x3131_564e => PixelFormat::Yuv411p,
        0x3231_564e => PixelFormat::Nv12,
        0x5956_5955 => PixelFormat::Uyvy422,
        0x3259_5559 => PixelFormat::Yuyv422,
        0x3231_5659 => PixelFormat::Yuv420p,
        0x5559_5659 => PixelFormat::Yvyu422,
        0xe436_eb7d => PixelFormat::Bgr24,
        0xe436_eb7e => PixelFormat::Bgr0,
        0xe436_eb7c => PixelFormat::Rgb555le,
        0xe436_eb7b => PixelFormat::Rgb565le,
        0xe436_eb7a => PixelFormat::Rgb8,
        _ => PixelFormat::Unsupported,
    }
}

/// The video fields of a converter format, as `init_video_format` fills them.
pub struct VideoFormat {
    pub width: u32,
    pub height: u32,
    pub video_flags: u64,
    pub subtype: Guid,
}

/// A `DMO_MEDIA_TYPE`; `cbFormat` is the length of `format`.
pub struct DmoMediaType<'a> {
    pub major_type: Guid,
    pub subtype: Guid,
    pub format_type: Guid,
    pub format: &'a [u8],
}

/// Maps an MF video subtype to the DMO subtype used by the converter DMOs.
pub fn get_dmo_subtype(subtype: &Guid) -> Guid {
    DMO_SUBTYPES
        .iter()
        .find(|(mf, _)| mf == subtype)
        .map_or(*subtype, |(_, dmo)| *dmo)
}

fn default_stride_sign(compression: u32) -> i32 {
    if compression == BI_RGB || compression == BI_BITFIELDS {
        -1
    } else {
        1
    }
}

/// Reads the format of a media type into a `VideoFormat`.
pub fn init_video_format(media: &DmoMediaType) -> Result<VideoFormat, HResult> {
    if media.format_type == FORMAT_VIDEO_INFO {
        return header_format(media, VIDEO_INFO_BMI, VIDEO_INFO_SIZE);
    }
    if media.format_type == FORMAT_VIDEO_INFO2 {
        return header_format(media, VIDEO_INFO2_BMI, VIDEO_INFO2_SIZE);
    }
    if media.format_type == FORMAT_MF_VIDEO_FORMAT {
        return mf_format(media);
    }
    Err(E_NOTIMPL)
}

fn header_format(media: &DmoMediaType, bmi: usize, size: usize) -> Result<VideoFormat, HResult> {
    let bytes = media.format;
    if bytes.len() < size {
        return Err(E_INVALIDARG);
    }
    let height = read_i32(bytes, bmi + BMI_HEIGHT);
    let bottom_up = default_stride_sign(read_u32(bytes, bmi + BMI_COMPRESSION)) < 0 && height > 0;
    Ok(VideoFormat {
        width: read_u32(bytes, bmi + BMI_WIDTH),
        height: height.unsigned_abs(),
        video_flags: if bottom_up { BOTTOM_UP_LINEAR_REP } else { 0 },
        subtype: media.subtype,
    })
}

fn mf_format(media: &DmoMediaType) -> Result<VideoFormat, HResult> {
    let bytes = media.format;
    if bytes.len() < MF_VIDEO_FORMAT_SIZE {
        return Err(E_INVALIDARG);
    }
    Ok(VideoFormat {
        width: read_u32(bytes, MF_WIDTH),
        height: read_u32(bytes, MF_HEIGHT),
        video_flags: read_u64(bytes, MF_VIDEO_FLAGS),
        subtype: get_dmo_subtype(&read_guid(bytes, MF_SUBTYPE)),
    })
}

/// Validates a media type for a converter output and returns the image size it implies.
/// `mf_image_size` stands in for `MFCalculateImageSize`.
pub fn check_dmo_media_type<F>(
    formats: &[Guid],
    media: &DmoMediaType,
    mf_image_size: F,
) -> Result<u32, HResult>
where
    F: FnOnce(&Guid, u32, u32) -> Result<u32, HResult>,
{
    if media.major_type != MEDIATYPE_VIDEO || !formats.contains(&media.subtype) {
        return Err(DMO_E_INVALIDTYPE);
    }
    let bytes = media.format;
    if media.format_type == FORMAT_VIDEO_INFO && bytes.len() >= VIDEO_INFO_SIZE {
        return header_image_size(bytes, VIDEO_INFO_BMI);
    }
    if media.format_type == FORMAT_VIDEO_INFO2 && bytes.len() >= VIDEO_INFO2_SIZE {
        return header_image_size(bytes, VIDEO_INFO2_BMI);
    }
    if media.format_type == FORMAT_MF_VIDEO_FORMAT && bytes.len() >= MF_VIDEO_FORMAT_SIZE {
        let width = read_u32(bytes, MF_WIDTH);
        let height = read_u32(bytes, MF_HEIGHT);
        if width == 0 || height == 0 {
            return Err(DMO_E_INVALIDTYPE);
        }
        return mf_image_size(&media.subtype, width, height);
    }
    Err(DMO_E_INVALIDTYPE)
}

fn header_image_size(bytes: &[u8], bmi: usize) -> Result<u32, HResult> {
    if read_i32(bytes, bmi + BMI_WIDTH) == 0 || read_i32(bytes, bmi + BMI_HEIGHT) == 0 {
        return Err(DMO_E_INVALIDTYPE);
    }
    Ok(read_u32(bytes, bmi + BMI_SIZE_IMAGE))
}

/// Validates an output media type, as `SetOutputType` does before it accepts the type.
/// A resizer only accepts an output with the subtype of its input.
pub fn check_output_type<F>(
    kind: ConverterKind,
    index: u32,
    input_subtype: &Guid,
    output: &DmoMediaType,
    mf_image_size: F,
) -> Result<u32, HResult>
where
    F: FnOnce(&Guid, u32, u32) -> Result<u32, HResult>,
{
    if index != 0 {
        return Err(DMO_E_INVALIDSTREAMINDEX);
    }
    if kind == ConverterKind::Resizer && *input_subtype != output.subtype {
        return Err(DMO_E_INVALIDTYPE);
    }
    check_dmo_media_type(kind.output_types(), output, mf_image_size)
}

/// Returns the subtype at `type_index` of a subtype list, as `GetInputType` does.
pub fn available_subtype(types: &[Guid], index: u32, type_index: u32) -> Result<Guid, HResult> {
    if index > 0 {
        return Err(DMO_E_INVALIDSTREAMINDEX);
    }
    types
        .get(type_index as usize)
        .copied()
        .ok_or(DMO_E_NO_MORE_ITEMS)
}

/// Maps a DMO HRESULT to the MF HRESULT that the MFT interface reports.
pub fn mf_result_from_dmo(hr: HResult) -> HResult {
    match hr {
        DMO_E_INVALIDSTREAMINDEX => MF_E_INVALIDSTREAMNUMBER,
        DMO_E_INVALIDTYPE => MF_E_INVALIDMEDIATYPE,
        DMO_E_TYPE_NOT_SET => MF_E_TRANSFORM_TYPE_NOT_SET,
        DMO_E_NOTACCEPTING => MF_E_NOTACCEPTING,
        DMO_E_TYPE_NOT_ACCEPTED => MF_E_INVALIDMEDIATYPE,
        DMO_E_NO_MORE_ITEMS => MF_E_NO_MORE_TYPES,
        S_FALSE => MF_E_TRANSFORM_NEED_MORE_INPUT,
        other => other,
    }
}

fn read_bytes<const N: usize>(bytes: &[u8], at: usize) -> [u8; N] {
    let mut out = [0; N];
    out.copy_from_slice(&bytes[at..at + N]);
    out
}

fn read_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(read_bytes(bytes, at))
}

fn read_i32(bytes: &[u8], at: usize) -> i32 {
    i32::from_le_bytes(read_bytes(bytes, at))
}

fn read_u64(bytes: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(read_bytes(bytes, at))
}

fn read_guid(bytes: &[u8], at: usize) -> Guid {
    Guid {
        data1: read_u32(bytes, at),
        data2: u16::from_le_bytes(read_bytes(bytes, at + 4)),
        data3: u16::from_le_bytes(read_bytes(bytes, at + 6)),
        data4: read_bytes(bytes, at + 8),
    }
}
