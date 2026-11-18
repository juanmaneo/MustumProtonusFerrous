//! wmphoto subset for MPF, ported from Wine dlls/wmphoto/jxrlib.c.
//!
//! Ported: the pixel_format_bpp table and its lookup, the stride rounding of
//! copy_pixels, the status mapping of stream Read, Write, SetPos and GetPos,
//! the frame check of wmp_decoder_get_frame_info, and wmp_decoder_initialize
//! over a JxrHost trait that stands in for the JXR decoder calls.
//!
//! Skipped: DllMain, the class factory, create and destroy, stream Close and
//! EOS, copy_pixels and its frame buffer, the palette and metadata stubs, the
//! color context copy, and every COM, IStream, PKImageDecode and wincodecs call.

mod decoder;
mod pixel_format;

#[cfg(test)]
mod oracle_checks;

pub use decoder::{
    DecoderStat, Dimensions, FrameInfo, Initialized, JxrHost, Resolution, WmpDecoder,
};
pub use pixel_format::pixel_format_bpp;

pub const S_OK: i32 = 0;
pub const E_FAIL: i32 = 0x8000_4005u32 as i32;
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
pub const WINCODEC_ERR_WRONGSTATE: i32 = 0x8898_2F04u32 as i32;
pub const WMP_ERR_SUCCESS: i64 = 0;
pub const WMP_ERR_FILE_IO: i64 = -102;

/// A GUID with the field layout of Wine's GUID structure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// Bytes per scan line, (width * bpp + 7) / 8, with the unsigned wrap of the C code.
pub fn frame_stride(width: u32, bpp: u32) -> u32 {
    width.wrapping_mul(bpp).wrapping_add(7) / 8
}

/// Status of a stream Read or Write: file I/O error when the call failed or moved a different byte count.
pub fn transfer_status(hr: i32, count: u32, len: usize) -> i64 {
    if failed(hr) || u64::from(count) != len as u64 {
        WMP_ERR_FILE_IO
    } else {
        WMP_ERR_SUCCESS
    }
}

/// Status of a stream SetPos or GetPos: file I/O error only when the seek failed.
pub fn seek_status(hr: i32) -> i64 {
    if failed(hr) {
        WMP_ERR_FILE_IO
    } else {
        WMP_ERR_SUCCESS
    }
}

/// Only frame 0 exists. copy_pixels treats the S_OK of frame 0 as success and continues.
pub fn frame_index_hresult(frame: u32) -> i32 {
    if frame > 0 {
        E_NOTIMPL
    } else {
        S_OK
    }
}

fn failed(hr: i32) -> bool {
    hr < 0
}
