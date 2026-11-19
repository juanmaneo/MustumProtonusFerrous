//! `ddsformat.c`: the header checks of `DdsDecoder_Initialize`, the legacy and DXGI format
//! tables, and the frame index arithmetic.

use crate::dxgi::*;
use crate::{
    HResult, WINCODEC_ERR_BADHEADER, WINCODEC_ERR_STREAMREAD, WINCODEC_ERR_UNKNOWNIMAGEFORMAT,
};

pub const DDS_MAGIC: u32 = 0x2053_4444;
pub const DDS_HEADER_SIZE: u32 = 124;
pub const DDPF_ALPHA: u32 = 0x2;
pub const DDPF_FOURCC: u32 = 0x4;
pub const DDPF_RGB: u32 = 0x40;
pub const DDPF_LUMINANCE: u32 = 0x2_0000;
pub const DDSCAPS2_CUBEMAP: u32 = 0x200;
pub const DDSCAPS2_VOLUME: u32 = 0x20_0000;
pub const DDS_RESOURCE_MISC_TEXTURECUBE: u32 = 0x4;
pub const DDS_DIMENSION_TEXTURE1D: u32 = 2;
pub const DDS_DIMENSION_TEXTURE2D: u32 = 3;
pub const DDS_DIMENSION_TEXTURE3D: u32 = 4;

pub const FOURCC_DXT1: u32 = u32::from_le_bytes(*b"DXT1");
pub const FOURCC_DXT2: u32 = u32::from_le_bytes(*b"DXT2");
pub const FOURCC_DXT3: u32 = u32::from_le_bytes(*b"DXT3");
pub const FOURCC_DXT4: u32 = u32::from_le_bytes(*b"DXT4");
pub const FOURCC_DXT5: u32 = u32::from_le_bytes(*b"DXT5");
pub const FOURCC_BC4U: u32 = u32::from_le_bytes(*b"BC4U");
pub const FOURCC_BC4S: u32 = u32::from_le_bytes(*b"BC4S");
pub const FOURCC_BC5U: u32 = u32::from_le_bytes(*b"BC5U");
pub const FOURCC_BC5S: u32 = u32::from_le_bytes(*b"BC5S");
pub const FOURCC_ATI1: u32 = u32::from_le_bytes(*b"ATI1");
pub const FOURCC_ATI2: u32 = u32::from_le_bytes(*b"ATI2");
pub const FOURCC_RGBG: u32 = u32::from_le_bytes(*b"RGBG");
pub const FOURCC_GRGB: u32 = u32::from_le_bytes(*b"GRGB");
pub const FOURCC_DX10: u32 = u32::from_le_bytes(*b"DX10");

/// `WICDdsDimension`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Dimension {
    #[default]
    Texture1D,
    Texture2D,
    Texture3D,
    Cube,
}

/// `WICDdsAlphaMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlphaMode {
    Unknown = 0,
    Straight = 1,
    Premultiplied = 2,
}

/// The WIC pixel formats that the DDS tables produce. Discriminants follow the order of the
/// `GUID_WICPixelFormat*` list in the oracle, so `as u32` gives the index in that list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PixelFormat {
    #[default]
    Undefined,
    Bpp32PBGRA,
    Bpp32BGRA,
    Bpp32FourChannels,
    Bpp64RGBA,
    Bpp16GrayHalf,
    Bpp64RGBAHalf,
    Bpp32GrayFloat,
    Bpp128RGBAFloat,
    Bpp32RGBA,
    Bpp32RGB,
    Bpp32BGR,
    Bpp32R10G10B10A2,
    Bpp32RGBA1010102,
    Bpp24BGR,
    Bpp24RGB,
    Bpp16BGR565,
    Bpp16BGR555,
    Bpp16BGRA5551,
    Bpp8Alpha,
    Bpp16Gray,
    Bpp8Gray,
    Bpp32RGBE,
    Bpp32RGBA1010102XR,
    Bpp96RGBFloat,
}

