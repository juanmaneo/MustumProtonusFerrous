//! DXGI formats from `include/dxgiformat.idl`, and the format classes that
//! `dlls/windowscodecs/ddsformat.c` keys on. Generated from the Wine sources.

/// A DXGI format value, as stored in a DDS `DDS_HEADER_DXT10`.
pub type DxgiFormat = u32;

pub const DXGI_FORMAT_UNKNOWN: DxgiFormat = 0x00;
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DxgiFormat = 0x01;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DxgiFormat = 0x02;
pub const DXGI_FORMAT_R32G32B32A32_UINT: DxgiFormat = 0x03;
pub const DXGI_FORMAT_R32G32B32A32_SINT: DxgiFormat = 0x04;
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DxgiFormat = 0x05;
pub const DXGI_FORMAT_R32G32B32_FLOAT: DxgiFormat = 0x06;
pub const DXGI_FORMAT_R32G32B32_UINT: DxgiFormat = 0x07;
pub const DXGI_FORMAT_R32G32B32_SINT: DxgiFormat = 0x08;
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DxgiFormat = 0x09;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DxgiFormat = 0x0a;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DxgiFormat = 0x0b;
pub const DXGI_FORMAT_R16G16B16A16_UINT: DxgiFormat = 0x0c;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DxgiFormat = 0x0d;
pub const DXGI_FORMAT_R16G16B16A16_SINT: DxgiFormat = 0x0e;
pub const DXGI_FORMAT_R32G32_TYPELESS: DxgiFormat = 0x0f;
pub const DXGI_FORMAT_R32G32_FLOAT: DxgiFormat = 0x10;
pub const DXGI_FORMAT_R32G32_UINT: DxgiFormat = 0x11;
pub const DXGI_FORMAT_R32G32_SINT: DxgiFormat = 0x12;
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DxgiFormat = 0x13;
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DxgiFormat = 0x14;
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DxgiFormat = 0x15;
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DxgiFormat = 0x16;
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DxgiFormat = 0x17;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DxgiFormat = 0x18;
pub const DXGI_FORMAT_R10G10B10A2_UINT: DxgiFormat = 0x19;
pub const DXGI_FORMAT_R11G11B10_FLOAT: DxgiFormat = 0x1a;
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DxgiFormat = 0x1b;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DxgiFormat = 0x1c;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DxgiFormat = 0x1d;
pub const DXGI_FORMAT_R8G8B8A8_UINT: DxgiFormat = 0x1e;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DxgiFormat = 0x1f;
pub const DXGI_FORMAT_R8G8B8A8_SINT: DxgiFormat = 0x20;
pub const DXGI_FORMAT_R16G16_TYPELESS: DxgiFormat = 0x21;
pub const DXGI_FORMAT_R16G16_FLOAT: DxgiFormat = 0x22;
pub const DXGI_FORMAT_R16G16_UNORM: DxgiFormat = 0x23;
pub const DXGI_FORMAT_R16G16_UINT: DxgiFormat = 0x24;
pub const DXGI_FORMAT_R16G16_SNORM: DxgiFormat = 0x25;
pub const DXGI_FORMAT_R16G16_SINT: DxgiFormat = 0x26;
pub const DXGI_FORMAT_R32_TYPELESS: DxgiFormat = 0x27;
pub const DXGI_FORMAT_D32_FLOAT: DxgiFormat = 0x28;
pub const DXGI_FORMAT_R32_FLOAT: DxgiFormat = 0x29;
pub const DXGI_FORMAT_R32_UINT: DxgiFormat = 0x2a;
pub const DXGI_FORMAT_R32_SINT: DxgiFormat = 0x2b;
pub const DXGI_FORMAT_R24G8_TYPELESS: DxgiFormat = 0x2c;
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DxgiFormat = 0x2d;
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DxgiFormat = 0x2e;
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DxgiFormat = 0x2f;
pub const DXGI_FORMAT_R8G8_TYPELESS: DxgiFormat = 0x30;
pub const DXGI_FORMAT_R8G8_UNORM: DxgiFormat = 0x31;
pub const DXGI_FORMAT_R8G8_UINT: DxgiFormat = 0x32;
pub const DXGI_FORMAT_R8G8_SNORM: DxgiFormat = 0x33;
pub const DXGI_FORMAT_R8G8_SINT: DxgiFormat = 0x34;
pub const DXGI_FORMAT_R16_TYPELESS: DxgiFormat = 0x35;
pub const DXGI_FORMAT_R16_FLOAT: DxgiFormat = 0x36;
pub const DXGI_FORMAT_D16_UNORM: DxgiFormat = 0x37;
pub const DXGI_FORMAT_R16_UNORM: DxgiFormat = 0x38;
pub const DXGI_FORMAT_R16_UINT: DxgiFormat = 0x39;
pub const DXGI_FORMAT_R16_SNORM: DxgiFormat = 0x3a;
pub const DXGI_FORMAT_R16_SINT: DxgiFormat = 0x3b;
pub const DXGI_FORMAT_R8_TYPELESS: DxgiFormat = 0x3c;
pub const DXGI_FORMAT_R8_UNORM: DxgiFormat = 0x3d;
pub const DXGI_FORMAT_R8_UINT: DxgiFormat = 0x3e;
pub const DXGI_FORMAT_R8_SNORM: DxgiFormat = 0x3f;
pub const DXGI_FORMAT_R8_SINT: DxgiFormat = 0x40;
pub const DXGI_FORMAT_A8_UNORM: DxgiFormat = 0x41;
pub const DXGI_FORMAT_R1_UNORM: DxgiFormat = 0x42;
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DxgiFormat = 0x43;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DxgiFormat = 0x44;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DxgiFormat = 0x45;
pub const DXGI_FORMAT_BC1_TYPELESS: DxgiFormat = 0x46;
pub const DXGI_FORMAT_BC1_UNORM: DxgiFormat = 0x47;
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DxgiFormat = 0x48;
pub const DXGI_FORMAT_BC2_TYPELESS: DxgiFormat = 0x49;
pub const DXGI_FORMAT_BC2_UNORM: DxgiFormat = 0x4a;
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DxgiFormat = 0x4b;
pub const DXGI_FORMAT_BC3_TYPELESS: DxgiFormat = 0x4c;
pub const DXGI_FORMAT_BC3_UNORM: DxgiFormat = 0x4d;
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DxgiFormat = 0x4e;
pub const DXGI_FORMAT_BC4_TYPELESS: DxgiFormat = 0x4f;
pub const DXGI_FORMAT_BC4_UNORM: DxgiFormat = 0x50;
pub const DXGI_FORMAT_BC4_SNORM: DxgiFormat = 0x51;
pub const DXGI_FORMAT_BC5_TYPELESS: DxgiFormat = 0x52;
pub const DXGI_FORMAT_BC5_UNORM: DxgiFormat = 0x53;
pub const DXGI_FORMAT_BC5_SNORM: DxgiFormat = 0x54;
pub const DXGI_FORMAT_B5G6R5_UNORM: DxgiFormat = 0x55;
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DxgiFormat = 0x56;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DxgiFormat = 0x57;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DxgiFormat = 0x58;
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DxgiFormat = 0x59;
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DxgiFormat = 0x5a;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DxgiFormat = 0x5b;
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DxgiFormat = 0x5c;
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DxgiFormat = 0x5d;
pub const DXGI_FORMAT_BC6H_TYPELESS: DxgiFormat = 0x5e;
pub const DXGI_FORMAT_BC6H_UF16: DxgiFormat = 0x5f;
pub const DXGI_FORMAT_BC6H_SF16: DxgiFormat = 0x60;
pub const DXGI_FORMAT_BC7_TYPELESS: DxgiFormat = 0x61;
pub const DXGI_FORMAT_BC7_UNORM: DxgiFormat = 0x62;
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DxgiFormat = 0x63;
pub const DXGI_FORMAT_AYUV: DxgiFormat = 0x64;
pub const DXGI_FORMAT_Y410: DxgiFormat = 0x65;
pub const DXGI_FORMAT_Y416: DxgiFormat = 0x66;
pub const DXGI_FORMAT_NV12: DxgiFormat = 0x67;
pub const DXGI_FORMAT_P010: DxgiFormat = 0x68;
pub const DXGI_FORMAT_P016: DxgiFormat = 0x69;
pub const DXGI_FORMAT_420_OPAQUE: DxgiFormat = 0x6a;
pub const DXGI_FORMAT_YUY2: DxgiFormat = 0x6b;
pub const DXGI_FORMAT_Y210: DxgiFormat = 0x6c;
pub const DXGI_FORMAT_Y216: DxgiFormat = 0x6d;
pub const DXGI_FORMAT_NV11: DxgiFormat = 0x6e;
pub const DXGI_FORMAT_AI44: DxgiFormat = 0x6f;
pub const DXGI_FORMAT_IA44: DxgiFormat = 0x70;
pub const DXGI_FORMAT_P8: DxgiFormat = 0x71;
pub const DXGI_FORMAT_A8P8: DxgiFormat = 0x72;
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DxgiFormat = 0x73;
pub const DXGI_FORMAT_P208: DxgiFormat = 0x82;
pub const DXGI_FORMAT_V208: DxgiFormat = 0x83;
pub const DXGI_FORMAT_V408: DxgiFormat = 0x84;

