//! Headless subset of Wine's `dlls/d3dxof`, the DirectX File parser.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`mszip`]: the MSZIP inflater `mszip_decompress` (`mszip.c`), a `CK` signature followed by
//!   deflate blocks that expand within a 32 KiB window.
//! - [`header`]: the 16-byte file header and the chunk walk of `parse_header` (`parsing.c`) for
//!   the compressed formats `bzip` and `tzip`.
//!
//! Skipped: the token lexer and the template parser in `parsing.c`, the COM objects and their
//! reference counts, file mapping and handles, and the DLL entry points.
//!
//! Where Wine reads past the end of its input, writes past the window, or frees a table pointer
//! it never set, the port returns an error instead. Where Wine returns success with bytes it
//! never produced, the port returns [`InflateError::OutputShort`].

pub mod header;
pub mod mszip;

mod huffman;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The failures of [`mszip::decompress`], with the values of Wine's `DECR_*` codes.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum InflateError {
    /// `DECR_DATAFORMAT`: the input or the requested output is past the MSZIP limits.
    DataFormat = 1,
    /// `DECR_ILLEGALDATA`: the stream is malformed.
    IllegalData = 2,
    /// `DECR_INPUT`: the stream ends before its last block. Wine does not report this.
    InputExhausted = 5,
    /// `DECR_OUTPUT`: the stream expands to fewer bytes than requested. Wine does not report this.
    OutputShort = 6,
}

/// `ZIPWSIZE`: the window, which bounds the output.
pub(crate) const WINDOW_SIZE: usize = 0x8000;

/// Refuses output past the window. Wine writes past its buffer here.
pub(crate) fn check_room(out: &[u8], count: usize) -> Result<(), InflateError> {
    if out.len() + count > WINDOW_SIZE {
        return Err(InflateError::IllegalData);
    }
    Ok(())
}
