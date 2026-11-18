//! Constants and header types from `vfw.h`, `wingdi.h` and `mmsystem.h` that `msvideo1.c` uses.

use crate::LResult;

pub const ICERR_OK: LResult = 0;
pub const ICERR_UNSUPPORTED: LResult = -1;
pub const ICERR_BADFORMAT: LResult = -2;
pub const ICERR_MEMORY: LResult = -3;
pub const ICERR_BADPARAM: LResult = -6;
pub const ICERR_BADSIZE: LResult = -7;

pub const DRV_LOAD: u32 = 0x0001;
pub const DRV_ENABLE: u32 = 0x0002;
pub const DRV_OPEN: u32 = 0x0003;
pub const DRV_CLOSE: u32 = 0x0004;
pub const DRV_DISABLE: u32 = 0x0005;
pub const DRV_FREE: u32 = 0x0006;

pub const ICM_USER: u32 = 0x4000;
pub const ICM_RESERVED: u32 = 0x5000;
pub const ICM_COMPRESS_GET_FORMAT: u32 = ICM_USER + 4;
pub const ICM_COMPRESS_QUERY: u32 = ICM_USER + 6;
pub const ICM_COMPRESS: u32 = ICM_USER + 8;
pub const ICM_COMPRESS_END: u32 = ICM_USER + 9;
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = ICM_USER + 10;
pub const ICM_DECOMPRESS_QUERY: u32 = ICM_USER + 11;
pub const ICM_DECOMPRESS_BEGIN: u32 = ICM_USER + 12;
pub const ICM_DECOMPRESS: u32 = ICM_USER + 13;
pub const ICM_DECOMPRESS_END: u32 = ICM_USER + 14;
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = ICM_USER + 30;
pub const ICM_DECOMPRESSEX_BEGIN: u32 = ICM_USER + 60;
pub const ICM_DECOMPRESSEX_QUERY: u32 = ICM_USER + 61;
pub const ICM_DECOMPRESSEX: u32 = ICM_USER + 62;
pub const ICM_GETINFO: u32 = ICM_RESERVED + 2;
pub const ICM_CONFIGURE: u32 = ICM_RESERVED + 10;

pub const ICTYPE_VIDEO: u32 = u32::from_le_bytes(*b"vidc");
pub const ICVERSION: u32 = 0x0104;
pub const ICINFO_SIZE: u32 = 568;
pub const BITMAPINFOHEADER_SIZE: u32 = 40;
pub const RGBQUAD_SIZE: u32 = 4;
pub const BI_RGB: u32 = 0;

pub const CRAM_MAGIC: u32 = u32::from_le_bytes(*b"CRAM");
pub const MSVC_MAGIC: u32 = u32::from_le_bytes(*b"MSVC");
pub const WHAM_MAGIC: u32 = u32::from_le_bytes(*b"WHAM");

/// `compare_fourcc`: zero when two FOURCCs match, ignoring the ASCII case bits.
pub const fn compare_fourcc(a: u32, b: u32) -> u32 {
    (a ^ b) & !0x2020_2020
}

/// `BITMAPINFOHEADER`, with the field widths of the Win32 structure.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}