/// `get_bytes_per_block_from_format` in `ddsformat.c`. Formats the C switch does not list
/// return 0. The C `WARN` for them is not ported.
pub fn bytes_per_block_from_format(format: DxgiFormat) -> u32 {
    match format {
        DXGI_FORMAT_R8_TYPELESS
        | DXGI_FORMAT_R8_UNORM
        | DXGI_FORMAT_R8_UINT
        | DXGI_FORMAT_R8_SNORM
        | DXGI_FORMAT_R8_SINT
        | DXGI_FORMAT_A8_UNORM => 1,
        DXGI_FORMAT_R8G8_TYPELESS
        | DXGI_FORMAT_R8G8_UNORM
        | DXGI_FORMAT_R8G8_UINT
        | DXGI_FORMAT_R8G8_SNORM
        | DXGI_FORMAT_R8G8_SINT
        | DXGI_FORMAT_R16_TYPELESS
        | DXGI_FORMAT_R16_FLOAT
        | DXGI_FORMAT_D16_UNORM
        | DXGI_FORMAT_R16_UNORM
        | DXGI_FORMAT_R16_UINT
        | DXGI_FORMAT_R16_SNORM
        | DXGI_FORMAT_R16_SINT
        | DXGI_FORMAT_B5G6R5_UNORM
        | DXGI_FORMAT_B5G5R5A1_UNORM
        | DXGI_FORMAT_B4G4R4A4_UNORM => 2,
        DXGI_FORMAT_R10G10B10A2_TYPELESS
        | DXGI_FORMAT_R10G10B10A2_UNORM
        | DXGI_FORMAT_R10G10B10A2_UINT
        | DXGI_FORMAT_R11G11B10_FLOAT
        | DXGI_FORMAT_R8G8B8A8_TYPELESS
        | DXGI_FORMAT_R8G8B8A8_UNORM
        | DXGI_FORMAT_R8G8B8A8_UNORM_SRGB
        | DXGI_FORMAT_R8G8B8A8_UINT
        | DXGI_FORMAT_R8G8B8A8_SNORM
        | DXGI_FORMAT_R8G8B8A8_SINT
        | DXGI_FORMAT_R16G16_TYPELESS
        | DXGI_FORMAT_R16G16_FLOAT
        | DXGI_FORMAT_R16G16_UNORM
        | DXGI_FORMAT_R16G16_UINT
        | DXGI_FORMAT_R16G16_SNORM
        | DXGI_FORMAT_R16G16_SINT
        | DXGI_FORMAT_R32_TYPELESS
        | DXGI_FORMAT_D32_FLOAT
        | DXGI_FORMAT_R32_FLOAT
        | DXGI_FORMAT_R32_UINT
        | DXGI_FORMAT_R32_SINT
        | DXGI_FORMAT_R24G8_TYPELESS
        | DXGI_FORMAT_D24_UNORM_S8_UINT
        | DXGI_FORMAT_R24_UNORM_X8_TYPELESS
        | DXGI_FORMAT_X24_TYPELESS_G8_UINT
        | DXGI_FORMAT_R9G9B9E5_SHAREDEXP
        | DXGI_FORMAT_R8G8_B8G8_UNORM
        | DXGI_FORMAT_G8R8_G8B8_UNORM
        | DXGI_FORMAT_B8G8R8A8_UNORM
        | DXGI_FORMAT_B8G8R8X8_UNORM
        | DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM
        | DXGI_FORMAT_B8G8R8A8_TYPELESS
        | DXGI_FORMAT_B8G8R8A8_UNORM_SRGB
        | DXGI_FORMAT_B8G8R8X8_TYPELESS
        | DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => 4,
        DXGI_FORMAT_BC1_UNORM
        | DXGI_FORMAT_BC1_TYPELESS
        | DXGI_FORMAT_BC1_UNORM_SRGB
        | DXGI_FORMAT_BC4_TYPELESS
        | DXGI_FORMAT_BC4_UNORM
        | DXGI_FORMAT_BC4_SNORM
        | DXGI_FORMAT_R16G16B16A16_TYPELESS
        | DXGI_FORMAT_R16G16B16A16_FLOAT
        | DXGI_FORMAT_R16G16B16A16_UNORM
        | DXGI_FORMAT_R16G16B16A16_UINT
        | DXGI_FORMAT_R16G16B16A16_SNORM
        | DXGI_FORMAT_R16G16B16A16_SINT
        | DXGI_FORMAT_R32G32_TYPELESS
        | DXGI_FORMAT_R32G32_FLOAT
        | DXGI_FORMAT_R32G32_UINT
        | DXGI_FORMAT_R32G32_SINT
        | DXGI_FORMAT_R32G8X24_TYPELESS
        | DXGI_FORMAT_D32_FLOAT_S8X24_UINT
        | DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS
        | DXGI_FORMAT_X32_TYPELESS_G8X24_UINT => 8,
        DXGI_FORMAT_R32G32B32_TYPELESS
        | DXGI_FORMAT_R32G32B32_FLOAT
        | DXGI_FORMAT_R32G32B32_UINT
        | DXGI_FORMAT_R32G32B32_SINT => 12,
        DXGI_FORMAT_BC2_UNORM
        | DXGI_FORMAT_BC2_TYPELESS
        | DXGI_FORMAT_BC2_UNORM_SRGB
        | DXGI_FORMAT_BC3_UNORM
        | DXGI_FORMAT_BC3_TYPELESS
        | DXGI_FORMAT_BC3_UNORM_SRGB
        | DXGI_FORMAT_BC5_TYPELESS
        | DXGI_FORMAT_BC5_UNORM
        | DXGI_FORMAT_BC5_SNORM
        | DXGI_FORMAT_BC6H_TYPELESS
        | DXGI_FORMAT_BC6H_UF16
        | DXGI_FORMAT_BC6H_SF16
        | DXGI_FORMAT_BC7_TYPELESS
        | DXGI_FORMAT_BC7_UNORM
        | DXGI_FORMAT_BC7_UNORM_SRGB
        | DXGI_FORMAT_R32G32B32A32_TYPELESS
        | DXGI_FORMAT_R32G32B32A32_FLOAT
        | DXGI_FORMAT_R32G32B32A32_UINT
        | DXGI_FORMAT_R32G32B32A32_SINT => 16,
        _ => 0,
    }
}