/// `DDS_PIXELFORMAT`, without its size field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DdsPixelFormat {
    pub flags: u32,
    pub four_cc: u32,
    pub rgb_bit_count: u32,
    pub r_mask: u32,
    pub g_mask: u32,
    pub b_mask: u32,
    pub a_mask: u32,
}

/// `DDS_HEADER`, without its flags, pitch, reserved and caps fields that the decoder does not read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DdsHeader {
    pub size: u32,
    pub height: u32,
    pub width: u32,
    pub depth: u32,
    pub mip_map_count: u32,
    pub pixel_format: DdsPixelFormat,
    pub caps2: u32,
}

/// `DDS_HEADER_DXT10`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DdsHeaderDxt10 {
    pub dxgi_format: DxgiFormat,
    pub resource_dimension: u32,
    pub misc_flag: u32,
    pub array_size: u32,
    pub misc_flags2: u32,
}

/// `dds_info`: what the decoder keeps after reading the header.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DdsInfo {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub frame_count: u32,
    pub data_offset: u32,
    pub bytes_per_block: u32,
    pub format: DxgiFormat,
    pub dimension: Dimension,
    pub alpha_mode: u32,
    pub pixel_format: PixelFormat,
    pub pixel_format_bpp: u32,
}

/// The array, mip and slice of one frame, as `get_frame_dds_index` reports them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameIndex {
    pub array_index: u32,
    pub mip_level: u32,
    pub slice_index: u32,
}

type LegacyRow = (u32, u32, u32, [u32; 4], PixelFormat, u32, DxgiFormat);

const NO_MASKS: [u32; 4] = [0; 4];

