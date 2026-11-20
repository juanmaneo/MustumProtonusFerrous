//! Port of the deterministic logic in Wine's `dlls/cabinet/fdi.c`, the
//! Cabinet Decompression Interface (FDI) that `cabinet.dll` implements.
//!
//! Ported:
//! - [`checksum`]: the checksum of a CFDATA block (`checksum`).
//! - [`huffman`]: the canonical Huffman decode table builder (`make_decode_table`).
//! - [`quantum`]: the QTM model state, its update and the table setup that
//!   `QTMfdi_init` does (`QTMupdatemodel`, `QTMfdi_initmodel`, `QTMfdi_init`).
//! - [`header`]: the CFHEADER parser with its reserved sizes and prev and next
//!   names, `FDIIsCabinet` and `FDI_read_string` (`FDI_read_entries`).
//!
//! Skipped: the LZX setup and decoder and the MSZIP path, so the LZX tables
//! and `make_decode_table` callers are not ported; the QTM window buffer that
//! `QTMfdi_init` allocates; the decoding loops and `FDICopy`; the FDI handle,
//! allocator and file callbacks; and the error bookkeeping in `set_error`.
//!
//! Deviations:
//! - The header parser works on a byte slice, so a name needs its NUL
//!   terminator inside the bytes it is given. Wine reads blocks and can see
//!   stale or zero-filled bytes past a short read; the port reports that as
//!   a corrupt cabinet.
//! - [`checksum::checksum`] accepts any length. Wine's byte count is a 16-bit
//!   `WORD`, so Wine's version wraps at 64 KiB.
//! - [`huffman::make_decode_table`] rejects `nbits` above 15 and more than
//!   65535 symbols. It returns [`huffman::TableError::TableTooSmall`] instead
//!   of writing past the end of the table.
//! - [`quantum::Model::update`] panics when its index is past the sentinel
//!   record, and [`quantum::Model::new`] panics when the symbol range leaves
//!   the 256-entry lookup table. Wine would write past the arrays instead.

pub mod checksum;
pub mod header;
pub mod huffman;
pub mod quantum;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A failed FDI query, with the code Wine reports for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The data is shorter than a CFHEADER or has no `MSCF` signature
    /// (`FDIERROR_NOT_A_CABINET`).
    NotACabinet,
    /// The format version is newer than 1.3 (`FDIERROR_UNKNOWN_CABINET_VERSION`).
    UnknownVersion,
    /// A reserved-size block or a name is truncated (`FDIERROR_CORRUPT_CABINET`).
    CorruptCabinet,
    /// The QTM window size is outside 2^10 to 2^21 (`DECR_DATAFORMAT`).
    DataFormat,
}

impl Error {
    /// The numeric code that Wine returns for this error.
    pub fn code(self) -> u32 {
        match self {
            Error::NotACabinet => 2,
            Error::UnknownVersion => 3,
            Error::CorruptCabinet => 4,
            Error::DataFormat => 1,
        }
    }
}