/// `is_compressed` in `ddsformat.c`: the block-compressed formats the DDS decoder accepts.
pub fn is_compressed(format: DxgiFormat) -> bool {
    matches!(
        format,
        DXGI_FORMAT_BC1_TYPELESS
            | DXGI_FORMAT_BC1_UNORM
            | DXGI_FORMAT_BC1_UNORM_SRGB
            | DXGI_FORMAT_BC2_TYPELESS
            | DXGI_FORMAT_BC2_UNORM
            | DXGI_FORMAT_BC2_UNORM_SRGB
            | DXGI_FORMAT_BC3_TYPELESS
            | DXGI_FORMAT_BC3_UNORM
            | DXGI_FORMAT_BC3_UNORM_SRGB
            | DXGI_FORMAT_BC4_TYPELESS
            | DXGI_FORMAT_BC4_UNORM
            | DXGI_FORMAT_BC4_SNORM
            | DXGI_FORMAT_BC5_TYPELESS
            | DXGI_FORMAT_BC5_UNORM
            | DXGI_FORMAT_BC5_SNORM
            | DXGI_FORMAT_BC6H_TYPELESS
            | DXGI_FORMAT_BC6H_UF16
            | DXGI_FORMAT_BC6H_SF16
            | DXGI_FORMAT_BC7_TYPELESS
            | DXGI_FORMAT_BC7_UNORM
            | DXGI_FORMAT_BC7_UNORM_SRGB
    )
}
