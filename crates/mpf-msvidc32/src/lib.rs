//! Headless port of Wine's `dlls/msvidc32`, the Microsoft Video 1 decoder (CRAM, MSVC, WHAM).
//!
//! The C reference is a `DriverProc` driver. Its messages arrive through ICM, run on driver
//! instances and write through Win32 heaps and resources. The deterministic parts run here on
//! plain byte slices and follow the C control flow, so decoded frames match Wine byte for byte.
//!
//! Ported (names in `dlls/msvidc32/msvideo1.c`):
//! - `msvideo1_decode_8bit`, `msvideo1_decode_16bit`: [decode_8bit], [decode_16bit].
//! - `get_stride`: [get_stride].
//! - `convert_depth`: [convert_depth].
//! - `CRAM_Decompress` and `CRAM_DecompressEx`: [Msvideo1Context::decompress].
//! - `CRAM_DecompressQuery`: [Msvideo1Context::decompress_query].
//! - `CRAM_DecompressGetFormat`: [Msvideo1Context::decompress_get_format].
//! - `CRAM_DecompressBegin`: [Msvideo1Context::decompress_begin].
//! - `CRAM_GetInfo`: [get_info].
//! - The `DRV_OPEN` type check, and the context it creates: [open_accepts],
//!   [Msvideo1Context::opened].
//! - The constant replies of `CRAM_DriverProc`: [driver_proc_constant_result].
//! - `compare_fourcc`: [compare_fourcc].
//!
//! Skipped: `DllMain`; the heap allocation and release of driver instances (`DRV_OPEN` and
//! `DRV_CLOSE`); the `LoadStringW` name and description in `CRAM_GetInfo`, which need resources;
//! `TRACE` and `FIXME` logging; and the `DriverProc` dispatch itself.
//!
//! Deviations from Wine, where Wine has dead code, undefined behaviour or uninitialised reads:
//! - The end-of-stream test `byte_a == 0 && byte_b == 0 && total_blocks == 0` is not ported. Each
//!   visited block consumes one slot, so `total_blocks` is never zero when that test runs.
//! - A pixel buffer must hold every block of the frame before decoding starts. Otherwise the call
//!   returns [IcError::BadSize]. Wine writes past a short buffer instead.
//! - The stream is capped at the bytes supplied and at `biSizeImage`, whichever is smaller.
//! - A context whose depth is neither 8 nor 16 (see [Msvideo1Context::decompress_begin]) makes
//!   [Msvideo1Context::decompress] return [IcError::BadFormat]. Wine decodes it with the 16-bit
//!   decoder into a zero-size buffer.
//! - The scratch buffer for a depth change is zero-filled. Wine's `HeapAlloc` is not, so a stream
//!   that ends early leaves different bytes behind.
//! - A scratch buffer that cannot hold the frame's blocks returns [IcError::BadSize]. Wine writes
//!   past it, as the 5x4 frame at 16 to 24 bits does.
//! - Signed `int` and `LONG` arithmetic wraps instead of overflowing.
//! - A scratch size that is negative returns [IcError::Memory]. A scratch size that cannot be
//!   allocated aborts, as Rust allocation does, rather than returning `ICERR_MEMORY`.
//! - [Msvideo1Context::decompress_get_format] returns the whole header with the output fields set.
//!   Wine copies `biSize` bytes, which include the palette, and the caller copies the bytes.

mod abi;
mod convert;
mod decode;
mod decompress;
mod driver;
mod frame;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use abi::*;
pub use convert::convert_depth;
pub use decode::{decode_16bit, decode_8bit};
pub use decompress::FormatQuery;
pub use driver::{
    driver_proc_constant_result, get_info, open_accepts, IcInfo, InfoResult, Msvideo1Context,
};
pub use frame::get_stride;

/// An `LRESULT`, the reply of a driver message.
pub type LResult = isize;

/// The result of a call that can fail with an ICM error code.
pub type IcResult<T> = Result<T, IcError>;

/// The failures this port reports, each with its `ICERR_*` code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcError {
    /// `ICERR_BADFORMAT`: the compression, depth or output format is unsupported or inconsistent.
    BadFormat,
    /// `ICERR_BADPARAM`: the driver context is missing or has the wrong signature.
    BadParam,
    /// `ICERR_BADSIZE`: a buffer cannot hold the frame it must receive.
    BadSize,
    /// `ICERR_MEMORY`: a scratch buffer has a size that cannot be allocated.
    Memory,
}

impl IcError {
    /// The `ICERR_*` value that Wine returns for this error.
    pub const fn code(self) -> LResult {
        match self {
            Self::BadFormat => ICERR_BADFORMAT,
            Self::BadParam => ICERR_BADPARAM,
            Self::BadSize => ICERR_BADSIZE,
            Self::Memory => ICERR_MEMORY,
        }
    }
}
