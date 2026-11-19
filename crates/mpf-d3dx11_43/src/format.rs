//! Pixel format mapping from `texture.c`: `enum d3dx_pixel_format_id` to `DXGI_FORMAT`.
//!
//! Variant names follow the C identifiers in `dlls/d3dx9_36/d3dx_helpers.h`. The C aliases
//! `D3DX_PIXEL_FORMAT_BC1/2/3_UNORM`, `R8G8B8A8_SNORM`, `R8G8_SNORM`, `R16G16_SNORM` and
//! `R16G16B16A16_SNORM` name the variants `Dxt1Unorm`, `Dxt3Unorm`, `Dxt5Unorm`,
//! `U8v8w8q8Snorm`, `U8v8Snorm`, `U16v16Snorm` and `U16v16w16q16Snorm` here.

/// `DXGI_FORMAT`, as the numeric values of `include/dxgiformat.idl`.
pub type DxgiFormat = u32;

pub const DXGI_FORMAT_UNKNOWN: DxgiFormat = 0x00;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DxgiFormat = 0x02;
pub const DXGI_FORMAT_R32G32B32_FLOAT: DxgiFormat = 0x06;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DxgiFormat = 0x0a;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DxgiFormat = 0x0b;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DxgiFormat = 0x0d;
pub const DXGI_FORMAT_R32G32_FLOAT: DxgiFormat = 0x10;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DxgiFormat = 0x18;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DxgiFormat = 0x1c;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DxgiFormat = 0x1f;
pub const DXGI_FORMAT_R16G16_FLOAT: DxgiFormat = 0x22;
pub const DXGI_FORMAT_R16G16_UNORM: DxgiFormat = 0x23;
pub const DXGI_FORMAT_R16G16_SNORM: DxgiFormat = 0x25;
pub const DXGI_FORMAT_R32_FLOAT: DxgiFormat = 0x29;
pub const DXGI_FORMAT_R8G8_UNORM: DxgiFormat = 0x31;
pub const DXGI_FORMAT_R8G8_SNORM: DxgiFormat = 0x33;
pub const DXGI_FORMAT_R16_FLOAT: DxgiFormat = 0x36;
pub const DXGI_FORMAT_R16_UNORM: DxgiFormat = 0x38;
pub const DXGI_FORMAT_R8_UNORM: DxgiFormat = 0x3d;
pub const DXGI_FORMAT_A8_UNORM: DxgiFormat = 0x41;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DxgiFormat = 0x44;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DxgiFormat = 0x45;
pub const DXGI_FORMAT_BC1_UNORM: DxgiFormat = 0x47;
pub const DXGI_FORMAT_BC2_UNORM: DxgiFormat = 0x4a;
pub const DXGI_FORMAT_BC3_UNORM: DxgiFormat = 0x4d;
pub const DXGI_FORMAT_BC4_UNORM: DxgiFormat = 0x50;
pub const DXGI_FORMAT_BC4_SNORM: DxgiFormat = 0x51;
pub const DXGI_FORMAT_BC5_UNORM: DxgiFormat = 0x53;
pub const DXGI_FORMAT_BC5_SNORM: DxgiFormat = 0x54;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DxgiFormat = 0x57;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DxgiFormat = 0x58;

/// `enum d3dx_pixel_format_id`, without the `D3DX_PIXEL_FORMAT_COUNT` sentinel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3dxPixelFormatId {
    B8g8r8Unorm,
    B8g8r8a8Unorm,
    B8g8r8x8Unorm,
    R8g8b8a8Unorm,
    R8g8b8x8Unorm,
    B5g6r5Unorm,
    B5g5r5x1Unorm,
    B5g5r5a1Unorm,
    B2g3r3Unorm,
    B2g3r3a8Unorm,
    B4g4r4a4Unorm,
    B4g4r4x4Unorm,
    B10g10r10a2Unorm,
    R10g10b10a2Unorm,
    R16g16b16Unorm,
    R16g16b16a16Unorm,
    R8Unorm,
    R8Snorm,
    R8g8Unorm,
    R16Unorm,
    R16g16Unorm,
    A8Unorm,
    L8a8Unorm,
    L4a4Unorm,
    L8Unorm,
    L16Unorm,
    Dxt1Unorm,
    Dxt2Unorm,
    Dxt3Unorm,
    Dxt4Unorm,
    Dxt5Unorm,
    Bc4Unorm,
    Bc4Snorm,
    Bc5Unorm,
    Bc5Snorm,
    R16Float,
    R16g16Float,
    R16g16b16a16Float,
    R32Float,
    R32g32Float,
    R32g32b32Float,
    R32g32b32a32Float,
    P1Uint,
    P2Uint,
    P4Uint,
    P8Uint,
    P8UintA8Unorm,
    U8v8w8q8Snorm,
    U16v16w16q16Snorm,
    U8v8Snorm,
    U8v8SnormCx,
    U16v16Snorm,
    U8v8SnormL8x8Unorm,
    U10v10w10SnormA2Unorm,
    R8g8B8g8Unorm,
    G8r8G8b8Unorm,
    Uyvy,
    Yuy2,
}

