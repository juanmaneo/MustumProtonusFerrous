//! `DecideBufferSize` of the ASF reader's output pins, from Wine's `dlls/qasf/asfreader.c`: the
//! buffer count, size and alignment a pin asks its allocator for, raised to the format's size.

/// The `ALLOCATOR_PROPERTIES` fields the decision reads and writes. `cbPrefix` passes through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AllocatorProperties {
    pub buffers: i32,
    pub buffer_size: i32,
    pub align: i32,
    pub prefix: i32,
}

/// The subtype of a `WAVEFORMATEX` stream, as far as the sizing looks at it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subtype {
    Pcm,
    Float,
    Other,
}

/// The format of a stream's media type, with the header field the sizing reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// `FORMAT_VideoInfo`, sized by the `biSizeImage` of its header.
    Video { size_image: u32 },
    /// `FORMAT_WaveFormatEx`, sized by the `nAvgBytesPerSec` of its header when the subtype is PCM
    /// or float.
    Wave {
        avg_bytes_per_sec: u32,
        subtype: Subtype,
    },
    /// Any other format type.
    Other,
}

/// The size used when the format does not give one.
pub const DEFAULT_BUFFER_SIZE: u32 = 16384;

/// The size of one buffer of `format`.
pub fn buffer_size(format: Format) -> u32 {
    match format {
        Format::Video { size_image } => size_image,
        Format::Wave {
            avg_bytes_per_sec,
            subtype: Subtype::Pcm | Subtype::Float,
        } => avg_bytes_per_sec,
        Format::Wave { .. } | Format::Other => DEFAULT_BUFFER_SIZE,
    }
}

/// The properties `DecideBufferSize` passes to `SetProperties`: the count and alignment are raised
/// to at least 1, and the buffer size to the format's size.
///
/// Wine's `max` compares `cbBuffer` with an `unsigned int`, so the comparison is unsigned: a
/// negative request is larger than any size and is kept as it is.
pub fn decide_buffer_size(format: Format, req: AllocatorProperties) -> AllocatorProperties {
    let size = buffer_size(format);
    AllocatorProperties {
        buffers: req.buffers.max(1),
        buffer_size: if (req.buffer_size as u32) > size {
            req.buffer_size
        } else {
            size as i32
        },
        align: req.align.max(1),
        prefix: req.prefix,
    }
}