/// `dds_format_table`, the rows with a pixel format. Each row is (flags, fourCC, rgbBitCount,
/// masks, WIC format, WIC bits per pixel, DXGI format).
#[rustfmt::skip]
const LEGACY_FORMATS: [LegacyRow; 40] = [
    (DDPF_FOURCC, FOURCC_DXT1, 0, NO_MASKS, PixelFormat::Bpp32PBGRA, 32, DXGI_FORMAT_BC1_UNORM),
    (DDPF_FOURCC, FOURCC_DXT2, 0, NO_MASKS, PixelFormat::Bpp32PBGRA, 32, DXGI_FORMAT_BC2_UNORM),
    (DDPF_FOURCC, FOURCC_DXT3, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC2_UNORM),
    (DDPF_FOURCC, FOURCC_DXT4, 0, NO_MASKS, PixelFormat::Bpp32PBGRA, 32, DXGI_FORMAT_BC3_UNORM),
    (DDPF_FOURCC, FOURCC_DXT5, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC3_UNORM),
    (DDPF_FOURCC, FOURCC_BC4U, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC4_UNORM),
    (DDPF_FOURCC, FOURCC_BC4S, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC4_SNORM),
    (DDPF_FOURCC, FOURCC_BC5U, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC5_UNORM),
    (DDPF_FOURCC, FOURCC_BC5S, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC5_SNORM),
    (DDPF_FOURCC, FOURCC_ATI1, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC4_UNORM),
    (DDPF_FOURCC, FOURCC_ATI2, 0, NO_MASKS, PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_BC5_UNORM),
    (DDPF_FOURCC, FOURCC_RGBG, 0, NO_MASKS, PixelFormat::Bpp32FourChannels, 32, DXGI_FORMAT_R8G8_B8G8_UNORM),
    (DDPF_FOURCC, FOURCC_GRGB, 0, NO_MASKS, PixelFormat::Bpp32FourChannels, 32, DXGI_FORMAT_G8R8_G8B8_UNORM),
    (DDPF_FOURCC, FOURCC_DX10, 0, NO_MASKS, PixelFormat::Undefined, 0, DXGI_FORMAT_UNKNOWN),
    (DDPF_FOURCC, 0x24, 0, NO_MASKS, PixelFormat::Bpp64RGBA, 64, DXGI_FORMAT_R16G16B16A16_UNORM),
    (DDPF_FOURCC, 0x6E, 0, NO_MASKS, PixelFormat::Bpp64RGBA, 64, DXGI_FORMAT_R16G16B16A16_SNORM),
    (DDPF_FOURCC, 0x6F, 0, NO_MASKS, PixelFormat::Bpp16GrayHalf, 16, DXGI_FORMAT_R16_FLOAT),
    (DDPF_FOURCC, 0x70, 0, NO_MASKS, PixelFormat::Undefined, 0, DXGI_FORMAT_R16G16_FLOAT),
    (DDPF_FOURCC, 0x71, 0, NO_MASKS, PixelFormat::Bpp64RGBAHalf, 64, DXGI_FORMAT_R16G16B16A16_FLOAT),
    (DDPF_FOURCC, 0x72, 0, NO_MASKS, PixelFormat::Bpp32GrayFloat, 32, DXGI_FORMAT_R32_FLOAT),
    (DDPF_FOURCC, 0x73, 0, NO_MASKS, PixelFormat::Undefined, 32, DXGI_FORMAT_R32G32_FLOAT),
    (DDPF_FOURCC, 0x74, 0, NO_MASKS, PixelFormat::Bpp128RGBAFloat, 128, DXGI_FORMAT_R32G32B32A32_FLOAT),
    (DDPF_RGB, 0, 32, [0xff, 0xff00, 0xff_0000, 0xff00_0000], PixelFormat::Bpp32RGBA, 32, DXGI_FORMAT_R8G8B8A8_UNORM),
    (DDPF_RGB, 0, 32, [0xff, 0xff00, 0xff_0000, 0], PixelFormat::Bpp32RGB, 32, DXGI_FORMAT_UNKNOWN),
    (DDPF_RGB, 0, 32, [0xff_0000, 0xff00, 0xff, 0xff00_0000], PixelFormat::Bpp32BGRA, 32, DXGI_FORMAT_B8G8R8A8_UNORM),
    (DDPF_RGB, 0, 32, [0xff_0000, 0xff00, 0xff, 0], PixelFormat::Bpp32BGR, 32, DXGI_FORMAT_B8G8R8X8_UNORM),
    (DDPF_RGB, 0, 32, [0x3ff0_0000, 0xffc00, 0x3ff, 0xc000_0000], PixelFormat::Bpp32R10G10B10A2, 32, DXGI_FORMAT_R10G10B10A2_UNORM),
    (DDPF_RGB, 0, 32, [0x3ff, 0xffc00, 0x3ff0_0000, 0xc000_0000], PixelFormat::Bpp32RGBA1010102, 32, DXGI_FORMAT_R10G10B10A2_UNORM),
    (DDPF_RGB, 0, 32, [0xffff, 0xffff_0000, 0, 0], PixelFormat::Undefined, 0, DXGI_FORMAT_R16G16_UNORM),
    (DDPF_RGB, 0, 32, [0xffff_ffff, 0, 0, 0], PixelFormat::Bpp32GrayFloat, 32, DXGI_FORMAT_R32_FLOAT),
    (DDPF_RGB, 0, 24, [0xff_0000, 0xff00, 0xff, 0], PixelFormat::Bpp24BGR, 24, DXGI_FORMAT_UNKNOWN),
    (DDPF_RGB, 0, 24, [0xff, 0xff00, 0xff_0000, 0], PixelFormat::Bpp24RGB, 24, DXGI_FORMAT_UNKNOWN),
    (DDPF_RGB, 0, 16, [0xf800, 0x7e0, 0x1f, 0], PixelFormat::Bpp16BGR565, 16, DXGI_FORMAT_B5G6R5_UNORM),
    (DDPF_RGB, 0, 16, [0x7c00, 0x3e0, 0x1f, 0], PixelFormat::Bpp16BGR555, 16, DXGI_FORMAT_UNKNOWN),
    (DDPF_RGB, 0, 16, [0x7c00, 0x3e0, 0x1f, 0x8000], PixelFormat::Bpp16BGRA5551, 16, DXGI_FORMAT_B5G5R5A1_UNORM),
    (DDPF_RGB, 0, 16, [0xf00, 0xf0, 0xf, 0xf000], PixelFormat::Undefined, 0, DXGI_FORMAT_B4G4R4A4_UNORM),
    (DDPF_ALPHA, 0, 8, [0, 0, 0, 0xff], PixelFormat::Bpp8Alpha, 8, DXGI_FORMAT_A8_UNORM),
    (DDPF_LUMINANCE, 0, 16, [0xffff, 0, 0, 0], PixelFormat::Bpp16Gray, 16, DXGI_FORMAT_R16_UNORM),
    (DDPF_LUMINANCE, 0, 16, [0xff, 0, 0, 0xff00], PixelFormat::Undefined, 0, DXGI_FORMAT_R8G8_UNORM),
    (DDPF_LUMINANCE, 0, 8, [0xff, 0, 0, 0], PixelFormat::Bpp8Gray, 8, DXGI_FORMAT_R8_UNORM),
];

