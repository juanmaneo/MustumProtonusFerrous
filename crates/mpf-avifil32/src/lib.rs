//! Headless port of Wine's `dlls/avifil32`, the AVI file and stream library.
//!
//! Most of the component is COM objects over files, codecs, the registry and dialogs:
//! `IAVIFile`, `IAVIStream`, the edit and get-frame streams, the save dialog and the clipboard.
//! Only its deterministic byte and value logic is ported. The oracle tests compare the port with
//! Wine's C code, built from the same source with the sanitizers.
//!
//! Ported:
//! - `ReadExtraChunk` and `WriteExtraChunk` (`extrachunk.c`): [`ExtraChunks::read_chunk`] and
//!   [`ExtraChunks::write_chunk`].
//! - `AVIFILE_CLSIDFromString` (`api.c`): [`clsid_from_str`].
//! - `AVIStreamSampleToTime` and `AVIStreamTimeToSample` (`api.c`): [`sample_to_time`] and
//!   [`time_to_sample`], over the four fields of [`StreamTiming`] that they read.
//!
//! Where the C code has undefined behavior, the port picks a result:
//! - A zero `dwScale` makes `AVIStreamSampleToTime` divide by zero, which traps in C. The port
//!   returns -1, as it does for an unusable stream.
//! - A float that overflows the `LONG` result saturates, and NaN becomes 0. C's cast is undefined
//!   here; on x86-64 it gives `INT_MIN` and so differs for some inputs.
//!
//! Skipped:
//! - `ReadChunkIntoExtra` and `FindChunkAndKeepExtras` (`extrachunk.c`): they read from an mmio
//!   handle.
//! - `AVIFILE_GetFileHandlerByExtension` and `AVIBuildFilter*` (`api.c`): registry lookups.
//! - The NULL-stream and `IAVIStream::Info` failure checks of the two stream conversions: they
//!   belong to the COM layer that calls [`sample_to_time`] and [`time_to_sample`].
//! - The other `api.c` exports, and `avifile.c`, `getframe.c`, `editstream.c`, `icmstream.c`,
//!   `acmstream.c`, `wavfile.c` and `tmpfile.c`: COM objects over files, codecs and dialogs.
//! - `DllMain`, the class factory (`factory.c`) and the registration script (`avifile.rgs`): COM
//!   and registry.
//! - The `AVIERR_MEMORY` path of `WriteExtraChunk`: a `Vec` aborts on allocation failure instead of
//!   returning an error.

mod clsid;
mod stream_time;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use clsid::{clsid_from_str, Clsid};
pub use stream_time::{sample_to_time, time_to_sample, StreamTiming};

/// An `HRESULT` as its raw 32 bits. Wine's signed `HRESULT` values have the same bits.
pub type HResult = u32;

/// A chunk id: the four characters of a `FOURCC`, read as a little-endian `DWORD`.
pub type FourCc = u32;

/// `AVIERR_NODATA`: the requested chunk is not in the store.
pub const AVIERR_NODATA: HResult = 0x8004_4073;

/// `CO_E_CLASSSTRING`: the CLSID string is malformed.
pub const CO_E_CLASSSTRING: HResult = 0x8004_01F3;

/// Bytes in a record header: the `ckid` and `size` `DWORD`s.
const HEADER_LEN: usize = 8;

/// The extra-chunk block of an `EXTRACHUNKS`, managed by `ReadExtraChunk` and `WriteExtraChunk`
/// (`extrachunk.c`).
///
/// Wine keeps the chunks in one heap block of `(ckid, size, data)` records, each header two
/// `DWORD`s. This port keeps the same records in a `Vec<u8>`, with the `DWORD`s little-endian as on
/// x86.
#[derive(Default)]
pub struct ExtraChunks {
    records: Vec<u8>,
}

impl ExtraChunks {
    /// `WriteExtraChunk`: appends a record holding `data` under `ckid`.
    ///
    /// # Panics
    ///
    /// Panics if `data` is empty, which the C code asserts, or if it is longer than `u32::MAX`
    /// bytes.
    pub fn write_chunk(&mut self, ckid: FourCc, data: &[u8]) {
        assert!(!data.is_empty(), "WriteExtraChunk asserts size > 0");
        let size = u32::try_from(data.len()).expect("chunk size fits in a DWORD");
        self.records.extend_from_slice(&ckid.to_le_bytes());
        self.records.extend_from_slice(&size.to_le_bytes());
        self.records.extend_from_slice(data);
    }

    /// `ReadExtraChunk`: finds the first record with id `ckid`.
    ///
    /// When `dst` is given, copies the first `dst.len()` bytes of the chunk into it. Returns the
    /// chunk's full size, or `AVIERR_NODATA` when no record has that id.
    pub fn read_chunk(&self, ckid: FourCc, dst: Option<&mut [u8]>) -> Result<u32, HResult> {
        let mut pos = 0;
        while pos < self.records.len() {
            let id = read_dword(&self.records[pos..]);
            let size = read_dword(&self.records[pos + 4..]);
            let data = pos + HEADER_LEN;
            if id == ckid {
                if let Some(dst) = dst {
                    let n = dst.len().min(size as usize);
                    dst[..n].copy_from_slice(&self.records[data..data + n]);
                }
                return Ok(size);
            }
            pos = data + size as usize;
        }
        Err(AVIERR_NODATA)
    }
}

fn read_dword(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
