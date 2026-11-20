//! The format helpers of `dlls/evr/main.c` and `dlls/evr/mixer.c`.

const UYVY: u32 = u32::from_le_bytes(*b"UYVY");
const YUY2: u32 = u32::from_le_bytes(*b"YUY2");
const AYUV: u32 = u32::from_le_bytes(*b"AYUV");
const IMC1: u32 = u32::from_le_bytes(*b"IMC1");
const IMC2: u32 = u32::from_le_bytes(*b"IMC2");
const YV12: u32 = u32::from_le_bytes(*b"YV12");
const NV11: u32 = u32::from_le_bytes(*b"NV11");
const NV12: u32 = u32::from_le_bytes(*b"NV12");
const Y210: u32 = u32::from_le_bytes(*b"Y210");
const Y216: u32 = u32::from_le_bytes(*b"Y216");

/// `MFIsFormatYUV`: whether a FOURCC or D3DFORMAT code is one of the YUV formats the EVR accepts.
pub fn is_format_yuv(format: u32) -> bool {
    matches!(
        format,
        UYVY | YUY2 | AYUV | IMC1 | IMC2 | YV12 | NV11 | NV12 | Y210 | Y216
    )
}

/// `MFVideoInterlaceMode`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoInterlace {
    Progressive = 2,
    FieldInterleavedUpperFirst = 3,
    FieldInterleavedLowerFirst = 4,
    FieldSingleUpper = 5,
    FieldSingleLower = 6,
}

/// `video_mixer_get_interlace_mode_from_video_desc`: maps a `DXVA2_SampleFormat` value to the
/// interlace mode. The DXVA2 values 3 to 6 are the interleaved and single field formats, and
/// every other value is progressive.
pub fn interlace_mode(sample_format: u8) -> VideoInterlace {
    match sample_format {
        3 => VideoInterlace::FieldInterleavedUpperFirst,
        4 => VideoInterlace::FieldInterleavedLowerFirst,
        5 => VideoInterlace::FieldSingleUpper,
        6 => VideoInterlace::FieldSingleLower,
        _ => VideoInterlace::Progressive,
    }
}

/// An entry of the render target format table: the device that reports `format`.
#[derive(Debug, PartialEq, Eq)]
pub struct RtFormat<D> {
    pub device: D,
    pub format: u32,
}

/// `video_mixer_append_rt_format`: adds `format` for `device`, unless the table already holds
/// that format. The first device that reports a format owns it.
pub fn append_rt_format<D>(formats: &mut Vec<RtFormat<D>>, device: D, format: u32) {
    if formats.iter().all(|entry| entry.format != format) {
        formats.push(RtFormat { device, format });
    }
}