/// The size-0 rows of `dds_format_table`, which `dxgi_format_to_wic_format` searches.
const WIC_BY_DXGI: [(DxgiFormat, PixelFormat); 18] = [
    (DXGI_FORMAT_A8_UNORM, PixelFormat::Bpp8Alpha),
    (DXGI_FORMAT_R8_UNORM, PixelFormat::Bpp8Gray),
    (DXGI_FORMAT_R16_UNORM, PixelFormat::Bpp16Gray),
    (DXGI_FORMAT_R16_FLOAT, PixelFormat::Bpp16GrayHalf),
    (DXGI_FORMAT_B5G6R5_UNORM, PixelFormat::Bpp16BGR565),
    (DXGI_FORMAT_B5G5R5A1_UNORM, PixelFormat::Bpp16BGRA5551),
    (DXGI_FORMAT_R32_FLOAT, PixelFormat::Bpp32GrayFloat),
    (DXGI_FORMAT_R8G8B8A8_UNORM, PixelFormat::Bpp32RGBA),
    (DXGI_FORMAT_B8G8R8A8_UNORM, PixelFormat::Bpp32BGRA),
    (DXGI_FORMAT_B8G8R8X8_UNORM, PixelFormat::Bpp32BGR),
    (DXGI_FORMAT_R10G10B10A2_UNORM, PixelFormat::Bpp32R10G10B10A2),
    (DXGI_FORMAT_R9G9B9E5_SHAREDEXP, PixelFormat::Bpp32RGBE),
    (
        DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM,
        PixelFormat::Bpp32RGBA1010102XR,
    ),
    (DXGI_FORMAT_R16G16B16A16_UNORM, PixelFormat::Bpp64RGBA),
    (DXGI_FORMAT_R16G16B16A16_FLOAT, PixelFormat::Bpp64RGBAHalf),
    (DXGI_FORMAT_R32G32B32_FLOAT, PixelFormat::Bpp96RGBFloat),
    (DXGI_FORMAT_R32G32B32A32_FLOAT, PixelFormat::Bpp128RGBAFloat),
    (DXGI_FORMAT_UNKNOWN, PixelFormat::Undefined),
];

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// Reads the 124 bytes after the magic. Offsets are from the start of `DDS_HEADER`.
fn read_header(bytes: &[u8]) -> DdsHeader {
    DdsHeader {
        size: u32_at(bytes, 0),
        height: u32_at(bytes, 8),
        width: u32_at(bytes, 12),
        depth: u32_at(bytes, 20),
        mip_map_count: u32_at(bytes, 24),
        pixel_format: DdsPixelFormat {
            flags: u32_at(bytes, 76),
            four_cc: u32_at(bytes, 80),
            rgb_bit_count: u32_at(bytes, 84),
            r_mask: u32_at(bytes, 88),
            g_mask: u32_at(bytes, 92),
            b_mask: u32_at(bytes, 96),
            a_mask: u32_at(bytes, 100),
        },
        caps2: u32_at(bytes, 108),
    }
}

