//! Pixel format and bitmap option checks of `bitmap.c`.

/// `DXGI_FORMAT` values accepted for bitmap formats.
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: u32 = 0x02;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: u32 = 0x0a;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: u32 = 0x0b;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 0x1c;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: u32 = 0x1d;
pub const DXGI_FORMAT_A8_UNORM: u32 = 0x41;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: u32 = 0x57;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: u32 = 0x58;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: u32 = 0x5b;

/// `D2D1_ALPHA_MODE` values.
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: u32 = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: u32 = 2;
pub const D2D1_ALPHA_MODE_IGNORE: u32 = 3;

const D2D1_BITMAP_OPTIONS_NONE: u32 = 0x0;
const D2D1_BITMAP_OPTIONS_TARGET: u32 = 0x1;
const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: u32 = 0x2;
const D2D1_BITMAP_OPTIONS_CPU_READ: u32 = 0x4;
const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: u32 = 0x8;

/// `D2D1_PIXEL_FORMAT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelFormat {
    pub format: u32,
    pub alpha_mode: u32,
}

const SUPPORTED_FORMATS: [PixelFormat; 17] = [
    PixelFormat {
        format: DXGI_FORMAT_R32G32B32A32_FLOAT,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_R32G32B32A32_FLOAT,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_R16G16B16A16_FLOAT,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_R16G16B16A16_FLOAT,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_R16G16B16A16_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_R16G16B16A16_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_R8G8B8A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_R8G8B8A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_STRAIGHT,
    },
    PixelFormat {
        format: DXGI_FORMAT_B8G8R8A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_B8G8R8A8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_B8G8R8X8_UNORM,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
    PixelFormat {
        format: DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        alpha_mode: D2D1_ALPHA_MODE_PREMULTIPLIED,
    },
    PixelFormat {
        format: DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        alpha_mode: D2D1_ALPHA_MODE_IGNORE,
    },
];

/// `format_supported`: whether the format and alpha mode pair is a supported bitmap format.
pub fn format_supported(format: PixelFormat) -> bool {
    SUPPORTED_FORMATS.contains(&format)
}

const VALID_OPTIONS: [u32; 6] = [
    D2D1_BITMAP_OPTIONS_NONE,
    D2D1_BITMAP_OPTIONS_TARGET,
    D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
    D2D1_BITMAP_OPTIONS_TARGET
        | D2D1_BITMAP_OPTIONS_CANNOT_DRAW
        | D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE,
    D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE,
    D2D1_BITMAP_OPTIONS_CANNOT_DRAW | D2D1_BITMAP_OPTIONS_CPU_READ,
];

/// `check_bitmap_options`: whether a `D2D1_BITMAP_OPTIONS` combination is one Wine accepts.
pub fn check_bitmap_options(options: u32) -> bool {
    VALID_OPTIONS.contains(&options)
}
