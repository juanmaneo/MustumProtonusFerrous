//! `SHCreateMemStream` and the `memstream_*` methods in Wine's `dlls/shcore/main.c`.

use crate::stream_open::STGM_READWRITE;
use crate::{
    Hresult, STG_E_INSUFFICIENTMEMORY, STG_E_INVALIDFUNCTION, STG_E_INVALIDPARAMETER, S_FALSE, S_OK,
};

/// `STREAM_SEEK_SET`: the offset is from the start of the stream.
pub const STREAM_SEEK_SET: u32 = 0;
/// `STREAM_SEEK_CUR`: the offset is from the current position.
pub const STREAM_SEEK_CUR: u32 = 1;
/// `STREAM_SEEK_END`: the offset is from the end of the stream.
pub const STREAM_SEEK_END: u32 = 2;

const STGTY_STREAM: u32 = 2;

/// The `STATSTG` fields that `memstream_Stat` fills in.
#[derive(Debug, PartialEq, Eq)]
pub struct Stat {
    pub stream_type: u32,
    pub size: u64,
    pub grf_mode: u32,
}

/// A memory stream, as `SHCreateMemStream` builds it: a byte buffer and a 32-bit position.
///
/// The length never exceeds `u32::MAX`, so every position and size fits the `DWORD` the C code uses.
pub struct MemStream {
    data: Vec<u8>,
    position: u32,
}

impl MemStream {
    /// `SHCreateMemStream`. A missing source gives an empty stream, as Wine does.
    pub fn new(data: Option<&[u8]>) -> Self {
        let data = data.unwrap_or_default();
        let len = data.len().min(u32::MAX as usize);
        Self {
            data: data[..len].to_vec(),
            position: 0,
        }
    }

    /// `memstream_Read`: copies up to `buf.len()` bytes from the position and returns the
    /// `HRESULT` with the byte count. At the end of the stream it returns `S_FALSE` and 0.
    pub fn read(&mut self, buf: &mut [u8]) -> (Hresult, usize) {
        let position = self.position as usize;
        if position >= self.data.len() {
            return (S_FALSE, 0);
        }
        let count = (self.data.len() - position).min(buf.len());
        buf[..count].copy_from_slice(&self.data[position..position + count]);
        self.position += count as u32;
        (S_OK, count)
    }

    /// `memstream_Write`: writes at the position and grows the stream with zeros when the write
    /// runs past its end. Fails with `STG_E_INSUFFICIENTMEMORY` if the end would pass `u32::MAX`.
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Hresult> {
        let end = u64::from(self.position) + buf.len() as u64;
        if end > u64::from(u32::MAX) {
            return Err(STG_E_INSUFFICIENTMEMORY);
        }
        let start = self.position as usize;
        let end = end as usize;
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        self.data[start..end].copy_from_slice(buf);
        self.position = end as u32;
        Ok(())
    }

    /// `memstream_Seek`: moves the position and returns it. The new position keeps the low 32 bits
    /// of the target, as Wine does. An unknown origin fails with `STG_E_INVALIDPARAMETER`, and a
    /// negative or overflowing target with `STG_E_INVALIDFUNCTION`.
    pub fn seek(&mut self, offset: i64, origin: u32) -> Result<u32, Hresult> {
        let base = match origin {
            STREAM_SEEK_SET => 0,
            STREAM_SEEK_CUR => i64::from(self.position),
            STREAM_SEEK_END => self.data.len() as i64,
            _ => return Err(STG_E_INVALIDPARAMETER),
        };
        let target = match base.checked_add(offset) {
            Some(target) if target >= 0 => target,
            _ => return Err(STG_E_INVALIDFUNCTION),
        };
        self.position = target as u32;
        Ok(self.position)
    }

    /// `memstream_SetSize`: resizes the stream to the low 32 bits of `new_size`, filling growth
    /// with zeros. A size of 0 fails with `STG_E_INSUFFICIENTMEMORY` and leaves the stream as it was.
    pub fn set_size(&mut self, new_size: u64) -> Result<(), Hresult> {
        let length = new_size as u32;
        if length == 0 {
            return Err(STG_E_INSUFFICIENTMEMORY);
        }
        self.data.resize(length as usize, 0);
        Ok(())
    }

    /// `memstream_Stat`: a stream of the current length, opened read and write.
    pub fn stat(&self) -> Stat {
        Stat {
            stream_type: STGTY_STREAM,
            size: self.data.len() as u64,
            grf_mode: STGM_READWRITE,
        }
    }
}