fn read_dxt10(bytes: &[u8]) -> DdsHeaderDxt10 {
    DdsHeaderDxt10 {
        dxgi_format: u32_at(bytes, 0),
        resource_dimension: u32_at(bytes, 4),
        misc_flag: u32_at(bytes, 8),
        array_size: u32_at(bytes, 12),
        misc_flags2: u32_at(bytes, 16),
    }
}

fn has_extended_header(header: &DdsHeader) -> bool {
    (header.pixel_format.flags & DDPF_FOURCC) != 0 && header.pixel_format.four_cc == FOURCC_DX10
}

/// The checks `DdsDecoder_Initialize` makes on a DDS file, in the order it makes them. Returns the
/// `dds_info` of a file it accepts, or the `HRESULT` it reports.
pub fn parse_header(file: &[u8]) -> Result<DdsInfo, HResult> {
    let magic = file.get(..4).ok_or(WINCODEC_ERR_STREAMREAD)?;
    if u32_at(magic, 0) != DDS_MAGIC {
        return Err(WINCODEC_ERR_UNKNOWNIMAGEFORMAT);
    }
    let header = read_header(file.get(4..128).ok_or(WINCODEC_ERR_STREAMREAD)?);
    if header.size != DDS_HEADER_SIZE {
        return Err(WINCODEC_ERR_BADHEADER);
    }
    let dxt10 = if has_extended_header(&header) {
        Some(read_dxt10(
            file.get(128..148).ok_or(WINCODEC_ERR_STREAMREAD)?,
        ))
    } else {
        None
    };
    let info = dds_info(&header, dxt10.as_ref());
    if info.dimension == Dimension::Cube
        || !matches!(
            info.format,
            DXGI_FORMAT_BC1_UNORM | DXGI_FORMAT_BC2_UNORM | DXGI_FORMAT_BC3_UNORM
        )
    {
        return Err(WINCODEC_ERR_BADHEADER);
    }
    Ok(info)
}

/// `get_dimension`.
pub fn dimension(header: &DdsHeader, dxt10: Option<&DdsHeaderDxt10>) -> Dimension {
    match dxt10 {
        Some(ext) if (ext.misc_flag & DDS_RESOURCE_MISC_TEXTURECUBE) != 0 => Dimension::Cube,
        Some(ext) => match ext.resource_dimension {
            DDS_DIMENSION_TEXTURE1D => Dimension::Texture1D,
            DDS_DIMENSION_TEXTURE2D => Dimension::Texture2D,
            DDS_DIMENSION_TEXTURE3D => Dimension::Texture3D,
            _ => Dimension::Texture2D,
        },
        None if (header.caps2 & DDSCAPS2_CUBEMAP) != 0 => Dimension::Cube,
        None if (header.caps2 & DDSCAPS2_VOLUME) != 0 => Dimension::Texture3D,
        None => Dimension::Texture2D,
    }
}

/// `get_alpha_mode_from_fourcc`.
pub fn alpha_mode_from_fourcc(four_cc: u32) -> AlphaMode {
    match four_cc {
        FOURCC_DXT1 | FOURCC_DXT2 | FOURCC_DXT4 => AlphaMode::Premultiplied,
        _ => AlphaMode::Unknown,
    }
}

/// `get_frame_count`: the frames of every mip of every array slice, with six faces for a cube.
pub fn frame_count(depth: u32, mip_levels: u32, array_size: u32, dimension: Dimension) -> u32 {
    let mut frames = 0_u32;
    let mut slices = depth;
    for _ in 0..mip_levels {
        frames = frames.wrapping_add(slices);
        if slices > 1 {
            slices /= 2;
        }
    }
    let frames = frames.wrapping_mul(array_size);
    if dimension == Dimension::Cube {
        frames.wrapping_mul(6)
    } else {
        frames
    }
}