/// `dxgi_format_from_d3dx_pixel_format_id`: the `DXGI_FORMAT` of a DDS_DXT10 pixel format.
/// Returns `DXGI_FORMAT_UNKNOWN` for formats without a DXGI equivalent.
pub fn dxgi_format_from_d3dx_pixel_format_id(format: D3dxPixelFormatId) -> DxgiFormat {
    use D3dxPixelFormatId::*;

    match format {
        R8g8b8a8Unorm => DXGI_FORMAT_R8G8B8A8_UNORM,
        B8g8r8a8Unorm => DXGI_FORMAT_B8G8R8A8_UNORM,
        B8g8r8x8Unorm => DXGI_FORMAT_B8G8R8X8_UNORM,
        R10g10b10a2Unorm => DXGI_FORMAT_R10G10B10A2_UNORM,
        R16g16b16a16Unorm => DXGI_FORMAT_R16G16B16A16_UNORM,
        R8Unorm => DXGI_FORMAT_R8_UNORM,
        R8g8Unorm => DXGI_FORMAT_R8G8_UNORM,
        R16Unorm => DXGI_FORMAT_R16_UNORM,
        R16g16Unorm => DXGI_FORMAT_R16G16_UNORM,
        A8Unorm => DXGI_FORMAT_A8_UNORM,
        R16Float => DXGI_FORMAT_R16_FLOAT,
        R16g16Float => DXGI_FORMAT_R16G16_FLOAT,
        R16g16b16a16Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
        R32Float => DXGI_FORMAT_R32_FLOAT,
        R32g32Float => DXGI_FORMAT_R32G32_FLOAT,
        R32g32b32Float => DXGI_FORMAT_R32G32B32_FLOAT,
        R32g32b32a32Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
        G8r8G8b8Unorm => DXGI_FORMAT_G8R8_G8B8_UNORM,
        R8g8B8g8Unorm => DXGI_FORMAT_R8G8_B8G8_UNORM,
        Dxt1Unorm => DXGI_FORMAT_BC1_UNORM,
        Dxt3Unorm => DXGI_FORMAT_BC2_UNORM,
        Dxt5Unorm => DXGI_FORMAT_BC3_UNORM,
        Bc4Unorm => DXGI_FORMAT_BC4_UNORM,
        Bc4Snorm => DXGI_FORMAT_BC4_SNORM,
        Bc5Unorm => DXGI_FORMAT_BC5_UNORM,
        Bc5Snorm => DXGI_FORMAT_BC5_SNORM,
        U16v16w16q16Snorm => DXGI_FORMAT_R16G16B16A16_SNORM,
        U8v8w8q8Snorm => DXGI_FORMAT_R8G8B8A8_SNORM,
        U8v8Snorm => DXGI_FORMAT_R8G8_SNORM,
        U16v16Snorm => DXGI_FORMAT_R16G16_SNORM,
        // Known DXGI formats that d3dx10+ explicitly refuses to load.
        B5g6r5Unorm | B5g5r5a1Unorm | B4g4r4a4Unorm => DXGI_FORMAT_UNKNOWN,
        _ => DXGI_FORMAT_UNKNOWN,
    }
}

/// `dxgi_format_from_legacy_dds_d3dx_pixel_format_id`: the `DXGI_FORMAT` of a legacy DDS pixel
/// format. Formats without an exact DXGI equivalent map to `R8G8B8A8_UNORM`.
pub fn dxgi_format_from_legacy_dds_d3dx_pixel_format_id(format: D3dxPixelFormatId) -> DxgiFormat {
    use D3dxPixelFormatId::*;

    match format {
        P8Uint | P8UintA8Unorm | R8g8b8a8Unorm | R8g8b8x8Unorm | B8g8r8Unorm | B8g8r8a8Unorm
        | B8g8r8x8Unorm | B5g6r5Unorm | B5g5r5x1Unorm | B5g5r5a1Unorm | B2g3r3Unorm
        | B2g3r3a8Unorm | B4g4r4a4Unorm | B4g4r4x4Unorm | L8a8Unorm | L4a4Unorm | L8Unorm => {
            DXGI_FORMAT_R8G8B8A8_UNORM
        }
        B10g10r10a2Unorm | R10g10b10a2Unorm => DXGI_FORMAT_R10G10B10A2_UNORM,
        U16v16w16q16Snorm => DXGI_FORMAT_R16G16B16A16_SNORM,
        L16Unorm | R16g16b16a16Unorm => DXGI_FORMAT_R16G16B16A16_UNORM,
        R16g16Unorm => DXGI_FORMAT_R16G16_UNORM,
        A8Unorm => DXGI_FORMAT_A8_UNORM,
        R16Float => DXGI_FORMAT_R16_FLOAT,
        R16g16Float => DXGI_FORMAT_R16G16_FLOAT,
        R16g16b16a16Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
        R32Float => DXGI_FORMAT_R32_FLOAT,
        R32g32Float => DXGI_FORMAT_R32G32_FLOAT,
        R32g32b32a32Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
        G8r8G8b8Unorm => DXGI_FORMAT_G8R8_G8B8_UNORM,
        R8g8B8g8Unorm => DXGI_FORMAT_R8G8_B8G8_UNORM,
        Dxt1Unorm => DXGI_FORMAT_BC1_UNORM,
        Dxt2Unorm | Dxt3Unorm => DXGI_FORMAT_BC2_UNORM,
        Dxt4Unorm | Dxt5Unorm => DXGI_FORMAT_BC3_UNORM,
        Bc4Unorm => DXGI_FORMAT_BC4_UNORM,
        Bc4Snorm => DXGI_FORMAT_BC4_SNORM,
        Bc5Unorm => DXGI_FORMAT_BC5_UNORM,
        Bc5Snorm => DXGI_FORMAT_BC5_SNORM,
        // Known formats that d3dx10+ explicitly refuses to load.
        U8v8w8q8Snorm
        | U8v8Snorm
        | U8v8SnormCx
        | U16v16Snorm
        | U8v8SnormL8x8Unorm
        | U10v10w10SnormA2Unorm
        | Uyvy
        | Yuy2 => DXGI_FORMAT_UNKNOWN,
        _ => DXGI_FORMAT_UNKNOWN,
    }
}
