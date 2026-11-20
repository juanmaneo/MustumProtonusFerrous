//! Headless port of Wine's dlls/itss/lzx.c, the LZX decompressor that CHM files use.
//!
//! Ported:
//! - LZXinit and LZXteardown: [Lzx::new]. Dropping the [Lzx] value frees the state.
//! - LZXreset: [Lzx::reset].
//! - LZXdecompress: [Lzx::decompress], with the bit reader, the canonical Huffman tables, the
//!   verbatim, aligned and uncompressed blocks, the repeated offsets, the window and the E8
//!   translation.
//!
//! Differences from the C code:
//! - The window must be a power of two from 2^15 to 2^21, the sizes LZX allows. The C code
//!   accepts any size.
//! - Input past the end reads as zero bytes, as the oracle harness pads its input.
//! - Cases that C handles with undefined behaviour return an error: an output length above the
//!   window, a match offset above the window, a match or run past the window end, and a Huffman
//!   tree that does not fit its table.
//! - When the output length is larger than the decoded position, the output wraps around the
//!   window. The C code reads before the start of its buffer.
//! - A code length that lzx.c would wrap below zero is an error. Only a run of 19 can produce one.
//! - A Huffman walk through a table that a failed build left is not cut off by the bit count
//!   that READ_HUFFSYM checks. Such a walk still ends at a symbol.
//!
//! Skipped:
//! - The HeapAlloc and HeapFree calls, and the LZX_CHM_TESTDRIVER main.

mod bits;
mod e8;
mod huffman;
mod lzx;
mod slots;

pub use lzx::Lzx;

/// The error codes of lzx.h, DECR_DATAFORMAT and DECR_ILLEGALDATA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecrError {
    /// A match or a run of output would reach past the end of the window.
    DataFormat,
    /// The input does not follow the LZX format, or it is cut short.
    IllegalData,
}

impl DecrError {
    /// The DECR_* value of lzx.h.
    pub fn code(self) -> u8 {
        match self {
            DecrError::DataFormat => 1,
            DecrError::IllegalData => 2,
        }
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