/// `get_frame_dds_index`. Returns `None` where the C divides by zero or loops forever: a zero
/// array size, a zero per-texture frame count, or a zero depth.
pub fn frame_dds_index(index: u32, info: &DdsInfo) -> Option<FrameIndex> {
    let per_texture = if info.dimension == Dimension::Cube {
        info.mip_levels
    } else {
        info.frame_count.checked_div(info.array_size)?
    };
    if per_texture == 0 || info.depth == 0 {
        return None;
    }
    let mut slice_index = index % per_texture;
    let mut depth = info.depth;
    let mut mip_level = 0;
    while slice_index >= depth {
        slice_index -= depth;
        mip_level += 1;
        if depth > 1 {
            depth /= 2;
        }
    }
    Some(FrameIndex {
        array_index: index / per_texture,
        mip_level,
        slice_index,
    })
}

fn legacy_format(pixel_format: &DdsPixelFormat) -> (DxgiFormat, PixelFormat, u32) {
    LEGACY_FORMATS
        .iter()
        .find(|row| {
            (pixel_format.flags & row.0) != 0
                && pixel_format.four_cc == row.1
                && pixel_format.rgb_bit_count == row.2
                && [
                    pixel_format.r_mask,
                    pixel_format.g_mask,
                    pixel_format.b_mask,
                    pixel_format.a_mask,
                ] == row.3
        })
        .map_or((DXGI_FORMAT_UNKNOWN, PixelFormat::Undefined, 0), |row| {
            (row.6, row.4, row.5)
        })
}

/// `dxgi_format_to_wic_format`.
pub fn dxgi_to_wic(format: DxgiFormat) -> PixelFormat {
    WIC_BY_DXGI
        .iter()
        .find(|row| row.0 == format)
        .map_or(PixelFormat::Undefined, |row| row.1)
}

/// `get_dds_info`.
pub fn dds_info(header: &DdsHeader, dxt10: Option<&DdsHeaderDxt10>) -> DdsInfo {
    let mut info = DdsInfo {
        width: header.width,
        height: header.height,
        depth: header.depth.max(1),
        mip_levels: header.mip_map_count.max(1),
        array_size: 1,
        ..DdsInfo::default()
    };
    match dxt10 {
        Some(ext) if has_extended_header(header) => {
            info.array_size = ext.array_size.max(1);
            info.format = ext.dxgi_format;
            info.dimension = dimension(header, Some(ext));
            info.alpha_mode = ext.misc_flags2 & 0x8;
            info.data_offset = 148;
            if is_compressed(ext.dxgi_format) {
                info.pixel_format = PixelFormat::Bpp32BGRA;
                info.pixel_format_bpp = 32;
            } else {
                info.pixel_format = dxgi_to_wic(ext.dxgi_format);
                info.pixel_format_bpp = bytes_per_block_from_format(ext.dxgi_format) * 8;
            }
        }
        _ => {
            let (format, pixel_format, bpp) = legacy_format(&header.pixel_format);
            info.format = format;
            info.dimension = dimension(header, None);
            info.alpha_mode = alpha_mode_from_fourcc(header.pixel_format.four_cc) as u32;
            info.data_offset = 128;
            info.pixel_format = pixel_format;
            info.pixel_format_bpp = bpp;
        }
    }
    info.bytes_per_block =
        if (header.pixel_format.flags & (DDPF_RGB | DDPF_ALPHA | DDPF_LUMINANCE)) != 0 {
            header.pixel_format.rgb_bit_count / 8
        } else {
            bytes_per_block_from_format(info.format)
        };
    info.frame_count = frame_count(info.depth, info.mip_levels, info.array_size, info.dimension);
    info
}

/// `rgb565_to_argb`: expands a 5-6-5 colour to 8 bits per channel, with the given alpha.
pub fn rgb565_to_argb(color: u16, alpha: u8) -> u32 {
    let color = u32::from(color);
    let r = (((color >> 11) & 0x1f) * 0xff + 0x0f) / 0x1f;
    let g = (((color >> 5) & 0x3f) * 0xff + 0x1f) / 0x3f;
    let b = ((color & 0x1f) * 0xff + 0x0f) / 0x1f;
    (u32::from(alpha) << 24) | (r << 16) | (g << 8) | b
}
