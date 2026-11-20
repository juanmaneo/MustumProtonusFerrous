//! The WAVE media sink of Wine's dlls/mfsrcsnk/wave.c, as a byte image.
//!
//! The stream is written at its current position, as IMFByteStream is. The
//! finalize patch writes at fixed offsets and then restores the position, so
//! a sample written after finalize on an empty sink overwrites the start of
//! the stream.

use crate::{HResult, E_INVALIDARG, MF_E_SHUTDOWN, MF_E_STREAMSINK_REMOVED};

/// sizeof(WAVEFORMATEX), which is packed in Wine's mmreg.h.
pub const WAVEFORMATEX_SIZE: usize = 18;

/// The WAVEFORMATEX that the WAVE sink writes into its fmt chunk.
pub struct WaveFormatEx {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

impl WaveFormatEx {
    /// Fills in nAvgBytesPerSec and nBlockAlign from the other fields, as MFCreateWAVEMediaSink does.
    ///
    /// The average is DWORD arithmetic and wraps. The block align is int
    /// arithmetic, which overflows in C only for channels and bits that
    /// multiply to more than INT_MAX. There the port wraps, as gcc does.
    pub fn with_derived_fields(mut self) -> Self {
        self.avg_bytes_per_sec = self
            .samples_per_sec
            .wrapping_mul(u32::from(self.channels))
            .wrapping_mul(u32::from(self.bits_per_sample))
            / 8;
        self.block_align =
            (i32::from(self.channels).wrapping_mul(i32::from(self.bits_per_sample)) / 8) as u16;
        self
    }

    /// The 18 little-endian bytes of the packed structure.
    pub fn to_bytes(&self) -> [u8; WAVEFORMATEX_SIZE] {
        let mut out = [0; WAVEFORMATEX_SIZE];
        out[..2].copy_from_slice(&self.format_tag.to_le_bytes());
        out[2..4].copy_from_slice(&self.channels.to_le_bytes());
        out[4..8].copy_from_slice(&self.samples_per_sec.to_le_bytes());
        out[8..12].copy_from_slice(&self.avg_bytes_per_sec.to_le_bytes());
        out[12..14].copy_from_slice(&self.block_align.to_le_bytes());
        out[14..16].copy_from_slice(&self.bits_per_sample.to_le_bytes());
        out[16..18].copy_from_slice(&self.cb_size.to_le_bytes());
        out
    }
}

struct MemoryStream {
    bytes: Vec<u8>,
    position: usize,
}

impl MemoryStream {
    fn write(&mut self, data: &[u8]) {
        let end = self.position + data.len();
        if end > self.bytes.len() {
            self.bytes.resize(end, 0);
        }
        self.bytes[self.position..end].copy_from_slice(data);
        self.position = end;
    }
}

/// The state and byte image of one WAVE media sink.
pub struct WaveSink {
    fmt: WaveFormatEx,
    stream: MemoryStream,
    riff_size_offset: usize,
    data_size_offset: usize,
    data_length: u32,
    full_length: u32,
    shut_down: bool,
    header_written: bool,
    data_chunk_started: bool,
    data_finalized: bool,
}

impl WaveSink {
    pub fn new(fmt: WaveFormatEx) -> Self {
        Self {
            fmt,
            stream: MemoryStream {
                bytes: Vec::new(),
                position: 0,
            },
            riff_size_offset: 0,
            data_size_offset: 0,
            data_length: 0,
            full_length: 0,
            shut_down: false,
            header_written: false,
            data_chunk_started: false,
            data_finalized: false,
        }
    }

    /// IMFStreamSink::ProcessSample. Writes the header and the data chunk on first use.
    pub fn process_sample(&mut self, data: &[u8]) -> Result<(), HResult> {
        if self.shut_down {
            return Err(MF_E_STREAMSINK_REMOVED);
        }
        if !self.header_written {
            self.write_header();
        }
        if !self.data_chunk_started {
            self.start_data_chunk();
        }
        self.write_data(data);
        Ok(())
    }

    /// IMFFinalizableMediaSink::Shutdown.
    pub fn shutdown(&mut self) -> Result<(), HResult> {
        if self.shut_down {
            return Err(MF_E_SHUTDOWN);
        }
        self.shut_down = true;
        Ok(())
    }

    /// The status that IMFFinalizableMediaSink::BeginFinalize passes to its callback.
    ///
    /// Writes the RIFF size, which is the full length less the 8-byte RIFF
    /// header, and the data length. The second call returns E_INVALIDARG.
    /// The full length wraps when the sink is empty, as the C does.
    pub fn finalize(&mut self) -> Result<(), HResult> {
        if self.data_finalized {
            return Err(E_INVALIDARG);
        }
        let riff_size = self.full_length.wrapping_sub(8);
        self.write_at(&riff_size.to_le_bytes(), self.riff_size_offset);
        self.write_at(&self.data_length.to_le_bytes(), self.data_size_offset);
        self.data_finalized = true;
        Ok(())
    }

    /// The byte image of the stream.
    pub fn bytes(&self) -> &[u8] {
        &self.stream.bytes
    }

    /// The sink's full length, which counts every byte written, including the finalize patches.
    pub fn full_length(&self) -> u32 {
        self.full_length
    }

    /// The number of sample bytes written to the data chunk.
    pub fn data_length(&self) -> u32 {
        self.data_length
    }

    fn write_raw(&mut self, data: &[u8]) {
        self.stream.write(data);
        self.full_length = self.full_length.wrapping_add(data.len() as u32);
    }

    fn write_pad(&mut self, size: u32) {
        self.write_raw(&vec![0; size as usize]);
    }

    fn write_junk(&mut self, size: u32) {
        self.write_raw(b"JUNK");
        self.write_raw(&size.to_le_bytes());
        self.write_pad(size);
    }

    fn write_header(&mut self) {
        self.write_raw(b"RIFF");
        self.riff_size_offset = self.stream.position;
        self.write_raw(&0u32.to_le_bytes());
        self.write_raw(b"WAVE");
        self.write_junk(28);
        self.write_raw(b"fmt ");
        self.write_raw(&(WAVEFORMATEX_SIZE as u32).to_le_bytes());
        self.write_raw(&self.fmt.to_bytes());
        self.header_written = true;
    }

    fn start_data_chunk(&mut self) {
        self.write_raw(b"data");
        self.data_size_offset = self.stream.position;
        self.write_pad(4);
        self.data_chunk_started = true;
    }

    fn write_data(&mut self, data: &[u8]) {
        self.write_raw(data);
        self.data_length = self.data_length.wrapping_add(data.len() as u32);
    }

    fn write_at(&mut self, data: &[u8], offset: usize) {
        let saved = self.stream.position;
        self.stream.position = offset;
        self.write_raw(data);
        self.stream.position = saved;
    }
}
